# Selecting entities

A selection names faces, edges or vertices of a solid so a later step
— a fillet, a split, a measurement — can refer to them. This page is
how that is said: the naming table, the four whole-body
materializers, the pattern language over the SHAPE of a name, the
geometric filters that narrow one, the doors from a name back to
geometry, and how flush contact glues.

The vocabulary is `pncad::select`, and every worked example below is
a doctest of this crate.


## A selector is a materializer

`select` answers *as of one evaluation* and hands back
`Vec<StableName>`, exactly like `all_edges`. You store the
result; from that moment it is a frozen selection like any other.
There is no live query in a recipe — a stored "all edges" would
silently grow under an upstream edit, which is the staleness the
freeze exists to prevent.

## The patterns are structural; geometry is a second stage

`Selector` speaks role paths: which op minted the entity, which
role it plays, which side it is on, what its sub-names look like.
It cannot say "carrier is a circle" or "the two adjacent faces are
a plane and a sphere" — and it never will, because matching on the
name value ALONE is what makes a pattern reusable anywhere a name
exists. Geometry is a FILTER at the materializer instead:
`select_where` with a conjunction of `GeomPred` atoms
(`docs/SELECT-DESIGN.md` §§1-2).

The atoms split, and the split is the whole design:

- **EXACT** — `GeomPred::CurveKind`, `GeomPred::SurfaceKind`,
  `GeomPred::AdjacentKinds` read the carrier's enum TAG. No
  funnel, no margin, no refusal: the tag IS the semantic kind, so
  these are total and trivially equivariant. Dressing a
  tag match as a decided predicate would be dimension-laundering in
  the other direction.
- **DECIDED** — `GeomPred::PlaneDistance` is a real length
  comparison, so it is a `k_stats` funnel site
  (`SEL_PLANE_DISTANCE`) with an honest margin, and an in-band
  candidate REFUSES rather than being silently included or dropped.

Position is relative to the body's own geometry, never world-frame:
the distance is measured from the plane of one of the body's own
planar faces, named like any other entity, so the rule commutes with
rigid motions — the face moves with the body and the selection is
unchanged. Convexity is reserved and unbuilt.

```
use pncad::prelude::*;

let tol = Tol::witness();
// v4: the profile payload is its PROGRAM.
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node: Box::new(node), fresh: Vec::new() }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let len = |v: f64| Formula::literal(v, Dimension::Length).expect("a length");

let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { loops: vec![square], ids: Vec::new(), on: None }),
);
doc = next;
let (next, cube) = insert(&doc, Node::Extrude { profile: profile.into(), distance: len(1.0) });
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
let params = doc.var_env::<f64>();
let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));
let faces = Selector::of(NamePat::of_kind(EntityKind::Face));
let cap = |end| Selector::of(
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(end)),
);
// The face the position rule is written against: the cube's own
// bottom cap, the plane z = 0 with its outward normal along -z. It is
// a name like any other, so the rule moves WITH the part.
let [floor] = select(&ev, cube, &cap(CapEnd::Start)).try_into().expect("one bottom cap");

// EXACT: every edge of a box is a line. Exact atoms are TOTAL, so
// this can never refuse — and an empty conjunction is plain
// `select`, which is why the two agree name for name.
let straight = select_where(
    &ev, cube, &edges,
    &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
    &params,
    tol,
).expect("exact atoms cannot refuse");
assert_eq!(straight, select(&ev, cube, &edges));
assert_eq!(straight.len(), 12);

// EXACT, and UNORDERED across the edge: no plane/sphere rim here.
assert!(select_where(
    &ev, cube, &edges,
    &[GeomPred::AdjacentKinds(
        SurfaceKindSet::just(SurfaceKind::Plane),
        SurfaceKindSet::just(SurfaceKind::Sphere),
    )],
    &params,
    tol,
).expect("total").is_empty());

// DECIDED: the one face a metre inside the floor's plane (a signed
// distance along its outward normal), found by POSITION — and it is
// the same face the role path names.
let top = select_where(
    &ev, cube, &faces,
    // The comparand is a query's, not a slot's: a formula, evaluated
    // with no document in scope, so it reads no name.
    &[GeomPred::PlaneDistance {
        face: floor.clone(),
        cmp: Cmp::Approx,
        value: len(-1.0),
    }],
    &params,
    tol,
).expect("no candidate is in-band here");
assert_eq!(
    top,
    select(&ev, cube, &cap(CapEnd::End)),
);

// The comparand of a distance must BE a distance.
assert!(matches!(
    select_where(
        &ev, cube, &faces,
        &[GeomPred::PlaneDistance {
            face: floor,
            cmp: Cmp::Approx,
            value: Formula::literal(1.0, Dimension::Angle).expect("an angle"),
        }],
        &params,
        tol,
    ),
    Err(SelectRefusal::NotALength { .. }),
));
```

