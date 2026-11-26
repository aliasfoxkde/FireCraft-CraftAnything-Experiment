# Drawing and Editing

## Vector drawing

1. Pick a shape from the **VECTOR** section of the Tools palette (Rectangle,
   Ellipse, Polygon, Star) or press **U**.
2. Drag on the canvas. Auto Shapes (Polygon, Star) show an amber diamond
   handle — drag it to change sides/points and proportions live. The Sides and
   Star values are also editable in the Tools palette under VECTOR.
3. **Line** (**N**) draws straight segments; hold Shift to constrain.
4. **Pen** (**P**) places path nodes click by click; double-click or press
   Enter to finish. **Freeform** (**F**) draws freely along the cursor path.

## Selecting

- **Pointer (V)** selects whole objects; drag a marquee rectangle around
  several, or Ctrl+click to add to the selection.
- **Subselection (A)** dives into a path's nodes; drag a node to reshape.
- **Ctrl+ArrowLeft/Right** cycles the selection through stacked objects.
- **Ctrl+A** selects all; **Esc** deselects; **Ctrl+Shift+I** inverts.

## Bitmap painting

- **Brush (B)** paints with the current stroke color; **[** and **]** change
  brush size.
- **Eraser (E)** removes pixels. **Blur/Sharpen/Dodge/Burn** are local
  retouch tools — hold and scrub.
- **Rubber Stamp (S)** clones: Alt+click sets the source, then paint.
- **Ctrl+E** flattens the selected objects into one bitmap.

## Alignment and distribution

Select multiple objects, then use the **Align** panel: the top-right toggle
switches between *Relative To Object* and *Relative To Canvas*. The Align and
Distribute rows act on the current selection; **Match Size** normalizes
widths/heights.

## Layers

The **Layers** panel (F2) lists layers per page. Objects live on the active
layer; drag rows to reorder z-order, or use **Ctrl+Up/Down** on the canvas.
The eye column toggles visibility, the padlock locks (**Ctrl+L** on
selection).

## Undo

FireCraft keeps per-document undo snapshots: **Ctrl+Z** steps back, **Ctrl+Y**
forward, and the **History** panel (Shift+F10) lets you jump to any step.

Next: [Working with Pages & States](pages-and-states.md).
