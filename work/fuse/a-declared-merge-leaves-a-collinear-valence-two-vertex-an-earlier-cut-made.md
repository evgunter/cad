---
id: a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made
kind: issue
title: A declared coplanar merge leaves a collinear valence-2 vertex that an earlier fold step's cut made, so a union's finished body depends on member order
status: dispatched
priority: P1
cost: M
opened: 2026-09-24
refs: [declared-flush-union-edge-and-vertex-names-follow-member-order, sweeps-build-one-rim-edge-per-segment-not-per-run, curved-joinable-vertices-are-left-unjoined]
branch: fuse/join-every-stage
---


## The finding

A `Node::Union` folds its members pairwise in list order (DM4). With a
declared flush pair, the finished body's VERTEX SET depends on that
order.

Found by EMIT while working
`work/emit/declared-flush-union-edge-and-vertex-names-follow-member-order.md`,
and measured with a scratch probe that prints each fused order's
vertices at x = 0.5. The probe ran on `emit/shared-rim-several` at
`4140d6886c`, whose union code is now on main, before any change of
`emit/declared-flush-order`. The counts are the kernel's body, which
naming does not touch.

The document:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1);
- `a` and `b` declared flush on both caps and both y-walls;
- a slab `s` = x∈(0.49999,0.50001), y∈(−1,2), z∈(0.5,2.5).

What each order gives:

| order | vertices | edges | faces |
|---|---|---|---|
| `[a, b, s]`, `[b, a, s]` | 30 | 42 | 14 |
| `[b, s, a]`, `[s, b, a]` | 32 | 44 | 14 |
| `[a, s, b]`, `[s, a, b]` | refuse (`DeclareResolve`, other rows) | | |

The two extra vertices are (0.5, 0, 0.5) and (0.5, 1, 0.5). Each has
valence 2. Its two edges run along x at z = 0.5 on the slab's bottom,
to (0.49999, y, 0.5) and (0.50001, y, 0.5). They are collinear and
opposed, and both lie between the same two faces: the slab's bottom
cap and the merged y-wall.

## How the vertex is left

1. The step `b ∪ s` cuts `b`'s vertical edge at x = 0.5, y = 0 with
   the slab's bottom. That mints (0.5, 0, 0.5) at valence 3.
2. The step `(b ∪ s) ∪ a` merges `a`'s y = 0 wall with `b`'s, which is
   declared coplanar. That kills the edge between them, the stretch of
   `b`'s old vertical edge below z = 0.5
   (`Body::merge_coplanar_faces_declared`, `crates/topo/src/merge_faces.rs`).
3. The vertex drops to valence 2 and stays.

In the orders that fold `a` and `b` first, the slab cuts the already
merged wall and no such vertex is ever made.

The straight-seam repair (`kev`, gated by
`Body::redundant_subdivision_vertex`) removes a valence-2 collinear
vertex only on the seam the merge group is killing. It does not look at
the group's boundary vertices, which the `kef` leaves at valence 2.

**It is specific to the declaration.** The same slab with `a` alone,
or with an undeclared `b` = x∈(0.5,1.5), y∈(0.2,0.8), z∈(0.2,0.8), at
±1e-5 and at ±1e-7, gives one vertex set in every order.

## Why it matters

Naming cannot make a union's names independent of member order when
its entities are not. In the orders above:
- the seam edge along the slab's bottom is one edge in one pair of
  orders and two edges in the other, so its names differ;
- the two extra vertices are named in two orders and absent in two;
- the merged wall's name binds different vertex sets.

EMIT's union naming is order-free only as far as the finished body is
(`crates/editor-core/src/names/emit_union.rs`, module doc). This is the
body-level half.

## The design question in the fix

A valence-2 vertex between two collinear, opposed edges on the same
two faces is redundant in any order. The repair that
`redundant_subdivision_vertex`'s docs argue for is "collinearity, not
provenance". Applied to every vertex a merge leaves in that shape, it
also removes the flush corners that every order has today, such as
`b`'s corner (0.5, 0, 0) on `a`'s bottom rim.

Removing those changes what the union names. The flush rim would become
one edge x∈(0,1.5), which lies within neither member's edge. The
naming vocabulary has no name for an edge made of two members' edges.
Keeping the corners and removing only the vertices a cut made is
provenance-based, and the kernel's docs refuse that.

So this needs a ruling on the canonical form before it is a unit. The
owner of the choice is ZIP, with EMIT consulted on the naming
consequence.

## The collinearity licence is gone (CONTACT-8, 2026-09-28)