## The end-to-end form

Evaluate, select, store. "The cap rim of the top face" of an
extruded square, with no coordinate and no arena key anywhere:

```
use pncad::prelude::*;

let tol = Tol::witness();
// A unit box, authored through the document layer (v4: the
// profile payload is its PROGRAM — a chain of Formula-bearing steps).
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node: Box::new(node), fresh: Vec::new() }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let len = |v: f64| Formula::literal(v, Dimension::Length).expect("a length");
let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { loops: vec![square], ids: Vec::new(), on: None }),
);
doc = next;
let (next, cube) = insert(
    &doc,
    Node::Extrude {
        profile: profile.into(),
        distance: Formula::literal(1.0, Dimension::Length).expect("a length"),
    },
);
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);

// The whole-body materializers: one door per entity kind.
assert_eq!(all_faces(&ev, cube).len(), 6);
assert_eq!(all_edges(&ev, cube).len(), 12);
assert_eq!(all_vertices(&ev, cube).len(), 8);
assert_eq!(all_bodies(&ev, cube).len(), 1);

// "The cap rim of the top face" — one segment pattern.
let top_rim = Selector::of(
    NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::End)),
);
let names = select(&ev, cube, &top_rim);
assert_eq!(names.len(), 4);

// What comes back is a frozen, canonical set — ready to STORE.
assert!(names.windows(2).all(|w| w[0] < w[1]));
```

## The pattern forms, without an evaluation

Every pattern is a value with a total `Selector::matches`, so the
forms can be shown against a name built by hand.

```
use pncad::prelude::*;

let node = RecipeNodeId::new(0, 7);
let rim = |end| StableName {
    kind: EntityKind::Edge,
    node,
    path: vec![RoleSeg::RimEdge(end, ProfileEdgeRef::Piece { step: StepId::new(0, 3), role: PieceRole::Leg }.into())],
};

// `SegPat::tag` — the variant, arguments free.
let any_rim = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge));
assert!(Selector::of(any_rim.clone()).matches(&rim(CapEnd::End)));
assert!(Selector::of(any_rim).matches(&rim(CapEnd::Start)));

// `.side` — the end/side tag, taken by `From` so the role enum's
// own types spell it.
let top = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::End));
assert!(Selector::of(top.clone()).matches(&rim(CapEnd::End)));
assert!(!Selector::of(top).matches(&rim(CapEnd::Start)));

// `SegPat::group` — everything one op minted.
let swept = NamePat::any().seg(SegPat::group(OpGroup::Extrude));
assert!(Selector::of(swept).matches(&rim(CapEnd::End)));

// `NamePat::node` — restrict to one recipe node; `SegPat::any` and
// `NamePat::any` are the wildcards.
assert!(Selector::of(NamePat::any().node(node).seg(SegPat::any())).matches(&rim(CapEnd::End)));
assert!(!Selector::of(NamePat::any().node(RecipeNodeId::new(0, 8))).matches(&rim(CapEnd::End)));

// A constrained path matches length for length.
assert!(!Selector::of(NamePat::any().path([SegPat::any(), SegPat::any()]))
    .matches(&rim(CapEnd::End)));
```

## Alternatives, and sub-name arguments

A selector is a UNION of patterns — which is how a real selection
reads ("the box edges AND the pip rim") — and a segment's sub-NAME
arguments are patterns too. A segment's slotted arguments (a
`Crossing`'s edge, then its face) are matched slot for slot, as a
prefix. A set is matched as a set: a seam's two sides, `Merged`
constituents, an `EdgeCrossing`'s lines and a band's chain. Each
pattern matches a distinct member, in any order, so a seam pattern does
not depend on which side's name sorts first. A seam junction's path,
the run of `Seam` lines meeting at a vertex, is a set the same way:
each segment pattern matches a distinct line.

