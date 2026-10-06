---
id: transverse-not-intrinsic-reads-declared-where-d2-says-declared-is-exempt
kind: issue
title: TransverseNotIntrinsic fires only on a DECLARED locus, while D2's text says a declared conventional description is exempt: a derived chart on a transverse edge passes tier 3
status: open
opened: 2026-09-26
---


## Finding

Found while measuring
`work/encl/must-carry-over-edge-reads-a-transverse-edge-as-under-determined.md`,
whose defect stored a conventional `Chart` image (`EdgeAuthority::Derived`)
on a definitely-transverse edge — a ruled band's cut-off arc, `sin θ = 1.0`
at every interior station — and `validate_geometric` passed the body.

`crates/topo/src/validate.rs`, check 4's prefer-intrinsic enforcement:
`if !escalated && all_transverse && curve.authority().is_declared()` pushes
`ValidationError::TransverseNotIntrinsic`. So the rule fires ONLY on a
**declared** locus (a sketch pushforward), and a definitely-transverse edge
carrying a kernel-**derived** chart image is exempt. The `Tangent` must-carry
arm reads the authority the same way (`mark == ContactMark::Tangent &&
curve.authority().is_declared() && …`).

`docs/DESIGN.md` D2 (the prefer-intrinsic paragraph) says the opposite of the
first half: "At rest, every *definitely-transverse* edge must carry
`Intersection` (`TransverseNotIntrinsic` otherwise) … The check reads the
edge's authority record, so a declared conventional description is exempt by
its own declaration". Written by `99cc678bfd` (the DESIGN editing pass); no
ratification of the sentence as it now reads was found.

Which one is right is a design question this finding does not settle. The
code's reading keeps every kernel-derived chart legal on a transverse edge
(`geom_brep::EdgeAuthority::Derived`'s doc lists seams, iso boundaries and cap
rims among them), which "every definitely-transverse edge" would not; the
text's reading would have caught the band-first chart. Whether any derived
chart on a definitely-transverse edge survives at rest today was not
measured. Either
the text or the check is wrong, and while they disagree, a constructor that
stores a derived chart on a corner has no at-rest backstop. Owner's call; if
the change is to D2's text or to what it decides, it waits for Ev.

## The tangent half, as a question (CLEAVE, PR 4157 review NOTE-1)

The same split runs one order up, in `TangentNotIntrinsic`. Three texts
say a derived conventional description on a jet-determinate tangency is
refused at rest:

- `docs/DESIGN.md` D2: "a jet-determinate tangency must carry
  `TangentIntersection` (`TangentNotIntrinsic`, where
  `geom_brep::tangent_certificate_lane` admits the class)".
- `crates/geom-brep/README.md` C7, a ratified clause: "the must-carry
  rule `TangentNotIntrinsic` fires only on `ContactMark::Tangent`".
- `geom_brep::tangent_second_order`'s doc (`crates/geom-brep/src/dihedral.rs`),
  on `Positive`: "a constructor that stores less is storing a
  description tier 3 will refuse".

The code disagrees. `crates/topo/src/validate.rs`, check 4, pushes
`ValidationError::TangentNotIntrinsic` only when
`mark == ContactMark::Tangent && curve.authority().is_declared() &&
tangent_certificate_lane(..)`. So a DERIVED chart image on a
jet-determinate edge passes.

Measured: before PR 4157, `splitting/finish.rs` stored a derived
`chart(s_self)` on the π seam of
`sweep/tests/wedge_end_doors.rs`'s
`a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`. That is plane
against cylinder, κ_rel = 1/r, in lane. Both products passed
`validate_geometric`, and that test asserted it. PR 4157 makes the
split store the intrinsic description, so the edge no longer tests the
gap.

The question for the owner: did the doc rot, or did the code drift? If
the answer changes C7 or D2, it waits for Ev. Either way, every
constructor that routes through `geom_brep::must_carry_over_edge` now
stores the intrinsic description where the rule demands it, so tier 3
is the backstop only for a constructor that does not.
