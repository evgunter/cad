//! The declared-cusp chain on the sweep side, and the exactness of the
//! cusp door's reversal.
//!
//! 1. **Reachability**: a `.cusp()` profile validates, `extrude` builds
//!    the cusp solid, and `validate_geometric` passes it with nothing
//!    declared at rest — the cusp strut is a jet-determinate tangency.
//!
//! 2. **The exactness of the reversal**: the cusp door negates the
//!    incoming ray rather than re-deriving it as `ang + π`. This probe
//!    measures the residual the distinction controls, so the claim is
//!    observable.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Open, Profile, SketchPlane, Start};
use sweep::{Extrusion, extrude};

/// The lune: the cross-section of D1's kissing-cylinders figure,
/// authored through the new door. Circles (0,1) r 1 and (0,2) r 2 are
/// internally tangent at the origin; the region kept is the x ≥ 0 lip.
fn lune() -> profile::ClosedLoop<f64> {
    let tol = Tol::witness();
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
}

/// **The reachability chain, executed.** Nothing on it proceeds
/// silently: the body passes at rest, and check 4 marked the strut on
/// the kiss `Tangent` — a tangency it judged, not one it skipped.
#[test]
fn r2_cusp_profile_extrudes_and_passes_at_rest() {
    let tol = Tol::witness();
    let loops = vec![profile::ProfileLoop::from(lune())];
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let validated = Profile::new(plane, loops)
        .validate(tol)
        .expect("the .cusp() profile validates: the joint is DECLARED");
    let ext =
        extrude(&validated, Extrusion::Distance(1.0), tol).expect("extrude BUILDS the cusp solid");
    let body = &ext.body;
    assert_eq!(topo::validate_closed(body), Ok(()));
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "the extruded cusp solid is legal at rest"
    );
    let marks = topo::contact_marks(body, tol).expect("valid");
    let on_the_kiss = |v| {
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        p.x.abs() < 1e-9 && p.y.abs() < 1e-9
    };
    let marked: Vec<_> = marks
        .iter()
        .filter(|(_, m)| **m == topo::ContactMark::Tangent)
        .map(|(e, _)| e)
        .collect();
    let [edge] = marked.as_slice() else {
        panic!("exactly one Tangent mark: {marks:?}");
    };
    let he = body.get_edge(*edge).unwrap().he_plus;
    assert!(
        on_the_kiss(body.get_half_edge(he).unwrap().start)
            && on_the_kiss(body.half_edge_end(he).unwrap()),
        "the one Tangent mark is the strut on the kiss"
    );
}

/// **The reversal's exactness, measured.** The departure ray must be
/// the incoming ray NEGATED. `Dir` is private, so this reads the
/// consequence the negation controls: the two arcs meeting at the kiss
/// have exactly opposite unit tangents there, reconstructed from the
/// stored vertices and bulges.
///
/// Printed rather than asserted at a threshold, so the number is the
/// evidence: with the exact negation it is 0; with `ang + π` it is a
/// few ulp, which every band in the project accepts — i.e. the
/// distinction the PR headlines is real but unobserved by any row.
#[test]
fn r2_the_cusp_reversal_residual_is_measured() {
    let lp = lune();
    let raw = profile::ProfileLoop::from(lp);
    let v: Vec<(Point2<f64>, f64)> = raw
        .vertices()
        .iter()
        .zip(raw.bulges())
        .map(|(&p, &b)| (p, b))
        .collect();
    let n = v.len();
    // The kiss is vertex 2 (the loop is: (0,4) → (0,2) → (0,0) kiss →
    // back). Incoming arc is v[1]→v[2]; outgoing arc is v[2]→v[3 % n].
    let tang_end = |a: usize, b: usize| {
        let p0 = v[a].0;
        let p1 = v[b].0;
        let bulge = v[a].1;
        // End tangent of a circular arc: chord direction rotated by
        // +2*atan(bulge) ... at the END the rotation is +Δ where
        // tan(Δ/2) = bulge.
        let d = p1 - p0;
        let delta: f64 = bulge.atan();
        let (s, c) = (2.0f64 * delta).sin_cos();
        let t = geom_core::Vec2::new(d.x * c - d.y * s, d.x * s + d.y * c);
        t / t.norm_squared().sqrt()
    };
    let tang_start = |a: usize, b: usize| {
        let p0 = v[a].0;
        let p1 = v[b].0;
        let bulge = v[a].1;
        let d = p1 - p0;
        let delta: f64 = bulge.atan();
        let (s, c) = (-2.0f64 * delta).sin_cos();
        let t = geom_core::Vec2::new(d.x * c - d.y * s, d.x * s + d.y * c);
        t / t.norm_squared().sqrt()
    };
    let incoming = tang_end(1, 2);
    let outgoing = tang_start(2, 3 % n);
    let sum = incoming + outgoing;
    println!(
        "R2 exactness: incoming {incoming:?} outgoing {outgoing:?} |sum| {:e}",
        sum.norm_squared().sqrt()
    );
}
