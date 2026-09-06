//! **The lowered parameter-identity channel, end to end from a
//! document** (VERB-SEAT-DESIGN §3, issue 1372).
//!
//! Every row here evaluates a real document through the ordinary door
//! and then asks the KERNEL's evidence function what it sees, because
//! the claim under test is exactly that: what a boolean germ would read
//! off two carriers built by two recipe nodes.
//!
//! # The reachable subset of THIS file, stated
//!
//! The acceptance the design sketches is "the cyl×cyl equal-radius germ
//! reaches its closed form end to end from a document declaring one
//! shared radius parameter". These rows are its BLEND half: the
//! evidence function asked over real evaluated fillet carriers. They do
//! not reach the germ, and cannot — a boolean OVER a filleted body is
//! still not reachable at all, because the kernel refuses
//! `FallbackExtentUnsupported` on the sphere octants every fillet
//! result carries, a frontier that predates this channel and is pinned
//! executed in `m6_5_downstream.rs`.
//!
//! **The single run that passes through both is pinned**, on the other
//! carrier: `seat7_sweep_lowering.rs`'s
//! `one_declared_radius_reaches_the_germ_from_a_document` unions two
//! extruded circles drawn at one declared `r` and reads `Declared` off
//! the germ's own refusal. The sweeps' walls are what made it
//! authorable — they carry the channel and take no fillet's octants
//! with them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::corpus;
use crate::fixture;

use corpus::{body_of, eval, failures};
use editor_core::param_source;
use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, DocRef, DocumentId, EntityKind, EvalOptions,
    Evaluation, Expr, Node, ParamName, PartResolver, ProfileDoc, ProfileVertexRef, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, SlotId, StableName, content_pin, evaluate,
};
use fixture::{
    ang, axis_in_plane, insert, len, on_frame, on_frame_keeping, prism_edges, square, step,
};
use geom_brep::RadiusEvidence;
use geom_core::Tol;
use profile::{RawLoop, SketchPlane};
use topo::{Body, FaceKey, SurfaceField};

/// The blend radius the documents below declare, meters (dyadic).
const R: f64 = 0.125;
/// The offset a wall is thinned by, meters (dyadic).
const T: f64 = 0.03125;

fn param(name: &str) -> Expr {
    Expr::param(ParamName::new(name), Dimension::Length)
}

/// A cube of side 1 at `cx`, with every edge blended by `radius`.
/// Returns the fillet node.
fn filleted_cube(
    doc: ProfileDoc,
    cx: f64,
    radius: Expr,
) -> (ProfileDoc, editor_core::RecipeNodeId) {
    // A frame node and the square drawn on it — the profile names the
    // plane it is sketched on.
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, blend) = insert(doc, Node::fillet(cube, radius, prism_edges(cube, 4)));
    (doc, blend)
}

/// A document declaring `r` and `t`, with one filleted cube per entry
/// of `radii` laid out along x so the bodies never meet.
fn document(radii: &[Expr]) -> (ProfileDoc, Vec<editor_core::RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive("seat6-param-source"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, R),
        },
    );
    let (mut doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("t"),
            value: DocParam::continuous(Dimension::Length, T),
        },
    );
    let mut blends = Vec::new();
    for (i, radius) in radii.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let (next, blend) = filleted_cube(doc, 4.0 * i as f64, radius.clone());
        doc = next;
        blends.push(blend);
    }
    (doc, blends)
}

/// One cylindrical blend carrier of `body`, in deterministic arena
/// order — the fillet's rolling band on a straight spine, which is the
/// carrier kind a cyl×cyl germ pairs.
fn a_cylinder_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
        })
        .expect("a filleted cube carries twelve quarter-cylinder blends")
}

/// One spherical corner carrier of `body` — the fillet's octant, the
/// second role the declared flow names.
fn sphere_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Sphere { .. }))
        })
        .expect("a filleted cube carries eight sphere octants")
}

/// The evidence between one cylinder carrier of each body.
fn evidence(a: &Body<f64>, b: &Body<f64>) -> RadiusEvidence {
    topo::field_source_evidence(
        a,
        a_cylinder_face(a),
        b,
        a_cylinder_face(b),
        SurfaceField::CylinderRadius,
    )
}

