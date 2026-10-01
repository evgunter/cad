---
id: circle-torus-meters-accept-an-unreadable-reading
kind: issue
title: The circle x torus half-angle door's noise and root-slack meters accept an Err reading (NaN or in-gap), where its parallel-axes sibling refuses
status: open
opened: 2026-10-01
refs: [circle-torus-root-slack-crowds-the-zero-band-at-1e-12, sphere-union-sphere-refuses-though-the-section-is-closed-form]
---

Found by the PR 3659 review (`reach-snowman` fix pass) while sweeping
every `*_noise` / `*_root_slack` meter in `topo::boolean` for one
posture.

## The sites (5 in all)

| site | on `Err` |
|---|---|
| `circle_sphere::circle_sphere_roots`, `bool_circle_sphere_noise` | refuses (`Uncertain`) — fixed in PR 3659 |
| `circle_sphere::circle_sphere_roots`, `bool_circle_sphere_root_slack` | refuses — fixed in PR 3659 |
| `circle_torus::parallel_axes_roots`, `bool_circle_torus_root_slack` | refuses ("a NaN slack … is `Err` and refuses too") |
| `circle_torus::half_angle_roots`, `rows.noise` | **accepts** (`Ok(Zero \| Negative) \| Err(_) => {}`) |
| `circle_torus::half_angle_roots`, `rows.root_slack` | **accepts** |

A meter that cannot be read — a NaN (a zero slope), or a reading in
the band's escalation gap — licenses nothing it meters, and the
parallel-axes arm already says so in its comment. The two half-angle
sites read the opposite way.

## Why it is filed rather than fixed in 3659

The noise meter's threshold is documented and MEASURED as "definitely
past the band's escalation threshold" (`circle_torus` module docs, "The
noise meter", with the ρ-coverage figures at R = 1, r = 0.25).
Refusing on `Err` moves that threshold from Kε down to ε and changes
those figures, so the change wants the lane that owns them to re-run
its measurement, alongside the 1e-12 crowding in
`circle-torus-root-slack-crowds-the-zero-band-at-1e-12`.
