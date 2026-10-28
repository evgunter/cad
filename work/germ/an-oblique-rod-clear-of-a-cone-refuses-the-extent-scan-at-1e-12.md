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

## 2026-10-14 — still red at 1e-6 and 1e-12 on main (NURBS, fresh target)

Re-run alone with a fresh `CARGO_TARGET_DIR` per commit: red at `a71403e3` and still red on main at `307ed404`. Not moved by NURBS PR 4984 (red on both sides of it). (NURBS build A)

## 2026-10-15 — still red at 1e-6 on main

Re-run by name at `CAD_TOLERANCE_EPS=1e-6` on main `8dc4a60767` by the FUSE curve-pair reader lane, after the dual review of PR 5154 found it red there: red with the same refusal on main and on the PR head, so it is not the PR's.

## 2026-10-15 — moved from TINT to GERM (CIW orchestrator, at Ev's direction)

TINT has no orchestrator and will not get one soon, so a row left there is not fixed. Nightly 61's `full suite` (run 38687151370) is still red on it. It goes to GERM, which owns the cone operand lanes and wrote the row. The refusal does not look like a test artifact. The rod is unit-scale, so the size-range argument that closed TANG's far-origin row does not apply. The row certifies at 1e-9 and refuses at both 1e-6 and 1e-12, which points at the extent scan's near-tangency reading (`topo/src/boolean/boxes.rs`, `FallbackExtentUnsupported`). GERM decides the kernel-vs-row question the body asks.

## 2026-10-19 — red since the row landed

CONTACTHOLD's nightly-red lane ran it at `7e9d9c8cfe`, the merge of PR 4522 (`germ/cone-pairs-general-pose`), which added it. The row fails there at 1e-6 and at 1e-12 with the refusal above. It is absent at the merge's first parent, so it has been red at those ε since it landed. Still red in the 2026-10-19 nightly (run 39205007024).

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Still red at 1e-6 and 1e-12 with the same refusal. The row is pinned
at both on `a rod clear, ∪: wanted Body`; it goes red the day the row
passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
