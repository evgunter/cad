# Selecting entities

A selection names faces, edges or vertices of a solid so a later step
— a fillet, a split, a measurement — can refer to them. This page is
how that is said: the naming table, the four whole-body
materializers, the pattern language over the SHAPE of a name, the
geometric filters that narrow one, the doors from a name back to
geometry, and the detect/declare protocol for flush contact.

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
- **DECIDED** — `GeomPred::DatumDistance` is a real length
  comparison, so it is a `k_stats` funnel site
  (`SEL_DATUM_DISTANCE`) with an honest margin, and an in-band
  candidate REFUSES rather than being silently included or dropped.

Position is datum-RELATIVE, never world-frame: the datum is
a node reference like any other input, so the rule commutes with
rigid motions — move the datum with the part and the selection is
unchanged. Convexity is reserved and unbuilt.

```
use pncad::prelude::*;

let tol = Tol::witness();
// v4: the profile payload is its PROGRAM.
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("a scalar");

let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { plane: SketchPlane::xy(), loops: vec![square] }),
);
doc = next;
let (next, cube) = insert(&doc, Node::Extrude { profile, distance: len(1.0) });
doc = next;
// The datum the position rule is written against — one argument,
// and the rule now moves WITH the part.
let (next, ground) = insert(
    &doc,
    Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.0)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }),
);
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
let params = doc.param_env::<f64>();
let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));
let faces = Selector::of(NamePat::of_kind(EntityKind::Face));

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

// DECIDED: the one face a metre above the datum, found by
// POSITION — and it is the same face the role path names.
let top = select_where(
    &ev, cube, &faces,
    &[GeomPred::DatumDistance { datum: ground, cmp: Cmp::Approx, value: len(1.0) }],
    &params,
    tol,
).expect("no candidate is in-band here");
assert_eq!(
    top,
    select(&ev, cube, &Selector::of(
        NamePat::of_kind(EntityKind::Face)
            .seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
    )),
);

// The comparand of a distance must BE a distance.
assert!(matches!(
    select_where(
        &ev, cube, &faces,
        &[GeomPred::DatumDistance {
            datum: ground,
            cmp: Cmp::Approx,
            value: Expr::literal(1.0, Dimension::Angle).expect("an angle"),
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
// profile payload is its PROGRAM — a chain of Expr-bearing steps).
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { plane: SketchPlane::xy(), loops: vec![square] }),
);
doc = next;
let (next, cube) = insert(
    &doc,
    Node::Extrude {
        profile,
        distance: Expr::literal(1.0, Dimension::Length).expect("a length"),
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
    NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::Top)),
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

let node = RecipeNodeId(7);
let rim = |end| StableName {
    kind: EntityKind::Edge,
    node,
    path: vec![RoleSeg::RimEdge(end, ProfileEdgeRef { loop_index: 0, segment: 2 })],
};

// `SegPat::tag` — the variant, arguments free.
let any_rim = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge));
assert!(Selector::of(any_rim.clone()).matches(&rim(CapEnd::Top)));
assert!(Selector::of(any_rim).matches(&rim(CapEnd::Bottom)));

// `.side` — the end/side tag, taken by `From` so the role enum's
// own types spell it.
let top = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::Top));
assert!(Selector::of(top.clone()).matches(&rim(CapEnd::Top)));
assert!(!Selector::of(top).matches(&rim(CapEnd::Bottom)));

// `SegPat::group` — everything one op minted.
let swept = NamePat::any().seg(SegPat::group(OpGroup::Extrude));
assert!(Selector::of(swept).matches(&rim(CapEnd::Top)));

// `NamePat::node` — restrict to one recipe node; `SegPat::any` and
// `NamePat::any` are the wildcards.
assert!(Selector::of(NamePat::any().node(node).seg(SegPat::any())).matches(&rim(CapEnd::Top)));
assert!(!Selector::of(NamePat::any().node(RecipeNodeId(8))).matches(&rim(CapEnd::Top)));

// A constrained path matches length for length.
assert!(!Selector::of(NamePat::any().path([SegPat::any(), SegPat::any()]))
    .matches(&rim(CapEnd::Top)));
```

## Alternatives, and sub-name arguments

A selector is a UNION of patterns — which is how a real selection
reads ("the box edges AND the pip rim") — and a segment's sub-NAME
arguments are patterns too, positionally, as a prefix.

