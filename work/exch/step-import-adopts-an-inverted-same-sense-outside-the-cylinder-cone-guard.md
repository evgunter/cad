---
id: step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard
kind: issue
title: STEP import adopts an inverted same_sense on any face outside the cylinder/cone wall guard
status: open
opened: 2026-09-20
priority: P3
cost: D
---


Filed by ATREST-2 (measurement unit), outside its fence.

`step-import`'s adoption phase (`adopt.rs`, `adopt_surfaces`) copies
each face's stated `advanced_face.same_sense` into the body verbatim,
through `topo::Body::set_face_sense`. Import's only defence against a
stated inversion is the pre-adoption refusal in `normalize.rs`, whose
own prose names its reach: it fires on **cylinder/cone wall faces**
whose `same_sense` disagrees with the winding of their two rims, and
it justifies itself by saying that adoption "would copy both encodings
verbatim and the kernel's tier-3 curved sense gate (check 6) would
refuse the built body".

That justification does not hold for the face classes check 6 skips,
and those classes are reachable in STEP. ATREST-2 measured the at-rest
battery on a `sweep::loft_body` solid and found all four `Face::sense`
readers gated shut on it:

- check 6's planar arm is `all_lines`-gated, so an arc-bounded planar
  cap is skipped;
- check 6's curved arm skips `Surface::spline_chart()`, so a NURBS
  wall is skipped;
- tier 2's C7 material arm is behind `nurbs_adjacent`;
- check 7's flux is winding-derived on both classes, so the enclosure
  is bit-identical under an inversion.

So a STEP file stating `same_sense = .F.` on every face of a solid
whose caps are arc-bounded planes and whose walls are B-spline
surfaces imports to a body that `validate_geometric` certifies
`Ok(())` with a positive enclosure — import's contract ("import
returns certified bodies") is met while the solid it describes is
inside out. Round-tripping it back out re-emits the inverted bits.

Evidence for the kernel half is pinned in
`crates/sweep/tests/m5_s10_face_sense.rs`
(`only_the_line_bounded_cap_refuses_a_whole_body_sense_inversion`,
`every_sense_reading_gate_shuts_on_the_arc_loft`,
`the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts`)
and written up in
`work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`.
No import-side row exists yet: this one is the import half, and it is
NOT measured — no lane has fed such a file through `step-import`, so
the reachability claim above is derived from the adoption path and the
guard's stated scope, not from a run. The opening step is that run.

Not schedulable against a kernel fix that does not exist: the two
kernel classes are `work/verdict/m6-sense-gate-recorded-residuals.md`'s
residuals 3 and 4, both with flip conditions of their own. What is
EXCH's independently is whether import should derive the material side
rather than adopt it on the classes the kernel cannot check — the
cylinder/cone guard is precedent that import already does so where it
can.
