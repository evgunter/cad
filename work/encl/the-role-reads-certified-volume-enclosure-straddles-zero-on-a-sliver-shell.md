---
id: the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell
kind: issue
title: The role read's certified volume enclosure on a sliver shell is 1e-9 to 5e-7 wide in V/A at ε = 1e-12, straddling zero where the point reading is about 30 bands positive
status: dispatched
branch: encl/shell-volume-local-origin
opened: 2026-10-08
priority: P2
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by JOIN's near-tangent census measurement
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its
`## Measured`), on main `047d10d5`, release.

A near-tangent ∩ keeps a sliver lump as its own shell
(`a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses`,
on JOIN's slate). At ε = 1e-9 that shell's point V/A is in band, and the
refusal is right. At ε = 1e-12 the same lump is many bands thick, yet the
door still refuses `ShellRoleUndecided`, now on the certified reading.

- **The margin.** `positive_volume_exact` is an enclosure of V/A
  1.4e-9 to 5.2e-7 wide that straddles zero. This holds on 74 runs at
  ε = 1e-12, at d = 1e-8 to 1e-10, ∩ on asym, notch307, shallow200,
  vee300 and w345. At d = ±1e-11 the lump really is in band: 16 of the
  35 refusals there hold a narrow enclosure inside the band (class (a)),
  and 19 straddle.
- **The true value.** At d = 1e-9 the lump is 2e-9 thick at its far end.
  The lumps measured at 1e-8 put V/A at about 0.17 × thickness, so about
  3.4e-10, which is 34 K·ε. The point reading decides it. The enclosure,
  [−5.2e-9, 5.8e-9] at the witness, does not.
- **Witness.** `notch307 nt e0 a0 d1e-9 pc I` at `CAD_TOLERANCE_EPS=1e-12`.

The enclosure's width is about what the divergence sum's absolute
rounding (coordinates of order 1 to 4 m) leaves, divided by the
sliver's area (4e-8 m²). It is a property of the certified volume's
absolute error on a small shell, not of the shell. `props::certify_role`
reads it after the point reading has decided, and an undecided
enclosure wins.

The oracle's own f64 clip volume at this scale is noise (−6e-17), so the
true V/A is scaled from the thickness, not measured.

Repro: `CAD_TOLERANCE_EPS=1e-12 NT_ONLY=notch307 NT_POSE="nt e0 a0 d1e-9"
NT_D=1e-9 cargo run -p sweep --release --example near_tangent_census_probe`.

## The shape to give

Re-derive the shell's volume about a local origin: the shell's own
centroid or `v`, so the rounding scales with the shell's size, not the
body's span. Then check that the enclosure decides at d ≥ 1e-10 under
ε = 1e-12. `volume-door-reads-a-tiny-valid-boolean-result-wrong`
(CONTACT, closed by PR 3977) is the same absolute-error class on the
point door.
