---
id: intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself
kind: issue
title: EdgeDescription::Intersection's (s1, s2) order is unpinned, and extrude writes it one way on cap rims and another on struts
status: dispatched
opened: 2026-09-16
refs: [2842]
priority: P0
cost: M
branch: carve/surface-pair-is-unordered
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
- `Named::keys` (`attach.rs`, the reading orphan hygiene and the
  validator take) is consumed only by an `any`, a
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

## Weighed (2026-10-06): the pair is unordered, and the type says so

Weighed by two designers (an Opus and a Fable lane, CARVE's sitting of
2026-10-06), who were given the problem and not the options. They
agreed on the final state, and neither found it to be Ev's fork, so it
is built (`memories/orchestration-model.md`).

- **The order carries nothing.** D2's locus is a component of S₁∩S₂,
  and every predicate the pair feeds is symmetric in it. Edge
  orientation is topology's (`he_plus`/`he_minus`), and `topo::revert`
  keeps descriptions verbatim across a swap. Pcurves are per face, and
  `mate_surface` reads the pair as a set. Roles (the plane against the
  NURBS wall) are recovered by kind. The tree has about eight local
  rules for minting the order, and about nine readers that check both
  ways (`Body::cites_pair` and its callers, an inline copy in
  `splitting/finish.rs`, `attach::Named::adjacent_to`, `mate_surface`,
  `certify::plane_nurbs_pair`, step-import's `recognize_pins`). Extrude
  is consistent with itself under a role rule, `(cap, wall)` on rims and
  `(prev, next)` on struts. The 8/12 disagreement comes from the test
  describer's `(he_plus, he_minus)` rule meeting it.
- **The one positional reader is diagnostic.** `CertCheck::Surface1Residual`,
  `Surface2Residual`, `WitnessSurface1` and `WitnessSurface2` name a
  slot, and which surface that slot holds depends on which builder
  minted the edge.
- **The final state.** A `geom_brep` `SurfacePair` holds two
  `SurfaceKey`s in private fields, stored in key order by its one
  constructor. It derives `Eq`/`Hash`, its `Debug` is set-shaped, and it
  offers `keys()`, `contains(k)`, `other(k)` and a remap through the
  constructor. `Intersection` and `TangentIntersection` hold
  `{ pair, witness }` on `EdgeDescription` and `EdgeDescriptionSpec`,
  with an `EdgeDescription::pair()` accessor. The both-ways checks
  collapse onto `==` and `contains`, and `Body::cites_pair` goes. The
  four position-named checks become key-carrying ones
  (`SurfaceResidual { surface }`, `WitnessSurfaceResidual { surface }`).
- **Same-surface** (the designers split; the orchestrator's call): the
  constructor stays infallible. `IntersectionSameSurface` stays a
  refusal at the certification door, where the other geometric
  refusals are and the body is untouched (D4). A fallible constructor
  would spread one geometric refusal to every mint site.
- **Rejected.** Ratifying `he_plus`'s face first: it copies topology
  into the description, so tier 3 would have to police the copy, it is
  undefined where `chord_join` describes a chord against an auxiliary
  plane no face wears, and it goes stale whenever an edge's halves
  swap. Deriving the pair from the edge's faces: the pair is the
  certificate's subject, which is how a re-chart is caught as stale.
- **Ratified text.** D2's `{ s1, s2, witness }` and the geom-brep
  README's C1/C7 notation are re-worded to follow the change. That is
  not a second decision.
- **Rides along.** `topo/src/replace_face.rs`'s
  `(n1, n2) = if s1 == old_key { (old_key, s2) } else { (s1, old_key) }`
  always equals `(s1, s2)`, so it goes with the change. Stale "`Seam`
  description" prose in `topo/src/attach.rs` (`set_edge_curve`) and
  `topo/src/boolean/ops.rs` is fixed in passing.
