//! **A ray within the band of a curved edge meets it once, for both
//! faces it bounds** (D10, Booleans): half a crossing for each face, at
//! the carrier crossing of each it stands for, decided once from where
//! the ray passes the edge (`topo::boolean::solid_contain::curved_edges`).
//! The curved half of the edge-meeting ray
//! (`work/flush/a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim.md`):
//! a cylinder's rim and seam, a boss's foot, a bore's mouth, a fillet's
//! tangent edges, a frustum's rims, a hemisphere's equator and a half
//! torus's end circles.

#![allow(clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{
    bored_block_of_arcs, boss_of_arcs, disc_of_arcs, hemisphere_on_flat_base, prism,
    revolved_about_y,
};
use topo::{Body, ContactRecords, SolidContainment};

/// The bodies, each with a curved boundary, built through the public
/// doors, each asserted legal at tier 3′ with no contacts.
fn bodies(tol: Tol) -> Vec<(&'static str, Body<f64>)> {
    let quarter = (PI / 8.0).tan();
    let tube = || vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let out = vec![
        ("cylinder 2 arcs", disc_of_arcs(2, 0.5, 1.0, tol)),
        ("cylinder 3 arcs", disc_of_arcs(3, 0.5, 1.0, tol)),
        ("boss on a box", boss_of_arcs(2, 1.0, 0.3, 0.5, 1.0, tol)),
        ("bored block", bored_block_of_arcs(2, 1.0, 1.0, 0.3, tol)),
        (
            "filleted edge",
            prism(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.8), quarter),
                    (Point2::new(0.8, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                1.0,
                tol,
            ),
        ),
        (
            "cone frustum",
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(0.6, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                Revolution::Full,
                tol,
            ),
        ),
        ("hemisphere", hemisphere_on_flat_base(1.0, tol)),
        (
            "half torus",
            revolved_about_y(tube(), Revolution::Partial(PI), tol),
        ),
    ];
    for (name, body) in &out {
        assert_eq!(
            topo::validate_pseudomanifold(body, &ContactRecords::default(), tol),
            Ok(()),
            "{name} at ε {:e}: tier 3′ with no contacts",
            tol.eps()
        );
    }
    out
}

/// Whether a frame's edge is a line between two planes, which the
/// planar half reads.
fn planar_line(label: &str) -> bool {
    label.starts_with("Line") && label.contains("[Plane | Plane]")
}

/// **Rays aimed within the band of a curved edge read by it.** At a
/// third of the band and at the band from the edge's middle, into each
/// face, every ray reads the side at infinity right or reads nothing
/// (a ray that clips the edge's corner crosses nothing net there), and
/// none is set aside on a crossing in band of a face's boundary, which
/// each face's own trim read before the curved edge-meeting ray. At
/// least 60% of each body's rays read `Out`, where at ε 1e-9 12–38% did
/// before, the rest set aside on such a crossing.
#[test]
fn rays_in_band_of_a_curved_edge_read_it_once() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    for (name, body) in bodies(tol) {
        let (mut read, mut aimed) = (0, 0);
        for f in topo::test_support::edge_frames(&body, &[0.5], band) {
            if planar_line(&f.label) {
                continue;
            }
            for o in [0.3 * k_eps, k_eps] {
                let r =
                    topo::test_support::ray_at(&body, f.face, f.point + f.into * o, f.normal, band);
                let what = format!("{name} at ε {:e}, {} o={o:e}: {r}", tol.eps(), f.label);
                assert!(!r.contains("Side(In)"), "{what}: a wrong side");
                assert!(
                    !r.contains("SetAside(Crossing"),
                    "{what}: set aside in band of a face's own boundary"
                );
                aimed += 1;
                read += usize::from(r.contains("Side(Out)"));
            }
        }
        assert!(
            5 * read >= 3 * aimed,
            "{name} at ε {:e}: {read} of {aimed} rays in band of a curved edge read",
            tol.eps()
        );
    }
}

/// **No ray about a curved edge counts a crossing twice or not at all**
/// (shape 1, a counterexample search): rays aimed within three bands of
/// a random point of every edge, either side of it, in a random direction
/// about each face's normal, read the side at infinity of each body
/// `Out`, and of each body turned inside out `In`, or read nothing. A
/// crossing counted by both faces' walks, or by neither, reads the other
/// side.
#[test]
fn no_ray_about_a_curved_edge_reads_the_wrong_side() {
    test_utils::gated_to![
        "crates/topo/src/boolean/solid_contain.rs",
        "crates/topo/src/boolean/solid_contain/curved_edges.rs",
    ];
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let mut rng = test_utils::fuzz::start("no_ray_about_a_curved_edge_reads_the_wrong_side");
    let rays = test_utils::fuzz::scaled(3);
    for (name, body) in bodies(tol) {
        let reverted = body.revert().expect("the body reverts");
        for (inside_out, body, want) in [(false, &body, "Side(Out)"), (true, &reverted, "Side(In)")]
        {
            let fractions: Vec<f64> = (0..rays).map(|_| rng.range(0.05, 0.95)).collect();
            for f in topo::test_support::edge_frames(body, &fractions, band) {
                let o = rng.range(-3.0, 3.0) * k_eps;
                let tilt = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                ) * 0.6;
                let normal = (f.normal + tilt).normalize();
                let r =
                    topo::test_support::ray_at(body, f.face, f.point + f.into * o, normal, band);
                assert!(
                    r.contains(want) || !r.contains("Side("),
                    "{name} (inside out: {inside_out}) at ε {:e}, {} at {:?} o={o:e}, about {normal:?}: \
                     {r}, want {want} or nothing; {}",
                    tol.eps(),
                    f.label,
                    f.point,
                    test_utils::fuzz::replay()
                );
            }
        }
    }
}

/// **Points about a cylinder's rims and seams are read right or not at
/// all** (shape 1): points two to four bands off a random point of a rim
/// or a seam, in a random direction, against the cylinder `r ≤ ½`,
/// `0 ≤ z ≤ 1`, through point containment's schedule rays, which meet
/// the edges they pass within the band of.
#[test]
fn points_about_a_cylinders_edges_read_right_or_not_at_all() {
    test_utils::gated_to![
        "crates/topo/src/boolean/solid_contain.rs",
        "crates/topo/src/boolean/solid_contain/curved_edges.rs",
    ];
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let mut rng =
        test_utils::fuzz::start("points_about_a_cylinders_edges_read_right_or_not_at_all");
    let body = disc_of_arcs(2, 0.5, 1.0, tol);
    let truth = |p: Point3<f64>| {
        if (p.x * p.x + p.y * p.y).sqrt() < 0.5 && p.z > 0.0 && p.z < 1.0 {
            SolidContainment::In
        } else {
            SolidContainment::Out
        }
    };
    let fractions: Vec<f64> = (0..test_utils::fuzz::scaled(2))
        .map(|_| rng.range(0.05, 0.95))
        .collect();
    for f in topo::test_support::edge_frames(&body, &fractions, band) {
        let off = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        )
        .normalize();
        let q = f.point + off * (rng.range(2.0, 4.0) * k_eps);
        // A point in band of a face reads `OnBoundary`, and one the rays
        // set aside refuses: neither is a side.
        if let Ok(got) = topo::point_in_solid(&body, q, band, tol)
            && got != SolidContainment::OnBoundary
        {
            assert_eq!(
                got,
                truth(q),
                "{} at ε {:e}: {q:?}; {}",
                f.label,
                tol.eps(),
                test_utils::fuzz::replay()
            );
        }
    }
}
