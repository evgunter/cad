---
id: pcurve-radial-amplitude-sums-the-radial-pair
kind: issue
title: pcurve chart: radial_amplitude reads a radial circle of radius δ as 2δ, so a sub-escalation circle is decided moving
status: open
opened: 2026-10-12
priority: P3
cost: E
---


(PCERT implementer, the sweep of
`pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation`.)

## What

`radial_amplitude` in `crates/geom-brep/src/pcurve_cache.rs` is
`‖a_r‖ + ‖b_r‖ + ‖l_r‖`, and both cylinder class decisions read it
(`pcurve_chart_radial_moving`, in `chart_pcurve`'s cylinder arm and in
`chart_iso_family`). A radial circle `a_r ⊥ b_r`, both of length `δ`,
moves `δ` off its centre and reads `2δ`. The in-band arm takes the
meridian (a D9 tie-break, and check 4's drift term then bounds the
motion at its exact sup), so the overshoot only bites past the band:
a circle with `δ ∈ [escalate/2, escalate)` is decided definitely
moving on a margin twice its true size.

## Why it is not the envelope row's fix

It is a class selector, not a bound. Reading the pair at its exact sup
(`harmonic_sup(a_r, b_r) + ‖l_r‖`) moves those carriers to the
meridian, where a rim of a cylinder whose radius is under the
escalation band — today certified on the moving class with a zero
radius gap — reads its whole radius as drift and refuses. Whether
that trade is right (both readings concern a cylinder thinner than the
band) is the question, and it wants its own row.
