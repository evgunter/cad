---
id: debug-in-prose-residue-after-finding-sink
kind: issue
title: editor-core - remaining Debug-in-prose debt after the finding sink (D54's successor class)
status: open
opened: 2026-08-25
github: 985
refs: [984, 981]
---

## From GitHub issue 985

Opened 2026-08-25; 0 comments.

The finding-sink unit (PR #984, #981 part 1) cleaned the finding/refusal surfaces it named (checks, node errors, assembly incl. `ProductError` at the fix pass) and discharged the D54 list. Two sweeps — the implementer's (awk over `impl Display`) and the reviewer's (`{:?}`-shaped grep over all of editor-core/src, which caught the forwarding class the first instrument could not see) — leave this recorded residue, none of it in that unit's scope:

**Document-layer sites still rendering Debug payloads in user-facing prose:**

- `PersistError` — the Snapshot / ProfileProgram / Replay arms Debug-dump their payloads; a `SnapshotError` Display is D54-shaped debt that was never on the D54 list (the smell-scan's D81 family is the adjacent pointer).
- `NamingError` — payload Debug in Display.
- `refactor::SplitError` / `InlineError` — `{name:?}` StableName dumps.
- `EditError` — ~18 arms render `StableName` via `{name:?}` (braces). Deliberately fenced by the module header's identifier-is-the-location argument (edit.rs:590-592); if that fence is kept, these want the `EntityKind::noun` kind+node rendering the sink introduced, applied wholesale rather than per-arm drift.

The natural shape when picked up: the `finding.rs` sink's noun/composition vocabulary already exists; this is application, not design. Each fix is prose-only (no variant reshapes — pncad-py's tags contract).

**Kernel-side notes, out of editor-core's boundary (DS1), recorded for their owners:**

- `topo::ValidationError::UndeclaredContact` renders `CensusContact` via derived Debug (braces reach any document-layer message that forwards this story verbatim).
- `topo` `FIT_DEFERRAL`'s steer contains the literal `` `Fit { gap }` `` — a code-literal naming the vocabulary, arguably legitimate; noted because it means the document layer's `!contains("{")` negative-pin class cannot be applied to kernel-forwarding stories.
- The census's cross-instance Rest refutation renders a NaN/poisoned-enclosure margin as a *contradiction*, which reads as the wrong class (review NOTE-7) — a message-honesty question for the census rung's owner.

Pointers: PR #984 (both sweep patterns and their stated blind spots are in its record), `crates/editor-core/src/finding.rs`, DISCIPLINES-DESIGN DS8/#981.

## Home

A structural prose-debt class (D54's successor), which is the code-quality register's ground: a live finding no row cites is a `kind: issue` file there.
