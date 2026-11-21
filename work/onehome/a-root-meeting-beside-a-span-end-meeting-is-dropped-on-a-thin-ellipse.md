---
id: a-root-meeting-beside-a-span-end-meeting-is-dropped-on-a-thin-ellipse
kind: issue
title: A root's meeting 1.4e-9 m from a span end's meeting on a thin ellipse is dropped, one meeting answered for two
status: closed
opened: 2026-11-20
priority: P1
cost: M
closed: 2026-11-21
---


Filed by ONEHOME from PR 7208's dense-oracle probe. It is **on main
too** (`c8c00a2a33`, the same answer), so it is not 7208's. The
priority is soundness: a missed meeting is a wrong decided answer.

**The answer.** An ellipse of semi-axes `a = 2.7876759954941264e-4` and
`b = 2.652877017520002e-9` m in `z = 0`, its span
`(-0.017214096000831052, 0.23723376360108975)`. A segment through
`(a·cos tp, b·sin tp, h)`, `tp = 0.017491853781884382`,
`h = 3.9834331835459853e-10`, along `(cos ψ, sin ψ, 3.6362101703388338e-6)`,
`ψ = 5.543433140289637`, its span
`(-4.4045566398379894e-5, 4.000262738647992e-5)`. The band is
`(1e-9, 1.141210283307668e-9)`.

The dense oracle (the conic point's distance from the segment, 40,000
samples, minima refined) finds two parts within the zero:
- the conic's start, `9.4·10⁻¹⁰` m off;
- the root at `θ = 0.01749`, `3.98·10⁻¹⁰` m off.

Their points are `1.4·10⁻⁹` m apart, past the escalation, and the
distance between them rises to `2.9·10⁻⁸` m. They are two meetings.
`meet` answers one, the start's (`End(Start)` on the ellipse), in both
orders, on both heads.

**What is known.**
- On 7208's head the pose offers the root at `0.01749` as a candidate,
  its floor `3.98·10⁻¹⁰` (`WindowSpans::MayPass`). It is lost after the
  pose, where it is read beside the known end meeting
  (`crates/geom-brep/src/curve_pair.rs:424`, `posed`, `read_pair`).
  Why is not yet traced.
- The probe's draws 3534 (`a = 3.03·10⁻⁵`, `b = 1.05·10⁻¹⁰`) and 3082 (on
  main) look like the same class.

**Not 7208's** to fix. That PR's probe counts these as its residual
misses: 3 in 60,000 reads, against 6 on main.

## Fixed

**The cause.** The deflation premise "a meeting so found is a root the
pose already holds" (`pose`'s module docs, "The spans' ends, and
deflation") fails here. The ellipse's start, `θ₀ = −0.0172`, is found by
the ends as a meeting, and `owed` takes it for a root and offers its
reflection `2θ_hi − θ₀ ≈ +0.0172`. On the thin ellipse that is the same
`x` on the other branch, `9·10⁻¹¹` m from the end's meeting, so it rings
onto it. The second root, `θ = 0.01749`, `1.4·10⁻⁹` m away, is never
offered.

**The fix** (`line_conic`, the Level arm; `reflected`). One found
meeting is still deflated, its reflection offered, except where the
reflection is shown to be the other branch's: it rings onto the found
meeting's point in metres (`curve_pair_meet_same`) while its angle from
`θ₀`, the least way round, is not decided within the found meeting's
ring in angle, `e/|C′|max` (`curve_pair_pose_reflection_far`). There
both roots `θ_hi ± φ₀` are read (`Slant::roots`), and ringing drops any
within the band of the found meeting's point. Reading both roots
whenever a meeting is found breaks the deflation's own premise: where
the found meeting is a root, its twin from the square root lands
`~√ulp·r` off and past the span's end, and 14 sweep rows refuse on
`curve_pair_span_past_reach_straddle`.

**What it answers now.** Both draws are never fewer than two meetings.
Each refuses: 4009 on `curve_carrier_gap`, 3534 on
`curve_pair_meet_window_contact`. The root on the end's side lies just
past the ellipse's span, `1.2·10⁻⁹` m from the end's meeting, outside its
ring. Rows:
- `meet_rows::a_roots_meeting_beside_a_span_ends_meeting_is_read` (red
  under the reflection);
- `meet_rows::a_segment_ending_at_a_near_tangency_reads_its_ends_meeting_once`.

The dense probe, 10,000 draws per seed × 2 orders × 3 seeds, now reads
0 missed and 0 two-as-one at both bands.
