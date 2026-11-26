# Contributing

## Before you start

- Read [CLAUDE.md](../CLAUDE.md) for the agent/human working contract, [docs/architecture.md](../docs/architecture.md)
  for seams and [GLOSSARY.md](../docs/GLOSSARY.md) for vocabulary.
- Open items are in [docs/TASKS.md](../docs/TASKS.md). Tasks without proof are not done.

## Workflow

1. Branch from `main`.
2. Make one focused change. Keep `app/Cargo.toml` the only version source; never hardcode a version.
3. Run the gates:

   ```sh
   ./scripts/ci-local.sh
   ```

   The quick subset (`--quick`) is fmt, clippy, tests and the docs check.
4. Add a test at the seam you changed. If the behavior cannot be tested, say why in the PR.
5. Update affected docs and add a line to [docs/PROGRESS.md](../docs/PROGRESS.md) when the change is
   substantial.
6. Commit with a conventional message, e.g. `fix(canvas): clip radial gradients to the object`.

## Invariants

- No placeholders, stubs, `todo!()`, or simulated output presented as real behavior.
- No new dependency without a reason recorded in the PR.
- `app/MECHANICS.md` must agree with `app/mechanics.json` and `app/src/constants.rs`; the docs gate
  checks the headline values.
- `unsafe` only in `app/src/shell_windows.rs`.
- Never commit build output (`app/target/`, `app/dist/`, `app/shots/`).

## Documentation

`./scripts/check-docs.sh` fails on broken relative links and hardcoded version literals. Update the
docs index ([docs/README.md](../docs/README.md)) when you add a document.
