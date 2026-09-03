//! **M7-8 — the plane × NURBS edge lane** (`geom_brep::edge_nurbs`).
//!
//! The declare-and-check contract, exercised on the smallest honest
//! fixture of the class: a rational quarter-cylinder wall extruded in
//! `z`, and the PLANE that meets it along the wall's `u = 0` ruling.
//! The wall's own boundary column is the carrier the file would state.
//!
//! Three lane rows, one per acceptance clause: the true carrier
//! certifies with its measured residuals; a carrier displaced off the
//! locus refuses with the measured bound; a plane made TANGENT to the
//! wall along the same ruling refuses with the transversality
//! vocabulary.
//!
//! SU3 adds the same three rows driven through the CERTIFICATION DOOR
//! (`EdgeCurve::certify_nurbs_lane`) — the call the importer's attach
//! door actually makes, so these pin that the lane's verdicts arrive
//! in `certify`'s own vocabulary rather than as a private dialect —
//! plus the row proving the LANELESS door still refuses the class.
//!
//! The R1 fix pass adds the WALL-SIDE falsifier — a carrier that stays
//! exactly on the plane and leaves only the wall, so the NURBS side
//! alone can refuse it. The between-samples envelope, which no row
//! here observes, is pinned next door in `r1_pxn_probes.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::fixture::segment;
use crate::shared::fixture::{quarter_cylinder_wall, transverse_plane};
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, EdgeNurbsLane, PlaneNurbsRefusal,
};
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

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
    let eps = Tol::witness().get().eps;
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
    // Off the ruling along +y: off the plane AND off the wall. The
    // displacement SCALES with the run's ε — a fixed 1e-6 m is not a
    // falsifier at the 1e-6 matrix row, where it sits inside the
    // budget and the honest verdict is acceptance.
    let off = 1e3 * Tol::witness().get().eps;
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

/// **The WALL-SIDE falsifier** (R1 MINOR-2). The row above displaces
/// along `+y`, which leaves the plane as well as the wall — so the
/// analytic operand's closed-form residual alone is enough to catch
/// it, and the NURBS-side certification is never load-bearing. This
/// row removes that escape: the carrier is displaced RADIALLY, to
/// `x = 1 + off`, `y = 0`, which lies EXACTLY on the `y = 0` plane
/// (its residual is zero in closed form at every sample) and exactly
/// `off` metres off the cylinder `x² + y² = 1`. Nothing but the
/// foot-point residual against the NURBS wall can refuse it, so a
/// wall-side certification that silently stopped working would turn
/// this row red at the lane rather than only cross-crate.
#[test]
fn an_on_plane_off_wall_carrier_is_refused_by_the_nurbs_side() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    // Scales with the run's ε, like its sibling: a fixed displacement
    // is not a falsifier at the coarse matrix row.
    let off = 1e3 * Tol::witness().get().eps;
    let carrier = segment(
        Point3::new(1.0 + off, 0.0, 0.0),
        Point3::new(1.0 + off, 0.0, 1.0),
    );
    // The premise, asserted rather than assumed: the plane side is
    // blind to this displacement, so the refusal below is the wall's.
    for i in 0..=8 {
        let p = carrier.eval(f64::from(i) / 8.0);
        assert_eq!(p.y, 0.0, "the carrier stays exactly on the y = 0 plane");
    }
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Err(PlaneNurbsRefusal::Limb { limb, value }) => {
            println!(
                "M7-8 wall-side falsifier: {} measured {value:e} m (planted {off:e} m)",
                limb.name()
            );
            assert!(
                value >= off * 0.5,
                "the refusal carries the radial displacement the wall alone saw: {value:e}"
            );
        }
        other => panic!("a carrier {off:e} m off the WALL must refuse typed: {other:?}"),
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

// ---------------------------------------------------------------
// The certification DOOR (M7-8 SU3): the same three rows driven
// through `EdgeCurve::certify_nurbs_lane`, which is what the
// importer's attach door actually calls. These pin the MAPPING —
// that the lane's verdicts arrive in `certify`'s own vocabulary
// rather than as a private dialect — and that the door refuses the
// class identically when no lane is injected.
// ---------------------------------------------------------------

