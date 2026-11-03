---
id: an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand
kind: issue
title: An angle read clear of its cut, anchored at a seed, is spelled five ways, and Conic::angle_of is written by hand at about seven sites outside geom
status: dispatched
opened: 2026-11-03
priority: P1
cost: M
refs: [6449]
branch: onecopy/angle-reads
---


Found by the FULL review of PR 6449 (`onecopy/foot-and-angle`, style S5 and NOTE-5), at head `5751445aa5`.

## The anchored family

"The angle of an in-plane point, read clear of `atan2`'s cut by anchoring at a seed" is spelled in five places:
- `geom::curves::on_carrier::angle_near` (the caller's branch, half a turn on where the point is behind);
- `geom_brep::curve_pair::pose`'s `separation`;
- `geom_brep::curve_pair::span::Span::branch`, with its own antipode fallback;
- `geom::on_span_branch`;
- `sweep::blend::surgery`'s `past`.

PR 6449 settled `Azimuth` (a windowed read, which keeps `Sym` folding and documents why it does not anchor) against `angle_near`, and kept both. The anchored readers among themselves are the open question. Is there one anchored reading in `geom` that all five call? If so, does it belong on `Azimuth` as an exit, against that type's documented reason, or beside it?

## `Conic::angle_of` by hand

The hand-written `w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref))` reads of a point on a conic appear in production at:
- `topo/src/boolean/rim_wedge.rs` `locus_param` (waits on PR 6443 and stage 3 of the placement decision);
- `topo/src/ring_path.rs` (two; `CircleArc` is being repointed by `the-frame-carries-every-placement-reading`, so take what remains after it lands);
- `sweep/src/blend/surgery.rs`;
- `topo/src/boolean/circle_torus.rs`;
- `mesh/src/walk.rs` (two).

The sweep that found them is `atan2(` alongside `u_ref|v_ref|cross(`, outside geom. It cannot see an angle spelled through local names.

## Related class

A branch or formula tie-break decided at a caller's band, rather than at `Band::least()`, records an escalation the answer does not rest on. PR 6449's fix pass moves the pose's phase read back to `Band::least()` and looks at `angle_near`'s half-turn flip. An anchored home built here should decide its tie-break at `Band::least()`.

## Decided (2026-11-03: three designers, agreed after one round)

The design-fork procedure weighed this (draw on `analysis/design-fork/onecopy-anchored-angle`). All three designers agree on the final state below. They also agree it elaborates `[one-way.readings]` and C4, and restores the exit Ev ratified on PR 5142 (2026-10-15, "an `Azimuth` exit from `atan2` goes through a stated reference": `near(reference)`). Agent PR 5225 removed that exit in `4e96d3a2eb` ("no caller"), and wrote `azimuth.rs`'s no-anchor paragraph without Ev's sign-off. So this lands without an `[ev]` PR.

PR 5142's body describes `near` as "rotate by the reference … no floor and no decision". That is agent wording, approved in passing. The decision it carries is kept: an exit through a stated reference that lands within half a period of it. The mechanism changes, as below, and the unit's PR body quotes the 5142 text and says so.

**Premise correction.** The "five anchored spellings" are three questions:
1. A direction's angle near a seed: `angle_near`, `Conic::param_near`, `Curve3::param_near`'s spiric arm, and surgery's `past`.
2. Moving an angle already held by whole turns: `on_span_branch`, `half_angle::near_midpoint` (which also calls itself "the one home"), `Span::branch`, topo `chart_iso::unwrap_near`, step-import `chart::unwrap`, `rim_wedge::locus_param`'s `floor`, and step-import `normalize`'s `while t <= t0 { t += τ }`.
3. The distance round the turn between two angles: pose's `separation`.

**Final state.**
- **`Period::near(t, reference)`** in `geom_core`, beside `Period::decide`, is the one whole-period rename. It returns `t + k·P` with `k` chosen by deciding the half-period marks at `Band::least()` through `check_unlogged`. It NEVER uses `floor`, `round` or `periodic_branch` on an enclosure, because a floor is a whole turn wide at the middle's antipode. At an undecided mark it takes the upper copy (`r + P/2`): this matches `Window`'s `(a, a + τ]`, the windowed read's own resolution, and `atan2(0, −1) = π`. The side is stated once, here. `Period::decide` stays the logged decision for readers that tell the copies apart. All the question-2 spellings retire into `Period::near`. `Curve3::branch_near` (through `is_periodic`) is one-line sugar for it.
- **`Azimuth::near(reference)`** is the windowed read followed by `Period::near`. It does not rotate, so the reference enters the answer only through the integer `k`, and `k = 0` is the windowed read bit for bit. It is narrow at the window's cut and at the reference's antipode; the designers checked this arithmetic against `stable_azimuth` and `periodic_branch`. That retires `Curve3::param_near`'s "third precondition" and `seam_split_param`'s seam-only anchoring. Question-1 spellings retire into it: `angle_near`, the `param_near` arms, and `past` as `near(t + π)`.
- **Births.** `Conic::azimuth(p)` reads unit coordinates (an ellipse as `(y·a, x·b)`, with no division). `Frame::azimuth(p)` is added once `Frame` has its module (PR 6463). The spiric's tube section is one birth, which also retires the duplicated `(ρ − R, h)` in `Curve3::foot` and `param_near`. **`Conic::angle_of` is retired.** `param_in(p, span)` becomes `azimuth(p).near(mid)`.
- **`geom::separation(θ, at) = atan2(|sin(θ − at)|, cos(θ − at))`** takes no decision at all. Its box lies in the closed upper half-plane, so `Interval`'s `atan2` stays continuous; confirm `interval.rs`'s backend on a `y` lower end of exactly 0 with `x < 0`. Pose's `separation` retires into it.
- **Ledger.** F31 widens to "a renaming between copies equal modulo a period", and `Period::near` and `Azimuth::near` both cite it. `curve_pair_span_branch` (unaudited today) goes.
- **Chart reads keep `principal_in` as it is in this unit**, logged. Whether the chart's cut-side choice (`pcurve_chart_cut_side`) may leave the log is a fact to measure (`sym11`, `m10_10`, `r2_rounded_pad`, revalue, census goldens), and is filed as its own row.
- **Gate.** No `.atan2(` in production code outside `geom::azimuth` and the scalar implementations. It starts from a site table with a class per site, which may only shrink, and allows no in-frame angle class. This is a backstop; the doors above are the canonical reading.
- **Names.** The orchestrator ruled `Period::near`, not `onto`: it pairs with `Azimuth::near`, and the two live on different types.

**Rejected:** a rotated `near`, because it puts the anchor's bits into every answer and keeps the seam precondition; keeping `angle_near` and `Azimuth` both, which leaves two readings of one question; a `geom` free function for the rename, because C4 gives whole-period questions to the channel's `Period`, a spline's knot length included.
