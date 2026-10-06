//! Review probes for PR #4092 (sided radius headroom): bodies the sided
//! read newly admits are built end to end and checked tier 3 and against
//! a closed-form Pappus volume; bodies a wrong side would admit are
//! checked to still refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, fillet_edges};
use sweep::test_support::pappus::{pappus_volume, sector, segment, triangle};
use sweep::test_support::{corners, prism, revolved_about_y, rim_arcs_at, rod_creases};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn vol(b: &Body<f64>) -> f64 {
    mass_properties(b, tol()).expect("mass properties").volume
}

/// `quad(k, fp, c, fs)` as two triangles.
fn quad(k: (f64, f64), fp: (f64, f64), c: (f64, f64), fs: (f64, f64)) -> [(f64, (f64, f64)); 2] {
    [(1.0, triangle(k, fp, c)), (1.0, triangle(k, c, fs))]
}

fn build_and_check(what: &str, body: &Body<f64>, rim: (f64, f64), r: f64, delta: f64) {
    let v0 = vol(body);
    let edges = rim_arcs_at(body, rim.0, rim.1);
    assert!(!edges.is_empty(), "{what}: rim found");
    let f = fillet_edges(body, &edges, r, tol())
        .unwrap_or_else(|e| panic!("{what}: r {r} refused: {:?}", e.error));
    assert_eq!(topo::validate(&f.body), Ok(()), "{what}: tier 1");
    assert_eq!(topo::validate_closed(&f.body), Ok(()), "{what}: tier 2");
    assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "{what}: tier 3");
    let v1 = vol(&f.body);
    eprintln!("{what}: r {r} V0 {v0:.12} V1 {v1:.12} dV {:.12} closed form {delta:.12}", v1 - v0);
    assert!(
        ((v1 - v0) - delta).abs() < 1e-9 * v0.abs(),
        "{what}: ΔV {} vs closed form {delta}",
        v1 - v0
    );
}

/// A bore of radius 0.5 through a 2-thick ring plate; its top rim is
/// CONVEX with the ball in the material OUTSIDE the bore. r = 0.8 > R.
#[test]
fn bore_rim_past_the_bore_radius_builds_at_the_closed_form() {
    let body = revolved_about_y(
        corners(&[(0.5, 0.0), (3.0, 0.0), (3.0, 2.0), (0.5, 2.0)]),
        Revolution::Full,
        tol(),
    );
    let r = 0.8;
    let (k, c) = ((0.5, 2.0), (0.5 + r, 2.0 - r));
    let (fp, fs) = ((c.0, 2.0), (0.5, c.1));
    let mut pieces = quad(k, fp, c, fs).to_vec();
    pieces.push((-1.0, sector(c, r, fp, fs)));
    build_and_check("bore rim", &body, (0.5, 2.0), r, -pappus_volume(&pieces));
}

/// A rod of radius 0.5 standing on a plate: the base crease is CONCAVE
/// with the ball in the void OUTSIDE the rod. r = 0.8 > R.
#[test]
fn boss_base_past_the_rod_radius_builds_at_the_closed_form() {
    let body = revolved_about_y(
        corners(&[(0.2, 0.0), (3.0, 0.0), (3.0, 1.0), (0.5, 1.0), (0.5, 2.5), (0.2, 2.5)]),
        Revolution::Full,
        tol(),
    );
    let r = 0.8;
    let (k, c) = ((0.5, 1.0), (0.5 + r, 1.0 + r));
    let (fp, fs) = ((c.0, 1.0), (0.5, c.1));
    let mut pieces = quad(k, fp, c, fs).to_vec();
    pieces.push((-1.0, sector(c, r, fp, fs)));
    build_and_check("boss base", &body, (0.5, 1.0), r, pappus_volume(&pieces));
}

