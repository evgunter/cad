//! **The lowered parameter-identity channel, end to end from a
//! document** (VERB-SEAT-DESIGN §3, issue 1372).
//!
//! Every row here evaluates a real document through the ordinary door
//! and then asks the KERNEL's evidence function what it sees, because
//! the claim under test is exactly that: what a boolean germ would read
//! off two carriers built by two recipe nodes.
//!
//! # The reachable subset, stated
//!
//! The acceptance the design sketches is "the cyl×cyl equal-radius germ
//! reaches its closed form end to end from a document declaring one
//! shared radius parameter". The germ half of that is pinned in `topo`
//! (`boolean::join`'s frame-dispatch rows) because a boolean OVER a
//! filleted body is not reachable at all today — the kernel refuses
//! `FallbackExtentUnsupported` on the sphere octants every fillet result
//! carries, a frontier that predates this channel and is pinned executed
//! in `m6_5_downstream.rs`. So the document half is pinned here at the
//! evidence function over real evaluated fillet carriers, and the germ
//! half is pinned there over the dispatch; what is NOT pinned anywhere,
//! and is said out loud rather than implied, is the single run that
//! passes through both.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{body_of, eval, failures};
use editor_core::param_source;
use editor_core::{
    Dimension, DocEdit, DocParam, DocumentId, Expr, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, SlotId,
};
use fixture::{insert, len, prism_edges, square, step};
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
    let loops = vec![LoopProgram::polygon(square(cx, 0.0, 0.5)).unwrap()];
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops,
        }),
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
    let loops = vec![LoopProgram::polygon(square(0.0, 0.0, 0.5)).unwrap()];
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops,
        }),
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
