//! **The interior-loop guard reads a NURBS face against a plane through
//! C3's whole reading.** Lily's blade and sheath meet cap to wall, and a
//! plane cuts the blade's loft across its spine and lengthwise. The
//! union of blade and sheath stops at the rim first (their walls touch
//! at one point of it, `a_sheath_grafted_on_its_blade`), so the cap
//! pairs are read by the guard outside the boolean.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::loft_body;
use topo::Body;

fn lens([width, ridge, keel]: [f64; 3]) -> Vec<ProfileLoop<f64>> {
    let half = 0.5 * width;
    vec![bulge_loop(vec![
        (Point2::new(half, 0.0), -ridge / half),
        (Point2::new(-half, 0.0), -keel / half),
    ])]
}

/// A lens lofted along an arc of `curl` radians and length `len` from
/// the origin along `d`, bending toward `v`, its section at `s ∈ [0, 1]`.
fn blade(
    d: Vec3<f64>,
    v: Vec3<f64>,
    (len, curl): (f64, f64),
    section: impl Fn(f64) -> [f64; 3],
    tol: Tol,
) -> topo::AtRestBody<f64> {
    const STATIONS: usize = 5;
    let band = Band::linear(tol).expect("the run's tolerance forms a band");
    let u = v.cross(d);
    let r = len / curl;
    let mut sections = Vec::with_capacity(STATIONS);
    let mut places = Vec::with_capacity(STATIONS);
    for k in 0..STATIONS {
        #[allow(clippy::cast_precision_loss)]
        let s = (k as f64) / ((STATIONS - 1) as f64);
        let (sa, ca) = (curl * s).sin_cos();
        let p = Point3::origin() + d * (r * sa) + v * (r * (1.0 - ca));
        let vk = v * ca - d * sa;
        sections.push(lens(section(s)));
        places.push(SketchPlane::from_frame(
            RigidMotion::gram_schmidt(p, u, vk, "sheath_blade", band)
                .expect("the station's two directions span a plane"),
        ));
    }
    let lofted = loft_body::<f64>(&sections, &places, 3, tol)
        .expect("the lens blade lofts")
        .body;
    topo::test_support::finished("a lens blade", lofted, tol)
}

/// Lily's blade (a leaf curling down from the origin along `x`) and her
/// sheath (straight back along `−x`), standing on each other's base cap.
fn lily(tol: Tol) -> (topo::AtRestBody<f64>, topo::AtRestBody<f64>) {
    let base = [0.170, 0.028, 0.020];
    let (d, v) = (Vec3::unit_x(), Vec3::unit_z());
    let leaf = blade(
        d,
        v,
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    );
    let sheath = blade(-d, v, (0.34, 0.85), |_| base, tol);
    (leaf, sheath)
}

fn kind(body: &Body<f64>, face: topo::FaceKey) -> geom::SurfaceKind {
    let fd = body.get_face(face).expect("a reported face resolves");
    body.get_surface(fd.surface)
        .expect("its surface resolves")
        .kind()
}

/// **Lily's four cap × wall pairs clear through the whole reading.**
/// Each body's base cap is the plane `x = 0`, and the other body's two
/// walls stand on it along their rim: the plane holds a side of each
/// wall's rectangle, a `Side` region, the one component, which reaches
/// the wall's chart boundary (W1′). Read in both operand orders, at the
/// run's ε (the suite runs it at 1e-9, 1e-6 and 1e-12). Red against the
/// whole reading dropped (each pair refuses on reach, as before), and
/// against a `Side` region listed as a loop (it carries no witness, so
/// R-undec).
#[test]
fn lilys_four_cap_pairs_clear_through_the_whole_reading() {
    let tol = Tol::witness();
    let (leaf, sheath) = lily(tol);
    for (what, a, b) in [
        ("blade, sheath", &leaf, &sheath),
        ("sheath, blade", &sheath, &leaf),
    ] {
        let t = std::time::Instant::now();
        let caps: Vec<String> = topo::test_support::no_crossings_section_report(a, b, tol)
            .expect("the section pass reads every pair")
            .into_iter()
            .filter_map(|(fa, fb, v)| match (kind(a, fa), kind(b, fb)) {
                (geom::SurfaceKind::Plane, geom::SurfaceKind::Nurbs) => {
                    Some(format!("Plane × Nurbs: {v}"))
                }
                (geom::SurfaceKind::Nurbs, geom::SurfaceKind::Plane) => {
                    Some(format!("Nurbs × Plane: {v}"))
                }
                _ => None,
            })
            .collect();
        eprintln!("{what}: the pass read every pair in {:.2?}", t.elapsed());
        let mut want = vec![
            "Plane × Nurbs: Ok([Reaches(G)])".to_string(),
            "Plane × Nurbs: Ok([Reaches(G)])".to_string(),
            "Nurbs × Plane: Ok([Reaches(F)])".to_string(),
            "Nurbs × Plane: Ok([Reaches(F)])".to_string(),
        ];
        let mut got = caps.clone();
        got.sort();
        want.sort();
        assert_eq!(
            got, want,
            "{what}: the four cap × wall pairs, each one side region"
        );
    }
}

/// **A plane cutting the blade's loft clears, across the spine and
/// lengthwise.** The plane `x = 0.5` crosses the curled blade's two
/// walls square to its spine; the plane `z = −0.01` runs along it
/// through the keel. Each meets each wall in branches that end on the
/// wall's rectangle, so every component reaches its chart boundary and
/// the guard clears the pair (W1′). The timing of each reading is
/// printed. Red against the open branches read as loops (no witness
/// places them: R-undec), and against the whole reading dropped
/// (R-reach).
#[test]
fn a_plane_cutting_the_blades_loft_clears() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let (leaf, _) = lily(tol);
    let walls: Vec<Surface<f64>> = leaf
        .faces()
        .filter_map(|(_, f)| match leaf.get_surface(f.surface) {
            Some(s @ Surface::Nurbs(_)) => Some(s.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(walls.len(), 2, "the blade's two walls");
    let across = Surface::Plane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let along = Surface::Plane {
        origin: Point3::new(0.0, 0.0, -0.01),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (what, plane) in [("across the spine", &across), ("lengthwise", &along)] {
        for (i, w) in walls.iter().enumerate() {
            let t = std::time::Instant::now();
            let got = topo::test_support::section_verdict(plane, w, (Point3::origin(), 1.0), band);
            eprintln!(
                "plane {what} × blade wall {i}: {got} in {:.2?}",
                t.elapsed()
            );
            let cleared = got
                .strip_prefix("Ok([")
                .and_then(|r| r.strip_suffix("])"))
                .is_some_and(|r| r.split(", ").all(|c| c == "Reaches(G)"));
            assert!(
                cleared,
                "plane {what} × wall {i}: every component reaches the wall's boundary: {got}"
            );
        }
    }
}
