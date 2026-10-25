---
id: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
kind: unit
title: A closed-form Intersection carrier passes its certificate on nine samples with no stated bound between them; the standing is provenance read off the variant
status: open
opened: 2026-10-24
priority: P0
cost: H
refs: [offset-lane-has-no-conic-carrier]
parent: offset-lane-has-no-conic-carrier
---


Filed by OFFSET's conic designer pair (2026-10-24, `[ev]` PR 5652; both
designers agree on the finding, and split on the fix Ev is asked about).

`crates/geom-brep/src/certify.rs` `EdgeCurve::certify` gives an
`Intersection` edge whose carrier is a `Line`, `Circle`, `Ellipse` or
`Spiric` the nine-sample residuals, transversality and the witness
mid-pin only; a `Curve3::Nurbs` carrier additionally owes C2's hull limb
and tube. Nothing in that sampled check bounds the residual BETWEEN
samples (only the tangent lane states a sup bound). Its standing is C1's
"zero by construction because the kernel derived them from their pair"
(`crates/geom-brep/README.md`), which the code spells as "not the `Nurbs`
variant": provenance the door cannot check.

For the conic kinds a bound exists and is not written: a conic composed
with a quadric has a residual of fixed low degree (a degree-2
trigonometric polynomial for a circle or ellipse), so nine samples bound
it over the span up to a fixed constant; the pcurve cache already
argues this way for `Pcurve::Harmonic` ("a corruption hiding between
samples is unrepresentable"). For `Spiric`, and for `Circle` against a
torus, the residual is not of finite degree, so the samples alone bound
nothing between them (unsure whether the stored surfaces constrain it
another way).

What closes it depends on Ev's answer on PR 5652. If the obligation
follows the CLASS read from the carrier's data, the certificate door
states a between-samples constant per class and routes the classes
without one to the hull limb. If it follows the KIND, C1 states the
low-degree reason for the conic kinds and the `Spiric` question is
answered separately. Either way the sentence C1 rests on today should not
stand as it is. Filed by the OFFSET orchestrator.

## Claimed by OFFSET (2026-10-24) — conic build, unit 1 of 3

Ev ruled on PR 5652 (`offset-lane-has-no-conic-carrier`, "Ruling"):
the certificate is derivation-blind. This row is its first half, now
`crates/geom-brep/README.md` C2: **limbs 1 and 2 are owed by every
`Intersection` carrier of every kind, however it was minted.**

Build: `EdgeCurve::certify` (`crates/geom-brep/src/certify.rs`) runs limb
2 (the composite hull bound between samples, `analytic_rung3`'s limb-2
path, reading the carrier through `carrier_net`'s enclosing net) for
`Line`, `Circle`, `Ellipse` and `Spiric` carriers too, on every surface
including the cone and torus per-span lever (`net_offset_sup`). The tube
obligation is unchanged in this unit: closed-form kinds keep not owing it
for now, which is today's behaviour and what unit 3 replaces with the
pair rule. Delete what C1/C2's retired derivation wording justified in
code comments. Expect pins to move at rest (runtime and stored
certificates): measure the corpus cost of the new limb 2 on closed-form
edges and report it; re-baseline and say what moved. Name the spiric
net's tightness on a long oval (both designers unsure) with a row.

Banded P0 as part of the P0 build; it is also what closes the latent gap
(a spiric slightly off its torus owes no bound today).
