---
id: pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted
kind: issue
title: PR 7131's frame cost at eps 1e-12 is larger than it counted: slow-set rows refuse past its margin range
status: open
opened: 2026-11-22
priority: P1
cost: M
---


**What Ev accepted.** On PR 7131 (comment 7046735324, 2026-11-20 17:54Z) the cost at ε 1e-12 was given as *"about 11–12 rows that certify on main and refuse here … refusing at margins of 1.02–1.32e-12 against the 1e-12 band"*. Ev replied *"yep that's no problem"* at 17:56Z. The later correction to 13 rows, and the merge note's 10, were taken as covered without a reply. The count was taken on the `ci` profile, where the slow set runs at the default ε only (7131's body: "All … at the default ε unless noted"). So no slow-set row was counted at 1e-12.

**What it is.** Two slow rows of `crates/topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs` refuse at 1e-12 since merge `9bc9d6517b`; `e4d679d37e` is green (GREEN PR 7402, bisected):
- `a_curved_junction_certifies_across_the_departure_arm`;
- `a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval`.

Both fail at the same build. `pipe::<Interval>`'s roof ellipse stalls against the cylinder on `ssi_hull_sup`, in the two tilted frames at the pose whose floor is 100 m up the axis:
- `Stalled { operand: Cylinder, cause: Indeterminate { margin: Enclosure { lo: 0.0, hi: 2.193467629756736e-12 }, … predicate: Some("ssi_hull_sup") } }` along `(1, 2, 3)`;
- the same refusal at `hi: 1.4153678229453856e-12` in the far frame.

**2.193e-12 is 1.66× the top of the accepted range.** Under 7131's mutant patch:
- F1X alone builds both;
- A alone narrows them to 1.619e-12 and 1.079e-12, still refusing;
- F1F moves neither.

So the cause is bracket, and A also narrows it.

**Pinned against this row.** PR 7402's `FRAME_COST` pins both refusals at ε ≤ 1e-12, in the test file: windows, cause and mutants are there. The pin goes red when the poses build.

**More of the same window.** Nightly 44989234943 (dispatched on `green/nightly-build` at `1942290ead`, which is main `4d3402ecb1`'s kernel) is the first 1e-12 suite with the slow set since 7131 merged. Against the 11-21 nightly 44870651337 it has 19 new 1e-12 reds and none fixed. Several carry frame-cost-shaped margins (`a_ring_on_a_cone_face`, `verbs_germarms2_interval`, `torus_touch_off_faces`, `union_fold_face_face_touch`, `spheres_crossing_off_every_edge` at 3.277e-12). GREEN's U12 is attributing them, and adds the full count here.

**Reproduce:**

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p topo --ignore-default-filter \
      -E 'test(/a_seamless_pipe_cut_by_an_oblique_plane_measures::a_(curved_junction|seamless_pipe_cut_by_an_oblique_plane_encloses)/)'

**Done when** the counted cost is re-measured with the slow set at 1e-12 and put to Ev as the cost it is. Each row in it is then built at 1e-12 or re-pinned under that count.
(GREEN, U8)
