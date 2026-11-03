---
id: the-sweep-doors-finish-refuses-what-the-box-studies-read
kind: issue
title: The sweep doors' finish refuses at the box scalars what the box studies used to read: Interval NURBS lofts, the macroscopic slab box, the derived-frame boss's plain rungs
status: closed
opened: 2026-11-02
closed: 2026-11-02
pr: 6371
---

**Closed by a ruling and a split** (Ev on PR 6371, 2026-11-02: "keep
aiming for this final state, but it seems like it might be necessary to
keep it disabled/selectively disabled for now so it doesn't break
everything. we should prioritize making it actually work for all these
cases, though, with the common cases as P0 (nurbs might be P1)").

PR 6371 switches the sweep doors' own finish off at the box scalars
(`topo::AtRestPolicy::leaf`), which restores main's behaviour at those
scalars for effects 1–4 and 6 below. Each effect now has its own row:

- 2, 3, 4 (the side reader over a box):
  `the-side-reader-cannot-read-a-solids-orientation-over-a-box` (P0);
- 6 (the cost): `tier-3-at-every-box-replay-leaf-multiplies-the-decisions` (P0);
- 5 (the `f64` losses, not behind the switch):
  `f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves` (P0);
- 1 (NURBS quadrature):
  `check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends` (P1);
- the deletion of the switch: `delete-the-box-scalar-sweep-leaf-switch`.

The finding as filed follows.


Found by the ONEHOME finished-body lane (branch `onehome/finished-body`),
measured on its full suite. **This needs a ruling**: it is the price of
the decision on `the-at-rest-result-gate-and-its-typing-live-in-five-homes`
("the sweep doors finish at their own end"), paid at the enclosure and
box scalars, which nothing in that decision weighed.

The sweep doors (extrude, revolve, loft, sweep, fillet, chamfer) now
finish their result at their own end, through the scalar's `finish`. At
`f64` that changes nothing a suite measured. At `Interval` and `Sym` the
finish is the certifying tier 3, and it now runs on every leaf body a
document builds, where before only a consumer that needed a finished
body ran it (a Boolean's operand, the product). Three measured effects:

1. **`Interval` lofts and sweeps over NURBS walls do not finish.**
   Check 7's quadrature refuses a NURBS wall whose pcurve endpoints are
   not exact structure (`QuadratureUnsupported`: "the rectangle-trim
   certificate needs the minted exact 0/1 chart values"), so the door
   refuses with `LoftError::NotFinished { VolumeUncomputable }`. Every
   `Interval` loft in the sweep suites hit it. The suites that read the
   walls' enclosures now read the door's construction state through
   `sweep::test_support::{loft_body,loft_profiles,sweep_body}_construction`.
   The document row
   `editor-core/tests/eval10_section_reads_the_nominal.rs::a_box_over_a_section_width_lofts_the_family_it_spans`
   now pins the refusal: a boxed loft document no longer evaluates at
   `Interval`.
2. **The macroscopic slab box no longer certifies.**
   `editor-core/tests/m10_3_driver_interval.rs::a_macroscopic_box_certifies_at_the_shipped_depth_budget`
   (±0.05 on a 1.0 slab) certified in one leaf. Now the extrude's finish
   cannot decide the slab's orientation over the box ("6 of the 6 rays
   aimed at its faces read nothing"), and the drive bisects until the
   32-leaf budget is spent: 0 certified, every refusal `Budget(Leaves)`.
   The row pins that as it stands.
3. **The derived-frame boss's plain and A0-alone rungs refuse.**
   `m10_derived_frame_interval.rs::m10_the_derived_frames_refusal_is_not_a_freeze`
   at 5e-2: both rungs refuse at the extrude's finish, on the same
   unread orientation, and so do the shipped rules
   (`m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width`,
   a slow-set row), where the authored twin still certifies. The rows
   pin the refusal.

4. **The tour's tolerance stop certifies nothing.** `demos/tour`'s
   `demo-tour certified` (`src/tolerance.rs`, the real study) now
   reports certified 0.0000%, refused (budget) 99.4607%, and panics by
   design ("the real study certified nothing"), which reds
   `eps_regression::certified_cells_run_green_at_default_eps`. This
   branch leaves that red standing: making it green means either
   weakening the stop's own assertion or deciding the fork below.

5. **`f64` capability losses** (the door now refuses what it used to
   return unfinished; each is pinned where a test asserted a body):
   - `f64`, ε = 1e-12: `sweep/tests/verbs_tubewall_r1_probes.rs`
     (`closed_forms_at_varied_geometry`,
     `the_cavity_is_a_two_wall_torus_at_the_inner_bits`). The hollow
     tube R 1300, ro 2.5, wall 0.001, full period, refuses at the
     revolve's finish: `VolumeUncomputable` at `props_rim_span_winding`,
     margin [0, 1.15e-12], in band.
   - `f64`, default ε: `sweep/tests/bool5r2_wedge_reach.rs` (the hairline
     revolve of a dimpled ring) and `bool5r1_probes.rs` (the dimple lune
     across the slit window). The revolve's finish refuses: tier 3's
     full walk, `VolumeUncomputable` / the knife-edge caps. Those probes
     now read the door's construction state, because the props door's
     refusal of the body is their subject.
   - `f64`, untested: CARVE's measured witness (a tilted 1 × 1 profile
     with a 200–400 m arc side, extruded 0.7, ε = 1e-12) refused tier 3
     in 4 of 20 poses, so the extrude now refuses there.

6. **Cost.** The finish at every door is paid in every replay. Rows
   that ran in about a second now take minutes:
   `editor-core` `docm9_range::a_certified_side_stops_at_the_seeds_edge`
   189 s, `a_parameter_field_boxes_directly` 186 s,
   `the_query_does_not_touch_the_input_document` 157 s, and
   `sweep`'s `pinned_corners_at_the_loft_door::an_elbow_builds_from_every_spelling`
   37 s, `revolve_washer::m10_9_…` 33 s,
   `one_segment_loop::a_one_segment_strut_…` 27 s (CI, opt-1). On main
   each ran under the 1.0 s budget. In an earlier CI run on this branch
   the `docm9_range` rows took 253–299 s, `driving_writes_nothing` 47 s
   and `the_answer_names_the_parameters_it_pinned` 37 s. The cause is
   the certifying finish at every leaf, in every box-scalar replay. The
   m10_8 slab drive's decision counts measure it: plain
   2898/1612 → 12642/8573 (symbolic/numeric), about 4.4× the decisions
   per replay. The rows join the slow set so the gate can run. That
   placement is disclosed here, not a fix, and the cost stands for the
   fork. The slow set's own rows that this
   branch did not touch were not run here: the nightly runs them.

The decision counts the box studies pin also moved, because the
finish's battery now decides in every replay:
`m10_8_the_shipped_set_is_inert_on_straight_geometry` (plain
2898/1612/0 → 12642/8573/36), the boss's nominal split in
`m10_bulge_interval.rs`, `m10_10`'s walked-row envelope 24 → 32, the
cert-key golden `tests/golden/m10_6_certifying_keys.txt`,
`kstats_bracket_rows`' extrude log 679 → 1556, and `asm2a`'s part total
834 → 1711. Those re-pins are bookkeeping. The three effects above are
not.

The fork: either a box study accepts that a door it evaluates certifies
its result at the box scalar, and the side reader and check 7's
quadrature learn to read these bodies over a box. Or the finish at an
enclosure scalar is weaker than tier 3 for a leaf no consumer reads (a
`Replay`-like type at `Sym`/`Interval`). That would re-open
`[topology.tiers]`/`[scalars]` and is Ev's call.