```
use pncad::prelude::*;

let node = RecipeNodeId::new(0, 3);
let face = |path| StableName { kind: EntityKind::Face, node, path };
// A boolean seam edge: the end cap of one operand crossing a
// revolve band of the other.
let seam = StableName {
    kind: EntityKind::Edge,
    node,
    path: vec![RoleSeg::Seam {
        a: face(vec![RoleSeg::Cap(CapEnd::End)]).into(),
        b: face(vec![RoleSeg::Band(ProfileEdgeRef::Piece { step: StepId::new(0, 1), role: PieceRole::Leg }.into())]).into(),
    }],
};

// `.of` — both sides constrained: "every Seam{Cap, Band} edge".
let cap_band = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of([
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::End)),
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Band)),
]));
assert!(Selector::of(cap_band).matches(&seam));

// A seam's sides are a set: the same pattern written band first
// matches too.
let band_cap = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of([
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Band)),
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::End)),
]));
assert!(Selector::of(band_cap).matches(&seam));

// Constraining one side is coarser and still honest: it matches a
// seam where either side fits.
let any_band_seam = NamePat::any().seg(SegPat::tag(SegTag::Seam).of([
    NamePat::any().seg(SegPat::tag(SegTag::Band)),
]));
assert!(Selector::of(any_band_seam).matches(&seam));

// Each pattern takes a side of its own: two caps fit no seam with one.
let two_caps = NamePat::any().seg(SegPat::tag(SegTag::Seam).of([
    NamePat::any().seg(SegPat::tag(SegTag::Cap)),
    NamePat::any().seg(SegPat::tag(SegTag::Cap)),
]));
assert!(!Selector::of(two_caps).matches(&seam));

// `Selector::any_of` / `.or` — the union. An EMPTY selector
// matches nothing.
let union = Selector::any_of([NamePat::of_kind(EntityKind::Vertex)])
    .or(NamePat::of_kind(EntityKind::Edge));
assert!(union.matches(&seam));
assert!(!Selector::default().matches(&seam));
```

## From a name to geometry

A selection is only half a question. The other half is "so where
IS the face I selected?", and the answer is a door that speaks
NAMES and hands back VALUES: `face_frame`, `edge_frame`,
`vertex_position`, and `denotation` for how many entities a name
denotes.

No arena key crosses this surface in either direction. Arena keys
are body-lineage-scoped and meaningful only against the evaluation
that built them, so the naming layer's rule is that they never
leave `editor-core` — which means a consumer never holds one, and
never has to launder one into a coordinate by indexing the body.
That is also why `denotation` answers with a COUNT rather than a
list of candidates: the candidates are keys.

What comes back is a `Pose`: the carrier's own stored frame,
copied out. A VALUE, never a verdict — no door here answers "is
this face planar" or "is this edge convex" — geometric predicates
are deferred. And no convention is invented where the
model fixes none: **a NURBS face has no canonical frame, so
`face_frame` refuses it** with `ReadbackError::NoCanonicalFrame`
rather than nominating S(0,0) and a chart normal as though the
kernel had chosen them. Analytic carriers all answer.

