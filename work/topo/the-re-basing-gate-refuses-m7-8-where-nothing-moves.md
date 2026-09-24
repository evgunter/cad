---
id: the-re-basing-gate-refuses-m7-8-where-nothing-moves
kind: issue
title: the re-basing gate refuses the plane x NURBS class even where the new vertex takes the old one's point
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
priority: P0
cost: H
pr: 3148
needs_ev: true
---

## What

Filed by S93's fix pass, as the residue of the one thing both reviews
asked for that could not be built.

`Body::certify_rebased_run` (`crates/topo/src/euler.rs`) asks
`EdgeCurve::recertify` of every re-based edge. For the plane × NURBS
`Intersection` class (M7-8) that door answers `CertifyError::Unimplemented`
BEFORE it reaches any endpoint check — it needs an injected lane the
operator's bound cannot supply — so the gate refuses, and it refuses
**whether or not the move moves anything**. A fan `mev` whose new
vertex takes the old vertex's own point is a surgery in which no
endpoint changes, and on such a body it still cannot run.

That is the one case in which `Body::mev`'s rustdoc could not be made
true as written; it now says so instead of claiming otherwise.

## Why the obvious fix is not available

The exact question is *is `p_new` the same point as `p_old`* — bitwise,
not within band, because a point within band of the old one is still a
move and carrying a certificate across it is the staleness S93 exists
to close. `Point3<T>` at `T: Real` has no door for it:

- `Point3` derives `Clone, Copy, Debug` and no `PartialEq`
  (`crates/geom-core/src/linalg/point.rs`), deliberately — a point
  comparison is a decision;
- `Real` offers no bit accessor; `Real::register_equal` is a
  registered-identity axiom allowlisted by SITE
  (`scripts/gates/register-equal-allowlist.sh`) and is not an equality;
- `Decide::enclosure_probe` is documented as "an instrument, not a
  decision channel: nothing in the funnel may branch on it";
- `Decide::sign_within` over `distance_squared` is a BAND decision,
  which is the wrong question and has an `Indeterminate` arm.

That list holds at `T: Real`, the gate's bound, and not above it: at
`T: Bounds` the tree already has a bit comparison of points,
`crates/topo/src/query.rs`'s `same_point_bits`, which `rim_of` uses in
production (below, and
`work/tquery/rim-of-compares-point-bits-in-production-where-no-gate-looks.md`).
So what keeps the gate from asking is the retirement, not the absence
of a door.

`Body::mev_null` rests on the same fact structurally rather than
numerically — it COPIES the old point, so it never has to ask — and
that is why it skips the gate.

## The gate's other arm, which did land

The endpoint-residual arm of the same question needed no comparison:
where the re-certification fails on `EndpointStart`/`EndpointEnd`, the
gate re-asks against the endpoints the edge has NOW and carries an
identical answer, so a carrier `kev`'s fan merge had already made stale
is never named by a `mev` that does not touch it. That closes the
reviewers' third goal; this row is the first two.

## Shapes

- **An exact structural-identity door on `Point3<T>`** (a
  `fn is_bitwise(self, other: Self) -> bool` on `Real`, or a
  `Point3::structurally_identical`), with the Q1 argument that it is
  STRUCTURAL discrimination rather than a geometric decision — the
  argument `Real::is_poison` already carries in its own doc. A
  geom-core design change, not `topo`'s to make alone, and it would
  also answer `a-null-edge-can-be-re-based-onto-a-distinct-point`.
- **Carry `Unimplemented` when it was already `Unimplemented` at rest**,
  the same differential arm the endpoint residuals take. It makes the
  coincident case right and the MOVED case wrong: an M7-8 edge would be
  re-based onto a different vertex with a certificate nobody can check,
  which is the S93 defect for that class.
- **Leave it, and say so** — where it is now.

The first is the only one that is right in both directions, and it is
the expensive one because it is another crate's ratified surface.

## Shape 2 is Ev's: the proposal, with its measurement (TOPO-B5 slot 2)

TOPO-B5 slot 2 (branch `topo/rebasing-gate-null-edges-and-no-move`)
took the brief's shape 1. The gate now refuses a null edge where the
moved run holds exactly ONE of its halves
(`EulerOpError::RebasedNullEdge`), and carries one whose two halves
both move, since both ends then land on the one new vertex with
nothing compared. This row's over-refusal stays where it is. The
reason for both is stated once, in `Body::certify_rebased_run`'s
rustdoc; the over-refusal is pinned through the public `Body::mev` by
`euler::tests::a_fan_mev_refuses_the_plane_x_nurbs_class_where_nothing_moves_and_mev_null_splits_it`.

**What governs is the retirement, not the absence of a door.** Three
facts, with where each was checked:

- `docs/DESIGN.md` Q1: "**`Real` trait surface**: comparison-free by
  construction (no `PartialOrd`/`PartialEq`, plus a style rule and a
  CI tripwire for the residual channels)". The sentence was written by
  `03353d5da` ("docs: ratify D4 ¶1 revision (single ε, derived angular
  thresholds), Q1 residue status …";
  `git log --all -S'comparison-free by construction' -- docs/DESIGN.md`,
  a commit with a parent, so not the shallow graft).
