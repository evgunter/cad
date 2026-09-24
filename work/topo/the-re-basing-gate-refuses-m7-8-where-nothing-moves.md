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
took the brief's shape 1: the gate now refuses a null edge in a moved
run full stop (`EulerOpError::RebasedNullEdge`), and this row's
over-refusal stays where it is, stated in `Body::certify_rebased_run`'s
and `Body::mev`'s rustdoc and pinned by
`euler::tests::the_gate_refuses_the_plane_x_nurbs_class_where_nothing_moves_and_mev_null_splits_it`.
Shape 2 was not built because the surface it changes is ratified:

- `docs/DESIGN.md` Q1, "**`Real` trait surface**: comparison-free by
  construction (no `PartialOrd`/`PartialEq`, plus a style rule and a
  CI tripwire for the residual channels)" — written by `03353d5da`
  ("docs: ratify D4 ¶1 revision …, Q1 residue status", Ev, 2026-07-16;
  `git log --all -S'comparison-free by construction' -- docs/DESIGN.md`,
  a non-graft commit).
- `docs/DESIGN.md`'s standing outcome "**Production bit-identity
  coincidence checking is RETIRED** (Ev, #53; #102)", with
  `geom_core::bit_identity`'s production allowlist EMPTY and
  `scripts/gates/bit-identity-consumer.sh` armed; and
  `crates/geom-core/src/bit_identity.rs`'s header, "deliberately NOT
  part of the [`Real`] trait surface" (`19032e09f`, Ev, 2026-07-21).

**The question for Ev.** May a kernel gate ask "is `p_new` the point
`p_old`, bit for bit", and through which door? Three shapes:

1. **`Real::is_bitwise(self, other) -> bool`** (or
   `Point3::structurally_identical`), argued as `Real::is_poison` is:
   structural discrimination, not a geometric decision. The argument
   is weaker than `is_poison`'s: `is_poison` asks one value about its
   own structure, while this asks whether two values are the same
   description bit for bit — which is word for word what
   `bit_identity::eq_bits` answers and N6 retired from production. It
   is not `register_equal` (a registered-identity axiom, allowlisted by
   site, whose witness "cannot tell an identity from a coincidence"),
   but it IS a second door onto the retired channel, on the surface Q1
   keeps comparison-free.
2. **A production consumer of `bit_identity::eq_bits`** at the gate,
   allowlisted in `bit-identity-consumer.sh` with N6's
   retirement-scheduled note. The same capability without widening
   `Real`, at the cost of the empty allowlist.
3. **No comparison: a certified no-move door.** A `mev` variant that
   copies `p_old` as `mev_null` does and certifies the new edge's
   spec against `(p_old, p_old)`, skipping the gate structurally. It
   needs nothing from `geom-core`. It exists today as two calls —
   `mev_null` then `set_edge_curve` on the new edge — and the row
   above pins that spelling on an M7-8 run; a one-door form is sugar.

**What shape 1 or 2 would buy, measured at the branch's merge base
`5a34a6342`.** Every `MevSite::Fan` site in production code (outside
`#[cfg(test)]` modules, `tests/` and the `test-support` fixtures) was
read: the certified `mev`/`mev_line` fan sites (`sweep`'s `extrude`,
`loft`, `revolve/{chain,full,partial}`, `blend/{surgery,open/planar}`,
`step-import`'s `assemble` and `adopt`, `topo`'s `boolean/vtxfac` chord)
are all struts (`he1 == he2`), whose run is empty and never reaches the
gate; the three run sites (`splitting/insert.rs`, `boolean/insert.rs`,
`boolean/vtxfac.rs`) call `mev_null`, which skips it. **No production
outcome flips under a comparison door.** What it would flip is two
rows' no-move points — the M7-8 row above, and
`the_gate_refuses_a_null_scaffolded_edge_in_a_moved_run_wherever_it_moves`
at the null edge's own point — both reachable only through the public
`mev` with a closed carrier for the new edge (its two ends are one
point), which is the shape-3 spelling's case. Shape 3 therefore buys
the same as 1 and 2 without the ratified surface; it is recommended.