/// **The acceptance row: ONE declared radius parameter reaching two
/// bodies' carriers is `Declared` at the germ's evidence door.**
///
/// Two independent fillet nodes, two separate bodies, one shared
/// document parameter — and the evidence the kernel reads is
/// `Declared`, computed with zero numerics from the two carriers'
/// tokens. Nothing compared a radius.
#[test]
fn one_shared_radius_parameter_declares_across_two_bodies() {
    let (doc, blends) = document(&[param("r"), param("r")]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "shared-r document:\n{}", bad.join("\n"));
    let (a, b) = (body_of(&ev, blends[0]), body_of(&ev, blends[1]));
    assert_eq!(
        evidence(a, b),
        RadiusEvidence::Declared,
        "two carriers built from the same declared parameter must be declared-equal"
    );
    // The flow declares THREE roles, and the corner spheres are the
    // second: the row that would go silent if the attach ever read only
    // the first field of the declaration.
    for body in [a, b] {
        let corner = sphere_face(body);
        let surface = body.get_face(corner).expect("a live face").surface;
        assert!(
            body.surface_field_source(surface, SurfaceField::SphereRadius)
                .is_some(),
            "the corner sphere's radius is a declared field of the fillet's flow"
        );
    }
    assert_eq!(
        topo::field_source_evidence(
            a,
            sphere_face(a),
            b,
            sphere_face(b),
            SurfaceField::SphereRadius
        ),
        RadiusEvidence::Declared,
        "the corner spheres share the same declared radius too"
    );
}

/// **The absence row: the SAME geometry with no channel is `None`, and
/// that is permanent.**
///
/// The body is built by the kernel's own door at the same radius — the
/// hand-built and imported posture — so the values coincide exactly and
/// the evidence is still `None`. Value equality never becomes
/// declaration (P3).
#[test]
fn the_same_geometry_without_the_channel_refuses() {
    let (doc, blends) = document(&[param("r")]);
    let ev = eval::<f64>(&doc);
    let evaluated = body_of(&ev, blends[0]);

    // The same fillet, run through the kernel doors directly: the same
    // profile, the same extrude, the same blend at the same radius —
    // and no recipe layer above them, so no records anywhere.
    let raw = {
        let lp = profile::ProfileLoop::new(
            square(0.0, 0.0, 0.5)
                .into_iter()
                .map(|(x, y)| profile::ProfileVertex::new(geom_core::Point2::new(x, y), 0.0))
                .collect(),
        );
        let sketch = profile::Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .expect("a unit square is a valid profile");
        let cube = sweep::extrude(&sketch, sweep::Extrusion::Distance(1.0), Tol::witness())
            .expect("the kernel extrudes it")
            .body;
        let edges = topo::query::all_edges(&cube);
        sweep::blend::build::fillet_edges(&cube, &edges, R, Tol::witness())
            .expect("the kernel door blends the same cube")
            .body
    };
    assert_eq!(
        evidence(evaluated, &raw),
        RadiusEvidence::None,
        "an unsourced carrier must route the general rung whatever its radius reads"
    );
    assert_eq!(
        evidence(&raw, &raw),
        RadiusEvidence::None,
        "two unsourced carriers agree on nothing — absence is not identity"
    );
}

/// **The issue's offset question, answered by construction.**
///
/// Two walls thinned by the SAME declared `t` carry the same `r - t`
/// expression and stay declared-equal; a wall at `r` and a wall at
/// `r - t` do not. No rule about offsets was written anywhere — the
/// channel carries expression identity, so the answer falls out of the
/// syntax.
#[test]
fn the_same_declared_offset_agrees_and_a_different_one_does_not() {
    let thinned = || Expr::sub(param("r"), param("t")).unwrap();
    let (doc, blends) = document(&[param("r"), thinned(), thinned()]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "offset document:\n{}", bad.join("\n"));
    let plain = body_of(&ev, blends[0]);
    let (t1, t2) = (body_of(&ev, blends[1]), body_of(&ev, blends[2]));
    assert_eq!(
        evidence(t1, t2),
        RadiusEvidence::Declared,
        "both walls offset by the same declared t lower to the same r - t"
    );
    assert_eq!(
        evidence(plain, t1),
        RadiusEvidence::None,
        "r and r - t are different expressions and must not be declared equal"
    );
}

/// **A literal is an expression too**, so two nodes spelling the same
/// literal share a token — and two spelling different ones do not.
/// This is the row that would go red if the lowering ever collapsed to
/// "the parameter's name", which would make a literal channel-less.
#[test]
fn equal_literals_declare_and_different_literals_do_not() {
    let (doc, blends) = document(&[len(R), len(R), len(R * 0.5)]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "literal document:\n{}", bad.join("\n"));
    let (a, b, c) = (
        body_of(&ev, blends[0]),
        body_of(&ev, blends[1]),
        body_of(&ev, blends[2]),
    );
    assert_eq!(evidence(a, b), RadiusEvidence::Declared);
    assert_eq!(evidence(a, c), RadiusEvidence::None);
}

