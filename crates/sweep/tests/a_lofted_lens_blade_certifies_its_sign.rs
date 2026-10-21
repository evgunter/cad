//! **Check 7 decides a sign the reporting target cannot reach.** A
//! blade lofted through lens sections (two arcs on one chord) along an
//! arching spine, tapering and rolling: every lateral wall is a
//! rational skin with cubic knots along the path. Its volume is
//! positive beyond doubt, and at the default ε no round of the
//! quadrature's schedule can meet the reporting target `1024·ε` on its
//! walls, so the lane's budget exit refuses after round 0. That refusal
//! is the reporting door's. The sign walk still refines the faces until
//! the sign settles, and the gate admits the body; the reporting door
//! keeps its refusal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::MassPropsError;

/// The lens on chord `width`, rising `ridge` above it and dropping
/// `keel` below, run counterclockwise from the `+s` margin.
fn lens(width: f64, ridge: f64, keel: f64) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

/// The blade: a spine of length 5.1 arching through −0.62 rad, nine
/// stations, the section widening from the base to a belly at 0.22 of
/// the way and narrowing to a small lens at the tip, rolled 160° eased
/// toward the tip — the lily tour's long basal leaf.
fn blade(tol: Tol) -> topo::Body<f64> {
    const STATIONS: usize = 9;
    let (len, curl) = (5.10, -0.62);
    let band = Band::linear(tol).expect("the run's tolerance forms a band");
    let (d, v) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let u = v.cross(d);
    let r = len / curl;
    let lerp = |a: [f64; 3], b: [f64; 3], s: f64| [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * s);
    let (base, belly, tip) = (
        [0.170, 0.028, 0.020],
        [0.420, 0.034, 0.016],
        [0.060, 0.010, 0.006],
    );
    let mut sections = Vec::with_capacity(STATIONS);
    let mut places = Vec::with_capacity(STATIONS);
    for k in 0..STATIONS {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((STATIONS - 1) as f64);
        let a = curl * s;
        let (sa, ca) = a.sin_cos();
        let p = Point3::origin() + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        let th = 160.0_f64.to_radians() * s.powf(2.6);
        let (st, ct) = th.sin_cos();
        let [w, ridge, keel] = if s < 0.22 {
            lerp(base, belly, s / 0.22)
        } else {
            lerp(belly, tip, (s - 0.22) / 0.78)
        };
        sections.push(lens(w, ridge, keel));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, u * ct + vk * st, vk * ct - u * st, "lens_blade", band)
                .expect("the station's two directions span a plane"),
        ));
    }
    loft_body::<f64>(&sections, &places, 3, tol)
        .expect("the lens blade lofts")
        .body
}

#[test]
fn a_lofted_lens_blade_certifies_its_sign_past_the_budget_exit() {
    let tol = Tol::witness();
    let body = blade(tol);
    let certificate = topo::validate_geometric_certificate(&body, tol).unwrap_or_else(|e| {
        panic!(
            "check 7 refuses the lens blade at ε = {:e}: {e:?}",
            tol.eps()
        )
    });
    let enclosure = certificate.enclosure();
    assert!(
        enclosure.volume_lo > 0.0,
        "the admitted blade's volume enclosure is positive: {enclosure:?}"
    );
    // The reporting door's refusal is the budget exit's, at round 0: the
    // sign walk did not answer the reporting question, it only stopped
    // being cut off by it.
    if tol.eps() <= geom_core::tolerance::DEFAULT_EPS {
        match topo::mass_properties(&body, tol) {
            Err(MassPropsError::Face {
                source: geom_brep::PropsError::QuadratureBudget { rounds: 1, .. },
                ..
            }) => {}
            other => panic!(
                "the reporting door refuses the blade at the budget exit at ε = {:e}: {other:?}",
                tol.eps()
            ),
        }
    }
}
