---
id: the-sweep-doors-finish-refuses-what-the-box-studies-read
kind: issue
title: The sweep doors' finish refuses at the box scalars what the box studies used to read: Interval NURBS lofts, the macroscopic slab box, the derived-frame boss's plain rungs
status: open
opened: 2026-11-02
---


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

4. **Cost.** The finish at every door is paid in every replay. Rows
   that ran in about a second now take minutes:
   `editor-core` `docm9_range::a_certified_side_stops_at_the_seeds_edge`
   189 s, `a_parameter_field_boxes_directly` 186 s,
   `the_query_does_not_touch_the_input_document` 157 s, and
   `sweep`'s `pinned_corners_at_the_loft_door::an_elbow_builds_from_every_spelling`
   37 s, `revolve_washer::m10_9_…` 33 s,
   `one_segment_loop::a_one_segment_strut_…` 27 s (CI, opt-1). They
   join the slow set in this change. The slow set's own rows that this
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
