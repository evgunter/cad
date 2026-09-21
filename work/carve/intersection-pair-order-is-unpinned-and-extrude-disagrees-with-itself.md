---
id: intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself
kind: issue
title: EdgeDescription::Intersection's (s1, s2) order is unpinned, and extrude writes it one way on cap rims and another on struts
status: open
opened: 2026-09-16
refs: [brick-has-two-constructions-and-two-homes]
priority: P0
cost: D
---


## Finding

- **Where**: the extrude machinery under `crates/sweep/src/` that mints
  `EdgeDescription::Intersection` for a prism's edges, against
  `geom_brep`'s `EdgeDescription::Intersection` docs, which state no
  order convention for the pair.
- **Importance**: low today, medium latent
- **Confidence**: sure about the asymmetry (measured); unsure whether
  the right remedy is a ratified order or a type that cannot carry one
- **Raised by**: S-DUP's `dup-brick-measure` lane, 2026-09-16, out of
  `work/dup/brick-has-two-constructions-and-two-homes.md` — found while
  measuring two builders of the same box against each other, not while
  looking for this

**The rule that is not a rule.** `topo`'s test-side
`describe_as_intersections` sets `s1 = surface(face(he_plus))` and
`s2 = surface(face(he_minus))`, and that holds on **12 of 12** edges of
the Euler-built box. On an extruded box — `sweep::test_support::prism_at`
over a rectangle, which is what `sweep::test_support::brick` was when
this row was opened — it holds on **8 of 12**: the four bottom-rim
edges (bottom cap against a side wall) carry the pair swapped, while
the four struts and the four top-rim edges agree.

**`he_plus` and `he_minus` are identical between the two bodies** — the
measurement compared them edge for edge — so this is not two different
topologies described consistently. It is one topology described by two
different rules, and the extrude machinery is naming the pair by
something other than the edge's own direction on the cap it closes.
Nothing else about the two bodies differs: same counts, same keys, same
arena order in every topological arena, same face surfaces and senses,
same `mass_properties` bits.

## Why it is a row and not a note

**Nothing reads the order today, and that is the whole hazard.** Every
consumer accepts either arrangement:

- `crates/topo/src/validate.rs`'s `DescriptionNotAdjacent` is
  `(s1 == fs_plus && s2 == fs_minus) || (s1 == fs_minus && s2 == fs_plus)`;
- `crates/topo/src/boolean/ops.rs`'s staleness test is the same shape;
- `Body::description_surfaces` is consumed only by a `contains`, a
  refcount and a `for`.

So neither builder violates a ratified rule, because there is no rule —
and an unordered pair carried in two ordered fields is the shape
`docs/prompts/reviewer-style-lane.md` Q7 names: an invariant held by
convention where a type would do. Two things follow. Every future
consumer has to remember to check both arrangements, and the day one
forgets, the bug is a cap rim on an extruded body and nothing else.
And **two bodies that are the same solid are not `Debug`-equal**, so
any row that compares bodies by dump trips on a difference that means
nothing — which is exactly how this was found.

## Measured 2026-09-19: nothing reads the order, by mutation

The claim above — *"nothing reads the order today"* — was read off
three consumers by eye. `brick-has-two-constructions-and-two-homes`'s
measurement disclosed that as a blind spot in its own words: *"Whether
the (s1, s2) order matters is read, not measured. No mutation was
planted to prove a swapped pair changes no verdict."*

It is measured now. `describe_as_intersections` — the step every
`topo::test_support` box, prism and cube builder ends with — was
changed to write `Intersection { s1: s2, s2: s1, witness }`, swapping
the pair on **every** edge of **every** fixture in the tree, and
`cargo test --workspace` was re-run at the merge base of
`dup/sweep-brick-delegation` (`07ba310ef`, default features):

**8231 passed, 0 failed, 75 binaries — identical to the unmutated
run.** Not one row in the workspace can see the pair order.

So the hazard this row names is the whole of it: the order is free, and
the day a consumer starts depending on it, nothing existing will say
so. That is an argument for fork (2) — a type that cannot carry an
order — over fork (1), since a ratified order with no guard behind it
would be the same unenforced convention under a better name.

**What the mutation could not see**: default features only (the
`interval` and `probe` lanes were not re-run under it), and the
`sweep` extrude machinery's own `Intersection` writes were not mutated
— only `topo`'s test-side describer. A consumer that reads the order
on an extruded body specifically is outside this measurement.

## The fork

1. **Ratify an order** — `s1` is the `he_plus` face's surface, say —
   and make the extrude path honour it, with something that goes red
   when it does not.
2. **Make the pair unordered in the type**, by a normalising
   constructor or a representation that cannot express an order, and
   delete the both-ways checks at the three consumers above.

(2) is the stronger answer if `Intersection`'s two surfaces are
genuinely symmetric, and the three both-ways consumers are evidence
that they are. (1) is cheaper and keeps the dumps stable.

Related, on this slate:
`work/blend/blend-contact-edges-mint-the-intrinsic-description-without-the-rule.md`
— a different question in the same family (*which* description an edge
carries, rather than the order inside one), and worth reading beside
this because both are about a description minted without the rule that
governs it.

## Provenance of the measurement

Two throwaway probes, one in each crate's test binary, sharing **one**
dump function by `include!` so the two sides could not drift; the topo
side ran against the real `common::prism_z`, nothing vendored. Full
numbers, and what the probe could not see (f64 only; two axis-aligned
boxes; default and 1e-12 eps), are in
`work/dup/brick-has-two-constructions-and-two-homes.md`'s
`## Measurement (2026-09-16)`.

## Reference retired at BLEND's sweep (2026-09-17)

This row's `refs` named `blend-contact-edges-mint-the-intrinsic-description-without-the-rule`,
BLEND's unit 14, closed at its merge (PR #2509) and deleted with
`work/blend/` at `docs/DOC-LEDGER.md` sweep 17; the finding it pointed
at (the blend's contact edges routed through the must-carry rule) is
in the tree, so the reference is dropped rather than re-pointed.
