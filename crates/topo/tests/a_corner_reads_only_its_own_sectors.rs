//! **A corner meeting an unglued pair reads only its own sector.**
//!
//! A parallelepiped `A` holds a 1 mm and a 10 m edge on its bottom face
//! at the origin; a unit parallelepiped `B` below it is cornered there,
//! its top face tilted by `θ = 100·ε`. At the corner the two faces read
//! parallel at the shorter arm (`1 mm·θ = ε/10`, Zero), so the pair search
//! records a coincidence, while `A`'s 10 m bound stands `10 m·θ` off
//! `B`'s plane, decided. The glue door reads the faces apart over their
//! extent and glues nothing, so the corner refuses on the sector's own
//! reading: `Coincidence(Sectors)`, the far bound's departure rejected
//! by the corner's coplanar question (which goes on only on Zero), with
//! the lever alone.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Band, ErrorTextReading, Point3, Sign, Tol, Vec3};
use topo::test_support::{finished, mapped_cube};
use topo::{BooleanDecision, BooleanError, Coincide, DeclarationRead};

/// `A` and `B`, `B`'s top face dipping `θ·(ta·x + tb·y)` and spanning
/// the directions `b0`° to `b1`° about the origin.
fn scene(theta: f64, (ta, tb): (f64, f64), (b0, b1): (f64, f64), tol: Tol) -> [topo::Body<f64>; 2] {
    let o = Point3::new(0.0, 0.0, 0.0);
    let a = mapped_cube(
        |u, v, w| {
            o + Vec3::new(10.0, 0.0, 0.0) * u
                + Vec3::new(0.0, 1e-3, 0.0) * v
                + Vec3::new(0.0, 0.0, 1.0) * w
        },
        tol,
    );
    let dip = |deg: f64| {
        let (x, y) = (deg.to_radians().cos(), deg.to_radians().sin());
        Vec3::new(x, y, -theta * (ta * x + tb * y))
    };
    let (u, v) = (dip(b0), dip(b1));
    let b = mapped_cube(
        move |s, t, w| o + v * s + u * t + Vec3::new(0.0, 0.0, -1.0) * w,
        tol,
    );
    [a, b]
}

#[test]
fn a_far_bound_off_the_plane_refuses_on_the_sectors_own_reading() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let theta = 100.0 * band.zero();
    // (tilt, B's span, A's far bound's departure per θ)
    let poses = [
        ((1.0, 0.0), (-45.0, 45.0), 10.0),
        ((0.6, -0.8), (30.0, 60.0), 6.0),
    ];
    for (tilt, span, far) in poses {
        let [a, b] = scene(theta, tilt, span, tol);
        let (a, b) = (finished("A", a, tol), finished("B", b, tol));
        let ops = [
            ("A ∪ B", topo::union(&a, &b, tol)),
            ("A ∩ B", topo::intersect(&a, &b, tol)),
            ("A ∖ B", topo::subtract(&a, &b, tol)),
        ];
        for (op, out) in ops {
            let label = format!("tilt {tilt:?}, span {span:?}, {op}");
            let Err(err @ BooleanError::Escalated { decision, diag }) = &out else {
                panic!("{label}: refuses on an escalation: {out:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Moot),
                "{label}: the corner's question, no class admitted: {diag:?}"
            );
            assert_eq!(
                diag.predicate,
                Some("bool_sector_coplanar"),
                "{label}: the sector's reading"
            );
            assert_eq!(
                diag.margin.rejected_sign(),
                Some(Sign::Positive),
                "{label}: a decided off, rejected"
            );
            let ErrorTextReading::Value(m) = diag.margin.diagnostic_f64_for_error_text() else {
                panic!("{label}: a value margin: {diag:?}");
            };
            // The corner reads A's sector, whose 10 m bound is the
            // steeper: not its 1 mm one, which reads Zero.
            let want = far * theta;
            assert!(
                (m - want).abs() <= 1e-6 * want,
                "{label}: the far bound's departure {want:e}, got {m:e}"
            );
            assert!(!diag.offers_tolerance(), "{label}: no tolerance to offer");
            let text = err.render(geom_brep::recourse::Reading::Build);
            assert!(
                text.contains("move the parts so they clearly meet or clearly stand apart there")
                    && !text.contains("tighten"),
                "{label}: the Sectors lever alone: {text}"
            );
        }
    }
}
