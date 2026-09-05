# MSOLVE-3 — The mate solve reports the evaluation's own refusal: the `DanglingHead` catch-all closes (spec)

**Program:** MSOLVE (`work/msolve/plan.md`), unit `MSOLVE-3`
(`work/msolve/MSOLVE-3.md`). **Ruling of record:** the MSOLVE
orchestrator, 2026-09-05, as S-MATE's successor, ruling in the
proposal of `work/msolve/mate-dangling-head-is-a-catch-all-that-
reports-a-false-cause.md` — read it in full; its measurement (the mate
fault POISONS the document, so the pattern node never evaluates and
the cause the doc promised "in its own voice" appears nowhere) is the
premise. Rider: `work/msolve/placement-frame-constructor-refuses-on-
the-frame-not-the-axis.md`. **Track:** kernel change — one style
review plus a correctness arm (§Review). No A/B row.

## What the tree says now

`derived_offset` (`mate/solve.rs`, post MSOLVE-1) folds the walk's
chain and, for every placer, evaluates its slots through two local
closures (`scalar`, `triple`) over `crate::expr::eval`, decides its
direction through `crate::eval::unit_direction`, and maps EVERY
failure but one onto `MateFault::DanglingHead { mate, side, head:
placer }` via the `dangling()` closure — a slot that does not
evaluate, a degenerate or non-finite direction (the door's own
`DegenerateDirection`/`NonFiniteDirection { role }` refusals), a count
that does not evaluate, a circular rule whose axis is not an axis
datum, an explicit rule (which the wire itself refuses
`PlacementRule(CountSpelling)`). The one exception is an in-band
decision, escalated to `MateFault::Indeterminate`, correctly. Every
other arm reports "dangling head" for a head that resolves and a
placer that exists, and the true refusal — which the evaluation layer
already types — is discarded at the closure. `head_of`'s and the
variant's docs list the swallowed causes as if they were kinds of
dangling.

Meanwhile `crates/editor-core/src/eval/slots.rs` has `eval_slots(node,
env) -> Result<SlotValues, (SlotId, EvalError)>` — the evaluation's own
door for "this node's slots at these bindings", whose failure IS the
`NodeErrorKind::Expr { slot, source }` the wire raises — and
`slots::{vec3, scalar, count}` read it the way `wire_transform` and the
pattern's stepped-rule builder do.

## What the unit builds

**1. One variant carrying the evaluation's refusal verbatim.**
```rust
MateFault::PlacerRefused {
    mate: RecipeNodeId,
    side: MateSide,
    /// The placer whose pose could not be derived — a pattern or a
    /// transform on the reference's chain.
    placer: RecipeNodeId,
    /// The evaluation layer's own typed refusal for it, unchanged.
    error: Box<NodeErrorKind>,
}
```
It replaces `derived_offset`'s `dangling()` closure everywhere a typed
evaluation refusal exists: `Expr { slot, source }` for a slot or count
that does not evaluate, `DegenerateDirection`/`NonFiniteDirection {
role }` from the direction door (the role word already names the
vector — pattern direction, datum axis direction, transform rotation
axis), `PlacementRule(CountSpelling)` for an explicit rule, and
whatever the wire refuses for a circular rule whose `axis` is not an
`Datum::Axis` (read `wire.rs`'s stepped-operands builder and carry the
same kind). `Indeterminate` stays. What stays `DanglingHead`: the
walk's own refusal (no member: the node the walk stopped at) and a
structural index at or beyond the evaluated count (the name says a
copy that does not exist). Nothing else. No `_ =>` arm remains: a
future `NodeErrorKind` reaching this door is carried, never relabeled.

**2. Slot evaluation through the evaluation's door.** The `scalar`/
`triple` closures go. `derived_offset` evaluates a placer's slots with
`eval_slots(node, &env)` and reads them with `slots::vec3`/`scalar`/
`count` under the same `SlotId`s the wire uses (`Translation`,
`RotationAxis`, `RotationAngle`, `Count`, `Direction`, `Spacing`,
`Step`, …), so a slot refusal here is byte-identical to the one the
node's own evaluation would raise. The circular rule's datum axis
stays re-derived from the datum node's expressions (issue 1570 is
SEAT's; do not migrate it), but through `eval_slots` on the datum node
rather than a private closure.

**3. The docs.** `DanglingHead`'s doc lists exactly its two causes;
`head_of`'s and `derived_offset`'s `# Errors` say which refusal each
door raises; the paragraph in `derived_offset` that records the
poisoning gap (added by PR 1738) is deleted, because the gap is closed.
`crates/editor-core/ASSEMBLY.md` A11 (5): one sentence that a member's
derived pose refuses in the placer's own voice. Present tense.

**4. Consumers.** `MateFault`'s `Display` (the cause, the placer, the
mate); the viewer's `blamed_mates` in `tree.rs` (names the mate) and
its row message (carries the cause — the viewer already renders
`NodeErrorKind`, reuse that rendering); `pncad-py`'s fault projection
in `py/mate.rs` (`mate`, `side` getters gain the arm; a `placer`
getter; the carried `error` exposed through the existing
`NodeErrorKind` mirror — D366's exhaustive projection, so the mirror's
census row and `TAG_INVENTORY` gain `mate_placer_refused`; the `.pyi`).
Grep `DanglingHead` across `crates/` and decide each site.

