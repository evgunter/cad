---
id: quadric-datums-unchecked-at-rest
kind: issue
title: Tier-3 check 1 checks the torus datums and no other quadric's
status: open
opened: 2026-09-05
refs: [S330]
track: P
---

## What

Disclosed while placing the named no-op arms that replaced check 1's
`_ => {}` (`crates/topo/src/validate.rs:3101-3112`). Each arm had to
state a true reason for saying nothing, and three of them state the
same one: **the datum is conventional-and-unchecked and no tier-3 check
reads it.**

- `Surface::Cylinder`'s `radius` (`crates/geom/src/surfaces.rs:121`) —
  "positive by convention", per the variant's own doc.
- `Surface::Sphere`'s `radius` (`crates/geom/src/surfaces.rs:198`).
- `Surface::Cone`'s `half_angle` (`crates/geom/src/surfaces.rs:163`),
  documented as ∈ (0, π/2).

The torus is the exception and has two checks at rest —
`NonpositiveTorusTube` and the `R > r > 0` ring convention
(`crates/topo/src/validate.rs:3054-3095`) — with an argument for why:
D3's convention is a **representability** claim, since a horn or
spindle torus puts a singular point on the axis that no chart in the
tree represents.

## The question this raises

That argument does not obviously stop at the torus. A cylinder of
radius `0` or `-1`, a sphere of radius `0`, a cone of half-angle `0` or
`π/2` are each a stored datum that **fails to describe a locus** rather
than describing a small one — the same sentence `NonpositiveTorusTube`
is justified by (`validate.rs:3044-3049`). Whether that makes them
tier-3 facts is a real question and not this unit's to answer: the
counter-argument is that `geom`'s conventional-and-unchecked rule
(`crates/geom/src/lib.rs:56-64`) says violating a convention yields
"well-defined garbage, not poison and not a panic", and tier 3 may be
entitled to take the convention at its word everywhere except where
representability is at stake.

What is NOT in doubt is that the asymmetry is currently undocumented as
a decision. Either the three quadrics get their datum checks, or the
torus arm's comment gets the sentence that says why they do not.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1). Reading, not editing,
`crates/geom/src/surfaces.rs` — a change to the conventions themselves
would be a `geom` question and a DESIGN.md one.