```
use pncad::prelude::*;

let node = RecipeNodeId(3);
let face = |path| StableName { kind: EntityKind::Face, node, path };
// A boolean seam edge: the top cap of one operand crossing a
// revolve band of the other.
let seam = StableName {
    kind: EntityKind::Edge,
    node,
    path: vec![RoleSeg::Seam {
        a: Box::new(face(vec![RoleSeg::Cap(CapEnd::Top)])),
        b: Box::new(face(vec![RoleSeg::Band(ProfileEdgeRef { loop_index: 0, segment: 0 })])),
    }],
};

// `.of` — both sides constrained: "every Seam{Cap, Band} edge".
let cap_band = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of([
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Band)),
]));
assert!(Selector::of(cap_band).matches(&seam));

// A PREFIX: constraining only the A side is coarser and still
// honest.
let any_cap_seam = NamePat::any().seg(SegPat::tag(SegTag::Seam).of([
    NamePat::any().seg(SegPat::tag(SegTag::Cap)),
]));
assert!(Selector::of(any_cap_seam).matches(&seam));

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
// of Expr-bearing steps.
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");
let mut doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
};
let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram { plane: SketchPlane::xy(), loops: vec![square] }),
);
doc = next;
let (next, cube) = insert(
    &doc,
    Node::Extrude {
        profile,
        distance: Expr::literal(1.0, Dimension::Length).expect("a length"),
    },
);
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);

// Select the top cap by NAME, then ask where it is.
let top = Selector::of(
    NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
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

## Detect and declare: flush contact as a conversation

Two bodies that touch face-to-face do not silently glue — the
boolean REFUSES an undeclared coincidence, and the recourse
menu has exactly two arms: declare the contact, or move the
geometry. This is the declare arm's protocol:
`find_flush_candidates` REPORTS the flush pairs as
`FlushFinding` values — the contact verifier itself run in
candidate-generation mode, so a finding can never disagree with
the boolean's own verify-at-use — and `declare` /
`declare_all` turn findings the caller has INSPECTED into the
shipped `Node::Declare` vocabulary. Detection and declaration are
separate doors on purpose (the ruled no-fusion boundary): findings
pass through your hands as values, never straight into a recipe.

```
use pncad::prelude::*;
use pncad::document::{BooleanOp, BooleanValue, NodeErrorKind, NodeResult};

let tol = Tol::witness();
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    (applied.doc, applied.record.minted.expect("a minted id"))
};
let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
// v4: the profile payload is its PROGRAM.
let footprint = |x0: f64, y0: f64, x1: f64, y1: f64, z: f64| ProfileProgram {
    plane: SketchPlane::from_frame(p3(0.0, 0.0, z), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)),
    loops: vec![
        LoopProgram::polygon([(x0, y0), (x1, y0), (x1, y1), (x0, y1)])
            .expect("finite corners"),
    ],
};

// A unit box, and a smaller box RESTING on its top cap.
let doc = Doc::<ProfileProgram>::empty_derived("select-example", tol);
let (doc, pf1) = insert(&doc, Node::Profile(footprint(0.0, 0.0, 1.0, 1.0, 0.0)));
let (doc, base) = insert(&doc, Node::Extrude { profile: pf1, distance: len(1.0) });
let (doc, pf2) = insert(&doc, Node::Profile(footprint(0.25, 0.25, 0.75, 0.75, 1.0)));
let (doc, block) = insert(&doc, Node::Extrude { profile: pf2, distance: len(0.5) });

// Undeclared, the union refuses — coincidence is never inferred
// from values (the coincidence ladder).
let (undeclared, uni) = insert(
    &doc,
    Node::Boolean { op: BooleanOp::Union, a: base, b: block, declare: None },
);
let ev = evaluate::<f64>(&undeclared, None, &CancelToken::new(), &EvalOptions::default(), tol);
let Some(NodeResult::Failed(e)) = ev.nodes.get(&uni) else {
    panic!("the undeclared union must refuse");
};
// The refusal IS the menu: it carries the candidate
// declaration — the pair by stable name, with its relation — in
// the detector's own value shape.
let NodeErrorKind::UndeclaredContact { finding, .. } = &e.kind else {
    panic!("expected the refusal menu, got {:?}", e.kind);
};
assert_eq!(finding.class, ContactClass::Rest);

// The declare arm: detect, INSPECT, declare, and the SAME doors
// that refused now verify the declared contact. (Declaring the
// menu's own finding — `declare(&doc, finding)` — is the same
// door; the detector shows the full inventory.)
let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
let findings = find_flush_candidates(&ev, base, block, tol).expect("definite findings");
assert_eq!(findings.len(), 1);
assert_eq!(findings[0].class, ContactClass::Rest);
let (doc, decl) = declare_all(&doc, &findings, tol).expect("declarable");
let (doc, uni) = insert(
    &doc,
    Node::Boolean { op: BooleanOp::Union, a: base, b: block, declare: Some(decl) },
);
let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
let ValuePayload::Boolean(BooleanValue::Body { body, .. }) =
    &ev.value(uni).expect("the declared union evaluates").payload
else {
    panic!("expected a body");
};
assert_eq!(mass_properties(body, tol).expect("mass").volume, 1.125);
```
