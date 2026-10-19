---
id: a-split-through-a-pinch-names-both-tip-copies-alike
kind: issue
title: A split whose plane passes through a pinch vertex refuses Naming(Duplicate): both copies of the vertex on one side are OnToolVertex of it
status: closed
opened: 2026-10-08
priority: P1
cost: M
pr: 5275
closed: 2026-10-18
---


## What

A split whose plane passes through a vertex that the cut leaves pinched
(its neighbourhood has two runs on one side, so two pieces of that side
touch there) refuses `NodeErrorKind::Naming(Duplicate)`. The split's
reduction gives the vertex one copy per run (`topo/src/splitting/insert.rs`
`insert_null_edges`), so the pinched side holds two vertices at the one
point, and `name_split`'s vertex pass
(`crates/editor-core/src/names/emit_topo.rs`, the `OnToolVertex` arm,
"the plane passed THROUGH an operand vertex") names each of them
`OnToolVertex { side, of }` with the operand vertex as `of`. Two
entities, one name.

Reproducer: the notched block of `topo/tests/m3_pr3_split.rs`
(`NOTCHED`, a V-notch whose tip `(4, 1)` has both neighbours above
`y = 1`), extruded one unit through the document and split by the plane
`y = 1`. The kernel's split builds (its own suite pins the halves); the
document's split node refuses:

```
Naming(Duplicate { name: StableName { kind: Vertex, node: <split>,
  path: [OnToolVertex { side: Above, of: <extrude>.CapVertex(Start, Piece { role: Leg }) }] } })
```

## Why it matters now

INTENT stage 4 PR B records the split's pinch as a coincidence row
(`DecisionSite::SplitOn`, `topo/src/splitting/mod.rs` `reduce`), and
`coincidence_door.rs` would read that row through the document. With
this refusal no document can reach a pinch split, so the row is pinned
at the kernel only (`topo/tests/m3_pr3_split.rs`
`a_split_records_its_pinch_and_nothing_where_it_only_cuts`).

## Fix shape

The copies are told apart by the run each took (the run's sector face,
or which piece of the side holds it); the name needs that qualifier, or
the pair needs a tie (N2). Which one is a naming decision for the wire
program.

## Evidence: the boolean's answer to the same shape

The pair boolean met the same shape, an operand corner held as several
result vertices, and Ev ruled it (PR 4881, fork-log row 124): each copy
is `From(x, v) + Fragment(Borders)` over the walls that divide the
corner there (`crates/editor-core/src/names/README.md`, N2 *Vertices*),
read by `borders::Obstacles::split_corner`. The Split's pieces are told
apart by `Keeps`, not `Borders`, so the ruling does not settle this row,
but it is the precedent a Split qualifier would be weighed against.

## FORK-PINCHNAME (2026-10-17): converged, and the question is Ev's

Two designers, briefed with the problem only, converged in round 1 on the
same final state: where one half holds an operand corner as several
vertices, each is `OnToolVertex { of } + Fragment(Keeps)`, the sorted set
of the corner's own edges (cited by their lines) the copy is an end of a
piece of in that half; a lone copy keeps its bare name; equal sets tie.
That extends N2's corner sentence, which binds future naming, so it is an
`[ev]` question; the PR edits `names/README.md` N2 and its body carries both
reports. Both found the union half of the companion row stale (measured
order-free on today's main). Both also found the next refusal behind this
one: the on-plane edge copies, carried as one name twice, which `Ends`
separates once the vertices are told apart; the unit that builds this
covers both.

Signed (WIRE orchestrator).

## A second reproducer (2026-10-17)

`work/issues/a-split-touching-at-a-notch-apex-mints-a-vertex-twice.md`
(opened 2026-10-10) is this defect on a smaller fixture, the notched
pentagon `(0,0) (2,0) (2,2) (1,1) (0,2)` extruded 1 and split at `y = 1`,
with a Python reproducer. Its naming half is this row's; its second
finding (pncad-py's prose gate panics on the refusal's `Debug` role
path) moved to BIND, whose ground it is.

Signed (WIRE orchestrator).

## Ratified (2026-10-18)

Ev approved the rule on PR 5258 ("ah ok, this sounds good!"), after asking why the vertex is duplicated (answered on the PR: one result vertex per piece of material at the point; a shared vertex would be non-manifold). N2 now carries it. Ready to build with `a-mirrored-split-through-a-pinch-refuses-emission-at-an-on-plane-vertex` riding along.

Signed (WIRE orchestrator).

## Resolution (2026-10-18)

Closed by PR 5275, per N2's Split vertex sentence (Ev, PR 5258): each
copy is `OnToolVertex{of} + Fragment(Keeps)` over the operand corner's
own edges it ends a piece of in that half; pinned literally by
`crates/editor-core/tests/split_pinch_keeps.rs` over the notched,
mirrored and revolved fixtures.
