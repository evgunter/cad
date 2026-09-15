---
id: missing-face-in-verify-tangent-declaration-reads-as-sense-true
kind: issue
title: verify_tangent_declaration defaults a missing face's sense to +1 instead of refusing
status: closed
opened: 2026-09-12
closed: 2026-09-15
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

## Closed (2026-09-15, SENSE-DOORS)

Closed by the fix, not by a ruling: SCALAR's `sense-sign-doors-take-the-bit`
was reaching this ground anyway and BOOL's slate is where the row lives,
so the unit closed it there.

**Reachability, settled.** The arm could not be reached. `surface_of`
two lines above resolved the same two keys and refused a missing face,
and through the public door `validate_declarations`' `inventory_face`
refuses one earlier still ("declared face key does not resolve"). So
neither `unreachable!` nor a second refusal string was the repair.

**What landed instead.** The defaulting closure is gone and the second
lookup with it: `verify_tangent_declaration` now resolves each declared
face ONCE, through a `face_of` that hands back the carrier and the
`Face::sense` bit together, and both stages read that one resolution.
There is no arm left that could answer from an invented sense, and the
refusal vocabulary did not grow — a key with no face behind it still
refuses `InvalidDeclaration { what: "declared face lost its surface" }`.
Pinned by `boolean::tests::a_declared_face_that_does_not_resolve_refuses_typed`,
which calls the verifier directly so the inner layer of the defence is
exercised and not merely present.
