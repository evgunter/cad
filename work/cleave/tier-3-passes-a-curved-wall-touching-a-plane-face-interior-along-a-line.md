---
id: tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line
kind: issue
title: tier 3 passes a body whose curved wall touches a plane face's interior along a line with no edge for the contact
status: open
opened: 2026-10-02
priority: P2
cost: H
refs: [a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact]
---


## What

A split piece in which a hole's wall touches the cut face's INTERIOR
along a ruling passes `validate_closed` and `validate_geometric`.
It is a zero-thickness, undeclared tangent contact, and there is no
edge for it. `topo::contact_marks` cannot report it either, since it
has no edge to mark.

## How to reach it

Only under a plant, so far. In `splitting/rules.rs` `wall_graze`,
change `(false, WallBend::OutOfMaterial) => side.opposite()` to
`=> side`, so a concave graze is sent with its neighbours. Then a
round hole (r = 0.5 in a 4 × 4 plate, h = 1) grazed from inside at
any azimuth, a conical socket, or a counterbore
(`crates/sweep/tests/split_tangent_edge_curved.rs`, the concave rows)
answers the true volumes. For example 6.0 / 9.2146 for the hole at
y = 0.5. Both pieces pass tier 3, and neither carries a plane-to-curve
edge marked `Tangent` or `SmoothUnderdetermined`: the contact runs
through the cut face's interior.

The review that found this read the contacts as knife edges marked
`SmoothUnderdetermined`. Measured, those marks are the hole's own seam
edges, cylinder to cylinder, which the operand carries too.

The split refuses every concave graze it was tried on, and the guards
`a_concave_graze_of_a_round_hole_refuses` and
`a_concave_graze_of_a_revolved_hole_refuses` hold that. Two Booleans
reach the state without a plant (below).

## Why it matters

Tier 3 is what the at-rest gate trusts. A contact it cannot see is a
body it calls valid that a downstream mesh or offset will treat as
manifold material where there is none.

## Found by

CLEAVE DR-51's review of PR 3892, measured in its fix pass.

## Reachability, measured (2026-10-03)

**Unplanted: yes.** Two public Boolean calls reach it, at `f64` and at
`Interval`:

1. `union_with(plate, rod, decls)`: a 4 × 4 × 1 plate, a rod r = 0.5,
   z ∈ [0.25, 0.75] resting on the side face y = 2, the wall × face
   pairs declared `Tangent`. That answers two shells touching along
   x = 0, y = 2, which is the DEV-1 lane's designed answer.
2. A union of that with a box bridging the rod's top to the plate's top
   (x ∈ [−1, 1], y ∈ [1, 2.5], z ∈ [0.6, 1.6]). The answer is one
   shell, volume 18.2 + 0.85·π/8 = 18.5337942 (true). The rod wall lies
   on y = 2 (axis at y = 2.5, r = 0.5) along z ∈ [0.25, 0.6], and no
   edge has both ends on that ruling.

