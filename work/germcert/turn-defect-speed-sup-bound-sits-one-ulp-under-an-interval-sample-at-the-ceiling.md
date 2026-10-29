---
id: turn-defect-speed-sup-bound-sits-one-ulp-under-an-interval-sample-at-the-ceiling
kind: issue
title: turn_defect_speed_hi's sup bound sits below an Interval sample of the speed at the ceiling, by 1 ulp at 1e-6 and about 11 ulps at 1e-12: an enclosure that does not enclose, found by r2_turn_probes' varying-seed fuzz
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Found by GREEN (PR 5966's gate, run 40805176279, `test (kernel)`,
2026-10-29). It is not that PR's: it reproduces on main `f533c1a9d5`
with the seed.

`crates/geom/tests/r2_turn_probes.rs`,
`turn_defect_speeds_bound_the_speed_on_the_true_defect_at_f64_and_interval`
(a varying-seed fuzz), at ε 1e-6:

> sup over the defect: 130.88286993304766 (f64) / 130.88286993304766
> (Interval) < 130.8828699330477 at 16/64: a -130.88286993304766 b
> 0.027471258741429194 t0 1.5708606597176065 gap 2.571067338045846e-4

The sup bound `turn_defect_speed_hi` reads exactly the ceiling `|a|`,
which the row's own ceiling check allows. The `speed_enclosure` sample at
16/64 has an upper end one ulp above it, the enclosure's own rounding.
So either the bound must stand an ulp clear of the ceiling where it
clamps there, or the row's "sup over the defect" comparison owes the
same `1e-15` relative slack its ceiling check already has. Which is
right is the owner's call; not looked into further.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0xcc21d29c318a7479
CAD_FUZZ_EFFORT=1 cargo nextest run -p geom --profile ci -E
'test(=r2_turn_probes::turn_defect_speeds_bound_the_speed_on_the_true_defect_at_f64_and_interval)'`.

## A second seed, 2026-10-29 (GREEN latency lane)

PR 6019's gate (run 40847689090, `test (kernel)`) reds the same row at
ε 1e-12, on a diff that touches only `scripts/ci-filter.py`:

> sup over the defect: 2.953798525442482 (f64) / 2.953798525442482
> (Interval) < 2.9537985254424868 at 21/64: a 1.0336476587993828 b
> 2.953798525442482 t0 0.0002488148581001326 gap 7.583415220477767e-4

The bound reads exactly `|b|` and the sample sits about 4.8e-15 above
it, about eleven ulps and more than a `1e-15` relative slack, so neither
fix named above covers this seed.

Reproduce: `CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x85193fcde51a223a`
with the same command.

## Re-homed and banded (2026-10-29, GREEN)

Moved from TINT to GERMCERT. The defect is in the bound,
`geom::carrier_speed::turn_defect_speed_hi`
(`crates/geom/src/carrier_speed.rs`), not in the test. The second seed's
11-ulp miss rules out a test-side slack as the fix, and no program
claims the file; GERMCERT's charter (certificate margins read at their
whole width) is the nearest. Banded P2, as interval and error
propagation: the sup bound is a certified enclosure that a sample
escapes.

The fuzz keeps running (the user, in chat, 2026-10-29, agreeing not to
disable it): while this row is open it will red an unrelated PR on
some seeds, about twice a day on 2026-10-29. That cost is the reason to
take it soon. When it is fixed, pin both seeds as deterministic rows
beside the fix.
