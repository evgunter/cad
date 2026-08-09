//! **Structural selectors, and the name doors they need** (LIB-U7).
//!
//! A [`StableName`](editor_core::StableName) used to be write-only at
//! this façade: the prelude handed you the type, and nothing to obtain
//! or inspect a value of it. This module is the door — the naming
//! table, the four whole-body materializers, the key→name inversions,
//! and a small selector language over the SHAPE of a name.
//!
//! Everything here is re-exported from `editor_core::names`; the
//! vocabulary lives beside the role enum it mirrors, which is what
//! makes its exhaustive match a compile-time tripwire when the role
//! enum grows. The façade's job is to make it reachable in one import.
//!
//! # A selector is a MATERIALIZER
//!
//! [`select`] answers *as of one evaluation* and hands back
//! `Vec<StableName>`, exactly like [`all_edges`]. You store the
//! result; from that moment it is a frozen selection like any other.
//! There is no live query in a recipe — a stored "all edges" would
//! silently grow under an upstream edit, which is the staleness the
//! freeze exists to prevent.
//!
//! # The PATTERNS are structural; GEOMETRY is a second stage
//!
//! [`Selector`] speaks role paths: which op minted the entity, which
//! role it plays, which side it is on, what its sub-names look like.
//! It cannot say "carrier is a circle" or "the two adjacent faces are
//! a plane and a sphere" — and it never will, because matching on the
//! name value ALONE is what makes a pattern reusable anywhere a name
//! exists. Geometry is a FILTER at the materializer instead:
//! [`select_where`] with a conjunction of [`GeomPred`] atoms
//! (`docs/SELECT-DESIGN.md` §§1-2, ratified #286 — the LB7 deferral
//! this discharges).
//!
//! The atoms split, and the split is the whole design:
//!
//! - **EXACT** — [`GeomPred::CurveKind`], [`GeomPred::SurfaceKind`],
//!   [`GeomPred::AdjacentKinds`] read the carrier's enum TAG. No
//!   funnel, no margin, no refusal: post-#256 the tag IS the semantic
//!   kind, so these are total and trivially equivariant. Dressing a
//!   tag match as a decided predicate would be dimension-laundering in
//!   the other direction.
//! - **DECIDED** — [`GeomPred::DatumDistance`] is a real length
//!   comparison, so it is a `k_stats` funnel site
//!   ([`SEL_DATUM_DISTANCE`]) with an honest margin, and an in-band
//!   candidate REFUSES rather than being silently included or dropped.
//!
//! Position is datum-RELATIVE, never world-frame (GS-Q6): the datum is
//! a node reference like any other input, so the rule commutes with
//! rigid motions — move the datum with the part and the selection is
//! unchanged. Convexity is reserved and unbuilt (GS-Q2).
//!
//! ```
//! use pncad::prelude::*;
//!
//! let square = ProfileLoop::new(
//!     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
//!         .into_iter()
//!         .map(|(x, y)| ProfileVertex { pos: p2::<f64>(x, y), bulge: 0.0 })
//!         .collect(),
//! );
//! let mut doc = Doc::<ProfileDesc>::empty();
//! let mut insert = |doc: &Doc<ProfileDesc>, node| {
//!     let applied = apply(doc, &DocEdit::InsertNode { node }).expect("the edit applies");
//!     let id = applied.record.minted.expect("a minted id");
//!     (applied.doc, id)
//! };
//! let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
//! let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("a scalar");
//!
//! let (next, profile) = insert(
//!     &doc,
//!     Node::Profile(ProfileDesc(Profile::new(SketchPlane::xy(), vec![square]))),
//! );
//! doc = next;
//! let (next, cube) = insert(&doc, Node::Extrude { profile, distance: len(1.0) });
//! doc = next;
//! // The datum the position rule is written against — one argument,
//! // and the rule now moves WITH the part.
//! let (next, ground) = insert(
//!     &doc,
//!     Node::Datum(Datum::Plane {
//!         origin: [len(0.0), len(0.0), len(0.0)],
//!         normal: [scl(0.0), scl(0.0), scl(1.0)],
//!     }),
//! );
//! doc = next;
//!
//! let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
//! let params = doc.param_env::<f64>();
//! let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));
//! let faces = Selector::of(NamePat::of_kind(EntityKind::Face));
//!
//! // EXACT: every edge of a box is a line. Exact atoms are TOTAL, so
//! // this can never refuse — and an empty conjunction is plain
//! // `select`, which is why the two agree name for name.
//! let straight = select_where(
//!     &ev, cube, &edges,
//!     &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
//!     &params,
//! ).expect("exact atoms cannot refuse");
//! assert_eq!(straight, select(&ev, cube, &edges));
//! assert_eq!(straight.len(), 12);
//!
//! // EXACT, and UNORDERED across the edge: no plane/sphere rim here.
//! assert!(select_where(
//!     &ev, cube, &edges,
//!     &[GeomPred::AdjacentKinds(
//!         SurfaceKindSet::just(SurfaceKind::Plane),
//!         SurfaceKindSet::just(SurfaceKind::Sphere),
//!     )],
//!     &params,
//! ).expect("total").is_empty());
//!
//! // DECIDED: the one face a metre above the datum, found by
//! // POSITION — and it is the same face the role path names.
//! let top = select_where(
//!     &ev, cube, &faces,
//!     &[GeomPred::DatumDistance { datum: ground, cmp: Cmp::Approx, value: len(1.0) }],
//!     &params,
//! ).expect("no candidate is in-band here");
//! assert_eq!(
//!     top,
//!     select(&ev, cube, &Selector::of(
//!         NamePat::of_kind(EntityKind::Face)
//!             .seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
//!     )),
//! );
//!
//! // The comparand of a distance must BE a distance.
//! assert!(matches!(
//!     select_where(
//!         &ev, cube, &faces,
//!         &[GeomPred::DatumDistance {
//!             datum: ground,
//!             cmp: Cmp::Approx,
//!             value: Expr::literal(1.0, Dimension::Angle).expect("an angle"),
//!         }],
//!         &params,
//!     ),
//!     Err(SelectRefusal::NotALength { .. }),
//! ));
//! ```
//!
//! # The end-to-end form
//!
//! Evaluate, select, store. "The cap rim of the top face" of an
//! extruded square, with no coordinate and no arena key anywhere:
//!
//! ```
//! use pncad::prelude::*;
//!
//! // A unit box, authored through the document layer.
//! let square = ProfileLoop::new(
//!     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
//!         .into_iter()
//!         .map(|(x, y)| ProfileVertex { pos: p2::<f64>(x, y), bulge: 0.0 })
//!         .collect(),
//! );
//! let mut doc = Doc::<ProfileDesc>::empty();
//! let mut insert = |doc: &Doc<ProfileDesc>, node| {
//!     let applied = apply(doc, &DocEdit::InsertNode { node }).expect("the edit applies");
//!     let id = applied.record.minted.expect("a minted id");
//!     (applied.doc, id)
//! };
//! let (next, profile) = insert(
//!     &doc,
//!     Node::Profile(ProfileDesc(Profile::new(SketchPlane::xy(), vec![square]))),
//! );
//! doc = next;
//! let (next, cube) = insert(
//!     &doc,
//!     Node::Extrude {
//!         profile,
//!         distance: Expr::literal(1.0, Dimension::Length).expect("a length"),
//!     },
//! );
//! doc = next;
//!
//! let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
//!
//! // The whole-body materializers: one door per entity kind.
//! assert_eq!(all_faces(&ev, cube).len(), 6);
//! assert_eq!(all_edges(&ev, cube).len(), 12);
//! assert_eq!(all_vertices(&ev, cube).len(), 8);
//! assert_eq!(all_bodies(&ev, cube).len(), 1);
//!
//! // "The cap rim of the top face" — one segment pattern.
//! let top_rim = Selector::of(
//!     NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::Top)),
//! );
//! let names = select(&ev, cube, &top_rim);
//! assert_eq!(names.len(), 4);
//!
//! // What comes back is a frozen, canonical set — ready to STORE.
//! assert!(names.windows(2).all(|w| w[0] < w[1]));
//! ```
//!
//! # The pattern forms, without an evaluation
//!
//! Every pattern is a value with a total [`Selector::matches`], so the
//! forms can be shown against a name built by hand.
//!
//! ```
//! use pncad::prelude::*;
//!
//! let node = RecipeNodeId(7);
//! let rim = |end| StableName {
//!     kind: EntityKind::Edge,
//!     node,
//!     path: vec![RoleSeg::RimEdge(end, ProfileEdgeRef { loop_index: 0, segment: 2 })],
//! };
//!
//! // `SegPat::tag` — the variant, arguments free.
//! let any_rim = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge));
//! assert!(Selector::of(any_rim.clone()).matches(&rim(CapEnd::Top)));
//! assert!(Selector::of(any_rim).matches(&rim(CapEnd::Bottom)));
//!
//! // `.side` — the end/side tag, taken by `From` so the role enum's
//! // own types spell it.
//! let top = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::RimEdge).side(CapEnd::Top));
//! assert!(Selector::of(top.clone()).matches(&rim(CapEnd::Top)));
//! assert!(!Selector::of(top).matches(&rim(CapEnd::Bottom)));
//!
//! // `SegPat::group` — everything one op minted.
//! let swept = NamePat::any().seg(SegPat::group(OpGroup::Extrude));
//! assert!(Selector::of(swept).matches(&rim(CapEnd::Top)));
//!
//! // `NamePat::node` — restrict to one recipe node; `SegPat::any` and
//! // `NamePat::any` are the wildcards.
//! assert!(Selector::of(NamePat::any().node(node).seg(SegPat::any())).matches(&rim(CapEnd::Top)));
//! assert!(!Selector::of(NamePat::any().node(RecipeNodeId(8))).matches(&rim(CapEnd::Top)));
//!
//! // A constrained path matches length for length.
//! assert!(!Selector::of(NamePat::any().path([SegPat::any(), SegPat::any()]))
//!     .matches(&rim(CapEnd::Top)));
//! ```
//!
//! # Alternatives, and sub-name arguments
//!
//! A selector is a UNION of patterns — which is how a real selection
//! reads ("the box edges AND the pip rim") — and a segment's sub-NAME
//! arguments are patterns too, positionally, as a prefix.
//!
//! ```
//! use pncad::prelude::*;
//!
//! let node = RecipeNodeId(3);
//! let face = |path| StableName { kind: EntityKind::Face, node, path };
//! // A boolean seam edge: the top cap of one operand crossing a
//! // revolve band of the other.
//! let seam = StableName {
//!     kind: EntityKind::Edge,
//!     node,
//!     path: vec![RoleSeg::Seam {
//!         a: Box::new(face(vec![RoleSeg::Cap(CapEnd::Top)])),
//!         b: Box::new(face(vec![RoleSeg::Band(ProfileEdgeRef { loop_index: 0, segment: 0 })])),
//!     }],
//! };
//!
//! // `.of` — both sides constrained: "every Seam{Cap, Band} edge".
//! let cap_band = NamePat::of_kind(EntityKind::Edge).seg(SegPat::tag(SegTag::Seam).of([
//!     NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
//!     NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Band)),
//! ]));
//! assert!(Selector::of(cap_band).matches(&seam));
//!
//! // A PREFIX: constraining only the A side is coarser and still
//! // honest.
//! let any_cap_seam = NamePat::any().seg(SegPat::tag(SegTag::Seam).of([
//!     NamePat::any().seg(SegPat::tag(SegTag::Cap)),
//! ]));
//! assert!(Selector::of(any_cap_seam).matches(&seam));
//!
//! // `Selector::any_of` / `.or` — the union. An EMPTY selector
//! // matches nothing.
//! let union = Selector::any_of([NamePat::of_kind(EntityKind::Vertex)])
//!     .or(NamePat::of_kind(EntityKind::Edge));
//! assert!(union.matches(&seam));
//! assert!(!Selector::default().matches(&seam));
//! ```