/// A spherical dimple (sphere radius 0.5 about (0, 2)) in a plate; its rim
/// is CONVEX with the ball in the material OUTSIDE the sphere. r = 0.8 > R.
#[test]
fn dimple_rim_past_the_sphere_radius_builds_at_the_closed_form() {
    let big = 0.5;
    let yb = 2.0 - (big * big - 0.04_f64).sqrt();
    let theta = (0.2_f64 / big).acos();
    let body = revolved_about_y(
        vec![
            (Point2::new(0.2, 0.0), 0.0),
            (Point2::new(3.0, 0.0), 0.0),
            (Point2::new(3.0, 2.0), 0.0),
            (Point2::new(big, 2.0), -(theta / 4.0).tan()),
            (Point2::new(0.2, yb), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let o = (0.0, 2.0);
    let v_dimple = {
        // sanity: the plate minus the lower hemisphere outside the bore
        let pi = core::f64::consts::PI;
        let h = yb - (2.0 - big);
        let core = pi * 0.04 * (2.0 - yb) + pi * h * h * (3.0 * big - h) / 3.0;
        pi * (9.0 - 0.04) * 2.0 - (2.0 / 3.0 * pi * big.powi(3) - core)
    };
    assert!((vol(&body) - v_dimple).abs() < 1e-9, "fixture volume {} vs {v_dimple}", vol(&body));
    for r in [0.6, 0.8] {
        let xc = ((big + r).powi(2) - r * r).sqrt();
        let (k, c) = ((big, 2.0), (xc, 2.0 - r));
        let fp = (xc, 2.0);
        let s = big / (big + r);
        let fs = (o.0 + s * (c.0 - o.0), o.1 + s * (c.1 - o.1));
        let mut pieces = quad(k, fp, c, fs).to_vec();
        pieces.push((-1.0, sector(c, r, fp, fs)));
        pieces.push((-1.0, segment(o, big, fs, k)));
        build_and_check("dimple rim", &body, (big, 2.0), r, -pappus_volume(&pieces));
    }
}

/// A 45° countersink (cone from (0.8, 2) down to (0.3, 1.5)) in a plate:
/// its top rim is CONVEX with the ball OUTSIDE the cone. r = 1.0 > ρ.
#[test]
fn countersink_rim_past_rho_builds_at_the_closed_form() {
    let body = revolved_about_y(
        corners(&[(0.3, 0.0), (3.0, 0.0), (3.0, 2.0), (0.8, 2.0), (0.3, 1.5)]),
        Revolution::Full,
        tol(),
    );
    let r = 1.0;
    let d = r * (core::f64::consts::PI / 8.0).tan(); // r / tan(67.5°)
    let k = (0.8, 2.0);
    let c = (0.8 + d, 2.0 - r);
    let fp = (c.0, 2.0);
    let h = r * core::f64::consts::FRAC_1_SQRT_2;
    let fs = (c.0 - h, c.1 + h);
    let mut pieces = quad(k, fp, c, fs).to_vec();
    pieces.push((-1.0, sector(c, r, fp, fs)));
    build_and_check("countersink rim", &body, (0.8, 2.0), r, -pappus_volume(&pieces));
}

/// The ball INSIDE a frustum's top rim (ρ = 0.5 there): r = 0.6 must
/// still refuse on the cone's curvature.
#[test]
fn ball_inside_a_cone_rim_still_refuses() {
    let body = revolved_about_y(
        corners(&[(0.1, 0.0), (1.0, 0.0), (0.5, 1.0), (0.1, 1.0)]),
        Revolution::Full,
        tol(),
    );
    let edges = rim_arcs_at(&body, 0.5, 1.0);
    match fillet_edges(&body, &edges, 0.6, tol()) {
        Err(e) => {
            eprintln!("cone inside: {:?}", e.error);
            assert!(
                matches!(e.error, BlendError::RadiusHeadroom { .. } | BlendError::SpineIrregular { .. }),
                "{:?}",
                e.error
            );
        }
        Ok(_) => panic!("a ball inside a cone past ρ built"),
    }
}

/// A D-rod (radius 1, flat at y = −0.8): its two creases are line spines,
/// so spine regularity saturates and ONLY the headroom stands between a
/// ball inside the rod at r ≥ R and a folded band.
#[test]
fn ball_inside_a_d_rod_at_r_past_r_refuses_headroom() {
    let body = prism(
        vec![
            (Point2::new(-0.6, -0.8), 0.0),
            (Point2::new(0.6, -0.8), 3.0),
        ],
        4.0,
        tol(),
    );
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2);
    for r in [1.0, 1.2] {
        match fillet_edges(&body, &creases[..1], r, tol()) {
            Err(e) => {
                eprintln!("d-rod r {r}: {:?}", e.error);
                assert!(matches!(e.error, BlendError::RadiusHeadroom { .. }), "{:?}", e.error);
            }
            Ok(_) => panic!("r {r}: a ball inside a rod past R built"),
        }
    }
}

/// The issue's cone-foot defect, end to end: a frustum's base rim whose
/// ball foot sits far up a flat (60° half-angle) cone. Local fit at the
/// foot fails for r > ~0.268 while the edge read ρ = 1 passes to r < 1.
/// Spine regularity (ρ_c = ρ_f − r·cos α > r) must refuse first.
#[test]
fn a_cone_foot_past_its_bend_is_caught_by_the_spine() {
    let t30 = (core::f64::consts::PI / 6.0).tan();
    let body = revolved_about_y(
        corners(&[(0.1, 0.0), (1.0, 0.0), (0.2, 0.8 * t30), (0.1, 0.8 * t30)]),
        Revolution::Full,
        tol(),
    );
    let edges = rim_arcs_at(&body, 1.0, 0.0);
    for r in [0.22, 0.27, 0.3, 0.5] {
        match fillet_edges(&body, &edges, r, tol()) {
            Ok(f) => {
                let v = validate_geometric(&f.body, tol());
                panic!("cone foot r {r}: BUILT, tier3 {v:?}");
            }
            Err(e) => {
                eprintln!("cone foot r {r}: {:?}", e.error);
                assert!(!matches!(e.error, BlendError::RadiusHeadroom { .. }));
            }
        }
    }
}

fn sliver_outcome<T>(delta: f64, convex: bool, r: f64) -> String
where
    T: geom_core::Decide + geom_core::Bounds + topo::AtRestPolicy + core::fmt::Debug,
{
    let v = |x: f64, y: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    // convex: a near-flat countersink (cone dips δ below the top plane);
    // concave: a near-flat mound (cone rises δ above the plate).
    let pts = if convex {
        vec![v(0.3, 0.0), v(3.0, 0.0), v(3.0, 2.0), v(0.8, 2.0), v(0.3, 2.0 - delta)]
    } else {
        vec![v(0.3, 0.0), v(3.0, 0.0), v(3.0, 2.0), v(0.8, 2.0), v(0.3, 2.0 + delta), v(0.3, 2.0 + delta)]
            .into_iter()
            .take(5)
            .collect()
    };
    let Ok(body) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Body<T> {
        sweep::test_support::revolved_about_y_at(pts, Revolution::Full, tol())
    })) else {
        return "fixture does not revolve".into();
    };
    let edges = rim_arcs_at(&body, 0.8, 2.0);
    match fillet_edges(&body, &edges, T::from_f64(r), tol()) {
        Ok(f) => format!("BUILT tier3 {:?}", topo::validate_closed(&f.body)),
        Err(e) => format!("{:?}", e.error).chars().take(140).collect(),
    }
}

