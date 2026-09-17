---
id: debug-in-prose-residue-after-finding-sink
kind: issue
title: editor-core - remaining Debug-in-prose debt after the finding sink (D54's successor class)
status: closed
opened: 2026-08-25
github: 985
refs: [984, 981]
branch: edit/error-prose
pr: 2719
closed: 2026-09-16
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

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Built (2026-09-16) — re-measured, repaired, and what is left

Closes at merge. The 2026-08-25 list was re-measured against the tree
before anything was touched; three of its four entries had moved.

**The instrument.** `rg -n ':\?\}' crates/editor-core/src/` plus the
positional `{:?}` form, restricted to `impl Display` bodies by brace
matching from each `Display for` (a 20-line reader, not a regex over the
file), then each hit read against its field's declared type. **63
placeholders in 20 `impl Display` bodies** at the merge base; 51 in 18
after this unit. Cross-checked against `crates/pncad-py/src/prose_census.rs`'s
two rosters, which read the same sites with a real type resolver.

### The old list, entry by entry

- **`EditError` — "~18 arms with `{name:?}`": already closed.** Zero
  hits. That impl's header records the amendment that removed the quotes
  and the category prefix. What it still had was the SLOT id, disclosed
  in the same header as "filed rather than taken here" — this row was
  where it was filed. **Six arms repaired** (`slot.label()`).
- **`PersistError`: repaired, and it was worse than the entry said.**
  `DisplayUnit` rendered `{declared:?}` and `{unit:?}` over
  `crate::expr::Dimension`, which HAS a `Display` that is "the one home
  of the dimension-in-prose rule". Repaired. This also contradicts a
  closed row's closing claim — see below.
- **`NamingError`: not repaired, not EDIT's.**
  `crates/editor-core/src/names/emit.rs` is WIRE's by its explicit path
  list. Its five remaining sites render `topo` keys and a role path
  through `Debug`; the census reaches one of them and calls it
  `<positional>`.
- **`refactor::SplitError` / `InlineError`: not repaired, and not the
  claimed shape.** `crates/editor-core/src/refactor.rs` is FIX's
  territory (`work.py territory`), so the sweep did not edit it — but
  re-reading it, the three sites are `{:?}` over a `String` (`param.0`,
  `key`), not the `StableName` dumps the 2026-08 entry describes.
  `StableName` gained a `Display` in the interim. That is the
  quoting-convention question, filed below.

### Repaired here

| site | payload | was | now |
| --- | --- | --- | --- |
| `edit.rs` `EditError` ×6 | `SlotId` | `{slot:?}` | `slot.label()` |
| `persist/check.rs` `ProgramFault` | `SlotId` | `{slot:?}` | `slot.label()` |
| `persist/check.rs` `ProgramFault` | `StepArg` | `{arg:?}` | `arg.label()` |
| `program.rs` `ProgramRefusal` | `SlotId` | `{slot:?}` | `slot.label()` |
| `resolve/mod.rs` `Diagnosis` | `SlotId` | `{param:?}` | `param.label()` |
| `persist/mod.rs` `PersistError` | `Dimension` ×2 | `{declared:?}` `{unit:?}` | `{declared}` `{unit}` |

Twelve placeholders. `prose_census.rs`'s `KNOWN_BRACED` loses four
entries and `UNDECIDED` one; the rosters compare in both directions,
so the strikes are the rows working. No variant was reshaped — the
tags contract is untouched, and `pncad-py`'s tag tests are green.

### Left standing, each with its reason

- **`eval/mod.rs` `NodeErrorKind` ×2** — WIRE's file. Filed as
  `work/wire/node-error-kind-renders-the-slot-id-through-debug`, and
  `KNOWN_BRACED`'s entry now cites it. This one is a live binding panic,
  not a cosmetic dump.
- **`edit.rs` `EditError` `{path:?}` over `ExprPath`** — a named-field
  struct, so it panics `typed_err`. Already filed as
  `work/lib/the-expression-path-edit-cannot-refuse-as-prose`; repairing
  it needs a prose spelling for an expression path, which is a door
  minted, not a rendering re-pointed. Not this unit.
- **`persist/check.rs` `ProgramFault::Lattice`'s `{verb:?}` `{state:?}`**
  — ratified residue. `work/fix/verb-and-dimension-render-through-debug`
  gave `Verb` a `Display` for the viewer's sentence and kept this pair
  as the transition table's COORDINATE, naming this arm.
- **The `{:?}`-over-`String` class** — `{key:?}`, `{:?}` on a
  `ParamName`'s `.0`, and `PersistError::HeaderId`'s `{found:?}` over a
  truncated raw line, at doors across `persist/`, `range.rs`, `edit.rs`,
  `refactor.rs`, `parse.rs` and `expr.rs`. `Debug` here IS the prose
  plus delimiting quotes; there is no variant identifier and no brace.
  Filed as `work/edit/quoted-parameter-name-in-error-prose-has-no-decision`,
  because the crate spells it both ways and `EditError`'s header says so
  and declines to decide. The hazard the brief names — the day the type
  grows a field — is stated there.
- **The kernel-side notes (`topo`, the census's Rest refutation)** —
  untouched, as the row says; they are recorded on their owners' slates.

### What the sweep could not match

The reader keys on `impl Display` bodies, so it is blind to exactly what
`work/census/prose-census-cannot-see-a-bypassed-prose-renderer`'s Gap 1
names: a wording composed in an INHERENT impl and delegated to by a bare
`{}`. `editor-core` has no measured instance, but the sweep would not
have found one. It is also blind to `Debug` reaching a user through a
`panic!`, a log line or a `#[error(...)]` attribute, and to any
rendering assembled by `format!` outside a `Display` body.

### A correction to a closed row

`work/fix/verb-and-dimension-render-through-debug` closed on 2026-09-11
saying *"Re-swept: no `Dimension` reaches any user surface through
`Debug` anywhere in the tree"*. `PersistError::DisplayUnit` did, at two
placeholders, and had throughout. The row is closed and its defect is
repaired here, so there is nothing to re-open; the instructive part is
that the census could not have disagreed with that sweep — `Dimension`
is fieldless, so the site's verdict is `Prose`. Recorded as evidence on
`work/census/prose-census-cannot-see-a-bypassed-prose-renderer`.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2719 on green CI (run 35047419997, full matrix) and the
orchestrator's read. Residue is in its own files, named in the Built
section above.
