---
id: the-transform-gesture-moves-a-placed-body-through-a-transform
kind: issue
title: The viewer's Transform gesture moves a placed body through a Transform; under D10 moving a placed body edits its placement
status: dispatched
opened: 2026-10-09
priority: P1
cost: M
branch: placing/gesture-placements
---


Found by the second review of INTENT stage 2 C (PR #4359, MINOR-D);
kept as built on the orchestrator's ruling, a known interim gap.

`add_transform` (`crates/viewer/src/session.rs`) inserts a `Transform`
of the target and re-points the target's world placements to it
(`feature_over`, residue 1's feature-gesture rule), so a placed body is
moved by a `Transform` under its placement, and its world pose has two
homes: the placement's pose and the `Transform`. Before C, A10's tip
transfer had the same visible effect.

Under D10, moving a placed body edits its placement (Ev, 2026-10-03:
"you can't Transform an already placed part"); that edit arrives with
stage 3's placements (`a-placement-is-the-bundle-of-mates`), and
`Transform` as an operand-maker retires with `[ev]` #4326 ("nothing
moves a body").

Why not changed in C: there is no door that edits a placement's pose in
place, and the gesture's result is read as an operand (the combine
suite's moved second block), which a placement's copy may not be
(`ReadsWorldCopy`). Refusing the gesture on a placed body turned 24
viewer rows red, among them `story_parametric::the_parametric_living_walk`,
and would leave the GUI no way to make a moved body.

## Priced (2026-10-11)

P1 M, parked on INTENT stage 3's `a-placement-is-the-bundle-of-mates`: moving a placed body edits its placement there, and no door edits a placement's pose in place until then.

## Re-parked at INTENT stage 3 B's close (PR 5232, 2026-10-17)

B built placements: `Place { shapes, constraints }`, with a world pin as the constraint that sets a pose. It left `Transform` an operand-maker, and the viewer's mate tool commits into a bundle, so the gesture this row asks for now waits on `Transform` retiring into a placement. That is stage 3's unit D, `transform-retires-into-a-placement`.

## Parked (2026-11-01, PLACING orchestrator)

Parked on the row in `blocked_on`, a live lane rewriting the same gesture code; the reason is in `work/placing/log.md` (2026-11-01, track claimed).

## Unparked and dispatched (2026-11-01, PLACING orchestrator)

`a-new-body-lands-unplaced` closed in PR 6269: no gesture writes a world `Place` on the person's behalf, and one `Takeover` re-points a copy's source. The lane `placing/gesture-placements` re-reads this row against that code first.

## Re-read (2026-11-01, `placing/gesture-placements`, PR 6280) and weighed by three designers

**Still true on main, stated against the code as it now stands.**
- `add_transform` (`crates/viewer/src/session.rs`) no longer inserts a `Node::Transform`. On a copy it commits `Place { source: the copy, target: the copy's space, values }`, then re-points the copy's `⊔` operands, the product slot and its copies onto it (`Takeover`).
- The result is a copy of the copy, and the copy's own bundle is untouched, so its pose has two homes.
- The `Takeover` never re-points a mate's pose or a pick. So parts mated to the moved copy stay where it was (traced, not run).
- The premise above that "no door edits a placement's pose in place" is stale. Value slots (`SlotId::ValueSlide`/`ValueSpin`, written by `SessionOp::SetSlot`) and `SetConstraints` both exist.

**The designers' agreed recommendation.** Transform retires into the placement editor `mate-tool-commits-one-mate` decided:
- **On a copy**, it is the editor's Edit door on the copy's bundle, its value rows as written.
- **On an unplaced body**, it is the Create door with no mates, into the body's own space.
- **The axis-and-angle input** goes.
- **The Create door's takeover** re-points what read the body along the path it was seen through (`⊔` operands, the product), never a copy's source.

**Split, put to Ev:** clarifying `docs/DESIGN.md` `[placement.placement]`. The `[ev]` PR carries it.

**Interim (orchestrator's call):** nothing is built before the editor. Refusing Transform on a copy now would be a stopgap thrown away one unit later, and nothing is released. Once Ev answers, this row rides with `mate-tool-commits-one-mate`.
