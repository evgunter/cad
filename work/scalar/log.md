# SCALAR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/scalar/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, all seven from `work/code-quality/` — this is the one track
the cut built entirely out of that directory.

It is small on purpose. The scalar rows that already had a fix written
(`placement-lifts-its-affine-by-hand-beside-affine3-map`, the two
profile lift doors) went to the seat's successor WIRE and to DOOR, where
their class puts them; what is left here is what actually needs the
substrate decided first.

No branch exists yet. The first act is an `[ev]` PR carrying `D6`,
`D283` and the unit-vector question as one conversation.

## Orchestrator seated; the eighth row on the table (2026-09-12)

A SCALAR orchestrator is seated (remote box; branch prefix `scalar/`,
orchestrator branch `scalar/orchestrator`). DOOR re-homed
`curve3-eval-and-deriv-at-one-t-run-two-basis-passes` here on
2026-09-12 after the plan's slate table was written; the table now
carries it at class **M** beside `S393`, and the program text counts
eight rows. Sequencing, per the plan and Ev in-chat (2026-09-12):
the `[ev]` PR carrying `D6`, `D283` and the unit-vector question is
drafted first; `D290` dispatches beside it rather than behind it;
`H5`'s own questions (Q1, RingInterval) go to a SECOND `[ev]` sitting
once the door rows are in and its decomposition is cut.

## D290 and S393 dispatched; the seams announced (2026-09-12)

Both door rows are in flight on their own branches with a spec each
(`docs/D290-SPEC.md`, `docs/S393-SPEC.md`), block SCALAR-B1 slots 0 and
1; the block record is branch-side per the A/B log's redaction shape.

**Seams, announced here and on each PR when it opens.** `D290` reaches
PROPS' `crates/geom-core/src/spline/knots.rs`, `crates/geom/src/curves/nurbs.rs`
and `crates/geom-brep/src/offset_fit.rs`, and TRIM's
`crates/geom-brep/src/edge_nurbs.rs` — one `KnotVector` rescale door
with exact pinned ends, replacing the private `offset_fit::rescaled_knots`
and the inline map in `edge_nurbs::on_carrier_domain` (which does not
pin its ends, so its image domain can sit an ulp off the carrier
interval — the one behaviour change, argued in the spec). `S393`
reaches S-TCOST's and S-TINT's `crates/sweep/tests/*`, BLEND's
`crates/sweep/src/skin.rs` (docs only) and `demos/tour/src/skinned.rs`.

**S393's premise corrected before dispatch.** The row says no public
door hands out the start frame. `geom_core::linalg::frame::path_start_frame`
does, is public, and is already bound into Python; the unit is the two
copies going onto it, with the one semantic difference (a hard 0.9
helper cone against the door's decided reference ladder) measured
fixture by fixture. Class corrects M → E in the plan table via the PR.

**The third door row waits on D290.** The v-reversal door on
`NurbsSurface` needs the same exact-ends argument for a REFLECTED knot
vector (`k ↦ lo + hi − k`) that D290 makes for a rescaled one; the test
that rebuilds the net carries the knots verbatim, which is the same
point set only when the v knots are symmetric — a door has to say what
it does when they are not.
