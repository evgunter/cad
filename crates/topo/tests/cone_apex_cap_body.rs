//! **The cone apex cap inside a body, and the traversal that bounds
//! nothing** (`work/props/cone-apex-cap-refuses-degenerateface.md`).
//!
//! A cone face bounded by one rim and nothing else is served by the
//! flux lane's apex fold, which reads no sense bit: the apex is the
//! only candidate for the missing extreme, so the closed form is the
//! same whichever way the rim runs. That is exactly why the public
//! door cannot tell the cap from its complement — the rest of the
//! nappe, which runs to infinity and is no finite face of any solid —
//! and the item says the complement is caught inside a BODY, at tier
//! 3's check 6, through `boundary_material_sign`'s cone arm.
//!
//! This is that claim executed rather than inherited. Through the
//! Euler doors, one rim row stated as two half arcs gives a cone face
//! and its complement on the same chart, traversed opposite ways; the
//! two encode opposite material sides, so exactly one of them
//! disagrees with its stored `Face::sense` and check 6 names it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, FaceKey, FaceSurface, MefSite, MevSite, ValidationError, validate_geometric};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// Apex at the origin, axis `+Z`, half-angle 45°; the rim sits 10 mm
/// of slant from the apex.
const SL: f64 = 0.010;
const SIN_A: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// The apex cap and its unbounded complement, sharing one rim stated
/// as two half arcs, through the Euler doors.
fn cone_rim_row() -> Body<f64> {
    let tol = Tol::witness();
    let r = SL * SIN_A;
    let h = SL * SIN_A;
    let a = p3(r, 0.0, h);
    let b = p3(-r, 0.0, h);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, h),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cone {
            apex: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim.clone(), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e1.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(rim, core::f64::consts::PI, core::f64::consts::TAU).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// **Exactly one of the two faces is the apex cap, and check 6 names
/// the other.**
///
/// The two faces carry the same rim traversed opposite ways, so
/// `boundary_material_sign` reads opposite `Encoded` sides off them —
/// which it can only do because the apex fold gives the parse an
/// extent to read a side against. One agrees with its stored bit and
/// one does not, and the disagreement is `CurvedSenseInverted`.
#[test]
fn the_unbounded_complement_is_caught_by_check_6() {
    let tol = Tol::witness();
    let body = cone_rim_row();
    let band = geom_core::Band::linear(tol).expect("band");

    let mut encoded: Vec<(FaceKey, bool, geom_brep::props::MaterialSign)> = Vec::new();
    for (k, f) in body.faces() {
        let surface = body.get_surface(f.surface).expect("surface").clone();
        let (outer, _hes) = topo::props::loop_edges(&body, f.outer).expect("loop edges");
        let side = geom_brep::props::boundary_material_sign(&surface, &outer, band)
            .expect("the apex cap encodes a side");
        println!("face {k:?}: sense={} side={side:?}", f.sense);
        encoded.push((k, f.sense, side));
    }
    assert_eq!(encoded.len(), 2, "one rim row, two faces");

    let disagreeing: Vec<FaceKey> = encoded
        .iter()
        .filter(|(_, sense, side)| match side {
            geom_brep::props::MaterialSign::Encoded(s) => {
                (*s == geom_core::Sign::Positive) != *sense
            }
            geom_brep::props::MaterialSign::Unencoded => false,
        })
        .map(|(k, _, _)| *k)
        .collect();
    assert_eq!(
        disagreeing.len(),
        1,
        "the cap agrees with its bit and its complement does not: {encoded:?}"
    );

    let errs = validate_geometric(&body, tol)
        .expect_err("the complement of an apex cap is no face of any solid");
    let named: Vec<FaceKey> = errs
        .iter()
        .filter_map(|e| match e {
            ValidationError::CurvedSenseInverted { face } => Some(*face),
            _ => None,
        })
        .collect();
    assert_eq!(
        named, disagreeing,
        "check 6 names exactly the face whose boundary contradicts its bit: {errs:?}"
    );
}
