---
id: a-root-meeting-beside-a-span-end-meeting-is-dropped-on-a-thin-ellipse
kind: issue
title: A root's meeting 1.4e-9 m from a span end's meeting on a thin ellipse is dropped, one meeting answered for two
status: open
opened: 2026-11-20
priority: P1
cost: M
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
