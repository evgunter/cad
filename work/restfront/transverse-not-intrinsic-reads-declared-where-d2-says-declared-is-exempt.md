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
