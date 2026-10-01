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

## Evidence (2026-10-01, TANG's circle × cylinder cell): what refusing costs, measured

The circle × cylinder cell (`topo::boolean::circle_cylinder`) calls
`half_angle_roots` for a circle tilted to the wall, so this posture now
covers a second surface. TANG tried the sphere door's posture in the
shared door (`Ok(Zero | Negative) => {}`, `Ok(Positive) | Err(_) =>
Uncertain`, both rows) and measured it before deciding not to land it:

- **Default band**: the circle × torus ρ-sweep
  (`no_wrong_certified_answer_across_circle_radii`) answers
  `[16, 0, 0, 0]` of 32 at ρ = 10, 30, 100, 300 under both postures, so
  the module docs' figures would stand. On a grid of far misses against
  a wall (ρ ∈ {1, 10, 30, 100}, r ∈ {1, 0.1, 0.01}, 3 to 2000 m off), 9
  of 48 answers move from `Miss` to `Uncertain`, each a miss whose noise
  reading lies in the gap — for example a circle of radius 10 tilted
  45° against a unit wall 2000 m off.
- **`ε = 1e-12`**: three rows red that are green at the merge base,
  every one an ordinary unit-scale pose whose 16-ulp term-bound charge
  lands in the band's gap: `circle_torus::tests::a_clear_circle_is_a_miss`
  (a unit circle 4 m off the torus answers `Uncertain`),
  `roots_are_reported_within_half_a_turn_of_the_arc` (no certified
  count), and `germ_circle_torus::a_small_tilted_seam_crosses_the_wall_on_the_quartic_arm`
  (the sweep refuses `CurvedPierceUnsupported`).

So refusing on `Err` in the ladder is the right reading of an
unreadable meter, and the term bound it reads is too coarse for
`1e-12` to afford it: the charge, not the posture, is what wants work
first. The cylinder cell keeps the ladder's posture, and its own
first-harmonic arm keeps the sphere door's.
