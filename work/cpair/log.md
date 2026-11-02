# CPAIR — the log

## 2026-10-18 — opened

Opened by FUSE's close (`docs/doc-ledger/fuse-leaves-the-tracker.md`)
for the curve-pair reader's residue: nine rows moved from `work/fuse/`
by `git mv`, ids kept (their `work/fuse/` citations re-pointed). Nothing dispatched; no
orchestrator yet.

## 2026-10-18 — RESTREAD lands a router change in `pose.rs`

Branch `restread/meet-apart-on-one-line` (RESTREAD's
`census-meet-reads-two-segments-on-one-line-two-metres-apart-as-coincident-lines`)
changes `geom_brep::curve_pair::pose`'s router: after the parallel
argument, the gap between the two spans' projections on the line is
read (`curve_pair_pose_gap`), and a gap past the band is `Pose::Apart`.
`overlap` now returns the signed length, and `Route` has an `Apart`
arm. No CPAIR row covered it (the task named FUSE's
`curve-pair-meetings-are-runs`, which no longer exists, and
`curve-pair-polish-meters-refuse-shallow-meetings`, which is about the
polish meters, untouched). Its sweep filed
`the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc` here.

## 2026-10-20 — NURBS build A takes the polish meters

NURBS build A (dispatched by the NURBS orchestrator) claims
`curve-pair-polish-meters-refuse-shallow-meetings`: the basin meter in
`geom_brep::curve_pair::pose` (`Polish::closest`,
`curve_pair_pose_basin`) on shallow meetings with no found meeting
beside them, the needle at ε 1e-12 its first witness. No CPAIR or BAND
branch touches `crates/geom-brep/src/curve_pair/` (checked against every
live remote branch). Plan before code, at the NURBS orchestrator.

## 2026-10-20 — NURBS build A: one reading of two spans on one line (PR 1 of 4)

`two-spans-projections-on-one-line-are-read-four-ways`, staged in four PRs.
PR 1 adds `geom_core::one_line` (`OnOneLine`, `on_one_line`,
`on_one_line_2d`): b's projection on a's axis, read from a's span start,
charged `geom_core::rounding_charge` of the four points' metres plus the
metres the caller computed them at (`produced`; zero for given
vertices). The curve-pair router's line-line gap reads it, passing its
ends' evaluation `|o| + |d|·|t|`. `rounding_charge` and
`SHORT_CHAIN_ULPS` (was `HARMONIC_NOISE_ULPS`) move from
`geom_brep::implicit` to `geom_core`, with every caller on the one path.
Still to come, each on its own program's ground: check 9 (RESTFRONT),
`profile::seg` (PATHS) and the census (topo). — (NURBS build A)

## 2026-10-20 — NURBS build A: the line-line crossing measured, no wrong side

`the-curve-pair-line-line-crossing-is-read-from-the-carriers-origins-uncharged`
was taken as a correctness item: a crossing placed from far carrier
origins could decide a span end's side wrongly. A 40 000-read scan at
ε 1e-12 (shallow crossings, origins to 10⁴ m, ends within 1e-11 … 1e-5
of the exact crossing, both orders) found no wrong answer:
`Span::read` re-reads the candidate's point against each carrier and
end. What the misplacement costs is about 5 200 `curve_carrier_gap`
escalations. The item is downgraded to P3 with the table, and no fix is
built. Ground touched: `work/` only. — (NURBS build A)
- 2026-10-20 — **Seam (NURBS):** PR 5481 merged (`e8912770`). The gap and overlap of two spans on one line now have one home, `geom_core::one_line`, charged for both the projections' rounding and the evaluation of each end (`produced`). `curve_pair::pose::line_line` reads it; `line_conic` keeps its own reading (a different question, filed P3). `rounding_charge` and `SHORT_CHAIN_ULPS` now live in `geom_core`, and `geom_brep::implicit`'s re-export is gone. PRs 2–4 move check 9, `profile::seg` and the census onto the home. (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5477 merged (`6a61ea97`). `curve_pair::pose::window` gains a third on reading, `Window::along_b`: a second-order bound carried along the other conic. Fine-ε fillet joints now decide, and the needle and `leaning_prisms` pass at 1e-12. `curve-pair-polish-meters-refuse-shallow-meetings` keeps the meter's own families. The Interval polish is `curve-pair-polish-at-interval-reads-the-iterates-width` (`design: true`). (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5504 merged (`cbbe917a`, NURBS build A, FULL). `geom_core::one_line::on_one_line`'s charge is now taken at the spans' own scale, `rounding_charge(produced + |a1−a0| + |b0−a0| + |b1−a0|)`, instead of their distance from the origin. The derivation is in the module docs (9u per projection, 10u over the extent, under the shared 16u). Both callers are covered: `curve_pair::pose::line_line` (with `produced` for evaluated ends) and check 9's `lines_meet`, `gaps_apart` and overlap closure. Noise for metre spans 10³ m out falls from ~7e-12 to ~5e-15. A row pins the scaling (red under the old charge). Every caller's fuzz is green with the charge and red without it, and an exact-arithmetic hunt over 120k draws found a worst share of 0.084. No golden moved. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5521 merged (`8f5d93e9`, NURBS build A). It closes `the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin` as not a defect. The witness (ε 1e-12, 20k poses per origin placement, Interval oracle) found 0 unproven Aparts on main or the recipe. Moving the anchor to the span gains 14 marginal poses at the span and loses ~800 of ~9.6k at 1e3 m back, because a span-anchored read charges both ends' evaluations (2|o|) where main's carries |o| once. The only code change is one comment at `pose.rs` `line_conic` naming why. The resolution carries the table; the probe is at 8c00c108. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 3 of 4 (NURBS build A, FULL, `nurbs/seg-one-line`): `profile::seg::line_line` reads `geom_core::one_line` first (`gaps_apart`, `segment_gap`). It is the last of the four readers, so `two-spans-projections-on-one-line-are-read-four-ways` closes with it.
- 2026-10-21 — **Seam (NURBS):** PR 5542's fold puts `seg`'s collinear arm on `geom_core::one_line` too (`collinear_overlap` through `OnOneLine`, charged both ways), so all four readers read the home. A sweep against the old arm finds `lo` bit-identical and the verdict unchanged away from the band edges.
- 2026-10-26 — (tracker sweep) Closed `a-tangency-of-a-spline-edge-runs-the-census-search-past-its-budget` as superseded. `carrier_speed::accel_hi` now reads `NurbsCurve3::accel_upper_bound` (SPLICE PR 5715), and the creased case is `work/splice/a-creased-spline-gets-no-second-order-clearing.md`. Plan slate and program body refreshed (load 28/30). (tracker sweep)
- 2026-11-01 — Note from TANGRIM: claimed `two-coplanar-ellipses-have-no-meeting-method` by `git mv` into `work/tangrim/` and re-banded P1: its open touch bullet blocks tier 3′ on the ellipse-rim unions PR 6246 builds. (TANGRIM orchestrator)