/// **A chamfer node attaches nothing anywhere**, which is what its
/// declaredly EMPTY flow says: the setback positions planes and is
/// stored in no field.
///
/// **What this row can and cannot separate, stated.** The chamfer's
/// carriers are planes, and a plane stores no scalar a parameter could
/// land in — so an attach pass that ignored the declaration entirely
/// would ALSO leave this body clean, and this row would still pass. It
/// pins the OUTCOME, not the mechanism. The mechanism is pinned where
/// it is observable: dropping a role from the fillet's declared flow
/// reds the rows above, because that verb's carriers do store the
/// field.
#[test]
fn the_chamfer_attaches_nothing_because_its_flow_says_so() {
    let doc = ProfileDoc::empty(DocumentId::derive("seat6-chamfer"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, R),
        },
    );
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, cut) = insert(doc, Node::chamfer(cube, param("r"), prism_edges(cube, 4)));
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "chamfer document:\n{}", bad.join("\n"));
    let body = body_of(&ev, cut);
    for face in topo::query::all_faces(body) {
        let surface = body.get_face(face).expect("a live face").surface;
        for &field in SurfaceField::ALL {
            assert!(
                body.surface_field_source(surface, field).is_none(),
                "the chamfer stamped {field:?} on a carrier its flow names no field for"
            );
        }
    }
}

/// **The token inverts to the slot that produced it**, which is what
/// makes an opaque kernel token diagnosable upstairs.
#[test]
fn a_token_inverts_to_its_slot_address() {
    let (doc, blends) = document(&[param("r")]);
    let ev = eval::<f64>(&doc);
    let body = body_of(&ev, blends[0]);
    let surface = body
        .get_face(a_cylinder_face(body))
        .expect("a live face")
        .surface;
    let token = body
        .surface_field_source(surface, SurfaceField::CylinderRadius)
        .expect("the blend carrier carries the radius token")
        .clone();
    let at = param_source::invert(&doc, &token).expect("the token's slot is in this document");
    assert_eq!(at.node, blends[0], "the token names the fillet node's slot");
    assert_eq!(at.slot, SlotId::Radius);
    assert!(
        at.path.is_empty(),
        "the whole slot expression, not a subtree"
    );
}

/// **The records ride a rigid placement verbatim**, which is the
/// motion-invariance rule the channel rests on: a map that rewrites
/// every description's bits (and therefore CLEARS the `GeomSource`
/// records) cannot change a radius, so the field tokens survive it and
/// two placed copies of one declaration still declare.
#[test]
fn a_rigid_placement_carries_the_field_records() {
    let (doc, blends) = document(&[param("r")]);
    let ev = eval::<f64>(&doc);
    let body = body_of(&ev, blends[0]);
    let moved = topo::transform::transform_rigid(
        body,
        &geom_core::Affine3::translation(geom_core::Vec3::new(10.0, -3.0, 0.5)),
        Tol::witness(),
    )
    .expect("a translation is rigid");
    let surface = moved
        .get_face(a_cylinder_face(&moved))
        .expect("a live face")
        .surface;
    assert!(
        moved.surface_source(surface).is_none(),
        "transform_rigid clears the description-level GeomSource records"
    );
    assert_eq!(
        evidence(body, &moved),
        RadiusEvidence::Declared,
        "a radius is motion-invariant, so its source rides the placement verbatim"
    );
}

/// The radius of one cylinder carrier of `body`, read for a message.
fn cylinder_radius(body: &Body<f64>) -> f64 {
    match body
        .get_surface(body.get_face(a_cylinder_face(body)).expect("live").surface)
        .expect("carrier")
    {
        geom::Surface::Cylinder { radius, .. } => *radius,
        other => panic!("a cylinder carrier, got {other:?}"),
    }
}

