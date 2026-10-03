---
id: contact-verify-on-surface-residual-subtracts-its-sag
kind: issue
title: contact_verify's on-surface residual decides |r| − residual_sag where the edge certifier decides r + residual_sag, so the verify direction is padded the wrong way
status: open
opened: 2026-10-03
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
