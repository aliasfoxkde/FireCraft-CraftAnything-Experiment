//! File/session commands: open, save, export, close, and quit choreography.
//!
//! This module is the first slice of the architecture review's command-surface
//! recommendation. UI adapters (menus, shortcuts, toolbars, dialogs) call these
//! methods instead of owning dirty-state and save/close sequencing themselves.
//! Document I/O still goes through `formats`, so tests can exercise the same
//! path without a window.

use super::*;
use std::path::Path;

/// The file/session verbs the UI can request. Kept intentionally small; the
/// implementation owns path selection, dirty state, recent files, and shell
/// notifications.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileIntent {
    Open(PathBuf),
    SaveCurrent,
    SaveAs(PathBuf),
    CloseActiveTab,
    Quit,
}

/// What happened after a `FileIntent`, so adapters do not infer state from the
/// status line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileOutcome {
    Opened,
    Saved(PathBuf),
    WaitingForPath,
    WaitingForCloseConfirmation,
    WaitingForQuitConfirmation,
    Closed,
    Closing,
    Failed(String),
}

impl FileOutcome {
    /// Adapters that only need success/failure (the file dialog) collapse the richer
    /// outcome here instead of matching every state.
    pub fn result(self) -> Result<(), String> {
        match self {
            FileOutcome::Failed(err) => Err(err),
            _ => Ok(()),
        }
    }
}

/// The document I/O boundary. The real implementation delegates to `formats`
/// (disk-backed); tests can substitute a fake to drive open/save/export flows
/// without touching the filesystem or egui.
pub trait FileService: std::fmt::Debug {
    fn open(&self, path: &Path) -> Result<Document, String>;
    fn save(&self, doc: &Document, path: &Path) -> Result<(), String>;
}

#[derive(Debug, Default)]
pub struct FormatFileService;

impl FileService for FormatFileService {
    fn open(&self, path: &Path) -> Result<Document, String> {
        crate::formats::open(path)
    }
    fn save(&self, doc: &Document, path: &Path) -> Result<(), String> {
        crate::formats::save(doc, path)
    }
}

impl FireCraft {
    pub fn run_file_intent(&mut self, intent: FileIntent) -> FileOutcome {
        match intent {
            FileIntent::Open(path) => match self.open_path(&path) {
                Ok(()) => FileOutcome::Opened,
                Err(err) => FileOutcome::Failed(err),
            },
            FileIntent::SaveCurrent => match self.current_path.clone() {
                Some(path) => match self.save_to(&path) {
                    Ok(()) => FileOutcome::Saved(path),
                    Err(err) => FileOutcome::Failed(err),
                },
                None => {
                    self.begin_file_dialog(FileMode::Save);
                    FileOutcome::WaitingForPath
                }
            },
            FileIntent::SaveAs(path) => match self.save_to(&path) {
                Ok(()) => FileOutcome::Saved(path),
                Err(err) => FileOutcome::Failed(err),
            },
            FileIntent::CloseActiveTab => {
                if self.doc.dirty {
                    self.request_close_active_tab();
                    FileOutcome::WaitingForCloseConfirmation
                } else {
                    self.close_document_tab(self.active_doc);
                    FileOutcome::Closed
                }
            }
            FileIntent::Quit => {
                if self.has_unsaved_changes() {
                    self.request_quit();
                    FileOutcome::WaitingForQuitConfirmation
                } else {
                    self.ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    FileOutcome::Closing
                }
            }
        }
    }

    pub fn save_current(&mut self) {
        match self.current_path.clone() {
            Some(path) => {
                if let Err(e) = self.save_to(&path) {
                    self.status = format!("Save failed: {e}");
                }
            }
            None => self.begin_file_dialog(FileMode::Save),
        }
    }

