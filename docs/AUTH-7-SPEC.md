# AUTH-7 — a measure shows its value, or says why it has none

**Row**: `work/author/the-gui-shows-no-measure-value-and-no-clearance`
(P0, D). Read it in full.

**Branch** `author/measure-value`. **Never merge; I merge.**

## Why

A document can carry a measure that evaluated successfully, and a person
reading the GUI can't learn what it measured. The feature tree labels
the node `"Measure"` and stops there (`crates/viewer/src/tree.rs:385`,
where the row says `:190`). Nothing in `crates/viewer` reads a
measure's value.

## What is verified (2026-09-30)

- The kernel hands the value out already, through the facade the viewer
  uses (`pncad::document::ValuePayload`,
  `pncad::document::MeasureUnavailableAt`):
  - `ValuePayload::Measure { value, dim }` carries the value in
    canonical units, with the dimension it was measured in.
  - `ValuePayload::MeasureUnavailable { reason, dim }` is a typed
    absence (`crates/editor-core/src/eval/mod.rs` ~:563–590). Today its
    only source is `min_clearance`. A point scalar has no channel for
    that answer, which is an enclosure, so
    `MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door }` names
    the door that can answer. It has a `Display`
    (`crates/editor-core/src/measure.rs` ~:530).
- The row says `clearance` appears nowhere under `crates/viewer/src/`.
  That is now literally false, but the two hits are unrelated: a
  comment in `pane/profile.rs` and a parameter name in
  `pane/properties.rs`. The row's point stands, because nothing consumes
  the clearance engine.

## The unit: the row's first two tiers

1. **A measure that has a value shows it**, along the lines of
   `Measure  distance  12.500 mm`, in the author's units and in the
   right kind for its `dim` (length, angle, scalar).
2. **A measure with no value at this scalar shows the typed absence**
   and the door that can answer, as the kernel words it. Show the
   kernel's `Display` for `MeasureUnavailableAt`; **don't re-spell it**.
   AUTH-6's review found a copied kernel sentence that had already
   drifted.

**Not this unit: tier 3, a clearance consumer** (the verdict, its
witness point, the engine's eleven refusals). The row calls it "a
feature, not a repair", and it needs a design pass. File it as its own
`design: true` row under implementer-discipline §6 on the right program
(`work.py territory` on `crates/editor-core/src/clearance.rs` and the
viewer). Say in the PR where it went.

## Design calls: decide and say

1. **Where the value appears.** The row's cheapest suggestion is beside
   the tree row. The properties panel is another option, and so is
   both. Weigh them and say why. If it appears in two places, it comes
   from one home.
2. **How the number is spelled.** AUTH-2 settled the crate's split:
   `props::render_number` spells a number in an editable field and in
   sentences that quote one, and `readout::number` is the width-bounded
   display. A tree row is width-bounded. Pick the right one and say
   which unit the number is written in: the document parameter's
   display unit, the author's current unit picker, or canonical. **Use
   the crate's existing homes. Don't write a new formatter.** Every
   AUTHOR unit so far has minted one fresh duplication, and it has
   usually been a number or a sentence.
3. **A stale picture.** The tree reads the landed evaluation. When the
   document has moved on and a new evaluation hasn't landed yet, what
   does the measure row show? AUTH-4 shipped, and then had to fix, a
   value measured off a stale picture. Check how the tree already
   treats stale landed values, follow that, and say what you did.

**Assertions**: an `Assertion` node's verdict may be just as invisible.
Check it. If it is, file it rather than fix it, unless the change is
the same few lines as this unit's.

## Check before you build

If anything above is wrong about the tree, say so and don't implement
it. My premises have been falsified often on this program, and that has
been the cheap outcome every time.

## Scope, verification, deliverable

In: `tree.rs`, `pane/properties.rs` and/or `pane/features.rs`, plus
whichever number home you use (`props.rs` / `readout.rs`, read-mostly).
Out: `crates/editor-core`, and the clearance consumer.

- Drive the real panel (`crate::pane::headless` exists) and assert what
  is PAINTED, for three cases: a measure with a value, one with
  `MeasureUnavailable`, and one whose node failed.
- For every stated behaviour, add a row that goes red if the behaviour
  stops being true, and name the mutation. Restore mutations from a
  byte copy (never `git checkout --`) and **`touch`** the restored file,
  because `cp -p` keeps the old mtime and cargo won't rebuild.
- Local: fmt; clippy `--workspace --all-targets --features viewer/app -D
  warnings`; `doc-gate.sh`; every `scripts/gates/*.sh`; `cargo test -p
  viewer --features app --lib` **and `--test all` as a separate run**
  (the lib target's two no-adapter GPU rows stop `cargo test` before
  the integration suite); `work.py lint`; `work.py territory` output in
  the PR.
- `CARGO_TARGET_DIR=/root/auth-7-target` on every invocation, including
  the excluded roots. Scratch `/root/auth-7-scratch/`. Use `cargo` via
  `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: §2 sizes the per-PR gate for latency and the nightly runs the
  rest. Confirm the run's head SHA equals your branch head, read `gate
  ok`, and say what `change filter` selected.

PR titled `AUTH-7: a measure shows its value, or says why it has none`:
the three design calls, territory output, mutations, §6 rows. Report
to me; don't merge.
