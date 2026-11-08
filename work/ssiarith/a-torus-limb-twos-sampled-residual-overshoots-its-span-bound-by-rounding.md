---
id: a-torus-limb-twos-sampled-residual-overshoots-its-span-bound-by-rounding
kind: issue
title: A torus carrier's limb-two span bound is overshot by its sampled residual past the test's 1e-9 slack (fuzz seed, red on main)
status: open
opened: 2026-10-31
---


**Red on main** (`e8c93806`, 2026-10-31), found by hosted CI's
`test (kernel)` on PR 5781, which does not touch `ssi/`:
`geom-brep` `ssi::certify::tests::a_torus_carriers_limb_two_holds_its_sampled_residual`
(`crates/geom-brep/src/ssi/certify.rs:4005`) fails at

    CAD_FUZZ_SEED=0x6b189cf8dd3c1579 CAD_FUZZ_EFFORT=1

with

    span 7 at 0.75: residual 1.6705327390489462e-9 over its bound 1.670532735001086e-9;
    Torus { center: (0.6913937517453348, -0.42049902130363415, 0.3255380900973768),
            axis: (0.6396318415715722, 0.3954341847752617, 0.6591683493303386),
            major_radius: 0.0010897938199122756, minor_radius: 0.0013395862238738513, .. },
    lift 2.9137806653873414e-10

The sample sits on span 7's start (`t = 0.75`, a cut point). It exceeds
the span's bound by about `4·10⁻¹⁸`, a relative `2.4·10⁻⁹`, past the
test's slack `x·(1 + 10⁻⁹) + 10⁻¹⁵·big` (`certify.rs:3998`), whose
absolute term is about `1.3·10⁻¹⁸` for this millimetre torus (the
minor radius exceeds the major, so this is a spindle torus).

Not yet decided: is the bound short by a rounding term that scales
with the torus rather than with `big` (so `torus_readings`,
`certify.rs:3915`, or the bound it reads, owes a charge)? Or is the
test's slack too tight for a millimetre torus? Either way a bound that
a sample exceeds is not a bound, so the first reading is the one to
rule out first.

2026-11-01 (SPLICE, PR 6171's CI at 1e-12): another instance. `CAD_FUZZ_SEED=0x6a2d823d74e75b46
CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-12`: span 8 at 0.75, residual 1.0122415369981837e-8 over its
bound 1.0122415334767205e-8. It reproduces on `origin/main` a81699db.

## Seen again (2026-11-03)

Red on SPLITDOOR's PR 6424 at 35eb6409a5. That PR touches no part of `ssi/`. The failure: span 2 at 0.25, residual 2.772603573875806e-8 against a bound of 2.772603564314103e-8 (relative overshoot 3.4e-9). Replay with `CAD_FUZZ_SEED=0x2608a5ffadab24bf`. The next CI run, on a fresh draw, was green. (SPLITDOOR orchestrator)

**Still red (2026-11-03)**: hosted CI `test (kernel)` at eps 1e-12 on PR 6454
(`onecopy/ladder-climb`, which does not touch `ssi/`) failed with
`CAD_FUZZ_SEED=0xca64507d38d555ef CAD_FUZZ_EFFORT=1`: span 6 at `0.666…`
(a cut point again), residual `9.269270029140466e-9` over its bound
`9.269270008214833e-9`, a spindle torus (minor radius `0.00127` over major
`0.00098`). It reproduces locally with main's `geom-brep` sources.
