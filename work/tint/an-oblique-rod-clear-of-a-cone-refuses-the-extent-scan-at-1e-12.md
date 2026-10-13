---
id: an-oblique-rod-clear-of-a-cone-refuses-the-extent-scan-at-1e-12
kind: issue
title: an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms is red at CAD_TOLERANCE_EPS=1e-12 and 1e-6 on main (a rod clear: FallbackExtentUnsupported, near-tangent carriers)
status: open
opened: 2026-10-11
priority: P2
cost: E
refs: [cone-germ-pairs-against-a-curved-face-have-no-section-frame, a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12]
---



Seen by GERM's half-angle meters lane (PR 4600) running
`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep`, and re-run alone
by name on its fix pass; the lane's diff touches no code the row reaches
(it changes comments and `topo` test rows only), so it is main's. The row
is green at the default ε (1e-9) and red, with the same refusal, at both
1e-12 and 1e-6. It was added by GERM's
general-pose cone lane (`crates/sweep/tests/cone_operand_rows.rs`,
`an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms`,
the `a rod clear` fixture, through `common/solid_truth.rs`
`assert_is_but`):

    a rod clear, ∪: wanted Body(1.0511245420135849, 1.0511245420135848e-9),
    refused FallbackExtentUnsupported { operand: A, face: FaceKey(1v1),
    what: "a curved face and a face of the other solid have tangent or
    near-tangent carriers and no crossing layer saw an event; the
    section's component count is not certified there" }

The rod (`r = 0.05`, `h = 0.25`, tilted 0.8 rad at `(0.75, 0.55, 0.1)`)
is clear of the cone with the boxes overlapping; at 1e-12 the extent
scan reads the cone's wall and the rod's carrier as near-tangent and
refuses, and at 1e-6 it refuses the same way, while the default row certifies the clear union. Either the scan's
tangency reading is charging f64 rounding as nearness at this ε (a
kernel lever, the owner of the extent scan's), or the row is
eps-sensitive and wants a loud skip at that row — decide which, as for
`a-far-origin-rod-cut-escalates-its-mapped-source-at-eps-1e-12`.

## 2026-10-13 — still red at both ε on main

The FLUX cylinder-band lane (PR 4839) re-ran the row alone by name on
clean main `e71d719f1`. It fails at `CAD_TOLERANCE_EPS=1e-6` and at 1e-12,
and is green at the default. That lane's diff does not reach the cone
extent scan.