/// A two-surface arena and the spec that names its pair, for the
/// door rows: the wall is `s2`, the plane `s1`, and the witness is
/// the carrier's midpoint (the witness contract, unchanged here).
fn door_spec(
    plane: Surface<f64>,
    wall: NurbsSurface<f64>,
    carrier: NurbsCurve3<f64>,
) -> (
    impl Fn(SurfaceKey) -> Option<Surface<f64>>,
    EdgeCurveSpec<f64>,
) {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let s1 = arena.insert(plane);
    let s2 = arena.insert(Surface::Nurbs(std::sync::Arc::new(wall)));
    let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
    let witness = carrier.eval(0.5);
    (
        move |k| arena.get(k).cloned(),
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        },
    )
}

/// **The door's certifying row.** The true carrier goes in through
/// the attach door and comes out a certified `EdgeCurve` whose
/// `max_residual` is the lane's own certified sup — the number the
/// importer records.
#[test]
fn the_door_certifies_the_true_carrier_and_records_the_lane_sup() {
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let (arena, spec) = door_spec(transverse_plane(), quarter_cylinder_wall(), carrier);
    let edge = EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, arena, band())
        .expect("the stated carrier certifies through the attach door");
    println!(
        "M7-8 door: certified max_residual {:e} m over {} samples",
        edge.certificate().max_residual,
        edge.certificate().samples
    );
    assert!(
        edge.certificate().max_residual <= Tol::witness().get().eps,
        "the recorded residual is inside ε_in: {:e}",
        edge.certificate().max_residual
    );
}

/// **The door's falsifier row.** The displaced carrier's refusal
/// arrives as `CertifyError::PlaneNurbs`, still carrying the measured
/// bound — the declare-and-check payload survives the mapping.
#[test]
fn the_door_refuses_a_displaced_carrier_with_the_measured_bound() {
    // Scaled with ε, exactly as the lane row above (same reason).
    let off = 1e3 * Tol::witness().get().eps;
    let carrier = segment(Point3::new(1.0, off, 0.0), Point3::new(1.0, off, 1.0));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let (arena, spec) = door_spec(transverse_plane(), quarter_cylinder_wall(), carrier);
    match EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, arena, band()) {
        Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { limb, value })) => {
            println!("M7-8 door falsifier: {} measured {value:e} m", limb.name());
            assert!(value >= off * 0.5, "the measured bound: {value:e}");
        }
        other => panic!("the door must refuse a displaced carrier typed: {other:?}"),
    }
}

/// **The door's tangential row.** A tangential contact is NOT this
/// lane's to adopt (D2 sends it to `TangentIntersection`), and the
/// door says so in `certify`'s OWN transversality vocabulary —
/// `CertifyError::NotTransverse`, the same variant the analytic
/// `Intersection` arm raises — so the caller reads one refusal, not
/// two dialects. No `TangentIntersection` rung is invented here.
#[test]
fn the_door_refuses_a_tangential_plane_in_the_certify_vocabulary() {
    let tangent = Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let (arena, spec) = door_spec(tangent, quarter_cylinder_wall(), carrier);
    match EdgeCurve::certify_nurbs_lane(spec, ends.0, ends.1, arena, band()) {
        Err(CertifyError::NotTransverse { sample }) => {
            let msg = CertifyError::NotTransverse { sample }.to_string();
            println!("M7-8 door tangential: {msg}");
            assert!(
                msg.contains("tangent planes coincide"),
                "the tangency vocabulary, verbatim: {msg}"
            );
        }
        other => panic!("a tangential plane must refuse the precondition: {other:?}"),
    }
}

/// **The door that has no lane refuses the class exactly as before.**
/// `EdgeCurve::certify` — the `T: Decide` door — still meets a
/// described NURBS operand with `Unimplemented`. There is no third
/// outcome: no path accepts the description without a certificate.
#[test]
fn the_laneless_door_still_refuses_a_described_nurbs_operand() {
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let (arena, spec) = door_spec(transverse_plane(), quarter_cylinder_wall(), carrier);
    assert!(
        matches!(
            EdgeCurve::certify(spec, ends.0, ends.1, arena, band()),
            Err(CertifyError::Unimplemented)
        ),
        "the laneless door's refusal is unchanged by M7-8"
    );
}
