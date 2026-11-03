# PHASE 0: waiting for the orchestrator (splice/spline-join)

Scratch notes for `work/splice/joining-a-spline-carrier-is-unbuilt`;
this file comes out before the PR is opened for review.

## 1. Producer through the public boolean: none on main `d1ac6881`

Who can reach the door. `joinable` puts the edge that does not crease at
`w` in the kept role (`edge_join::crease_at`), so a `Nurbs` kept edge
needs a spline carrier whose parameter at `w` lies strictly inside its
domain and is no full-multiplicity knot. That carrier runs on past the
edge. Two things make one:

- (a) **a split of a spline edge mid-span.** `Body::split_edge` /
  `split_edge_onto` restrict the parent spec verbatim
  (`EdgeCurve::split_specs`): both children keep the parent's carrier
  with abutting intervals. The split vertex then has to come out of the
  op at valence 2 between the parent's two faces. That happens where the
  splitter contributes nothing at that point: a touch, or a flush
  contact in a difference.
- (b) **a minted spline edge over a sub-interval of a longer carrier.**
  The only section minter, `chord_join` (`cut_arc`), cuts clamped
  pieces, so both chord ends crease (PR 6013). I found no other minter
  in `topo`/`sweep`/`geom-brep` src that mints a `Curve3::Nurbs` edge
  over a sub-interval.

What I measured (scratch probe in `crates/sweep/tests`, not committed;
four ops each, `Tol::witness()`):

| operand | brick placements | every outcome |
|---|---|---|
| `lofted_cylinder(2)` (spline rims, seams, walls) | flush on the top cap straddling the rim; a flush strip; a corner on the rim; a side cut; a flush strip from below | refuses upstream: `CurvedPairUnsupported { site: RevertRoster, kind: Nurbs, other_kind: Plane }` (∩, both ∖), `CurvedBooleanUnsupported { kind: Nurbs }` (∪), `NurbsExtentUnsupported` (∪, corner), `GermFrameUnsupported { Nurbs × Plane }` (∪, side cut) |
| `disc_of_arcs(4)` with the top rim restated as exact rational quadratics (analytic faces, spline edges), pcurves minted | (not reached) | not an operand: `finished` fails `VolumeUncomputable { source: Face { .., Unimplemented } }` on the cap bounded by spline arcs |
| section-minted chords (`cylinder_sphere_frame`, `conic_edge_curved_face`) | (n/a) | every run stops at the result gate's volume, so no finished body carries a chord into a second boolean; and chords crease at both ends anyway |

None of `torus_doors_join`, `germ_tilted_rod`, `germ_torus_components`
or `cylinder_sphere_frame` pins the join stop in its expected tables
(the match arms are still there; no row expects them).

**The one public producer is construction, not the boolean**:
`Construct::split_edge` on a spline edge, then `Body::join_edges`.
`crates/sweep/tests/join_door_is_whole.rs` pins exactly that
(`JoinCarrierUnsupported { carrier: Nurbs, closed: false }` from the
`param_near` arm), and it is green on main. Through the boolean the door
is unreachable today. The first boolean producers will be shape (a) once
the spline × plane pair builds past `RevertRoster`, and the sibling
`splice/split-parameter` lane's point split on a spline boundary edge
(its halves are `split_edge` restrictions, so they are shape (a)).

## 2. The run-on's shape

**The joined edge's carrier is the kept edge's own carrier, unchanged:
no re-knotting, no concatenation, no inverse.** The far parameter is
carried, as `splice/split-parameter` carries the meeting's parameter:

- When `gone` lies on `kept`'s carrier (the same curve by content:
  knots, control points, weights, compared as values, which is what
  DESIGN [intrinsic] line ~141 allows when two things must be exactly
  the same; never by `Arc` address, D9), and both edges' parameters at
  `w` are equal exactly, then `gone`'s certified interval is an interval
  of that one carrier. `w` is one end of it and its far end is the
  other, so `tx` is `gone`'s own parameter at `far`. Shape (a) always
  meets this: a split keeps the carrier verbatim and both children are
  certified at the one split parameter.
- **The closed join needs no period**: two pieces of one closed spline
  carrier sharing both ends join over the union of their intervals,
  `tx` read the same way. The `Circle`/`Ellipse`/`Spiric` period arm is
  left as it is.
- **The interval is certified where every curved join is**: the spec
  goes through `kev_describing`, which certifies it at the merged
  endpoints and refuses one that does not certify. For a split pair the
  joined spec is the parent's spec over the parent's interval, with its
  selector re-pinned and its declaration restricted as now.
- **A crease at the joint cannot arise.** `kept` does not crease at `w`
  (that is how it was chosen). `gone` is on the same carrier at the same
  parameter, so it does not either. Each piece's interior is C¹ by its
  knot structure (Ev's PR 5913 ruling, quoted in the module docs). So
  the joined interval has no crease knot inside it, and the ruling holds
  on the output.
- **Concatenation is not needed.** Two spline pieces meeting at a knot
  of full multiplicity crease, and the ruled join leaves that vertex
  standing. Concatenating them would contradict that ruling, so the
  row's "or the two edges' spline images concatenated" is moot.
- **A general NURBS parameter inverse is not needed** by any producer.
  What is left, a kept spline beside a `gone` on a *different* carrier
  of the same locus that does not crease at `w` (another spline, or a
  line or circle), still refuses `JoinCarrierUnsupported` typed. It has
  no producer, and one would come with its own row. (Option, not
  recommended now: where carriers differ, put the analytic edge in the
  kept role so `param_near` runs it on. That is a role change with no
  witness.)
- **Scope**: the carry applies to `Nurbs` carriers only. Analytic
  carriers keep `param_near` and the period, so no analytic join's bits
  move in this unit. Taking the carry for every carrier is a possible
  follow-up; it would move bits in many curved-join rows.

Ratified text read: DESIGN.md "Maximal faces and maximal edges" and the
seam clause right after it ("not C¹ by its knot structure … The join
never removes a cut either exception requires"), [verdicts]
"Coincidence is a margined verdict", and the intrinsic clause on
comparing content; `crates/topo/README.md`'s maximal-edges paragraph
(joinable is one structural predicate, "nothing is compared between the
two edges") and tier 3's `JoinableVertexAtRest`. **None has to change.**
The predicate is untouched. The content comparison is in the
construction step (`joined_spec`), after joinability is decided, and it
picks how to state the joined edge, not whether to join. A mismatch
refuses typed and never builds anything else.

## Planned rows

- topo kernel row (in `edge_join`'s tests): a spline edge split at an
  interior parameter, then `join_edges`. The joined edge is the parent's
  carrier over the parent's interval (pinned by edge identity, ends and
  params), and nothing is left joinable. Plus a closed variant (one
  closed spline edge split twice, then joined), and a refusal row (a
  kept spline beside a `gone` on a different spline of the same locus
  still refuses `JoinCarrierUnsupported { carrier: Nurbs, closed: false }`).
- `join_door_is_whole`: its spline vertex now joins. It needs a refusal
  that only the kill meets, after an earlier join, and the
  different-carrier spline above is one: re-pin it there.
- A whole-arena equivalence row: split then join on a spline edge gives
  back the body's spec as found (carrier, interval, description,
  vertex/edge counts).
