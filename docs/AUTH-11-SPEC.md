# AUTH-11 — an unfinished chain awaiting a binder is drawn

**Row**: `work/author/an-unfinished-chain-awaiting-a-binder-draws-nothing`
(P1, M, design). Read it in full. Also read AUTH-5's closure on
`path-preview-draws-nothing-for-a-refused-step`, which built
`sketch::prefix_loop`.

**Branch** `author/binder-prefix`. **Never merge; I merge.**

## What a person gets

Mid-way through a fused step, a path preview goes blank. For example,
`fillet_arc` with a radius arrival, before both binders are written.
The steps before it are fine and replay, but the unfinished arm retries
the whole chain under one provisional `line_to Start`. A tip that no
`line_to` can leave (`RadiusArrival`, `RadiusArrivalAt`,
`RadiusArrivalDir`, `Open` after `arc_fillet`, `Angle`) refuses that
retry, and nothing is drawn.

After this unit, the walked-back prefix is drawn as
`LoopEnd::Unfinished`, and the form still says what the tip is waiting
for.

## The design call, decided

The row leaves one call open: the sentence. **Keep the tip-state
sentence beside the drawn prefix.** `PreviewHold::OpenChain`'s "its
last step has to target the start" is false for a tip whose next step
must be a binder, and the end-of-program sentence ("loop 0 never
closes — it ends with the tip a radius arrival still awaiting both
binders…") is the one that is true.
- Carry that refusal typed, e.g. a `PreviewHold` arm holding it, and
  render it through the kernel's own `Display`.
- Advisory tone: it is unfinished, not wrong.
- Don't re-spell the sentence.

If the tree shows this can't be done without re-spelling, say so and
stop.

## Check these first

- **The census of tips no `line_to` leaves.** The row measured five,
  and `ViaArrival` was not measured. Enumerate the tip states from
  `sketch::admits_at` and cover every one that refuses `line_to`, not
  only the five.
- **Whether `prefix_loop` already answers these chains.** AUTH-5 built
  it for refused steps. Reuse it; don't build a second walk-back.
- **What the `Angle` row pins.** `path_authoring.rs`,
  `an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written`.
  It re-pins to a drawn prefix plus the same sentence. Say what moved.

## Traps

- Ten of ten AUTHOR units have minted a fresh duplication while closing
  one. Here the likely culprits are a copied kernel sentence and a
  second walk-back beside `prefix_loop`.
- A composition no test holds is this program's recurring defect.
  Drive the real profile pane (`crate::pane::headless`) and assert what
  it paints, not only `sketch::preview`'s value.

## Scope, verification, deliverable

In: `sketch.rs`, `pane/profile.rs`, `crates/viewer/tests/path_authoring.rs`.
Out: `crates/editor-core`. The tip-state refusal is the kernel's; if it
lacks something, file it.

- Rows that go red:
  - each binder-awaiting tip draws its prefix and shows its own
    sentence;
  - a chain that is merely open still shows `OpenChain`;
  - a refused step still walks back as AUTH-5 built it.
  Name the mutations, restore each from a byte copy, and **touch**
  afterwards.
- Local:
  - fmt;
  - clippy, both feature sets, `-D warnings`;
  - `doc-gate.sh`;
  - every `scripts/gates/*.sh`;
  - `--lib` and `--test all` as separate runs;
  - `work.py lint`;
  - the `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-11-target` on every invocation,
  including excluded roots. Scratch in `/root/auth-11-scratch/`. Wrap
  `cargo` in `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-11: an unfinished chain awaiting a binder is drawn`.
Report to me; don't merge.
