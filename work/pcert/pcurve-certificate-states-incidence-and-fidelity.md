---
id: pcurve-certificate-states-incidence-and-fidelity
kind: unit
title: the Harmonic pcurve certificate states the carrier's incidence and the image's fidelity, so a minted row's identity is a theorem over a parameter box
status: open
opened: 2026-10-02
priority: P0
cost: H
design: true
needs_ev: true
refs: [3759]
---

**Why.** Once `sweep::extrude` mints its rows (PR 3759), the
`Harmonic` pcurve certificate runs on every extruded wall at every
scalar. Over a parameter box, its checks are residuals whose true
value is zero, but `Sym<Interval>` decides them numerically
(`pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box`,
filed on 3759's branch). The cost shows in the certified studies:
- M10-7's plate certifies whole only to 3.9e2·ε;
- `m10_3`'s drive certifies 0/1024;
- `m10_4` certifies nothing;
- the demo tour's tolerance study certifies 0%.

The cause is that the certificate routes a fact about the body through
a trig identity no arithmetic sees through. The stored image is an
angle `α = atan2(…)`, and checks 3 and 4 push it back through
`sin`/`cos`. The same shape recurs in `pcurve_loop_continuity`,
`_closure`, `_pole_joint` (UV-angle identities) and in
`pcurve_trim_containment` (a `max(0, X)` fold).

**The unit.**

1. **Check 4, the Harmonic envelope, is restated in the carrier's own
   coefficients** as two parts:
   - **incidence**: the carrier against the chart, per chart arm. For
     the cylinder these are:
     - centre on the axis;
     - `(R² − a_r·a_r)/(R + ‖a_r‖)`;
     - orientation `b_r − β·axis×a_r`;
     - the axial line.

     The cone, sphere and torus arms are read off
     `chart_image_harmonic`'s rows the same way.
   - **fidelity**: the stored image against the image re-derived from
     the carrier inside `certify`, up to branch, metered through the
     chart's stretch (`chart_stretch_sup`). This is today's snap slack
     extended to every channel.

   It is the same meter: the closed-form sup of `|S(P(t)) − C(t)|`,
   spelled without trig.
   - Each arm's lemma goes in `EnvelopeStatement::MapResidualClosedForm`'s
     docs: the re-derived image's map differs from the carrier by at
     most the incidence terms, including the drift the structure
     selections admit.
   - A property test must fail if a table is mutated.
   - A refusal names the incidence that failed.
2. **Check 3, the schedule `|S(P(tᵢ)) − C(tᵢ)|` on a `Harmonic` row,
   leaves the certificate over a parameter box.** Where it runs instead
   is Ev's question, open on an `[ev]` PR. The recommendation is:
   - it runs where the scalar is a point (the driver's f64 witness
     replay);
   - it runs as a property test of
     `chart_image_harmonic ∘ chart_pcurve = carrier_harmonic` over the
     covered classes.

   The alternative keeps it in the box certificate and needs a SYM tier
   rule `cos/sin(atan2(y, x)) = x,y/√(x²+y²)` with angle addition,
   whose reach is unmeasured. Part 1 does not depend on the answer.
3. **Measure before claiming a ceiling:**
   - the M10-7 plate, the `m10_3` drive, `m10_4`, `m10_9`, `sym11`, the
     demo tour's `eps_regression` and `chaintol`, under the shipped rule
     set and without the door;
   - each term's decision class (theorem / registered / numeric).

**Measured so far** (designers' scratch probes, nothing committed;
`Sym<Interval>`, shipped rules):
- On the `sym11` stadium (fixture door, no registrations) and on a
  two-arc disk with a parametric radius, every incidence term is a
  theorem on all 8 wall rows of each, at a box of ±ε/64 and of
  ±1e-3·ε. Today's spelling on the same nodes is numeric on every
  seam.
- On a hand-built cylinder wall with a `√` radius, the far strut
  refuses `MapResidual` today at ±1e-10. Its amplitude and phase
  fidelity terms decide Zero as theorems at ±1e-3.

**Not in it.** The UV loop decisions (continuity, closure, pole joint),
restated as a 3-D identity plus a branch margin of π, are the next
wall. They are a follow-on row once this lands. Trim containment's
`max(0, X)` lane is SYM's.

**Sequencing.** PR 3759 does not merge with the widening regression,
so this unit lands before it or with it. `pcurve_cache.rs` is in
PCTAIL's territory too; announce the seam at dispatch.
