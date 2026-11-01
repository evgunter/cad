---
id: review-4781-touch-pairs-is-red-at-1e-6-and-1e-12-on-main
kind: issue
title: contacthold: review_4781_r1_probes::review_4781_touch_pairs_every_op is red at eps 1e-6 and 1e-12 on main
status: closed
opened: 2026-10-12
branch: contacthold/review-4781-eps-rows
pr: 4986
closed: 2026-10-13
---


`cargo nextest run -p sweep` with `CAD_TOLERANCE_EPS=1e-6` and with
`1e-12` fails `review_4781_r1_probes::review_4781_touch_pairs_every_op`
(`crates/sweep/tests/review_4781_r1_probes.rs`) on main. It passes at the
default eps. Seen on 2026-10-12 by two BAND lanes re-running their eps rows
on clean `origin/main` worktrees (`852d68bb1d` and later): PR 4797's fix
pass and PR 4844's lane. Neither touches a Boolean; the row is a Boolean probe kept from PR 4781's review,
landed after main's last full nightly, so the nightly has not reported it
yet. The failure text was not captured; the owner should reproduce it at
both eps and decide whether the probe's pins or the touch reading move.

## Resolution

PR 4831 had re-pinned the row by literal ε before this item was
picked up (`eps == 1e-6 && s == 1e-3`, `eps == 1e-12 && s == 1e3`), so
it was green at all three CI rows on `9b88760`. It was red at any other
ε: at 1e-11 and 3e-12 the staggered kiss swaps which half of its
two-tolerance pair it refuses with. The pins now read the pose against
the running band:

- `coarse_at`: smallest feature under 1e4 ε. The membership oracle's
  rays graze in band.
- `past_f64_headroom`: 100·reach·2⁻⁵² > ε. A kiss may refuse
  `Escalated` or `CurvedPierceUnsupported`.
- The sample step is at least the escalation width.
- The near-tangent slides are multiples of the band's zero, not fixed
  lengths.

The kernel answer is honest in each case. Sibling sweep and the filed
corner-sample row are in the PR.
