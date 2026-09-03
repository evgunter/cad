# ARMS-3 — general sphere×sphere, and what a run-out at a seam vertex IS

**Status: RATIFIED** (Ev's 👍 on #992). Implemented by
VERBS-ARMS-3 (#1028), with one correction recorded below — A3-2's
RECOURSE rested on a premise the implementation lane found false
when it reproduced the witness. (VERBS program; the ARMS cut's
third unit, whose corner half OQ6 explicitly reserved for Ev:
"run-out policies are a taxonomy decision Ev should own before
any lands".) Proposals A3-1..A3-3.

## A3-1 — the sphere×sphere arm is plumbing; it dispatches on ratification

The only general-position arm (two spheres always intersect in a
circle; no coaxiality condition), already derived by ARMS-2's
circle×circle sheet-crossing closed form — the meridian sheet
through the rim contains both centres' traces. It rides the
existing family: material sides from stored sense bits, both
configurations unit-rowed, poison at tangency. The consumer is a
lentil (the solid between two unit spheres, bored), whose CONVEX
equator is a full closed rim ARMS-1's annulus door serves; a
two-sphere snowman's waist is CONCAVE and the arm refuses it on
convexity. **No conversation content here**; listed so the unit's
scope is visible.

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

### Correction, at implementation (2026-08-26) — flagged for Ev

The recourse's parenthesis above — *"true since ARMS-1, and the
actual answer for every consumer met so far"* — is **false at the
sites where the tag fires**, and reproducing the witness is what
showed it. The two facts are structurally linked:

- A seam vertex exists only on a rim a chart seam has SPLIT, i.e.
  on a full revolve of a POLE-TOUCHING profile, where every wall
  becomes two half-bands and every latitude rim two arcs.
- A rim that is ONE self-closed edge — every annular revolve: the
  dome, the bud, the snowman, the lentil — is a CLOSED chain, which
  registers no corners at all, so no seam vertex is ever reached
  there.

So "request the full closed rim; the annulus door carves it" names
a door that cannot serve the caller who was just refused: their
whole-rim request is a MULTI-LINK closed chain, and the ring-free
annulus band is a one-edge rim's. Measured on the lantern fixture,
all three of its rims refuse one door later (mouth, neck and lip),
including #319's own plane×sphere neck rim.

**What shipped instead**: the tag and its substantive claim are
unchanged (the vertex is not a corner; no run-out policy applies;
`policy` is `None`). The recourse named the REQUEST rather than
promising the carve — *"request the rim whole — every arc the chart
seam split it into — rather than a chain that stops at the seam,
which is a chart artifact the surface is smooth through"* — and the
missing door was filed as **#1022** and carried in the register's
run-out row. That kept the refusal true, which was the whole point
of A3-2.

### Where that stands now (BLEND-1, #1022 closed)

The missing door was built: the closed-rim annulus band accepts a
MULTI-LINK closed chain whose links are one rim's arcs across chart
seams, so the request the recourse names is CARVED. The sentence
gained that half — and kept a hedge, by the same standard.

**The standard, restated because it is what this section is for.**
The tag's firing rule is pure INCIDENCE and never reads convexity,
so it fires at a CONCAVE seam-split rim's vertex exactly as readily;
the material-adding closed-rim band is unbuilt (**#1244**), so an
unconditional promise would be false at those sites — the same
defect shape this correction records, one door further on. The
recourse therefore states the carve for the CONVEX side and names
what a concave rim meets instead. A boolean-repaired pole-touching
body is a second such boundary (**#1245**), recorded in the
register's row rather than in the sentence, because the tag does not
fire there at all (that rim's ends are trivalent).

The rule this section stands for is unchanged and is what both
hedges cost: **a recourse must be true at every site its tag can
fire**, and narrowing the sentence is always available where
widening the carve is not.

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

ONE implementation unit (sphere×sphere arm + the `SeamVertex`
refusal + register/vocabulary sync). #319 closed fully at its
merge (the coaxial half closed at ARMS-2; the corner finding
resolved as A3-2's re-description). The parked run-out
pair joins the register with this doc as its design record.

**Delivered** (VERBS-ARMS-3): the arm as one row in `coaxial_arm`
plus one `BlendArm` variant — the circle×circle sheet crossing was
already there — unit-rowed in both material configurations beside
the other curved arms, with a lentil (two unit spheres, bored)
whose convex equator fillets end to end. `CornerConfig::SeamVertex`
with `policy: Option<RunOutPolicy>` (the tag's own map is the one
source, so a payload cannot disagree with its tag) and its own
recourse. The parked pair sits in the register's new run-out row
with this doc as its record; the correction above is #1022.
