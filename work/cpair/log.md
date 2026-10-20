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