/// Near-tangent creases: the side must never be read off an undecided
/// dihedral. Prints the outcome at each δ (run at each eps row).
#[test]
fn sliver_creases_escalate_or_refuse() {
    for convex in [true, false] {
        for delta in [1e-2, 1e-4, 1e-6, 1e-8, 1e-10, 1e-12] {
            let f = sliver_outcome::<f64>(delta, convex, 0.3);
            let i = sliver_outcome::<geom_core::Interval>(delta, convex, 0.3);
            eprintln!("SLIVER convex {convex} δ {delta:e}: f64 {f} | Interval {i}");
        }
    }
}

/// A thin bent wall (WALL = 0.05, a 150° convex inner corner), the
/// Klein bottle's neck→flare in miniature: `(a, 1) → K = (a, 0)` down
/// the bore, then down-out along a 30° cone, back up the outer wall.
/// Returns the meridian corners and the outer corner K'.
fn thin_flare(a: f64) -> (Vec<(f64, f64)>, (f64, f64)) {
    let w = 0.05;
    let (s, c) = (0.5_f64, 0.75_f64.sqrt()); // sin 30°, cos 30°
    let dir = (s, -c);
    let n = (c, s);
    let k = (a, 0.0);
    let e = (k.0 + 2.0 * dir.0, k.1 + 2.0 * dir.1);
    let eo = (e.0 + w * n.0, e.1 + w * n.1);
    // outer corner: x = a + w on the outer cone line through k + w n.
    let t = (a + w - (k.0 + w * n.0)) / dir.0;
    let ko = (a + w, k.1 + w * n.1 + t * dir.1);
    (vec![(a, 1.0), k, e, eo, ko, (a + w, 1.0)], ko)
}

/// The point of the convex inner corner's fillet arc on the corner's
/// bisector, `K + (r/sin 75° − r)·b̂`: a convex fillet's band must lie
/// inside the original material, so this point must lie short of K'.
fn bisector_point(k: (f64, f64), r: f64) -> (f64, f64) {
    let b = (15.0_f64.to_radians().cos(), 15.0_f64.to_radians().sin());
    let d = r / 75.0_f64.to_radians().sin() - r;
    (k.0 + d * b.0, k.1 + d * b.1)
}

/// MAJOR probe: past r* = WALL/(sec 15° − 1)·… ≈ 1.466 the ball's band
/// pierces the far wall at the outer corner. On main predicate 1 refused
/// every r > 0.225 here (the bore's radius); on the PR head nothing does.
#[test]
fn a_thin_flared_wall_admits_a_band_that_pierces_its_far_wall() {
    let a = 0.225;
    let (pts, ko) = thin_flare(a);
    let body = revolved_about_y(corners(&pts), Revolution::Full, tol());
    let edges = rim_arcs_at(&body, a, 0.0);
    assert!(!edges.is_empty());
    for r in [1.4, 1.5, 1.6] {
        let p = bisector_point((a, 0.0), r);
        let pierces = p.0 > a + 0.05 && p.1 > ko.1;
        let out = match fillet_edges(&body, &edges, r, tol()) {
            Ok(f) => format!(
                "BUILT tier3 {:?} V {:.9}",
                validate_geometric(&f.body, tol()),
                vol(&f.body)
            ),
            Err(e) => format!("refused {:?}", e.error).chars().take(120).collect(),
        };
        eprintln!(
            "THIN r {r}: band point on bisector {p:?}, outer corner {ko:?}, past the far wall: {pierces}; {out}"
        );
        assert!(
            !(pierces && out.starts_with("BUILT")),
            "r {r}: the band leaves the material through the far wall, yet the fillet built: {out}"
        );
    }
}

