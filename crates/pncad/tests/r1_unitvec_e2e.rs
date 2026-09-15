//! R1 review probe (UNITVEC): a user's program through the façade.
//! Build datum nodes, evaluate, take the witnesses out of `DatumValue`,
//! and feed the doors that now take or mint the witness.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    CancelToken, Datum, DatumValue, Dimension, Doc, DocEdit, EvalOptions, Expr, Node, NodeResult,
    RecipeNodeId, ValuePayload, evaluate,
};
use pncad::geom_core::linalg::frame::{mirror_across_plane, path_start_frame, point_at};
use pncad::geom_core::{Affine3, Band, Point3, Tol, UnitVec3, UnitVec3Error, Vec3};
use pncad::document::ProfileProgram;
use pncad::topo::DATUM_UNIT_NORM;

type ProfileDoc = Doc<ProfileProgram>;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}
fn insert(doc: ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(&DocEdit::InsertNode { node }, Tol::witness())
        .unwrap();
    (applied.doc, applied.record.minted.unwrap())
}
fn datum_of(doc: &ProfileDoc, node: RecipeNodeId) -> DatumValue<f64> {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("the datum evaluated: {:?}", ev.nodes.get(&node));
    };
    let ValuePayload::Datum(d) = &v.payload else {
        panic!("a datum payload");
    };
    d.clone()
}
fn bits(v: Vec3<f64>) -> [u64; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

#[test]
fn a_users_datum_frame_hands_its_witnesses_to_the_doors() {
    let doc = ProfileDoc::empty_derived("r1_unitvec_e2e", Tol::witness());
    // An UNNORMALIZED plane normal and axis direction, as a user types them.
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(1.0), len(2.0), len(3.0)],
            normal: [scl(0.0), scl(0.0), scl(2.5)],
        }),
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(3.0), scl(4.0), scl(0.0)],
        }),
    );
    let DatumValue::Plane { origin, normal } = datum_of(&doc, plane) else {
        panic!("a plane datum");
    };
    let DatumValue::Axis { origin: ao, dir } = datum_of(&doc, axis) else {
        panic!("an axis datum");
    };
    // The witnesses are the normalized values.
    assert_eq!(bits(normal.get()), bits(Vec3::new(0.0, 0.0, 1.0)));
    assert_eq!(bits(dir.get()), bits(Vec3::new(0.6, 0.8, 0.0)));

    // 1. The witness door to the basis — no `.get()`, no precondition prose.
    let (b1, b2) = normal.orthonormal_basis();
    assert!((b1.dot(b2)).abs() < 1e-15 && (b1.dot(normal.get())).abs() < 1e-15);
    // 2. Negation stays a witness.
    let down = -normal;
    let (d1, _) = down.orthonormal_basis();
    assert!((d1.dot(down.get())).abs() < 1e-15);
    // 3. The public callers of `frame_from_unit_aim` — they take bare
    //    Vec3 and DECIDE again; a user holding a witness pays a second
    //    decision and a `.get()`.
    let frame = point_at(origin, origin + normal.get(), Vec3::unit_x(), Tol::witness()).unwrap();
    let z = frame.linear * Vec3::unit_z();
    assert_eq!(bits(z), bits(normal.get()));
    let start = path_start_frame(ao, dir.get(), Tol::witness()).unwrap();
    assert_eq!(bits(start.linear * Vec3::unit_z()), bits(dir.get()));
    let mirror = mirror_across_plane(origin, normal.get(), Tol::witness()).unwrap();
    let p = mirror.transform_point(Point3::new(1.0, 2.0, 4.0));
    assert!((p.z - 2.0).abs() < 1e-12, "{p:?}");
    // 4. `Affine3::from_frame` (next unit's) still wants bare vectors.
    let _ = Affine3::<f64>::from_frame;
}

/// Ergonomics: the site name is a free string, so a caller can mint a
/// datum direction under ANY name and build a `DatumValue` with it —
/// the "a datum's direction is decided under `datum_unit_norm`" fact is
/// now a convention in `editor-core`'s `datum_unit`, not the type's.
#[test]
fn any_site_name_mints_a_datum_direction() {
    let band = Band::linear(Tol::witness()).unwrap();
    let n = UnitVec3::new(
        Vec3::new(0.0, 0.0, 2.0),
        "r1_probe_not_a_registered_site",
        band,
    )
    .unwrap();
    let d = DatumValue::<f64>::Plane {
        origin: Point3::origin(),
        normal: n,
    };
    let DatumValue::Plane { normal, .. } = d else {
        panic!()
    };
    assert_eq!(bits(normal.get()), bits(Vec3::new(0.0, 0.0, 1.0)));
    // And the refusals are the same typed ones under the datum name.
    assert_eq!(
        UnitVec3::new(Vec3::new(0.0, 0.0, 0.0), DATUM_UNIT_NORM, band).err(),
        Some(UnitVec3Error::Degenerate)
    );
}
