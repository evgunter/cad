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
