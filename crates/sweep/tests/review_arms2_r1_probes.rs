//! **Reviewer probes for PR #962 (VERBS-ARMS-2), lane r1.**
//!
//! Independent consumer rows: solids of revolution authored HERE, with
//! hand-derived closed forms the shipped suites do not carry, plus a
//! bit-identity dump for the plane–sphere ANNULUS carve (the one
//! plane–sphere path `bitdump.rs` does not cover — the dome's one-edge
//! rim), for the C3 "bit-identical to ARMS-1" claim.
//!
//! RED-ability: every arm row here fails at the merge base (the arms do
//! not exist there), and each pins a hand number a wrong fold, wrong
//! branch, or wrong spine would move.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::fmt::Write as _;

use geom::{Curve3, Surface};
use geom_core::{Band, Point2, Tol};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::revolved_about_y;
use topo::{Body, EdgeKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::new(tol().eps(), tol().k() * tol().eps()).unwrap()
}

fn body_of(verts: Vec<ProfileVertex<f64>>) -> Body<f64> {
    revolved_about_y(verts, Revolution::Full, tol())
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(Point2::new(x, y), bulge)
}

/// Closed latitude rims of radius `r` (1e-9), with their axial station.
fn closed_rims(body: &Body<f64>, r: f64) -> Vec<(EdgeKey, f64)> {
    body.edges()
        .filter_map(|(k, e)| {
            let start = body.get_half_edge(e.he_plus)?.start;
            if Some(start) != body.half_edge_end(e.he_plus) {
                return None;
            }
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                Curve3::Circle { radius, center, .. } if (radius - r).abs() < 1e-9 => {
                    Some((k, center.y))
                }
                _ => None,
            }
        })
        .collect()
}

fn closed_rim_at(body: &Body<f64>, r: f64, y: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = closed_rims(body, r)
        .into_iter()
        .filter(|(_, cy)| (cy - y).abs() < 1e-9)
        .map(|(k, _)| k)
        .collect();
    assert_eq!(hits.len(), 1, "one closed rim of radius {r} at y = {y}");
    hits[0]
}

fn band_torus(body: &Body<f64>, face: topo::FaceKey) -> (f64, f64, f64) {
    let s = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    let Surface::Torus {
        center,
        major_radius,
        minor_radius,
        ..
    } = *s
    else {
        panic!("the band's surface is a torus, got {s:?}");
    };
    (major_radius, minor_radius, center.y)
}

// ------------------------------------------------------------------
// Row 1: a sphere×cone seam at a DIFFERENT latitude and slope than the
// bud's — (0.6, 0.8) on the unit sphere, pucker falling 2 units of
// radius per unit of axis (the bud's is (0.8, 0.6) at 3:1).
// ------------------------------------------------------------------

const R1: f64 = 0.04;

/// The ball centre from the two defining equations, solved here by the
/// offset-line-into-offset-circle substitution, root nearest the mouth
/// — independent of the kernel's spelling.
fn mouth_centre(mx: f64, my: f64, nx: f64, ny: f64, r: f64) -> (f64, f64) {
    let (tx, ty) = (-ny, nx);
    let (ox, oy) = (mx - nx * r, my - ny * r);
    let b = ox * tx + oy * ty;
    let c = ox * ox + oy * oy - (1.0 - r) * (1.0 - r);
    let disc = (b * b - c).sqrt();
    let (l1, l2) = (-b + disc, -b - disc);
    let near = |l: f64| (ox + tx * l - mx).powi(2) + (oy + ty * l - my).powi(2);
    let l = if near(l1) <= near(l2) { l1 } else { l2 };
    (ox + tx * l, oy + ty * l)
}

fn high_bud() -> Body<f64> {
    // Sphere zone equator → asin(0.8); cone (0.6,0.8) → (0.2,1.0).
    let bulge = (0.8f64.asin() / 4.0).tan();
    body_of(vec![
        v(0.2, 0.0, 0.0),
        v(1.0, 0.0, bulge),
        v(0.6, 0.8, 0.0),
        v(0.2, 1.0, 0.0),
    ])
}

