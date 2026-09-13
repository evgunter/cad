---
id: missing-face-in-verify-tangent-declaration-reads-as-sense-true
kind: issue
title: verify_tangent_declaration defaults a missing face's sense to +1 instead of refusing
status: open
opened: 2026-09-12
---


## Finding

`crates/topo/src/boolean/mod.rs`, `verify_tangent_declaration`: the
local closure `senses(body, f)` reads a face's sense sign as
`body.get_face(f).map_or_else(T::one, |face| face.sense_sign::<T>())`.
A `FaceKey` that resolves to no face therefore enters
`rim_wedge::classify_shared_rim` (and through it
`geom_brep::classify_material_pairing` and `material_kappa_rel`) as a
face whose chart normal points out of the material — a definite,
possibly wrong, answer where the door has a refusal vocabulary
(`surface_of(..)?` two lines above refuses a missing face on the very
same keys). Every other read of `Face::sense` in `crates/*/src` goes
through a resolved `&Face`; this is the one site that invents the bit.

Whether the arm is reachable — the keys come from a declared contact
record whose faces the caller has already resolved — is not settled
here; a default that cannot be reached is `unreachable!` with its
argument, not `T::one()`. Found by the `D6` survey (SCALAR, 2026-09-12);
it does not wait on that ruling. Filed on BOOL's slate as the owner of
`crates/topo/src/boolean/*`.