/// **Two DIFFERENT operators over the same operands are two tokens.**
/// Every other row pairs expressions that differ in a leaf; this is the
/// one that pairs two expressions differing ONLY in their operator tag,
/// so a duplicated tag constant in the lowering — `r + t` and `r - t`
/// encoding byte-identically — is caught by a document rather than by
/// reading the constant table.
#[test]
fn two_different_operators_are_two_tokens() {
    let plus = || Expr::add(param("r"), param("t")).unwrap();
    let minus = || Expr::sub(param("r"), param("t")).unwrap();
    let (doc, blends) = document(&[plus(), minus(), plus()]);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "operator document:\n{}", bad.join("\n"));
    let (a, b, c) = (
        body_of(&ev, blends[0]),
        body_of(&ev, blends[1]),
        body_of(&ev, blends[2]),
    );
    assert_eq!(evidence(a, c), RadiusEvidence::Declared, "r + t twice");
    assert_eq!(
        evidence(a, b),
        RadiusEvidence::None,
        "r + t and r - t are different expressions and must not share a token"
    );
}

/// The `blend5_rim_support` lantern at `cx`, with its mouth rim
/// filleted under `radius`: a closed chain, which is the one
/// configuration that mints a torus BAND — the carrier
/// `FieldRole::BandCarrierMinorRadius` names and nothing else in this
/// suite reaches.
fn filleted_lantern(doc: ProfileDoc, cx: f64, radius: Expr) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, profile) = on_frame_keeping(
        doc,
        [cx, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.2, 0.0),
            (1.0, 0.0),
            (0.8, 0.6),
            (0.35, 0.75),
            (0.2, 0.75),
        ]],
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    let mouth = StableName {
        kind: EntityKind::Edge,
        node: revolve,
        path: vec![RoleSeg::BandRim(ProfileVertexRef {
            loop_index: 0,
            vertex: 2,
        })],
    };
    insert(
        doc,
        Node::Fillet {
            target: revolve,
            radius,
            selection: vec![mouth],
        },
    )
}

/// The torus band carrier of a filleted lantern.
fn a_torus_face(body: &Body<f64>) -> FaceKey {
    topo::query::all_faces(body)
        .into_iter()
        .find(|&f| {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .is_some_and(|s| matches!(s, geom::Surface::Torus { .. }))
        })
        .expect("a closed-chain fillet mints a torus band")
}

/// **The third declared flow row, live.** A closed chain's band torus
/// carries the declared radius on its `minor_radius` and nothing on
/// its `major_radius` (derived from the rim, not a parameter); two
/// lanterns filleted under one document parameter read `Declared`,
/// and a third spelling the same value as a literal reads `None`.
///
/// This is the row the first delivery disclosed as missing: the band
/// role was attached by the same loop as the other two but exercised
/// by no torus of its own.
#[test]
fn a_closed_chain_fillet_declares_its_torus_minor_radius() {
    let doc = ProfileDoc::empty(DocumentId::derive("seat6-band"), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, 0.05),
        },
    );
    let (doc, a) = filleted_lantern(doc, 0.0, param("r"));
    let (doc, b) = filleted_lantern(doc, 4.0, param("r"));
    let (doc, c) = filleted_lantern(doc, 8.0, len(0.05));
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "lantern document:\n{}", bad.join("\n"));
    for id in [a, b, c] {
        let body = body_of(&ev, id);
        let surface = body
            .get_face(a_torus_face(body))
            .expect("a live face")
            .surface;
        assert!(
            body.surface_field_source(surface, SurfaceField::TorusMinorRadius)
                .is_some(),
            "the band torus carries no minor-radius token"
        );
        assert!(
            body.surface_field_source(surface, SurfaceField::TorusMajorRadius)
                .is_none(),
            "the major radius is derived from the rim and carries nothing"
        );
    }
    let ev_of = |x: RecipeNodeId, y: RecipeNodeId| {
        let (bx, by) = (body_of(&ev, x), body_of(&ev, y));
        topo::field_source_evidence(
            bx,
            a_torus_face(bx),
            by,
            a_torus_face(by),
            SurfaceField::TorusMinorRadius,
        )
    };
    assert_eq!(ev_of(a, b), RadiusEvidence::Declared, "one declared r");
    assert_eq!(
        ev_of(a, c),
        RadiusEvidence::None,
        "the literal is a different expression from the parameter"
    );
}

// ---------------------------------------------------------------------
// Scope: the token names a parameter OF A DOCUMENT.
// ---------------------------------------------------------------------

