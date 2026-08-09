//! **M7-8 — the plane × NURBS edge lane** (`geom_brep::edge_nurbs`).
//!
//! The declare-and-check contract, exercised on the smallest honest
//! fixture of the class: a rational quarter-cylinder wall extruded in
//! `z`, and the PLANE that meets it along the wall's `u = 0` ruling.
//! The wall's own boundary column is the carrier the file would state.
//!
//! Three rows, one per acceptance clause: the true carrier certifies
//! with its measured residuals; a carrier displaced off the locus
//! refuses with the measured bound; a plane made TANGENT to the wall
//! along the same ruling refuses with the transversality vocabulary.

use geom_brep::{EdgeNurbsLane, PlaneNurbsRefusal};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tolerance, Vec3};
use geom_curves::NurbsCurve3;
use geom_surfaces::{NurbsSurface, Surface};

/// The rational quarter cylinder `x² + y² = 1`, `0 ≤ z ≤ 1`: degree 2
/// in `u` (the classic three-point rational arc, middle weight
/// `√2/2`), degree 1 in `v` (the extrusion).
fn quarter_cylinder_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("u knots");
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("v knots");
    // Row-major in u: control[iu * nv + iv].
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).expect("the wall builds")
}

/// A straight degree-1 carrier through two points, on `[0, 1]`.
fn segment(a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("carrier knots");
    NurbsCurve3::new(k, vec![a, b], vec![1.0, 1.0]).expect("the carrier builds")
}

/// The `y = 0` plane — it contains the wall's `u = 0` ruling and its
/// normal is orthogonal to the wall's there (a 90° dihedral).
fn transverse_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn band() -> Band {
    Band::linear().expect("the run's linear band")
}

/// **The certifying row.** The file's carrier IS the locus, and every
/// limb passes: the plane residual is closed-form zero, the NURBS
/// residual is a certified foot distance, the between-samples bound is
/// the composite sup, and the uniqueness tube reports its margin.
#[test]
fn the_stated_carrier_certifies_against_both_surfaces() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let limbs = f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band())
        .expect("the true locus certifies");
    println!(
        "M7-8 quarter-cylinder ruling: on_locus_max = {:e} m, hull_sup = {:e} m, \
         tube_radius = {:e} m, tube_transversality = {:e} m over {} boxes, \
         min sin θ = {:e}",
        limbs.on_locus_max,
        limbs.hull_sup,
        limbs.tube_radius,
        limbs.tube_transversality,
        limbs.tube_boxes,
        limbs.min_sin_theta,
    );
    let eps = Tolerance::get().eps;
    assert!(
        limbs.hull_sup <= eps,
        "the certified sup bound must be within ε: {:e} > {eps:e}",
        limbs.hull_sup
    );
    assert!(
        limbs.min_sin_theta > 0.5,
        "the ruling meets the y = 0 plane at 90°: sin θ = {:e}",
        limbs.min_sin_theta
    );
}

/// **The planted falsifier.** A carrier displaced off the locus is
/// never trusted: it refuses with the limb that caught it and the
/// measured bound in the payload.
#[test]
fn a_displaced_carrier_refuses_with_the_measured_residual() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    // 1e-6 m off the ruling, along +y: off the plane AND off the wall.
    let off = 1e-6;
    let carrier = segment(Point3::new(1.0, off, 0.0), Point3::new(1.0, off, 1.0));
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Err(PlaneNurbsRefusal::Limb { limb, value }) => {
            println!(
                "M7-8 displaced carrier: {} measured {value:e} m",
                limb.name()
            );
            assert!(
                value >= off * 0.5,
                "the refusal must carry the real displacement: {value:e}"
            );
        }
        other => panic!("a carrier {off:e} m off the locus must refuse typed: {other:?}"),
    }
}

/// **The planted tangential case.** The `x = 1` plane touches the wall
/// along the same ruling with PARALLEL normals — the `Intersection`
/// precondition fails, and the lane says so in the transversality
/// vocabulary rather than accepting silently.
#[test]
fn a_tangential_plane_refuses_with_the_transversality_vocabulary() {
    let wall = quarter_cylinder_wall();
    let tangent = Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    match f64::plane_nurbs_limbs(&carrier, &tangent, &wall, 1.0, band()) {
        Err(PlaneNurbsRefusal::NotTransverse { sample }) => {
            println!("M7-8 tangential plane: refused at interior sample {sample}");
        }
        other => panic!("a tangential plane must refuse the Intersection precondition: {other:?}"),
    }
}
