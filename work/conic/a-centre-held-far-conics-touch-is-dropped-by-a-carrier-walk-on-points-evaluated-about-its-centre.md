---
id: a-centre-held-far-conics-touch-is-dropped-by-a-carrier-walk-on-points-evaluated-about-its-centre
kind: issue
title: A centre-held far conic's touch is dropped: the carrier walk's clear_by reads the conic clear of its own point at u·far, and line_conic answers Apart
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [7553, 7258]
---


Filed by GERM's `germ/harmonic-belly` lane (PR 7553), from that PR's hosted CI red.

## Finding

A wrong answer, not a refusal, on a conic **held at its centre**.
- **The row.** `geom_brep::curve_pair::far_graze_rows::a_line_grazing_a_far_centred_conic_is_read_wrong_nowhere_past_the_touchs_placement`: a line in the plane of an ellipse with semi-axes `far` and `far/3`, along its tangent at a point at any angle round the turn, set off it within ±2× the escalation, at ε 1e-12.
- **The red.** At far 1e6 and seed `0x310ae3910d809302` (CI, default ε), one pose of 1000 is answered `Apart` where the exact belly on the stored data is +7.5e-13 m. That is inside the zero band: the line touches the conic.

**The trace** (instrumented on the branch, red at `5e75a4afad` and on its pre-merge parent `42bd70f141` alike):
1. `curve_pair::pose::line_conic` reads the near extreme `lo = −7.53e-13`, the belly from `Conic::belly_along`. That is correct and in the zero band, so `Harmonic::Touch` at `θ = 4.728`.
2. `place(θ)`: `level.read` gives floor 0 and run 0, so the reach is `e` = 1e-11. Then `b.clear_by(k.point_at(θ), 1e-11)` on the conic's own span reads **Clear**; `a.clear_by` reads not clear. The line's distance from the point is 1.6e-11.
3. `place` returns `None`, and the pose answers `Apart`.

**The cause.**
- `Span::clear_by` → `Curve3::clear_by` (`crates/geom/src/curves/on_carrier.rs`, `walk_kept`) decides `curve_carrier_piece_clear_by` on each piece's middle point's distance from `q`, less its reach and the radius.
- Both `q` (`k.point_at(θ)`) and the pieces' points are evaluated about the conic's far centre, so each rounds at about `u·far` = 1.1e-10 m at 1e6. That is ten times the radius it is measured against, and the walk decides it as a distance, uncharged.
- So the conic reads clear of a point evaluated on itself.

Ev's ruling on PR 7258 (recorded on the closed `oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`) is that a conic built from a centre is read from its centre and "refuses those poses typed and reads nothing wrong". This pose reads wrong.

**The same exit** carries the arc-held residual that PR 7553's reviewer traced on `work/oneround/an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre.md` (`Harmonic::Touch` → `place` → `None`). This shows it is not only arc-held.

**What PR 7553 does about it.** The belly is right here; the PR does not fix this. The row counts these poses apart as `placed` and does not assert on them, and its doc points here. The belly's own reading of a touch is held without that exception by `curve_pair::pose::far_rows::the_harmonics_near_extreme_reads_a_far_centred_conics_belly`.

## Settles when

A touch on a centre-held far conic is either placed or refused typed, never dropped to `Apart`. The walk's clearance of a point on the conic is read to its own digits, or the walk refuses where the carrier's points cannot resolve the radius. The line row's `placed` count then reads 0 at far 1e4, 1e5 and 1e6 on seed `0x310ae3910d809302` and fresh seeds at three ε, and the row drops its exception.