- `docs/DESIGN.md`'s standing outcome "**Production bit-identity
  coincidence checking is RETIRED** (Ev, #53; #102)": the production
  allowlist of `geom_core::bit_identity` is EMPTY and
  `scripts/gates/bit-identity-consumer.sh` is armed.
  `crates/geom-core/src/bit_identity.rs`'s header keeps the channel
  "deliberately NOT part of the [`Real`] trait surface", under its own
  "Fencing (Ev, #53/#57/#58)" heading.
- **A production bit comparison of points already exists.**
  `crates/topo/src/query.rs`'s `same_bits` / `same_point_bits<T: Bounds>`
  compare `lo().to_bits()` and `hi().to_bits()`, and `rim_of`'s
  `CircleId::same_circle` decides with them which arcs are one rim. It
  landed in `c512a2e34` (2026-09-04) with no ratification found, and
  `bit-identity-consumer.sh` matches only
  `bit_identity::|repr_bits|eq_bits`, so it cannot see it. Filed as
  `work/tquery/rim-of-compares-point-bits-in-production-where-no-gate-looks.md`.
  The gate could not use that helper as it stands: `Body::mev` is
  bounded `T: Real`, and the helper needs `T: Bounds`.

**The question for Ev.** May a kernel gate ask "is `p_new` the point
`p_old`, bit for bit", and through which door? `query.rs`'s compare
can be read two ways, and each option below means something different
under each reading:

- **Reading A, a precedent**: comparing two stored values' bits to ask
  "is this the same stored value" is structural discrimination, not a
  coincidence decision, and the retirement does not cover it.
  `same_circle` is then legal as it stands, and the question here is
  only which bound and which door.
- **Reading B, a violation**: `same_circle` decides that two
  independently stored carriers are one locus by their bits, which is
  the retired coincidence channel reached through a door no gate
  watches. The tquery row then has to route it through the allowlist
  or retire it, and nothing here may cite it.

The three shapes, under each reading:

1. **`Real::is_bitwise(self, other) -> bool`** (or
   `Point3::structurally_identical`), argued as `Real::is_poison` is:
   structural discrimination, not a geometric decision. The argument
   is weaker than `is_poison`'s: `is_poison` asks one value about its
   own structure, and this asks whether two values are the same
   description bit for bit, which is word for word what
   `bit_identity::eq_bits` answers. *Under A* it moves a door that
   already exists at `T: Bounds` down to `T: Real`, and so onto the
   surface Q1 keeps comparison-free, which A by itself does not
   license. *Under B* it is a second door onto the retired channel, on
   that same surface.
2. **A production consumer of `bit_identity::eq_bits`** at the gate,
   allowlisted in `bit-identity-consumer.sh` with a
   retirement-scheduled note. The same capability without widening
   `Real`, at the cost of the empty allowlist. *Under A* it is the
   visible spelling of what `query.rs` already does out of the gate's
   sight, and `query.rs` should move onto it too. *Under B* it is the
   only honest spelling of any bit comparison, and `query.rs` has to
   answer the same way.
3. **No comparison: the no-move split.** Needs nothing from
   `geom-core` under either reading. It exists as two calls,
   `mev_null` and then `set_edge_curve` on the new edge, and it is the
   no-move SPLIT, not a no-move `mev`:
   - **not atomic.** A failing second call leaves a null edge at rest,
     which tier 2 refuses (`NullEdgeAtRest`), where the gate's `Err`
     leaves the body untouched. Pinned by
     `euler::tests::the_no_move_split_leaves_a_null_edge_at_rest_when_its_second_call_fails`.
   - **not at-rest certified.** On the M7-8 pillow the one spec that
     certifies at that point, a closed circle, passes tiers 1 and 2,
     and tier 3 adds `ScaffoldAtRest` and `PlanarBoundaryResidual` on
     the new edge to the pillow's own reading. No description could
     do better there, because the edge lies on the plane `y = 0` and
     the `z = 0` patch, which meet in a line. The M7-8 row above
     asserts that reading.

   A one-door form would make the split atomic by planning both steps
   before mutating. It would not make the result valid at rest, and
   neither would shape 1 or 2: a certified `mev` at the old point
   there would write the same geometry.

**What shape 1 or 2 would buy, measured at the branch's merge base
`5a34a6342`.** Every `MevSite::Fan` site in production code (outside
`#[cfg(test)]` modules, `tests/` and the `test-support` fixtures) was
read. The certified `mev`/`mev_line` fan sites (`sweep`'s `extrude`,
`loft`, `revolve/{chain,full,partial}`, `blend/{surgery,open/planar}`,
`step-import`'s `assemble` and `adopt`, `topo`'s `boolean/vtxfac`
chord) are all struts (`he1 == he2`), whose run is empty and never
reaches the gate. The three run sites (`splitting/insert.rs`,
`boolean/insert.rs`, `boolean/vtxfac.rs`) call `mev_null`, which skips
it. **No production outcome flips under a comparison door.** What it
would flip is the no-move point of three rows: the M7-8 row above
(`mev` at the old point with the closed spec), and the one-half null
rows at the null edge's own point
(`the_gate_refuses_a_run_that_moves_one_end_of_a_null_edge_wherever_it_moves`,
`a_fan_mev_refuses_to_move_one_end_of_a_null_edge_and_leaves_the_body_untouched`).
The both-halves case needs no door and carries today. All three are
reachable only through the public `mev` with a closed carrier for the
new edge (its two ends are one point).

So shapes 1 and 2 would buy one thing over shape 3, and no measured
caller needs it: atomicity for a no-move `mev`. TOPO recommends
**shape 3 as it stands**, the two-call split documented as a split,
and recommends settling reading A or B on the tquery row
independently, since `query.rs`'s compare is live in production
either way.
