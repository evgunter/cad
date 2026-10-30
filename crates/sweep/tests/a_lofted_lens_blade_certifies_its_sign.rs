//! **Check 7 decides a sign while the number is still open.** Tier 3's
//! +V check reads only the SIGN of a volume enclosure, so a body can be
//! admitted at a run's ε while a caller asking for its volume NUMBER is
//! refused: the faces' lanes stop (at their floor or their work wall)
//! short of the reporting target `1024·ε`. That is this file's subject.
//!
//! The blade lofted through lens sections — two arcs on one chord,
//! along an arching spine, tapering and rolling, every lateral wall a
//! rational skin with cubic knots along the path — was its fixture
//! while its walls could not reach the target. They reach it now (the
//! rational quotient statement narrowed their hulls, and the measured
//! work wall gives the knot-cut walls their round 7), so the blade is
//! the row that CERTIFIES, number and sign alike. The open number is
//! carried by [`round_tube`], which stops at the wall short of the
//! target at every ε row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;

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

/// **A round tube big enough that its number stays open**: two
/// circles (two semicircular arcs each, so every wall is rational) of
/// radii `23.4·s` and `11.7·s`, lofted `125·s` apart, with
/// `s = 65536·(ε / 1e-9)`. The reporting target is `1024·ε` and the
/// width a wall reaches at the work wall is a length that grows with the
/// part, so scaling the part with ε puts the same crossing at every ε
/// row. Measured at ε = 1e-9: the tube certifies its number at
/// `s = 1024`, and at 65536 each wall stops at the work wall, its
/// bracket holding the frustum's volume. The work wall is per knot-span
/// cell, so a multi-span wall refines each span as far as a one-span
/// face would. It is `pncad-py`'s `oversized_round_loft`.
fn round_tube(tol: Tol) -> topo::Body<f64> {
    let circle = |r: f64| {
        vec![bulge_loop(vec![
            (Point2::new(r, 0.0), -1.0),
            (Point2::new(-r, 0.0), -1.0),
        ])]
    };
    let s = 65536.0 * tol.eps() / geom_core::tolerance::DEFAULT_EPS;
    loft_body::<f64>(
        &[circle(23.4 * s), circle(11.7 * s)],
        &sweep::test_support::stacked_at(&[0.0, 125.0 * s]),
        1,
        tol,
    )
    .expect("the tube lofts")
    .body
}

/// The blade certifies: tier 3 admits it on a positive enclosure, the
/// reporting door measures it at the default ε and looser, and the
/// number sits in the gate's bracket. At ε = 1e-9:
/// `0.034605927320410755 ± 1.04e-6` m³, which holds the body's volume
/// as an independent quadrature of its two walls measures it (its own
/// rational de Boor and Gauss–Legendre per knot span, about a point on
/// both caps' planes), `0.0346059453481`.
#[test]
fn a_lofted_lens_blade_certifies_its_sign_and_its_number() {
    let tol = Tol::witness();
    let body = topo::test_support::finished("the lens blade", blade(tol), tol);
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
    // At ε = 1e-12 the resolution is a thousand times tighter and the
    // blade's number may stay open; the sign is the claim there.
    let measured = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the door measures the blade at ε = {:e}: {e:?}", tol.eps()));
    match measured.stop {
        topo::Stop::Met => {
            assert!(
                enclosure.volume_lo <= measured.enclosure.volume
                    && measured.enclosure.volume <= enclosure.volume_hi,
                "the blade's number {} sits in the gate's bracket {enclosure:?}",
                measured.enclosure.volume
            );
            if tol.eps() == geom_core::tolerance::DEFAULT_EPS {
                assert_eq!(
                    (
                        measured.enclosure.volume.to_bits(),
                        measured.enclosure.volume_pad.to_bits()
                    ),
                    (
                        0.034_605_927_320_410_755_f64.to_bits(),
                        1.035_232_624_013_726_6e-6_f64.to_bits()
                    ),
                    "the blade's bits at the default ε: {measured:?}"
                );
            }
        }
        topo::Stop::Floor | topo::Stop::Wall if tol.eps() < geom_core::tolerance::DEFAULT_EPS => {}
        stop => panic!(
            "the door meets the resolution on the blade at ε = {:e}: {stop:?}",
            tol.eps()
        ),
    }
}

/// **The sign settles while the number is open.** The gate admits the
/// tube on a positive enclosure; the door answers its bracket, stopped
/// at the wall short of the resolution; and the certificate's
/// continuation stops there too, with a positive bracket narrower than
/// the gate's.
#[test]
fn a_round_tube_certifies_its_sign_while_its_number_is_open() {
    let tol = Tol::witness();
    let body = topo::test_support::finished("the round tube", round_tube(tol), tol);
    let certificate = topo::validate_geometric_certificate(&body, tol)
        .unwrap_or_else(|e| panic!("check 7 refuses the tube at ε = {:e}: {e:?}", tol.eps()));
    let enclosure = certificate.enclosure();
    assert!(
        enclosure.volume_lo > 0.0,
        "the admitted tube's volume enclosure is positive: {enclosure:?}"
    );
    let door = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the door measures the tube at ε = {:e}: {e:?}", tol.eps()));
    assert_eq!(
        door.stop,
        topo::Stop::Wall,
        "the door stops the tube's number at the wall at ε = {:e}: {door:?}",
        tol.eps()
    );
    // The gate stopped each wall where the sign settled, long before any
    // reached its wall; its continuation runs the walls on to the same
    // stop as the door.
    let continued = certificate
        .measure(topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("the continuation measures the tube: {e:?}"));
    assert_eq!(
        continued.stop, door.stop,
        "the continuation stops as the door does"
    );
    // Its bracket is the walls' narrowest rounds, run on past where the
    // sign settled: strictly narrower than the gate's, so a tighter ε
    // never reads a wider answer.
    let b = continued.enclosure.enclosure();
    assert!(
        b.volume_lo > 0.0 && b.volume_hi - b.volume_lo < enclosure.volume_hi - enclosure.volume_lo,
        "the continuation's bracket [{}, {}] is narrower than the gate's [{}, {}]",
        b.volume_lo,
        b.volume_hi,
        enclosure.volume_lo,
        enclosure.volume_hi
    );
}

/// **The backstop pays a refused face once per body it reads.** The
/// volume backstop reads each body from round 0 and refines it while an
/// inequality straddles. Here the planted result is the tube itself, so
/// `vol(A ∪ B) ≥ vol(A)` straddles at every round, and the walk takes
/// both bodies' walls to their own stop and no further, as the reporting
/// door does: reading the tube twice (`A` and the planted result) costs
/// exactly twice the door's quadrature readings. Counted, not timed: a
/// round a refused face ran again would be another reading.
#[test]
fn the_volume_backstop_reads_a_refused_face_once_per_body() {
    let tol = Tol::witness();
    let body = topo::test_support::finished("the round tube", round_tube(tol), tol);
    let brick = crate::common::square_prism();
    let reporting = crate::common::quad_rounds(|| {
        let measured =
            topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
                .expect("the door measures the tube");
        assert_eq!(
            measured.stop,
            topo::Stop::Wall,
            "the tube's number stops at the wall"
        );
    });
    let backstop = crate::common::quad_rounds(|| {
        topo::test_support::volume_backstop(topo::BooleanOp::Union, &body, &brick, &body, tol)
            .expect_err("the backstop refuses a body it cannot measure");
    });
    assert_eq!(
        backstop,
        2 * reporting,
        "the backstop read the tube's refused wall past its stop ({backstop} quadrature \
         readings against the reporting door's {reporting} for one body)"
    );
}