//! # From a name to GEOMETRY (LIB-U5)
//!
//! A selection is only half a question. The other half — "so where
//! IS the face I selected?" — used to have no answer at this façade:
//! the naming table's `EntityRef`/`Entry` were exported, and reading
//! a coordinate meant unwrapping one into a `topo` arena key and
//! indexing the body yourself. Arena keys are body-lineage-scoped and
//! meaningful only against the evaluation that built them; the naming
//! layer's own rule is that they never leave `editor-core` (G1), and
//! LIBRARY-DESIGN §L3 names that laundering as the thing a consumer
//! must never have to do.
//!
//! **So they no longer leave.** `EntityRef` and `Entry` are gone from
//! this surface, replaced by doors that speak names and answer with
//! values: [`face_frame`], [`edge_frame`], [`vertex_position`], and
//! [`denotation`] for the tie information `Entry` used to carry (a
//! count, not candidates — the candidates are keys).
//!
//! What comes back is a [`Pose`]: the carrier's own stored frame,
//! copied out. A VALUE, never a verdict — no door here answers "is
//! this face planar" or "is this edge convex" (LIB-LOG LB7 defers
//! geometric predicates). And no convention is invented where the
//! model fixes none: **a NURBS face has no canonical frame, so
//! [`face_frame`] refuses it** with `ReadbackError::NoCanonicalFrame`
//! rather than nominating S(0,0) and a chart normal as though the
//! kernel had chosen them. Analytic carriers all answer.
//!
//! ```
//! use pncad::prelude::*;
//!
//! let square = ProfileLoop::new(
//!     [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
//!         .into_iter()
//!         .map(|(x, y)| ProfileVertex { pos: p2::<f64>(x, y), bulge: 0.0 })
//!         .collect(),
//! );
//! let mut doc = Doc::<ProfileDesc>::empty();
//! let mut insert = |doc: &Doc<ProfileDesc>, node| {
//!     let applied = apply(doc, &DocEdit::InsertNode { node }).expect("the edit applies");
//!     let id = applied.record.minted.expect("a minted id");
//!     (applied.doc, id)
//! };
//! let (next, profile) = insert(
//!     &doc,
//!     Node::Profile(ProfileDesc(Profile::new(SketchPlane::xy(), vec![square]))),
//! );
//! doc = next;
//! let (next, cube) = insert(
//!     &doc,
//!     Node::Extrude {
//!         profile,
//!         distance: Expr::literal(1.0, Dimension::Length).expect("a length"),
//!     },
//! );
//! doc = next;
//!
//! let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
//!
//! // Select the top cap by NAME, then ask where it is.
//! let top = Selector::of(
//!     NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
//! );
//! let names = select(&ev, cube, &top);
//! assert_eq!(names.len(), 1);
//!
//! // The name resolves uniquely...
//! assert_eq!(denotation(&ev, cube, &names[0]), Ok(Denotation::Unique));
//! // ...and the plane it names is z = 1. No arena key was held, and
//! // no coordinate was transcribed to learn it.
//! let pose = face_frame(&ev, cube, &names[0]).expect("an analytic cap");
//! assert_eq!(pose.origin.z, 1.0);
//! assert_eq!(pose.axis.z, 1.0);
//!
//! // Vertices answer with their stored position.
//! let corners = all_vertices(&ev, cube);
//! let zs: Vec<f64> = corners
//!     .iter()
//!     .map(|n| vertex_position(&ev, cube, n).expect("a live vertex").z)
//!     .collect();
//! assert_eq!(zs.iter().filter(|z| **z == 1.0).count(), 4);
//!
//! // Edges answer with their carrier's frame. Every edge of a box
//! // is a line: a direction, and honestly NO reference
//! // perpendicular — `u_ref` is `None` rather than invented.
//! let edge = &all_edges(&ev, cube)[0];
//! let line = edge_frame(&ev, cube, edge).expect("a certified carrier");
//! assert!(line.u_ref.is_none() && line.v_ref().is_none());
//!
//! // A face door handed an EDGE name refuses by type, not by panic.
//! assert!(matches!(
//!     face_frame(&ev, cube, edge),
//!     Err(InterrogateError::WrongKind { .. })
//! ));
//! ```

pub use editor_core::{
    ALL_SURFACE_KINDS, CapEnd, Cmp, CurveKind, CurveKindSet, Denotation, EntityKind, GeomPred,
    InterrogateError, MeridianEnd, NamePat, NameTable, OpGroup, ProfileEdgeRef, ProfileVertexRef,
    RimSupport, RolePath, RoleSeg, SEL_DATUM_DISTANCE, SegPat, SegTag, SelectRefusal, Selector,
    Side, SplitHalf, SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices,
    denotation, edge_frame, edge_name, face_frame, face_name, select, select_where,
    vertex_position,
};
/// The frame type the geometry doors answer with, and its refusal —
/// re-exported from the kernel's read-back module so a façade user
/// names one crate, not two.
pub use sweep::readback::{Pose, ReadbackError};