Plain, the second result carries no records and passes tier 3. The
census refuses it, `UndeclaredContact { VertexOnFace }` at the rod's
rim vertex (0, 2, 0.25). With the first result's records carried
(`CarriedVf`, `Tangent` or `Rest`), it passes tier 3 and
`validate_pseudomanifold` over its own records: every at-rest gate.
Reproducers (`#[ignore]`, pinning today's answer, a refusal expected):
`crates/sweep/tests/wall_face_tangent_reach.rs`,
`a_bridge_over_a_declared_tangent_rest_answers_an_edgeless_contact`
and its `_at_interval` twin. The door half is filed as
`work/contact/a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact.md`.

**The plant, reproduced.** With `wall_graze`'s
`(false, WallBend::OutOfMaterial)` arm returning `side`, the hole at
y = 0.5 answers 6.0 / 9.2146. Both pieces pass `validate_closed`,
`validate_geometric` and `validate_pseudomanifold` with no records, so
the census misses the plant's contact too. No edge lies on the ruling,
and `contact_marks` reports only `Transverse` and
`SmoothUnderdetermined` (the hole's seams). Unplanted, the same split
refuses `Join(DegenerateSection)`.

**Why tier 3 misses it.** Every check in `validate_geometric`'s list
(`crates/topo/src/validate.rs`) is local. Checks 1, 3, 6, 8 and 9 read
one face, 2, 4 and 5 one edge or one edge–face pair, 7 one solid's
volume, and 10 one vertex per shell. Two faces that share no edge
are compared by none of them. The doc's deferred list names this
class first: "Global self-intersection / minimum clearance".
`contact_marks` marks edges, and this contact has none. Check 9 sees
the contact only where it lands as a ring tangent to its own face's
outer loop: the bridge reaching y = 3.5 puts the rod's section circle
on the bridge's bottom face as such a ring, and tier 3 refuses that
body `RingMeetsOuter`. The census (`crates/topo/src/census.rs`) sees
vertices and edges against planar faces. It pairs faces only across
solids (`sweep_cross_solid_backstop`), so it sees this contact only
through a vertex the door happened to mint on the ruling, and one
vertex-on-face record silences it.

**What else was tried, and what it did.** "Refuses" is a typed
refusal. "Definite" means an offset past Kε, which answers a body with
a real gap or sliver: correct, and not this state.

| Input | Offsets | Outcome |
|---|---|---|
| Holed plate − / ∩ a box whose face is tangent to the hole (the split piece as a Boolean), six azimuths placed by `transform_rigid` | 0, ±1ε, ±5ε | refuses (`CurvedBooleanUnsupported`, `Escalated`) |
| same | ±20ε, ±1e3ε, ±1e6ε | definite: true volumes, no contact |
| Plate − a through or blind rod tangent to its side from inside | 0, ±1ε | refuses (`CurvedPierceUnsupported`, `Escalated`) |
| same | +1e3ε, +1e6ε / −1e3ε, −1e6ε | definite / refuses `SectionArcWindow` |
| Plate ∪ / ∩ / − a through or short rod tangent outside | 0, ±1ε / ±1e3ε, ±1e6ε | refuses / definite, or refuses `SectionArcWindow` |
| same, plate and rod rotated together (3 angles) | 0 | refuses (`CurvedPierceUnsupported`, `CurvedBooleanUnsupported`, a tangent-germ `SectionInvariant`) |
| Frame ∪ a rod in its slot touching the slot wall (short and through) | 0, ±1ε / +1e3ε, +1e6ε / −1e3ε, −1e6ε | refuses / refuses (`UndeclaredCoincidence` on the coplanar caps; `JoinDesync` on the through rod) / definite |
| Conical socket ∩ / − a half-space tangent along a ruling; frustum ∪ / ∩ one (4 azimuths) | 0 | refuses `CurvedPairUnsupported` (cone × plane has no arm) |
| Declared `Tangent`: plate − rod inside (through, blind, short), holed ∩ / − box | 0 | refuses (`ContactContradicted`, `RingHomingAmbiguous`) |
| Declared `Tangent`: a plane face that is not the touched one | 0 | refuses (`UnsupportedDeclarationClass`, `ContactContradicted`) |
| Declared `Tangent`: plate ∪ short rod outside; frame ∪ rod in slot | 0 | answers two touching shells with vertex-on-face records: the designed lane; tier 3′ passes with the records, refuses without |
| Declared: plate ∪ through rod (caps beyond the plate) | 0 | refuses `CurvedPierceUnsupported` |
| Declared rest ∪ bridge box, other bridges | 0 | y ≤ 2.5 bridge on top or underneath, and top from z = 0.5: answers the edgeless contact (above). y ≤ 3.5: answers, tier 3 refuses `RingMeetsOuter`. y ≤ 2.8, y ≤ 2.3 and three side bridges: refuse (`SectionArcWindow`, `VolumeUnmeasured`) |
| Rod ∪ bridge first, then plate ∪ that, declared or plain | 0 | refuses (`RingHomingAmbiguous`, `CurvedBooleanUnsupported`) |
| `graft_disjoint` of a rod tangent to the plate's side | 0 | tier 3 passes, as `instance.rs`'s module docs state for touching solids; the census refuses `CensusUndecidable` (cross-solid curved pair) |
| Split grazes of a through hole | ±1ε, ±5ε / ±20ε | refuses (`DegenerateSection`, `BellyGraze`) / definite |
| Split grazes of a blind hole, 4 poses | 0 | refuses (`DegenerateSection`, `SectionInvariant`) |
| Profiles: a ring tangent to the outer edge, two rings tangent, a rectangle ring tangent to a round one, an outer loop whose arc touches its own far edge | 0, ±1ε, ±5ε | refuses (`TangentialContact`, `NonSimple { Touch }`, `Escalated`) at profile validation, so extrude never sees them. At ±20ε: definite, or `NonSimple { Crossing }`, or (the self-touching loop) `UndeclaredTangency` at a joint of the fixture's own |
| `Interval`: the holed-plate, rod-in-side, rod-outside and profile rows, and the hole graze | 0 | refuses as at `f64` |

Revolve was not run. Its planar faces are meridian half-planes and
annuli about the axis, and a cone's or cylinder's ruling can reach one
only at the face's own boundary edge, so it is argued rather than
measured.

**Recommendation: P1.** The state is reachable through public doors
with no plant. In the carried form every at-rest gate passes it, and
the plain form's empty records make it tier-3 currency by the door's
own contract. By the letter of the bands this is "a live wrong
answer", which is P0. I recommend P1 because both reproducers start
from a deliberately declared tangent rest, and the volume is right.
The orchestrator should weigh that.
