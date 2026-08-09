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
//! # STRUCTURAL only
//!
//! The vocabulary speaks role paths: which op minted the entity, which
//! role it plays, which side it is on, what its sub-names look like.
//! It cannot say "carrier is a circle", "the two adjacent faces are a
//! plane and a sphere", "convex", or "z ≈ 1". Those are geometric —
//! decided predicates, subject to the margins-and-recorded-verdicts
//! discipline — and are deferred to a designed follow-up (LIB-LOG LB7;
//! GUI-DESIGN GQ7). The demo tour's `diefillet` filters stay written
//! against the kernel body for exactly that reason.
//!
//! # The end-to-end form
//!
//! Evaluate, select, store. "The cap rim of the top face" of an
//! extruded square, with no coordinate and no arena key anywhere:
//!
//! ```
//! use pncad::prelude::*;
//!
//! // A unit box, authored through the document layer (v4: the
//! // profile payload is its PROGRAM — a chain of Expr-bearing steps).
//! let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
//!     .expect("finite corners");
//! let mut doc = Doc::<ProfileProgram>::empty();
//! let mut insert = |doc: &Doc<ProfileProgram>, node| {
//!     let applied = apply(doc, &DocEdit::InsertNode { node }).expect("the edit applies");
//!     let id = applied.record.minted.expect("a minted id");
//!     (applied.doc, id)
//! };
//! let (next, profile) = insert(
//!     &doc,
//!     Node::Profile(ProfileProgram { plane: SketchPlane::xy(), loops: vec![square] }),
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

pub use editor_core::{
    CapEnd, EntityKind, EntityRef, Entry, MeridianEnd, NamePat, NameTable, OpGroup, ProfileEdgeRef,
    ProfileVertexRef, RimSupport, RolePath, RoleSeg, SegPat, SegTag, Selector, Side, SplitHalf,
    TagPat, all_bodies, all_edges, all_faces, all_vertices, edge_name, entity_name, face_name,
    select,
};
