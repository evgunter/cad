---
id: a-conics-points-are-restated-in-centre-form-outside-its-evaluator
kind: issue
title: Three sites restate a conic's points in centre form outside its evaluator and agree with it only by bit: an edge's chart image, param_near, and two statements of one circle
status: open
opened: 2026-11-20
priority: P2
cost: H
refs: [the-section-table-admits-a-curve-on-its-resolution, a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the `oneround/anchored-conic` lane (2026-11-20), checking the
verdicts and identities that moved under its variant that evaluates
every conic from a point on its arc
(`analysis/oneround/conic-vertex-probes`,
`analysis/oneround/conic-vertex/phase2-one-chain/`). None of the four
moved verdicts was a false answer. What moved them, and the two
identities, is that three sites compute a conic's points from its
centre outside the conic's evaluator, and today agree with the
evaluator bit for bit because it too sums from the centre:

- **An edge's chart image.** `Pcurve::Harmonic` with `Free { c, a, b }`
  channels (`c + a·cos t + b·sin t` in chart coordinates) for a circle
  on a plane; the angle channels through the surface's evaluator on a
  sphere, cylinder and cone; the cone's `FocalSection`
  (`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`).
  Certification's check 4 meters the carrier against it
  (`geom-brep/src/certify.rs`, `pcurve_map_residual`). The residual is
  exactly zero today wherever the frames are axis-aligned; with the
  conic summed from its anchor it is an ulp of the radius everywhere
  (5.55e-17 and 8.04e-17 at r = 0.5, 2.22e-16 at r = 1).
- **`Curve3::param_near`**, an angle read from `p − centre`.
  `step-import/src/build.rs` `settled_foot` iterates `p ↦
  c.eval(param_at(c, p))`; with the centre form the composition is
  idempotent at the printed points, with the anchored evaluator it
  alternates between two adjacent floats, `settled` takes the cycle's
  least member, and the point settled on is no longer the one the next
  export prints. `freecad::cross_dialect_fixed_point` loses its
  byte-for-byte fixed point by one ulp on `box_fillet_corner`.
- **Two statements of one circle compared bit-equal.** `mesh/src/curved.rs`
  raises `PinchWedge` only when two mesh ids land on a bit-identical
  chart point; `the_one_face_ball::a_slit_between_the_tips_is_not_the_strip`
  states one great circle twice (the revolve's meridian and the row's
  own `Curve3::circle`) and relies on their chord points coinciding. On
  main one of its two interior rows already differs in the last bits.

If Ev approves one stored form for every conic (PR 7194), the unit that
makes every conic anchored owes each site one of two things: it reads
the conic's one form (the image and the inverse anchored as the carrier
is), or it stops relying on bit equality. Until then certificates are
not reproducible by a re-run that lacks the stored image, and the STEP
fixed point is off by an ulp. If Ev declines, the first site still
holds for every anchored-born section (the pcert row is its instance).

The other two moved verdicts, for the unit that re-baselines them:
`ellipse_torus::the_slack_meter_charges_every_term` ("the top speed":
two true roots now certify, 0.000 and 0.013 zero bands from the exact
ones; the pose was tuned to the centre chain's running-error term) and
`touch_spread_rows::the_subdivisions_touch_whose_spread_exceeds_the_band_escalates`
(twice the spread 1.20e-13 against the band's 2e-13, was 5.13e-13; the
fixture needs a lower bend). `section_end_poses::a_torus_section_read_square_refuses…`
(row h0) balances on a margin 2.3e-18 m over ε, and under the variant
refuses `KernelLimit` at limb 2's sup where the built head types
`SquareToAxis`: a worse-typed refusal of the same input, and not from
`RESOLUTION_ROUNDINGS` going 32 → 40 as first reported.

P2 by `work/README.md` Priority (error propagation); H. It rides the
general-anchor unit if PR 7194 is approved.

**Wider than three sites (2026-11-21, from the designers weighing what a reader of a conic's centre reads; by grep and reading on `e04a87227`, nothing run).** `Conic::frame()` hands out a frame whose origin is the centre, and about 110 production reads take that origin and rebuild `centre + size·direction` themselves (covers through `Cover::arc(frame, semis, window)`, which is itself a centre-form copy of the conic; chart images; the inverse; extent bounds). About as many again read only a DIRECTION (`axis`, `u_ref`, `v_ref`) through the same `frame()`, so under a derived centre each of those derives a position it discards. Two shapes follow whatever the stored form turns out to be: a direction read does not go through a position, and a cover takes the conic, not a centre frame. The `Conic` trait's default `unit_in_plane`, `along` and `semi_axis_vectors` are centre-form readings built into the trait. On `oneround/anchored-conic`'s step A, `Curve3::cover_over` builds an anchored cone section's cover from its derived centre frame (being measured).
