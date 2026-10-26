---
id: a-torus-twin-fuzz-row-draws-points-inside-the-band-at-1e-6
kind: issue
title: The torus wall's clockwise-twin fuzz row draws points inside the band at 1e-6
status: open
opened: 2026-10-25
priority: P4
cost: E
---

## What

`solid_contain::stated_periods_rows::a_wall_and_its_clockwise_twin_read_complementary_sides`
(`crates/topo/src/boolean/solid_contain.rs`, about `:4776`) is a
counterexample search over random `(u, v)` on the torus `R = 2`, `r = 0.5`.
It keeps a point only where it is "a micron clear of the patch's boundary",
and its `clear` filter tests `1e-6` in **parameter**, whatever the band is.
At ε = 1e-6 the band escalates at `1e-5` m, so a kept point can lie in the
band of a rim, and the region door rightly refuses it there
(`Escalated(curve_carrier_gap)`).

Seed `0x7493a4fcbaad4a4b` at `CAD_TOLERANCE_EPS=1e-6` draws
`(u, v) = (1.4564335511001858, -0.7000131533068612)`. Its `v` is
`1.3e-5` rad off the rim at `-0.7`, which is `6.6e-6` m on the tube, inside
`1e-5`. The row fails identically on main `8d7bc1f6` and on PR 5674, where
CI drew it:

```
CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x7493a4fcbaad4a4b CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(=boolean::solid_contain::stated_periods_rows::a_wall_and_its_clockwise_twin_read_complementary_sides)'
```

## Owed

Make the clearance the band's, in metres: a point is clear when its
distance from each rim, along the surface, is definitely past
`band.escalate()`. That is `|v − v_i|·r` for a `v` rim and
`|u − u_i|·(R + r·cos v)` for a `u` rim, the latter at least `(R − r)`.
For example, require the parameter gap to exceed
`2·band.escalate()/r` on `v` and `2·band.escalate()/(R − r)` on `u`.
Then pin this seed as a deterministic row beside the fix.

## Slate triage (2026-10-26, CLEAVE)

- Banded P4: the test's own clearance filter, not the door: tooling whose payoff is CI going red less often, P4 by `work/README.md` Priority.
- Priced `E`, the effort the body describes.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
