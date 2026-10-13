---
id: review-4781-touch-pairs-is-red-at-1e-6-and-1e-12-on-main
kind: issue
title: contacthold: review_4781_r1_probes::review_4781_touch_pairs_every_op is red at eps 1e-6 and 1e-12 on main
status: open
opened: 2026-10-12
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