```
use pncad::prelude::*;

let tol = Tol::witness();
// v4: the profile payload is its PROGRAM — a chain
// of Formula-bearing steps.
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node: Box::new(node), fresh: Vec::new() }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let len = |v: f64| Formula::literal(v, Dimension::Length).expect("a length");
let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { loops: vec![square], ids: Vec::new(), on: None }),
);
doc = next;
let (next, cube) = insert(
    &doc,
    Node::Extrude {
        profile: profile.into(),
        distance: Formula::literal(1.0, Dimension::Length).expect("a length"),
    },
);
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);

// Select the end cap by NAME, then ask where it is.
let top = Selector::of(
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::End)),
);
let names = select(&ev, cube, &top);
assert_eq!(names.len(), 1);

// The name resolves uniquely...
assert_eq!(denotation(&ev, cube, &names[0]), Ok(Denotation::Unique));
// ...and the plane it names is z = 1. No arena key was held, and
// no coordinate was transcribed to learn it.
let pose = face_frame(&ev, cube, &names[0]).expect("an analytic cap");
assert_eq!(pose.origin.z, 1.0);
assert_eq!(pose.axis.z, 1.0);

// Vertices answer with their stored position.
let corners = all_vertices(&ev, cube);
let zs: Vec<f64> = corners
    .iter()
    .map(|n| vertex_position(&ev, cube, n).expect("a live vertex").z)
    .collect();
assert_eq!(zs.iter().filter(|z| **z == 1.0).count(), 4);

// Edges answer with their carrier's frame. Every edge of a box
// is a line: a direction, and honestly NO reference
// perpendicular — `u_ref` is `None` rather than invented.
let edge = &all_edges(&ev, cube)[0];
let line = edge_frame(&ev, cube, edge).expect("a certified carrier");
assert!(line.u_ref.is_none() && line.v_ref().is_none());

// A face door handed an EDGE name refuses by type, not by panic.
assert!(matches!(
    face_frame(&ev, cube, edge),
    Err(InterrogateError::WrongKind { .. })
));
```

## Flush contact glues

Two bodies that touch face-to-face glue where their margins decide
the faces one surface, and the boolean records each such coincidence,
and each corner it decided on the other body, on the node's value
(`NodeValue::coincidences`), each cell named in
the table of the member it came from. Nothing is declared: a pair the
margin leaves in the tolerance's ambiguity band refuses, and its
recourse is the geometry.

```
use pncad::prelude::*;
use pncad::document::{Bodies, BodyRead, BooleanValue, Coord, Operand, Placed, TargetAxis};

let tol = Tol::witness();
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node: Box::new(node), fresh: Vec::new() }, tol).expect("the edit applies");
    (applied.doc, applied.record.minted.expect("a minted id"))
};
let len = |v: f64| Formula::literal(v, Dimension::Length).expect("a length");
// v4: the profile payload is its PROGRAM, drawn in its own plane.
let footprint = |x0: f64, y0: f64, x1: f64, y1: f64| ProfileProgram {
    loops: vec![
        LoopProgram::polygon([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])
            .expect("finite corners"),
    ],
    ids: Vec::new(),
    on: None,
};

// A unit box, and a smaller box RESTING on its top cap. Each is a
// construction in its own coordinates; the block joins the box's space
// by a placement into it, a metre up that space's z, so the resting
// relation is visible in the tree before anything is evaluated.
let doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let (doc, pf1) = insert(&doc, Node::Profile(footprint(0.0, 0.0, 1.0, 1.0)));
let (doc, base) = insert(&doc, Node::Extrude { profile: pf1.into(), distance: len(1.0) });
let (doc, pf2) = insert(&doc, Node::Profile(footprint(0.25, 0.25, 0.75, 0.75)));
let (doc, block) = insert(&doc, Node::Extrude { profile: pf2.into(), distance: len(0.5) });
// The one bundle constructor: the slide its author writes, and the
// constant zero on every freedom left unnamed.
let placed = Placed::unmated(vec![(Coord::Slide(TargetAxis::Z), len(1.0))]);
// The copy is placed into the box's space, its bundle written whole.
let in_base = Operand::Var(doc.node_space(base).expect("a construction is in its own space"));
let (doc, resting) = insert(
    &doc,
    Node::place(BodyRead::plain(Operand::from(block)), in_base, placed.bundle()),
);

// The union glues the rest its margin decides, and records it.
let (doc, uni) = insert(
    &doc,
    Node::Union { members: Bodies::Spelled(vec![base.into(), resting.into()]) },
);
let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
let value = ev.value(uni).expect("the union evaluates");
let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
    panic!("expected a body");
};
let body = AtRestBody::validate(Body::clone(body.body()), tol).expect("a finished body");
assert_eq!(mass_properties(&body, tol, Settle::Resolution(Length::from_meters(1e-6))).expect("mass").met().expect("mass").volume, 1.125);
assert_eq!(
    value.coincidences.len(),
    5,
    "the one resting pair of faces, and the block's four corners on the cap"
);
```