#[test]
fn a_sphere_cone_seam_at_a_second_latitude_fillets_to_its_hand_torus() {
    let source = high_bud();
    let mouth = closed_rim_at(&source, 0.6, 0.8);
    let out = fillet_edges(&source, &[mouth], R1, tol())
        .unwrap_or_else(|e| panic!("the 2:1 pucker's sphere-cone seam fillets, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    // Cone direction (−2, 1)/√5, outward normal (1, 2)/√5.
    let s5 = 5.0f64.sqrt();
    let (cx, cy) = mouth_centre(0.6, 0.8, 1.0 / s5, 2.0 / s5, R1);
    let (major, minor, band_y) = band_torus(&out.body, out.band_faces[0]);
    assert!((minor - R1).abs() < 1e-12, "tube {minor}");
    assert!(
        (major - cx).abs() < 1e-12,
        "spine radius: hand {cx}, kernel {major}"
    );
    assert!(
        (band_y - cy).abs() < 1e-12,
        "axial station: hand {cy}, kernel {band_y}"
    );
    // Both trim circles at their closed forms.
    let sphere_trim = (cx / (1.0 - R1), cy / (1.0 - R1));
    let cone_trim = (cx + R1 / s5, cy + 2.0 * R1 / s5);
    for (want_r, want_y, which) in [
        (sphere_trim.0, sphere_trim.1, "sphere"),
        (cone_trim.0, cone_trim.1, "cone"),
    ] {
        let e = closed_rim_at(&out.body, want_r, want_y);
        let c = out
            .body
            .get_curve_geom(out.body.get_edge(e).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap();
        let Curve3::Circle { center, radius, .. } = *c.carrier() else {
            panic!("a trim circle")
        };
        assert!((radius - want_r).abs() < 1e-12, "{which} trim radius");
        assert!((center.y - want_y).abs() < 1e-12, "{which} trim station");
    }
}

// ------------------------------------------------------------------
// Row 2: cylinder×plane, BOTH material configurations, one washer —
// the outer drum rim folds R − r, the bore rim folds R + r.
// ------------------------------------------------------------------

fn washer() -> Body<f64> {
    body_of(vec![
        v(0.2, 0.0, 0.0),
        v(1.0, 0.0, 0.0),
        v(1.0, 0.5, 0.0),
        v(0.2, 0.5, 0.0),
    ])
}

#[test]
fn the_cylinder_plane_arm_carves_both_material_configurations() {
    let r = 0.06;
    // (rim radius, expected spine radius): the outer wall's outward
    // normal is its chart normal (R − r); the bore wall's is the
    // negation (R + r). Both ball centres sit at y = 0.5 − r.
    for (rim_r, want_major, which) in [(1.0, 1.0 - r, "drum"), (0.2, 0.2 + r, "bore")] {
        let source = washer();
        let rim = closed_rim_at(&source, rim_r, 0.5);
        let out = fillet_edges(&source, &[rim], r, tol())
            .unwrap_or_else(|e| panic!("{which} rim fillets, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{which}: tier-3 valid, got {e:?}"));
        let (major, minor, band_y) = band_torus(&out.body, out.band_faces[0]);
        assert!((minor - r).abs() < 1e-15, "{which} tube");
        assert!(
            (major - want_major).abs() < 1e-15,
            "{which}: R ∓ r exactly — want {want_major}, got {major}"
        );
        assert!((band_y - (0.5 - r)).abs() < 1e-15, "{which} station");
    }
}

// ------------------------------------------------------------------
// Row 3: cone×cone — the line×line crossing, from the two support
// equations solved here by 2×2 elimination.
// ------------------------------------------------------------------

fn double_cone() -> Body<f64> {
    body_of(vec![
        v(0.2, 0.0, 0.0),
        v(1.0, 0.0, 0.0),
        v(0.7, 0.3, 0.0),
        v(0.2, 0.4, 0.0),
    ])
}

#[test]
fn a_cone_cone_rim_fillets_to_the_hand_crossing() {
    let r = 0.03;
    let source = double_cone();
    let rim = closed_rim_at(&source, 0.7, 0.3);
    let out = fillet_edges(&source, &[rim], r, tol())
        .unwrap_or_else(|e| panic!("the cone-cone rim fillets, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    // Outward normals in the meridian: cone A runs (−1,1)/√2 so
    // n_a = (1,1)/√2; cone B runs (−5,1)/√26 so n_b = (1,5)/√26.
    // Solve (c−m)·n_a = −r, (c−m)·n_b = −r directly.
    let s2 = 2.0f64.sqrt();
    let s26 = 26.0f64.sqrt();
    let (nax, nay) = (1.0 / s2, 1.0 / s2);
    let (nbx, nby) = (1.0 / s26, 5.0 / s26);
    let det = nax * nby - nay * nbx;
    let dx = (-r * nby - -r * nay) / det;
    let dy = (nax * -r - nbx * -r) / det;
    let (cx, cy) = (0.7 + dx, 0.3 + dy);
    let (major, minor, band_y) = band_torus(&out.body, out.band_faces[0]);
    assert!((minor - r).abs() < 1e-12);
    assert!(
        (major - cx).abs() < 1e-12,
        "spine radius: hand {cx}, kernel {major}"
    );
    assert!((band_y - cy).abs() < 1e-12, "station: hand {cy}");
}

// ------------------------------------------------------------------
// Row 4: cylinder×sphere — the spec's "R ∓ r exactly" spine.
// ------------------------------------------------------------------

fn capped_drum() -> Body<f64> {
    // Base, outer cylinder 0.8, unit-sphere zone (0.8,0.6)→(0.6,0.8),
    // top plane, bore.
    let sweep = 0.8f64.asin() - 0.6f64.asin();
    let bulge = (sweep / 4.0).tan();
    body_of(vec![
        v(0.2, 0.0, 0.0),
        v(0.8, 0.0, 0.0),
        v(0.8, 0.6, bulge),
        v(0.6, 0.8, 0.0),
        v(0.2, 0.8, 0.0),
    ])
}

#[test]
fn a_cylinder_sphere_rim_fillets_to_r_minus_r_exactly() {
    let r = 0.05;
    let source = capped_drum();
    let rim = closed_rim_at(&source, 0.8, 0.6);
    let out = fillet_edges(&source, &[rim], r, tol())
        .unwrap_or_else(|e| panic!("the cylinder-sphere rim fillets, got {e:?}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    let (major, minor, band_y) = band_torus(&out.body, out.band_faces[0]);
    assert!((minor - r).abs() < 1e-15);
    // The ball centre: ρ = 0.8 − r (the cylinder fold, exact) and
    // |c| = 1 − r (inside the unit sphere), so y = √((1−r)² − ρ²).
    let rho = 0.8 - r;
    let want_y = ((1.0 - r).powi(2) - rho.powi(2)).sqrt();
    assert!(
        (major - rho).abs() < 1e-15,
        "R − r exactly: want {rho}, got {major}"
    );
    assert!((band_y - want_y).abs() < 1e-12, "station {want_y}");
}

// ------------------------------------------------------------------
// Row 5: the differential — a sphere×sphere waist (two DISTINCT
// centres). The ARM now exists, so the row's question moved one door
// down: what this waist meets is its own CONCAVITY, not a missing arm.
// ------------------------------------------------------------------

fn snowman() -> Body<f64> {
    // Unit sphere at the origin up to (0.8, 0.6); unit sphere centred
    // (0, 1.2) from (0.8, 0.6) up to its equator (1.0, 1.2). The waist
    // rim is the spheres' intersection circle.
    let b1 = (0.6f64.asin() / 4.0).tan();
    let b2 = (0.6f64.asin() / 4.0).tan();
    body_of(vec![
        v(0.2, 0.0, 0.0),
        v(1.0, 0.0, b1),
        v(0.8, 0.6, b2),
        v(1.0, 1.2, 0.0),
        v(0.2, 1.2, 0.0),
    ])
}

/// The waist of two UNIONED spheres is a valley: the rolling ball sits
/// in the void and its band would ADD material, which the composition
/// surgery does not build. So the honest refusal here is the concave
/// chain's, and the fact this row now pins is that the SPINE door is
/// passed — the battery classifies the pair, mints its torus, and hands
/// the chain on.
#[test]
fn a_sphere_sphere_waist_reaches_its_arm_and_refuses_as_a_concave_chain() {
    let source = snowman();
    // Both walls really are spheres with distinct centres.
    let mut sphere_centres: Vec<f64> = source
        .faces()
        .filter_map(|(k, _)| {
            let fd = source.get_face(k)?;
            match source.get_surface(fd.surface)? {
                Surface::Sphere { center, .. } => Some(center.y),
                _ => None,
            }
        })
        .collect();
    sphere_centres.sort_by(f64::total_cmp);
    assert_eq!(sphere_centres.len(), 2, "two sphere walls");
    assert!((sphere_centres[0]).abs() < 1e-12 && (sphere_centres[1] - 1.2).abs() < 1e-12);
    let waist = closed_rim_at(&source, 0.8, 0.6);
    match fillet_edges(&source, &[waist], 0.05, tol()).map_err(|r| r.error) {
        Err(BlendError::UnsupportedChain { detail, .. }) => {
            assert!(
                detail.contains("concave"),
                "the waist is a valley, so the refusal is the concave chain's: {detail}"
            );
        }
        other => panic!("a sphere-sphere waist refuses as a concave chain, got {other:?}"),
    }
    // The arm door itself is PASSED, and the roster says so: the pair
    // the refusal above no longer names is advertised as implemented.
    assert!(
        sweep::blend::battery::arm_roster().contains("sphere–sphere"),
        "the sphere-sphere arm is advertised"
    );
}

// ------------------------------------------------------------------
// Row 6: the C3 bit-identity dump — the plane–sphere ANNULUS carve
// (the dome's one-edge rim), the path `bitdump.rs` does not cover and
// the path this PR's host/mate surgery rewrite actually touched.
// Armed by BITDUMP_DIR exactly as `bitdump.rs` is; clean skip unarmed.
// ------------------------------------------------------------------

fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|vv| body.get_point(vv.point))
            .unwrap();
        let _ = writeln!(s, "V {k:?} ({:?}, {:?}, {:?})", p.x, p.y, p.z);
    }
    for (k, e) in body.edges() {
        let _ = write!(s, "E {k:?} he+={:?} he-={:?}", e.he_plus, e.he_minus);
        match body.get_curve_geom(e.curve).and_then(|g| g.certified()) {
            Some(c) => {
                let (t0, t1) = c.params();
                let _ = writeln!(
                    s,
                    " carrier={:?} params=({t0:?}, {t1:?}) desc={:?}",
                    c.carrier(),
                    c.description()
                );
            }
            None => {
                let _ = writeln!(s, " UNCERTIFIED");
            }
        }
    }
    for (k, _) in body.faces() {
        let fd = body.get_face(k).unwrap();
        let surf = body.get_surface(fd.surface).unwrap();
        let _ = writeln!(
            s,
            "F {k:?} sense={:?} rings={} surface={surf:?}",
            fd.sense,
            fd.rings.len()
        );
    }
    s
}

#[test]
fn bitdump_dome_annulus() {
    let Some(dir) = std::env::var("BITDUMP_DIR").ok().filter(|d| !d.is_empty()) else {
        return;
    };
    let source = revolved_about_y(
        sweep::test_support::dome_profile(1.0),
        Revolution::Full,
        tol(),
    );
    let rim = sweep::test_support::closed_plane_sphere_rim(&source, 1.0);
    let out = fillet_edges(&source, &[rim], 0.05, tol()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(text, "band={:?}", out.band_faces);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/dome_annulus.txt"), text).unwrap();
}