/// An in-memory part store, verifying the pin exactly as a document
/// store does.
#[derive(Debug, Default)]
struct Store {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl Store {
    fn insert(&mut self, doc: ProfileDoc) -> DocRef {
        let pin = content_pin(&doc, Tol::witness()).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for Store {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let doc = self.docs.get(&doc_ref.id).ok_or_else(|| ResolveFailure {
            fault: ResolveFault::Unresolved,
            message: "no such document".to_string(),
        })?;
        if content_pin(doc, Tol::witness()).ok() != Some(doc_ref.pin) {
            return Err(ResolveFailure {
                fault: ResolveFault::PinMismatch,
                message: "the pin does not hold".to_string(),
            });
        }
        Ok(doc.clone())
    }
}

/// A document of its own identity declaring `r = value`, with one
/// filleted cube under `r`.
fn own_document(label: &str, value: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, value),
        },
    );
    filleted_cube(doc, 0.0, param("r"))
}

/// **Two documents' `r` are two parameters, and the token says so.**
///
/// A part declares `r = 0.125` and fillets at `r`; the host declares
/// ITS OWN `r = 0.25`, fillets its own cube at `r`, and instantiates
/// the part — so the two carriers meet inside ONE evaluation, exactly
/// where a boolean germ would read them. A name is scoped to the
/// document that declares it, so the two tokens must differ, and the
/// evidence must read `None` on radii that differ.
///
/// This is the row the first delivery lacked: its token named a
/// parameter by NAME alone, and the instantiate seam carries a part's
/// tokens verbatim (rigid placement cannot change a radius), so the
/// two `r`s read `Declared` here — at a germ, a `JoinDesync` on a
/// legitimate assembly; with equal values, a silent false `Declared`.
#[test]
fn two_documents_r_are_two_parameters() {
    let (part, _) = own_document("seat6-scope-part", R);
    let mut store = Store::default();
    let doc_ref = store.insert(part);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let (host, host_blend) = own_document("seat6-scope-host", 2.0 * R);
    let (host, instance) = insert(host, Node::instantiate_part(doc_ref));
    let ev: Evaluation<f64> = evaluate(&host, None, &CancelToken::new(), &opts, Tol::witness());
    let bad = failures(&ev);
    assert!(bad.is_empty(), "host document:\n{}", bad.join("\n"));
    let (h, i) = (body_of(&ev, host_blend), body_of(&ev, instance));
    let (rh, ri) = (cylinder_radius(h), cylinder_radius(i));
    assert!(
        (rh - ri).abs() > 1e-9,
        "the fixture needs two radii: {rh} vs {ri}"
    );
    assert_eq!(
        evidence(h, i),
        RadiusEvidence::None,
        "host r = {rh}, instance r = {ri}: two documents' parameters share a NAME only"
    );
}

/// **Two instances of ONE part still declare**, which is what the
/// scope must not cost: both carriers were evaluated from the same
/// document at the same pin, so their radii are one expression of one
/// table, equal by construction.
#[test]
fn two_instances_of_one_part_declare() {
    let (part, _) = own_document("seat6-scope-twice", R);
    let mut store = Store::default();
    let doc_ref = store.insert(part);
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let host = ProfileDoc::empty(DocumentId::derive("seat6-scope-twice-host"), Tol::witness());
    let (host, first) = insert(host, Node::instantiate_part(doc_ref));
    let (host, second) = insert(host, Node::instantiate_part(doc_ref));
    let ev: Evaluation<f64> = evaluate(&host, None, &CancelToken::new(), &opts, Tol::witness());
    let bad = failures(&ev);
    assert!(bad.is_empty(), "host document:\n{}", bad.join("\n"));
    assert_eq!(
        evidence(body_of(&ev, first), body_of(&ev, second)),
        RadiusEvidence::Declared,
        "one part at one pin, instantiated twice, is one declaration"
    );
}

/// **Two documents evaluated APART do not share a token either.** The
/// scope claim of `topo::param_source` is per evaluation, and this row
/// is stronger than that claim asks: a token is a fact about a
/// document, so it can be compared out of band and still not lie.
#[test]
fn two_documents_evaluated_apart_do_not_share_a_token() {
    let (d1, f1) = own_document("seat6-scope-a", R);
    let (d2, f2) = own_document("seat6-scope-b", R / 2.0);
    let (e1, e2) = (eval::<f64>(&d1), eval::<f64>(&d2));
    assert!(failures(&e1).is_empty() && failures(&e2).is_empty());
    assert_eq!(
        evidence(body_of(&e1, f1), body_of(&e2, f2)),
        RadiusEvidence::None,
        "two documents' `r` are two parameters"
    );
}

// ---------------------------------------------------------------------
// The memo: a served body's token is the document's current one.
// ---------------------------------------------------------------------