`Body::redundant_subdivision_vertex` and its straight-seam gate no
longer exist. The merge now deletes a vertex only as the free end of a
shared edge the glue left dangling inside the merged face, at any
angle, decided by topology alone (`merge_group` in
`crates/topo/src/merge_faces.rs`). The vertices this row is about are
BOUNDARY vertices of the merged face (valence 2 between two live edges
on the slab's bottom and the merged wall), so the pruning does not
touch them, and the ratified maximal-faces clause in `docs/DESIGN.md`
now says the merge never removes a vertex from a face's boundary. The
"collinearity, not provenance" argument quoted under *The design
question in the fix* has no site left; the question stands as a
canonical-form ruling, and any boundary-vertex elision it chose would
re-open the record-carriage class that clause names.

## Ruled (Ev, PR 3881, 2026-10-03)

Maximal edges everywhere, with contact records restructured as cell
pairs that a join carries by substitution. A union's body is then the
unique complex with maximal faces and maximal edges over its face
partition, the same in every member order, and the form is checked at
tier 2 on the result alone. The ratified text is `docs/DESIGN.md`'s
merge-stage clause and DM4. The measured case for it: on the three
edge-contact documents, tier 3′ accepts 13 of 13 orders, all V16 E24.

Build order, each step its own unit:
1. Contact records as cell pairs, including the vertex-on-edge record
   `(vertex, edge)`. The census certifies vertex/edge and edge/edge
   kinds. Substitution carriage goes through a join, and edge-split
   lineage carries a `(vertex, E)` record onto the piece of a split `E`.
2. The join op at every output stage (boolean and sweep), with sweeps
   building one rim edge per run.
3. The tier-2 check that no joinable vertex remains.
4. Naming: a union edge spanning several member edges is named for the
   set (EMIT's ground).

## Step 1 landed (FUSE, PR 3955, 2026-10-06)

Contact records are cell pairs, recorded coincidences under D10:
- **Record kinds:** `VeContact (vertex, edge)`, and `EeContact` for
  two edge interiors that meet (crossing or overlapping). The census
  certifies both kinds in both directions.
- **Carriage:** one substitution door (`ops::carry`) replaces
  `remap_contacts` and `remap_carried`, and routes through main's link
  doors. Zip, merge and join rows are substitution sources.
- **Edge-split lineage** places a `(vertex, edge)` record on the
  piece of the split edge the vertex lies on.
- **The join:** `boolean/edge_join.rs` (`joinable_vertices`,
  `join_edges`) exists but is not wired into any output stage yet. On
  PR 3881's three documents it certifies 18 of 18 orders at V16 E24.
- **DESIGN.md tier 3′** restates the ruled structure in D10 terms.
- **Folded in and closed:** the P1
  `a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped`.
- **Review:** dual. Lane 2's three MAJORs (zip pairing off by one; the
  join dropping a crossing point contact; lineage's stay-on-parent arm
  untested) were fixed and re-checked. Lane 1 found D10 conformance
  held.
- **Residue:**
  - `an-edge-edge-record-has-no-carriage-into-a-later-op` (P2, step 2);
  - ZIP's re-filed `a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row`
    (P3);
  - INTENT stage 4 retires `StaleDeclaration::VertexOnEdge` / `EdgeEdge`
    and their tags (list in the PR body).

## Step 2 (FUSE, PR 4140, 2026-10-06)

Built: the boolean half of step 2. Steps 2 and 4 land together: the
join makes a flush rim one edge across two members' rims, which the
pair emitter can only name as a set (step 4, PR 4161 on
`fuse/set-names`, based on this branch).

- **The join at every boolean output stage:** `edge_join::join_stage`
  runs after the merge in the seamed path, the graft and
  single-operand fallbacks and the declared-REST lane, and writes its
  substitution rows into the op's one `Descendants`. Every output has
  maximal edges for the planar inventory.
- **The join's chords along an edge are substitution rows.** A chord
  the join mints on a segment whose locus is an edge holds that edge's
  interior where the op drops the edge (`join::Connected::along`), so a
  carried record on it lands without a search of the result.
- **Edge-edge carriage:** `CarriedContacts.ee`, checked at the door
  (line edges only), and `ee_lineage` placing a split edge's record on
  its pieces by the census's own segment questions. `split_lineage`'s
  vertex-on-edge side asks the same questions.
- **Fail-loud:** every pair `carry` drops is an explicit arm with its
  reason; the census's segment questions refuse a `None` with no
  escalation.
- **Naming facts for step 4:** `BooleanNaming::edge_joins`,
  `joined_edge`, `joined_into`, `stretch_through_joins`.
- **Review:** dual. Lane 1's two MAJORs (a door search for an edge's
  holder; untested stage paths) fixed; lane 2 accounted for the perf12
  census golden's every moved row.
- **Not built, filed P1:**
  - `sweeps-build-one-rim-edge-per-segment-not-per-run` (sweeps);
  - `curved-joinable-vertices-are-left-unjoined` (curved joins; ruled
    in PR 3881).
  Both precede step 3.

**Next:**
- step 2's rest: sweeps building one rim per run, and curved joins
  (the two P1 rows above);
- step 3, the tier-2 no-joinable-vertex check;
- step 4, merged-set edge names (EMIT).


## The refusing orders reach `SeamVertexParentage` (EMIT, 2026-10-06)

The table's `[a, s, b]` and `[s, a, b]` refuse `DeclareResolve` today.
With that refusal gone (INTENT's stage 4, or a scratch fan-out), they
refuse `NamingError::SeamVertexParentage` instead, and
`emit_union_flush_names::a_seam_a_leftover_vertex_splits_is_published_twice_under_two_names`
meets it in `[0,2,1]`. Measured in
`work/emit/union-refuses-in-some-member-orders-and-publishes-in-others.md`,
"Re-measured on main (2026-10-06)".
