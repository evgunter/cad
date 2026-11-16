---
id: the-conic-quadric-door-certifies-first-harmonic-roots-past-the-zero-band
kind: issue
title: The conic × quadric door certifies roots past the zero band - a small ellipse against a cone, two certified roots 0.78 and 1.39 bands along the arc from the true crossings (the first-harmonic arm drops a second harmonic that sits inside the band)
status: open
opened: 2026-11-15
priority: P0
cost: M
---

Filed by GREEN's `green/nightly-1115` lane (PR 6985), from nightly
43680833019, and re-stated in that PR's fix pass after its review
replayed the case. It takes over
`work/green/cone-rows-certified-answers-fail-at-1e-12-on-one-seed.md`
(closed there). **P0: a live wrong answer.** The door returns
`CircleRoots::Certified` for roots that are not where the carrier
crosses the cone to within the band it was handed.

## The counterexample, replayed without the seed

`topo boolean::conic_quadric::cone_rows::the_nightly_1115_ellipse_root_is_certified_past_the_band`
(the reviewer's probe, kept as the pinned row) builds the case from the
fuzz row's printed inputs, which reproduce bit for bit:

- `Ellipse { frame: Frame { origin: (0.5951044729266803, -0.4948492339322913, -0.48992849588759274), axis: (0.4075878663827065, 0.6480769646040943, 0.6433260286411057), u_ref: (0.881430409939996, -0.4633281245914571, -0.09169231917436711) }, semi_u: -0.0026403939123773564, semi_v: 0.00046833656424564605 }`
- `Cone { apex: (-0.5248723522085268, -0.2602737120283434, 0.5026482288829277), axis: (0.16386811977525384, -0.9491390527668793, -0.26885367364816554), half_angle: 1.1109037283720684, u_ref: (0.9854212451578827, 0.17013221209250257, 0.0) }`
- on `[5.709087228385727, 10.08162235503239]`, band `Band::new(1e-6, 1e-5)`.

Measured by the reviewer's replay (branch `green/nightly-1115-review`,
`6d8b64ce0`):

| | certified root | oracle's crossing | along the arc | residual's slope along the arc |
|---|---|---|---|---|
| 1 | 6.281528699625674 | 6.283185307178906 | 7.76e-7 | 0.66 |
| 2 | 7.0437617918100885 | 7.043009249272368 | 1.392e-6 | 0.166 |

At root 2 the cone residual is -2.316e-7 with a running bound of
1.2e-15, so the residual is resolved nine decades under the miss, and
neither crossing is shallow. The second root is outside the 1e-6 zero
band; the row pins that.

**The ambient ε does not enter.** The fuzz row builds its own bands
(1e-6, 1e-9, 1e-12) and the seed fails the same way at
`CAD_TOLERANCE_EPS` 1e-12, 1e-9 and 1e-6 (the reviewer's runs). The
nightly showed it on its 1e-12 leg only because each leg draws its own
seed.

Seeds that reach it through
`cone_rows::certified_answers_hold_against_the_quadric_form`:
`0x5a2a16bda0f5438c` (case 281, above) and `0x3bf35ddc97f968bd`
(case 41, PR 6598's delta-review runs; an ellipse root 2.7e-6 along the
arc, failing on that PR's base `fbc704f1` too).

## Mechanism (likely; the reviewer's check of the harmonics, not a trace of the door)

The residual round the turn carries a first and a second harmonic. Here
the second's amplitude, `A2 = 4.37e-7`, sits inside the 1e-6 zero band,
so the door takes its first-harmonic arm
(`geom_brep::curve_face::circle_roots::first_harmonic_roots`, called
from `conic_quadric/mod.rs`). The
certified roots equal the first harmonic's roots to 1e-13. They are
displaced from the true crossings by the dropped second harmonic over
the residual's slope, 2.45e-4 per radian: an amplitude inside the band
divided by a small slope is not inside the band. The arm's own slack
decision (`circle_roots.rs`, `harmonic_crossing`, the `root_slack`
row, which accepts `Zero | Negative`) charges the extremes' and the
phase's noise over that slope, and not the harmonic it dropped, so it
passed.

## Where the owner looks next

The first-harmonic arm is shared: both `first_harmonic_roots` calls in
`conic_quadric/mod.rs` (the sphere and the wall doors) and the rows in
`ellipse_rows` and `circle_wall_rows`. A small conic whose second
harmonic is just under the zero band, against a sphere or a wall, is the
shape to try. GREEN did not find or look for a second instance.

## Owed

Charge the dropped harmonic in the arm's slack (or refuse the arm where
`A2` over the slope is not inside the band), and turn the pinned row
round: it reds the day the second root comes inside the band or stops
being certified.

Reproduce: `cargo nextest run -p topo -E 'test(the_nightly_1115_ellipse_root_is_certified_past_the_band)' --success-output final`
(it prints both roots, the residuals and their slopes), or the fuzz row
with `CAD_FUZZ_SEED=0x5a2a16bda0f5438c CAD_FUZZ_EFFORT=1` at any ε.
