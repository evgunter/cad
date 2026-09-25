---
id: structural-suffix-means-two-things-across-the-six-doors
kind: issue
title: The _structural suffix means two things: validate_geometric_structural drops check 7, the other four run it closed-form
status: open
opened: 2026-09-21
priority: P3
cost: D
parent: ATREST-10
---

## Finding

Six public at-rest doors carry the `_structural` suffix and the suffix
means two different things, uniformly across the six since LANE-1
(PR 3010) applied H5 ruling 3's door rename to the props and tier-3′
families. Executed by both LANE-1 reviewers on
`crates/topo/src/cert_m3r1_probes.rs`'s M7-8 cube (a described NURBS
wall whose flux the closed form cannot compute):

- `validate_geometric_structural` / `_structural_declared`
  (`crates/topo/src/validate.rs`) do NOT MAKE check 7 — the composed
  door's split moved both derivations, quadrature and closed form,
  into the private certified half, so the structural door answers
  `Ok(())` on that body.
- `validate_pseudomanifold_structural`, `contact_marks_structural`
  (`validate.rs`), `mass_properties_structural` and
  `classify_shells_structural` (`crates/topo/src/props.rs`) DO make
  check 7, through the closed form alone, and refuse the same body
  typed: `Err([VolumeUncomputable])`.

One suffix, two shapes. Each door's rustdoc now states which it is and
`validate_geometric_structural`'s doc states the two side by side, so
a reader at the door is not misled; what nothing decides is whether
the two should be one. The shape predates LANE-1 (the composed door's
split is the 2026-09-02 entry in `geom-core/src/real.rs`'s
`bounds_allowlist`; the three lane-keeping doors ran check 7 through
the closed form at a dual before they were renamed), so the unit that
made it uniform did not change it.

## What to do

Decide, on ATREST's ground, whether `validate_geometric_structural`
should make check 7 through the closed form like its four siblings
(one meaning for the suffix; the composed door's argument for moving
the whole check behind the certified bound — "the sign is decided in
exactly one place" — would need re-arguing) or whether the four
siblings should stop making it (then `_structural` means "no
certificate at all", and a dual caller that wants the sign has no
door — H-R3's capability). Either way the doc sentences at the six
doors and `topo/tests/geometric_cube.rs`'s
`the_structural_half_does_not_judge_orientation_at_any_scalar` move
with the choice.