    pub fn save_to(&mut self, path: &Path) -> Result<(), String> {
        self.file_io.save(&self.doc, path)?;
        let pb = path.to_path_buf();
        self.current_path = Some(pb.clone());
        self.doc.file_path = Some(pb.clone());
        self.doc.dirty = false;
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            self.doc.name = name.to_string();
        }
        self.sync_active_doc_slot();
        if !self.recent_files.contains(&pb) {
            self.recent_files.push(pb);
            #[cfg(windows)]
            crate::shell_windows::update_jump_list(&self.recent_files);
        }
        self.status = format!("Saved {}", path.display());
        Ok(())
    }

    pub fn export_to(&mut self, path: &Path) -> Result<(), String> {
        let format = crate::formats::Format::from_path(path);
        if !format.writeable() {
            let msg = format!(
                "{} export is not supported yet; choose PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG or HTML.",
                format.label()
            );
            self.status = msg.clone();
            return Err(msg);
        }
        self.file_io.save(&self.doc, path)?;
        self.status = format!("Exported {}", path.display());
        Ok(())
    }

    pub fn open_path(&mut self, path: &Path) -> Result<(), String> {
        match self.file_io.open(path) {
            Ok(mut doc) => {
                self.sync_active_doc_slot();
                doc.file_path = Some(path.to_path_buf());
                self.open_docs.push(doc.clone());
                self.active_doc = self.open_docs.len() - 1;
                self.touch_document_tab(self.active_doc);
                self.doc = doc;
                self.selected = None;
                self.selected_layer = 0;
                self.undo.clear();
                self.redo.clear();
                self.textures.clear();
                let pb = path.to_path_buf();
                self.current_path = Some(pb.clone());
                if !self.recent_files.contains(&pb) {
                    self.recent_files.push(pb);
                    #[cfg(windows)]
                    crate::shell_windows::update_jump_list(&self.recent_files);
                }
                self.focus_canvas = true;
                self.status = format!("Opened {}", path.display());
                Ok(())
            }
            Err(e) => {
                self.status = format!("Could not open: {e}");
                Err(e)
            }
        }
    }

    pub fn import_path(&mut self, path: &Path) -> Result<(), String> {
        let imported = self.file_io.open(path)?;
        self.push_undo("Import");
        for layer in imported.page().layers.clone() {
            if layer.is_web_layer {
                continue;
            }
            for obj in layer.objects {
                let idx = self.push_object(obj);
                self.selected = Some(idx);
            }
        }
        self.textures.clear();
        self.show_welcome = false;
        self.status = format!("Imported {}", path.display());
        Ok(())
    }

    /// Saves every dirty document that already has a file path. Untitled documents cannot be
    /// saved without asking for a path, so they are reported rather than silently discarded.
    pub fn save_all_for_quit(&mut self) -> Result<(), String> {
        self.sync_active_doc_slot();
        for doc in &mut self.open_docs {
            if !doc.dirty {
                continue;
            }
            let Some(path) = doc.file_path.clone() else {
                return Err(format!(
                    "\"{}\" has no file path yet. Save it before closing.",
                    doc.name
                ));
            };
            self.file_io.save(doc, &path)?;
            doc.dirty = false;
        }
        self.doc = self.open_docs[self.active_doc.min(self.open_docs.len() - 1)].clone();
        self.current_path = self.doc.file_path.clone();
        Ok(())
    }

    pub fn request_quit(&mut self) {
        if self.has_unsaved_changes() {
            self.show_quit_confirm = true;
            self.allow_close = false;
        } else {
            self.allow_close = true;
            self.ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// Whether a close request must be stopped for the Unsaved Changes prompt. A discard
    /// already answered by the user sets `allow_close`, so re-sending Close does not loop
    /// back into the same prompt just because the discarded documents are still marked dirty.
    pub fn should_intercept_close(&self) -> bool {
        self.has_unsaved_changes() && !self.allow_close
    }

    /// The user chose Discard & Quit. Keep the dirty flag only for diagnostics; arm the close
    /// gate so the viewport command is not cancelled on the next frame.
    pub fn confirm_discard_and_quit(&mut self) {
        self.show_quit_confirm = false;
        self.pending_quit_after_save = false;
        self.allow_close = true;
    }

    /// One step of the Save All & Quit flow. Returns `Ok(true)` when a Save As browser was opened
    /// and the caller must wait; `Ok(false)` means every dirty document is saved and the app can
    /// close. Untitled documents are never silently discarded.
    pub fn save_all_and_quit_step(&mut self) -> Result<bool, String> {
        self.sync_active_doc_slot();
        let untitled = self
            .open_docs
            .iter()
            .position(|d| d.dirty && d.file_path.is_none());
        if let Some(idx) = untitled {
            if idx != self.active_doc {
                self.switch_document_tab(idx);
            }
            self.pending_quit_after_save = true;
            self.begin_file_dialog(FileMode::Save);
            return Ok(true);
        }
        self.save_all_for_quit()?;
        Ok(false)
    }

    pub fn request_close_document_tab(&mut self, idx: usize) {
        let is_dirty = if idx == self.active_doc {
            self.doc.dirty
        } else {
            self.open_docs.get(idx).is_some_and(|d| d.dirty)
        };
        if is_dirty {
            self.pending_close_tab = Some(idx);
        } else {
            self.close_document_tab(idx);
        }
    }

    pub fn request_close_active_tab(&mut self) {
        self.request_close_document_tab(self.active_doc);
    }

    /// Close several tabs, one at a time. Clean tabs close immediately; dirty tabs stop the queue
    /// and reuse the normal Save / Don't Save / Cancel dialog so a bulk close never discards work.
    /// Targets are de-duplicated and sorted descending so closing a later tab cannot shift an
    /// earlier target out from under the queue.
    pub fn request_close_document_tabs(&mut self, mut targets: Vec<usize>) {
        targets.retain(|&i| i < self.open_docs.len());
        targets.sort_unstable_by(|a, b| b.cmp(a));
        targets.dedup();
        self.pending_close_tabs = targets;
        self.continue_pending_tab_closes();
    }

    /// Advance the bulk-close queue until it is empty or hits a dirty document. The dialog and the
    /// post-save trampoline call this again once the user resolves that document.
    pub fn continue_pending_tab_closes(&mut self) {
        if self.pending_close_tab.is_some() {
            return;
        }
        while !self.pending_close_tabs.is_empty() {
            let idx = self.pending_close_tabs.remove(0);
            if idx >= self.open_docs.len() {
                continue;
            }
            let dirty = if idx == self.active_doc {
                self.doc.dirty
            } else {
                self.open_docs[idx].dirty
            };
            if dirty {
                self.pending_close_tab = Some(idx);
                return;
            }
            self.close_document_tab(idx);
        }
    }

    pub fn close_document_tab(&mut self, idx: usize) {
        if self.open_docs.len() <= 1 {
            self.close_document();
            return;
        }
        if idx >= self.open_docs.len() {
            return;
        }
        self.sync_active_doc_slot();
        let closing_active = idx == self.active_doc;
        let closing_before_active = idx < self.active_doc;
        self.open_docs.remove(idx);
        self.floating_doc_tabs.retain(|&i| i != idx);
        for i in &mut self.floating_doc_tabs {
            if *i > idx {
                *i -= 1;
            }
        }
        self.doc_tab_history.retain(|&i| i != idx);
        for recent in &mut self.doc_tab_history {
            if *recent > idx {
                *recent -= 1;
            }
        }
        if closing_active {
            self.active_doc = self.active_doc.min(self.open_docs.len() - 1);
        } else if closing_before_active {
            self.active_doc = self.active_doc.saturating_sub(1);
        }
        self.active_doc = self.active_doc.min(self.open_docs.len() - 1);
        self.touch_document_tab(self.active_doc);
        self.doc = self.open_docs[self.active_doc].clone();
        self.current_path = self.doc.file_path.clone();
        self.selected = None;
        self.selected_set.clear();
        self.selected_layer = 0;
        self.textures.clear();
        self.focus_canvas = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("firecraft-files-{}-{name}", std::process::id()))
    }

    /// In-memory `FileService` so open/save flows run without disk or egui.
    #[derive(Debug)]
    struct FakeFiles {
        docs: std::cell::RefCell<std::collections::HashMap<PathBuf, Document>>,
        saved: std::cell::RefCell<Vec<PathBuf>>,
    }

    impl FakeFiles {
        fn empty() -> Self {
            Self {
                docs: std::cell::RefCell::new(std::collections::HashMap::new()),
                saved: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl FileService for FakeFiles {
        fn open(&self, path: &Path) -> Result<Document, String> {
            self.docs
                .borrow()
                .get(path)
                .cloned()
                .ok_or_else(|| format!("no such file: {}", path.display()))
        }
        fn save(&self, doc: &Document, path: &Path) -> Result<(), String> {
            self.docs
                .borrow_mut()
                .insert(path.to_path_buf(), doc.clone());
            self.saved.borrow_mut().push(path.to_path_buf());
            Ok(())
        }
    }

    /// Rc wrapper so the test keeps a handle to the fake behind the box.
    #[derive(Debug)]
    struct FakeHandle(std::rc::Rc<FakeFiles>);

    impl FileService for FakeHandle {
        fn open(&self, path: &Path) -> Result<Document, String> {
            self.0.open(path)
        }
        fn save(&self, doc: &Document, path: &Path) -> Result<(), String> {
            self.0.save(doc, path)
        }
    }

    #[test]
    fn open_and_save_flows_run_through_an_injected_file_service() {
        let mut app = FireCraft::new(None);
        let fake = std::rc::Rc::new(FakeFiles::empty());
        let stored = Document::new(64.0, 64.0);
        let path = PathBuf::from("/fake/scene.fire");
        fake.docs.borrow_mut().insert(path.clone(), stored);
        app.file_io = Box::new(FakeHandle(fake.clone()));

        assert_eq!(
            app.run_file_intent(FileIntent::Open(path.clone())),
            FileOutcome::Opened
        );
        assert_eq!(app.doc.width, 64.0);
        assert_eq!(app.current_path.as_deref(), Some(path.as_path()));

        app.doc.dirty = true;
        assert_eq!(
            app.run_file_intent(FileIntent::SaveCurrent),
            FileOutcome::Saved(path.clone())
        );
        assert!(!app.doc.dirty);
        assert_eq!(fake.saved.borrow().as_slice(), &[path]);
    }

    #[test]
    fn a_failing_file_service_surfaces_errors_without_touching_disk() {
        let mut app = FireCraft::new(None);
        app.file_io = Box::new(FakeFiles::empty());
        match app.run_file_intent(FileIntent::Open(PathBuf::from("/fake/absent.fire"))) {
            FileOutcome::Failed(err) => assert!(err.contains("no such file")),
            other => panic!("expected failure, got {other:?}"),
        }
        assert!(!app.doc.dirty);
    }

    #[test]
    fn save_current_without_a_path_asks_for_one_instead_of_failing() {
        let mut app = FireCraft::new(None);
        app.doc.dirty = true;
        assert_eq!(
            app.run_file_intent(FileIntent::SaveCurrent),
            FileOutcome::WaitingForPath
        );
        assert!(app.doc.dirty, "an unsaved document must stay dirty");
    }

    #[test]
    fn save_as_writes_the_file_and_clears_dirty() {
        let mut app = FireCraft::new(None);
        let path = temp("saved.png");
        app.doc.dirty = true;
        let outcome = app.run_file_intent(FileIntent::SaveAs(path.clone()));
        assert_eq!(outcome, FileOutcome::Saved(path.clone()));
        assert!(!app.doc.dirty);
        assert_eq!(app.current_path.as_deref(), Some(path.as_path()));
        assert!(app.recent_files.contains(&path));
        let bytes = std::fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"\x89PNG"));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn saving_into_a_missing_directory_fails_and_keeps_dirty_state() {
        let mut app = FireCraft::new(None);
        app.doc.dirty = true;
        let outcome = app.run_file_intent(FileIntent::SaveAs(PathBuf::from(
            "/nonexistent-firecraft-dir/out.png",
        )));
        match outcome {
            FileOutcome::Failed(err) => assert!(err.contains("Could not write")),
            other => panic!("expected failure, got {other:?}"),
        }
        assert!(app.doc.dirty);
        assert!(app.current_path.is_none());
    }

    #[test]
    fn opening_a_missing_file_reports_failure() {
        let mut app = FireCraft::new(None);
        let before = app.doc.total_objects_on_page();
        match app.run_file_intent(FileIntent::Open(temp("absent.png"))) {
            FileOutcome::Failed(err) => assert!(!err.is_empty()),
            other => panic!("expected failure, got {other:?}"),
        }
        assert_eq!(app.doc.total_objects_on_page(), before);
    }

    #[test]
    fn closing_a_clean_tab_is_immediate_and_a_dirty_one_asks() {
        let mut app = FireCraft::new(None);
        app.doc.dirty = false;
        assert_eq!(
            app.run_file_intent(FileIntent::CloseActiveTab),
            FileOutcome::Closed
        );

        let mut app = FireCraft::new(None);
        app.doc.dirty = true;
        assert_eq!(
            app.run_file_intent(FileIntent::CloseActiveTab),
            FileOutcome::WaitingForCloseConfirmation
        );
        assert!(
            app.doc.dirty,
            "a dirty document must not be discarded silently"
        );
    }

    #[test]
    fn quit_with_unsaved_changes_asks_first() {
        let mut app = FireCraft::new(None);
        app.doc.dirty = true;
        assert_eq!(
            app.run_file_intent(FileIntent::Quit),
            FileOutcome::WaitingForQuitConfirmation
        );
        assert!(app.show_quit_confirm);
        assert!(!app.allow_close);
    }

    #[test]
    fn save_all_for_quit_refuses_an_untitled_document() {
        let mut app = FireCraft::new(None);
        app.doc.dirty = true;
        assert!(app.doc.file_path.is_none());
        let err = app.save_all_for_quit().unwrap_err();
        assert!(err.contains("has no file path yet"), "{err}");
    }

    #[test]
    fn save_all_and_quit_waits_for_untitled_save_as_then_saves_named_dirty_documents() {
        let mut app = FireCraft::new(None);
        let fake = std::rc::Rc::new(FakeFiles::empty());
        app.file_io = Box::new(FakeHandle(fake.clone()));

        let named_path = PathBuf::from("/fake/named.fire");
        let mut named = Document::new(32.0, 32.0);
        named.name = "named.fire".into();
        named.file_path = Some(named_path.clone());
        named.dirty = true;

        let mut untitled = Document::new(64.0, 64.0);
        untitled.name = "Untitled".into();
        untitled.dirty = true;

        app.open_docs = vec![named, untitled.clone()];
        app.active_doc = 0;
        app.doc = app.open_docs[0].clone();
        app.current_path = Some(named_path.clone());

        assert_eq!(app.save_all_and_quit_step(), Ok(true));
        assert_eq!(app.active_doc, 1, "Save As should target the untitled tab");
        assert!(app.pending_quit_after_save);
        assert!(app.show_file_dialog);
        assert_eq!(app.file_mode, FileMode::Save);

        app.file_dialog_dir = PathBuf::from("/fake");
        app.file_dialog_name = "untitled.fire".into();
        app.commit_file_dialog();

        let untitled_path = PathBuf::from("/fake/untitled.fire");
        assert!(!app.show_file_dialog);
        assert!(!app.pending_quit_after_save);
        assert!(
            !app.should_intercept_close(),
            "all dirty documents must be saved before quit"
        );
        assert!(app.open_docs.iter().all(|doc| !doc.dirty));
        assert_eq!(app.current_path.as_deref(), Some(untitled_path.as_path()));
        assert_eq!(
            fake.saved.borrow().as_slice(),
            &[untitled_path, named_path],
            "save-as should finish before the existing named document is saved"
        );
    }

    #[test]
    fn export_rejects_an_unwriteable_format_without_touching_the_status() {
        let mut app = FireCraft::new(None);
        let err = app.export_to(Path::new("/tmp/out.psd")).unwrap_err();
        assert!(err.contains("PSD"), "{err}");
        assert!(
            app.status.contains("PSD"),
            "status should explain it: {}",
            app.status
        );
    }
}
