---
id: the-cone-rows-certified-ellipse-roots-lie-past-the-band-along-the-arc-at-1e-12
kind: issue
title: conic_quadric's cone fuzz row at ε 1e-12: a certified ellipse root lies more than the zero band along the arc from the oracle's crossing, on two seeds
status: open
opened: 2026-11-15
priority: P2
cost: M
---


Filed by GREEN's `green/nightly-1115` lane. It takes over
`work/green/cone-rows-certified-answers-fail-at-1e-12-on-one-seed.md`
(closed there), which sat on GREEN's slate from PR 6637's review, and
adds the seed of nightly 43680833019.

`topo boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form`
is a counterexample search over a conic against a cone, at each of the
three zero bands inside the test. Run under `CAD_TOLERANCE_EPS=1e-12` it
fails `assert_placed` on two seeds, each on an ELLIPSE at the test's
1e-6 band (the ambient ε is what differs; neither fails at the default
ε):

- `CAD_FUZZ_SEED=0x5a2a16bda0f5438c`, case 281 (reproduced on main
  `3e7a8c93b`, `opt-level = 2`): the certified root 7.0437617918100885
  lies 1.3928e-6 along the arc from the oracle's 7.043009249272368, with
  3.3e-14 of that the oracle's own resolution. The carrier is
  `Ellipse { frame: Frame { origin: (0.5951044729266803, -0.4948492339322913, -0.48992849588759274), axis: (0.4075878663827065, 0.6480769646040943, 0.6433260286411057), u_ref: (0.881430409939996, -0.4633281245914571, -0.09169231917436711) }, semi_u: -0.0026403939123773564, semi_v: 0.00046833656424564605 }`
  against `Cone { apex: (-0.5248723522085268, -0.2602737120283434, 0.5026482288829277), axis: (0.16386811977525384, -0.9491390527668793, -0.26885367364816554), half_angle: 1.1109037283720684, u_ref: (0.9854212451578827, 0.17013221209250257, 0.0) }`
  on `[5.709087228385727, 10.08162235503239]`. The major semi-axis is
  negative, which the row builds on purpose on every fourth ellipse.
- `CAD_FUZZ_SEED=0x3bf35ddc97f968bd`, case 41 (PR 6598's delta-review
  runs; it fails on that PR's base `fbc704f1` too): an ellipse root
  2.7e-6 along the arc from the oracle's.

**What is not settled.** Whether the door's certificate is wrong (a
root certified at a band of 1e-6 that is 1.4 bands from the true
crossing along a shallow carrier) or the row's claim is too strong for
a crossing this shallow at an ambient ε three decades under the band it
passes. A certified root placed outside its band is a wrong certificate
if it is the first, and the band of this row should rise with it.

**Owed.** Decide which, and pin the first counterexample above as a
deterministic row beside the fix (its inputs are exact as printed).

Reproduce:
`CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x5a2a16bda0f5438c CAD_FUZZ_EFFORT=1 cargo nextest run -p topo -E 'test(=boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form)'`.