fn memo_eval(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// **The memo never serves a token the document no longer holds.**
///
/// A and B fillet at `r`. A's slot is then edited to the LITERAL of
/// the same value: the geometry is bit-identical, so a content key
/// hashing slot VALUES alone would hit the memo and hand back A's old
/// body — carrying `r`'s token while the document says A is a literal.
/// The channel would then read `Declared` between A and B, and when
/// `r` moves, B re-runs at the new radius while A stays memo-served at
/// the old one: two radii under one token, which is exactly the
/// contradiction the token exists to make impossible.
///
/// So the lowered expression of a flow-bearing slot is part of the
/// content key: an equal-value expression edit re-runs the node.
#[test]
fn the_memo_never_serves_a_stale_token() {
    let (doc1, blends) = document(&[param("r"), param("r")]);
    let (a, b) = (blends[0], blends[1]);
    let ev1 = memo_eval(&doc1, None);
    assert!(failures(&ev1).is_empty());
    assert_eq!(
        evidence(body_of(&ev1, a), body_of(&ev1, b)),
        RadiusEvidence::Declared
    );

    // A's radius slot becomes the literal 0.125: same value, different
    // expression.
    let (doc2, _) = step(
        doc1,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Radius,
            expr: len(R),
        },
    );
    let ev2 = memo_eval(&doc2, Some(&ev1));
    assert!(failures(&ev2).is_empty());
    assert!(
        ev2.reused > 0,
        "the untouched half of the document is memo-served"
    );
    assert_eq!(
        evidence(body_of(&ev2, a), body_of(&ev2, b)),
        RadiusEvidence::None,
        "A is a literal in the document and B is `r`; a memo-served token says otherwise"
    );
    let token_a = {
        let body = body_of(&ev2, a);
        let s = body.get_face(a_cylinder_face(body)).unwrap().surface;
        body.surface_field_source(s, SurfaceField::CylinderRadius)
            .cloned()
            .expect("A carries a token")
    };
    assert_eq!(
        param_source::invert(&doc2, &token_a).map(|at| at.node),
        Some(a),
        "A's token inverts to A's own slot, not to B's"
    );

    // Now move r: B re-runs at 0.25; A must not be left at 0.125 under
    // a token that claims r.
    let (doc3, _) = step(
        doc2,
        DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::continuous(Dimension::Length, 2.0 * R),
        },
    );
    let ev3 = memo_eval(&doc3, Some(&ev2));
    assert!(failures(&ev3).is_empty());
    let (ba, bb) = (body_of(&ev3, a), body_of(&ev3, b));
    let (ra, rb) = (cylinder_radius(ba), cylinder_radius(bb));
    assert!(
        (ra - rb).abs() > 1e-9,
        "the fixture needs two radii: {ra} vs {rb}"
    );
    assert_eq!(
        evidence(ba, bb),
        RadiusEvidence::None,
        "radii {ra} vs {rb} under one token would be a document-reachable contradiction"
    );
}

/// **A memo-served body compares correctly against a re-run sibling**
/// — the VS-Q4 scenario the encoding was argued on, run for real:
/// fillet A moves `r -> r + t`, fillet B stays at `r` and is served
/// from the prior evaluation, and the two read `None`.
#[test]
fn a_memo_served_body_compares_correctly_with_a_re_run_sibling() {
    let (doc1, blends) = document(&[param("r"), param("r")]);
    let ev1 = memo_eval(&doc1, None);
    assert!(failures(&ev1).is_empty());
    let (doc2, _) = step(
        doc1,
        DocEdit::SetParam {
            node: blends[0],
            slot: SlotId::Radius,
            expr: Expr::add(param("r"), param("t")).unwrap(),
        },
    );
    let ev2 = memo_eval(&doc2, Some(&ev1));
    assert!(failures(&ev2).is_empty());
    assert!(
        ev2.reused > 0 && ev2.recomputed > 0,
        "the scenario needs a reuse AND a re-run: reused {} recomputed {}",
        ev2.reused,
        ev2.recomputed
    );
    assert_eq!(
        evidence(body_of(&ev1, blends[1]), body_of(&ev2, blends[1])),
        RadiusEvidence::Declared,
        "B's token survives the re-evaluation unchanged"
    );
    assert_eq!(
        evidence(body_of(&ev2, blends[0]), body_of(&ev2, blends[1])),
        RadiusEvidence::None,
        "A now holds r + t and B holds r"
    );
}
