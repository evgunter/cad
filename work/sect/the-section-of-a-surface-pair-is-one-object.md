---
id: the-section-of-a-surface-pair-is-one-object
kind: unit
title: SECT step 1a: the section of a surface pair is one object, built at one door - cylinder x sphere first; the frames, the glue record and the guard read it
status: closed
opened: 2026-10-27
priority: P0
cost: H
branch: sect/section-object
refs: [cylinder-sphere-germ-pair-has-no-join-lane]
closed: 2026-10-27
---


The first half of step 1 (`work/sect/plan.md`, Order). The second half,
the chord lane and island closure, stays on
`cylinder-sphere-germ-pair-has-no-join-lane` (1b), parked on this row.

## The design (two designers, converged, 2026-10-27)

Weighed by one Opus and one Fable designer on one statement of the
problem; both reached the same final state, inside fork row 100, so no
`[ev]` fork. What is settled:

- **Two layers.** A *value* in `geom_brep` (beside `intersect`): the
  section of two distinct surfaces, built by one constructor that is
  the only dispatch on kind pairs. It holds the margins its arm decided
  (`decided`: every Zero verdict that chose the arm, with its margin),
  and its components, each with closure/winding per carrier, a witness,
  and a carrier: a closed form, a fitted NURBS curve with its
  `SsiCertificate`, or none yet (classification only, citing the arm's
  routing note), so pairs the guard certifies today and C5 cannot mint
  keep their classification. A *table* in `topo::boolean`, built once
  per operation at the glue door and held on the reduction, keyed by
  surface pair (two faces on one carrier share the geometry, and chords
  meeting at a vertex then cut the same bits); per face pair it carries
  the verdicts (records, interior-loop verdict). Its readers: the
  coincidence records (replacing `glue::frame_rows`' recomputation),
  the interior-loop guard, the germ mint and the join.
- **The door decides what is not transverse.** A margin decided Zero
  picks the closed-form arm and is recorded as a `Coincidence` from
  `decided` (coaxial → two circles, built as any other section after E);
  a tangency or touch refuses typed toward C7; **a margin in band
  escalates** (closes `in-band-axis-offset-…`); **a radius not definitely
  positive refuses `DegenerateOperand` on every arm**, the section
  table's posture.
- **One pose per pair.** For cylinder × sphere, one pose value (offset
  `d`, foot, the near and far reach) read by C5's arm, by
  `cylinder_sphere_ssi` (handed the decided pose instead of deciding
  tangency again), by `section_cert`'s cylinder × sphere arm, and by the
  table; closes `cylinder-sphere-tangency-is-decided-twice-…`.
- **Matching by foot parameter** on a component: a site's component is
  the one it lies within band of (two is a desync), its foot the nearest
  carrier parameter, its partner the next foot in the direction the germ
  leaves. For cylinder × sphere this retires `cs_pair_frame` and
  `cs_transverse_frame` in this unit; other pairs keep their frames
  until step 3.
- **The two classifiers coexist until step 3.** `section_cert::classify`
  stays authoritative for every pair but cylinder × sphere; its arms
  move into the C5 value arm by arm (torus and cone included) from
  step 3 on.

## Scope of this unit

The value and its cylinder × sphere arm; the table at the door and its
four readers for that pair; matching by foot parameter for that pair;
the two riders closed. The lane still refuses typed at `bool_connect`
(1b builds it), so no body changes; the 54 pinned runs of
`crates/sweep/tests/cylinder_sphere_frame.rs` keep their outcomes and
`every_segment_joins_neighbouring_sites_along_its_loop` is the matching
test. Behaviour changes only by the radius-sign and in-band rulings.

Left to 1b: the chord, its `Pcurve::Projected` pcurves, the
cylinder's arm in `ring_path::Quadric` and a spline arm in `LoopArc`,
and the seam question for a closed fitted loop (seat the loop at its
first site by knot insertion, or periodic NURBS in `geom`; one designer
leaned seat, unsure).

Review: **dual (concurrent pair)** — a new type every later step and
the torus rows build on, hard to change once they do.

## Closed

2026-10-27, by `sect/section-object`. `geom_brep::pair_section` is the
value: one constructor, its kind match exhaustive with no wildcard, the
cylinder × sphere arm built on `CylinderSpherePose` (decided once), and
every other kind pair answered `SectionRead::Elsewhere` with `route`'s
arm. It is classified (`PairSection`, no march) and carried on demand
(`PairSection::carry` → `CarriedSection`, the general rung marched at
`f64` through `AtRestPolicy::ssi_lane`, refused by name at any other
scalar). `topo::boolean::section_table` holds it per surface pair,
built at the glue door (`glue::section_door`) and carried lazily for
the pairs whose germs the join matches; its readers are the
coincidence records, the interior-loop guard, the germ mint and the
matcher, which pairs cylinder × sphere germs by foot parameter.
`cs_pair_frame`, `cs_transverse_frame` and `cs_germ_frame` retired.
The 54 pinned runs keep their refusal at `bool_connect`; three
turned-chart runs name the sphere's other face, the first segment the
join reaches now being found by foot travel. `route` keeps its own
match (`the-c5-kind-dispatch-is-spelled-twice`).
