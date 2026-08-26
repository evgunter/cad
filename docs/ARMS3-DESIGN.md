# ARMS-3 — general sphere×sphere, and what a run-out at a seam vertex IS

**Status: DRAFT — design conversation, awaiting Evan's sign-off**
(VERBS program; the ARMS cut's third unit, whose corner half OQ6
explicitly reserved for Evan: "run-out policies are a taxonomy
decision Evan should own before any lands"). Proposals A3-1..A3-3.

## A3-1 — the sphere×sphere arm is plumbing; it dispatches on ratification

The only general-position arm (two spheres always intersect in a
circle; no coaxiality condition), already derived by ARMS-2's
circle×circle sheet-crossing closed form — the meridian sheet
through the rim contains both centres' traces. It rides the
existing family: material sides from stored sense bits, both
configurations unit-rowed, poison at tangency. Consumer: the
snowman waist (two overlapping spheres), whose rims are FULL
closed rims — served by ARMS-1's annulus door. **No conversation
content here**; listed so the unit's scope is visible.

## A3-2 — the valence-4 "corner" is not a corner, and v1 should say so

The #319 second finding: an OPEN chain terminating at a rim vertex
where both supports carry their u = 0 seam meridian refuses
`FilletCornerUnsupported { NEdgeVertex { valence: 4 } }`. What that
vertex IS, geometrically: a point where the closed rim was split by
chart seams — the surface is SMOOTH through it (the seam is a chart
artifact; the dihedral along the rim does not change), and the two
extra incident edges are co-surface seam meridians with dihedral
zero. There is no wedge, no corner, no ball-rest configuration
distinct from the neighbouring rim points.

Consequently the OQ6 corner vocabulary is the wrong instrument
there: a "corner patch" at a smooth point has no geometric content,
and `RunOutStopAtVertex` as shipped (the sphere-octant machinery)
presumes a trihedral wedge that does not exist. **Recommendation:
v1 rules the seam-vertex termination ILL-POSED as a corner and
refuses with an honest, specific payload** — a new corner tag
(`SeamVertex`, zero constructor surface, per OQ6's
vocabulary-not-constructors ruling) whose recourse names the door
that exists: *request the full closed rim; the annulus door carves
it* (true since ARMS-1, and the actual answer for every consumer
met so far — the bud, the snowman, every solid of revolution).
This replaces a misdescribing refusal with a true one at zero
machinery cost, exactly the #554 shape.

## A3-3 — the genuine mid-curve run-out is REAL, parked, and named

A user who wants a fillet that stops PART-WAY along a smooth closed
rim (not at any vertex the topology gave them) wants a genuine
mid-curve termination policy. Two honest shapes exist:

- **Ball-cap stop**: the ball at rest at the final spine station
  caps the band with a sphere patch — well-defined at any smooth
  interior point, the `corner_ball` machinery's smooth sibling.
  New surgery (a cap face, two run-out trimline arcs), no new
  surface kinds.
- **Feather-out**: the blend radius tapers to zero approaching the
  station — OQ6's other named policy, variable-radius-shaped
  (frontier (f) adjacent), strictly more machinery.

**Recommendation: park both, consumer-gated**, with the ball-cap
named as the presumptive first pick when a consumer arrives.
Every consumer met by the whole ARMS program wanted the full rim;
building a reviewed termination policy with no caller is the
dead-code pattern the reviews punish. The register's fillet row
records the parked pair.

## Sequencing

On ratification: ONE implementation unit (sphere×sphere arm +
the `SeamVertex` refusal + register/vocabulary sync). #319 closes
fully at its merge (the coaxial half closed at ARMS-2; the corner
finding resolves as A3-2's re-description). The parked run-out
pair joins the register with this doc as its design record.
