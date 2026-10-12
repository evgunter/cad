---
id: contact-verify-on-surface-residual-subtracts-its-sag
kind: issue
title: contact_verify's on-surface residual decides |r| − residual_sag where the edge certifier decides r + residual_sag, so the verify direction is padded the wrong way
status: closed
opened: 2026-10-03
priority: P1
cost: E
pr: 4558
closed: 2026-10-10
---

Found by CLEAVE's `cleave/steep-tube-eps` lane, in its sweep for one-sided
gates that pass `Negative | Zero` and escalate the band between them.
Unmeasured: no failing pose is known. This is a reading of the code.

`crates/topo/src/boolean/contact_verify.rs`, in the per-sample walk
(`contact_tangent_on_1` / `contact_tangent_on_2`), decides

    let padded = Margin::of(r.abs() - bounds.residual_sag);

and passes on `Ok(Sign::Zero | Sign::Negative)`. The comment above it
says the sag "rides the residual so the between-sample interior is
covered by the same certificate the edge lane uses". The edge lane
(`crates/geom-brep/src/certify.rs`, `tangent_hull_sup`) ADDS it:
`Margin::of(tangent_resid_max + bounds.residual_sag)`.

Subtracting the sag pads the verify direction the wrong way. A sample
residual up to `residual_sag + ε` reads Zero and verifies. Between
samples the true residual can reach `|r| + residual_sag`, so that is
up to `2·residual_sag + ε`. The subtraction is sound only for the
`Positive` arm (`Contradicted`, a definite miss), where an under-stated
residual is the safe direction. That is one margin serving two
directions, which D4 ¶1 (iv) splits. A second effect: with `r ≈ 0` and
a sag in `(ε, K·ε)`, the margin is negative and in band, so it
escalates where nothing is undecided.

## Owed

Measure on the contact corpus: does any sample read `|r| − sag ≤ ε <
|r| + sag`? Then split the gate. Verify on `|r| + sag` (the edge lane's
pad) and contradict on `|r| − sag`.

## Parked on the D10 hold (2026-10-08)

After that unit this gate (`r.abs() - residual_sag`, `contact_verify.rs`) becomes the tangency glue decision, so that unit must carry this fix. (CONTACT close-out triage; CONTACT's log (`docs/doc-ledger/contact-leaves-the-tracker.md` names the SHA it is read at).)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-10)

The glue door runs this verifier on undeclared pairs as well, so the padding direction now decides undeclared glue too. The question this row asks still holds.

## Closed (PR 4558)

Measured on main first. A probe at the gate over `topo` + `sweep` +
`editor-core` saw 80 061 on-surface reads, and 1 376 of them read
`|r| − sag ≤ ε < |r| + sag`. All of those are torus-meridian rims of the G1
tube chain (`mate7a_torus_rest`, `mate7a_r1_probes`), with `|r| < 4e-16`
and `sag` = 1.21 m (or 0.32 m) at ε = 1e-9. The old gate verified a 1.2 m
pad. Split in `on_surface`: verify on `|r| + sag`, contradict on the
sample's own `r` (sound without the sag, and stronger than `|r| − sag`),
escalate between. One test moved (the chain's declared seam now
escalates at the witness lane). The meridian bound is filed on tang
(`a-torus-meridian-rims-span-bound-is-wider-than-the-band`).

## Closed (2026-10-10, PR 4558)

Merged after a full single review and a verified fix pass.