**5. The rider: the placement constructor decides its axis.**
`Frame::rotate_then_translate` (`placement.rs`) normalizes with a
bare `.normalize()` and lets `SetPlacement` refuse the non-finite FRAME
downstream. It takes a `Band` and decides the axis through
`unit_direction(axis, PLACEMENT_AXIS_ROLE, band)` (a new role constant
beside the three MSOLVE-1 minted), returning `Result<Frame,
NodeErrorKind>`; `DocEdit::SetPlacement`'s door carries that refusal
typed in the axis's voice (an `EditError` arm that wraps it, or the
existing non-finite-frame arm gaining the cause — read the door and
choose the one that does not invent a second spelling). Fifteen call
sites at the spec's writing, nearly all tests: `.expect` where the
axis is a literal. If the door's change reaches CHROME's free-move
tool beyond a `?`, STOP and report.

**6. The rows.** `crates/editor-core/tests/msolve3_placer_refused.rs`
through ordinary doors: the finding's own document (a patterned
instance whose direction slot is `1e200`) refuses `PlacerRefused {
error: NonFiniteDirection { role: "pattern direction" } }` naming the
pattern, while the pattern node reads `Poisoned` — the cause is in the
mate's fault and nowhere else; a zero direction (`DegenerateDirection`);
a slot naming an undefined parameter (`Expr { slot, source }`, slot
named); an explicit rule (`PlacementRule(CountSpelling)`); a transform
with a non-finite axis (`NonFiniteDirection { role: "transform
rotation axis" }`); a circular rule whose axis is a plane datum; an
index at the count still `DanglingHead`; a stranded operand still
`DanglingHead { head: operand }`. A viewer row: the tree row for the
`1e200` document names the direction, not a dangling head. A rider
row: `SetPlacement` with a zero axis refuses naming the axis. Every
existing suite unchanged but the expectations the variant moves (list
them in the PR).

## Acceptance

- **A1** Each swallowed cause above surfaces as `PlacerRefused` with
  the evaluation's own `NodeErrorKind`, byte-identical to what the
  node's evaluation raises for the same slot on an unpoisoned document
  (build the twin document without the mate and compare the kinds).
- **A2** `derived_offset` has no catch-all: every `Err` arm names a
  specific kind, and a grep for `dangling()` finds the two legitimate
  sites only.
- **A3** Slot evaluation in the solve goes through `eval_slots`; no
  private expression-evaluation closure remains in `mate/*`.
- **A4** The viewer and Python surfaces carry the cause (A1's document
  through both); the tag inventory and the mirror census pass.
- **A5** The rider: a degenerate placement axis refuses at
  `SetPlacement` naming the axis and its role.
- **A6** Every existing suite unchanged but the moved expectations;
  the no-refusal solve bit-for-bit (`msolve1`'s `a7`, the `asm_r2a`
  bit rows).

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted
  CI is the verification of record; poll it in the foreground; never
  end a turn with background work active.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools (no `gh`). Private `CARGO_TARGET_DIR` and scratch outside the
  worktree; `git status` before every `git add`; never `git add -A`;
  build narrowly.
- Fence: `crates/editor-core/src/mate/*`, `mate.rs` (the variant and
  its `Display`), `placement.rs` and the `SetPlacement` door in
  `edit.rs` (the rider), `eval/slots.rs` visibility only if `eval_slots`
  must open to `mate/*`, `crates/viewer/src/tree.rs` (the blame arm and
  the row message), `crates/pncad-py` (the projection, tags, census,
  `.pyi`), tests, `ASSEMBLY.md`. Nothing in `eval/wire.rs`, nothing in
  the walk's admission, nothing in `Member`.
- Do not render a `NodeErrorKind` into a string to carry it; do not
  keep a `_ =>` arm; do not migrate the datum-axis road (issue 1570).
- **Stop clause.** If a swallowed cause has NO typed `NodeErrorKind`
  on the evaluation side (so carrying it verbatim would mean minting
  one); if `eval_slots` cannot serve the solve without a `ParamEnv`
  shape the solve does not have; or if the rider's refusal cannot be
  carried through `SetPlacement` without a second spelling of the
  direction refusal — STOP, write what you measured in the PR as a
  draft, and end your turn.

## Out of scope

The two roads to a datum's direction (`work/seat/
direction-normalization-two-doors-one-home.md`); the vocabulary walk
(MSOLVE-2 owns `Member` and the walk); the gate's `Vanished` on a mate
read below a pattern; any change to what is admitted — this unit
changes only what a refusal SAYS.

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1 on the finding's document and on one of your own: the
  mate's fault carries the same `NodeErrorKind` the placer's own
  evaluation raises on the unpoisoned twin; the pattern node reads
  `Poisoned` and the cause is nowhere but the mate's fault.
- **C2** No catch-all remains (A2); slot evaluation is the evaluation's
  door (A3), with no second spelling of a slot read in `mate/*`.
- **C3** The viewer and Python surfaces render the cause (A4); the
  D366 mirror is exhaustive and its census row is present.
- **C4** The rider refuses in the axis's voice at the door (A5) and
  reaches no caller beyond a `?`/`expect`.
- **C5** Bit-for-bit on the no-refusal solve; every existing suite
  unchanged but the moved expectations, each named in the PR.
