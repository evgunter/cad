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
        let caps: Vec<String> = topo::test_support::no_event_section_report(a, b, tol)
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

/// Lily's leaf with its spine raised 0.15 rad: curling down 0.3 rad
/// along its length, its ridge peaks near the middle, so a plane `z = h`
/// just below that peak meets the ridge wall in one oval inside it.
fn raised_leaf(tol: Tol) -> topo::AtRestBody<f64> {
    let (sa, ca) = 0.15_f64.sin_cos();
    let base = [0.170, 0.028, 0.020];
    blade(
        Vec3::new(ca, 0.0, sa),
        Vec3::new(-sa, 0.0, ca),
        (1.2, -0.3),
        |s| [0, 1, 2].map(|i| base[i] + ([0.300, 0.034, 0.016][i] - base[i]) * s),
        tol,
    )
}

/// **A spline witness by a face's edge is placed on the band widened by
/// its pad, through the boolean's own placement.** The plane a little
/// below the raised leaf's ridge peak meets its ridge wall in one oval,
/// read as one loop with a witness within its pad `r` of it. A box whose
/// bottom face lies in that plane, its edge `d` from the witness on the
/// side the oval runs on, is read against the wall by the section pass
/// on real faces. The witness and its pad are those of the wall's
/// reading against the box's own floor, the plane as the box stores it,
/// which is what the pass reads:
/// - `d` = the band's escalation plus `1.5·r`: past the band and past
///   it padded by `r`, inside the band the rule pads by `2r`, so the
///   placement does not read the witness out of the box and the pair is
///   not cleared;
/// - `d` ten escalations: the witness is out of the box (W3).
///
/// Red against the boolean placing a spline witness on the run's band,
/// and against the rule padding by one pad rather than two (the first
/// box clears `Out`).
#[test]
fn a_spline_witness_by_an_edge_is_placed_on_the_widened_band() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let blade = raised_leaf(tol);
    let (ridge_face, ridge) = blade
        .faces()
        .filter_map(|(k, f)| match blade.get_surface(f.surface) {
            Some(s @ Surface::Nurbs(n)) => {
                let top = n.control().iter().map(|p| p.z).fold(f64::MIN, f64::max);
                Some((k, s.clone(), top))
            }
            _ => None,
        })
        .max_by(|a, b| a.2.total_cmp(&b.2))
        .map(|(k, s, _)| (k, s))
        .expect("the blade has spline walls");
    // The ridge's peak, sampled; the plane stands a little below it.
    let Surface::Nurbs(net) = &ridge else {
        unreachable!("a spline wall")
    };
    let ((u0, u1), (v0, v1)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let peak = (0..=64)
        .flat_map(|i| (0..=64).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (a, b) = (f64::from(i) / 64.0, f64::from(j) / 64.0);
            net.eval(u0 + (u1 - u0) * a, v0 + (v1 - v0) * b)
        })
        .max_by(|a, b| a.z.total_cmp(&b.z))
        .expect("samples");
    let h = peak.z - 2e-3;
    let plane = Surface::Plane {
        origin: Point3::new(peak.x, 0.0, h),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let Ok(geom_brep::SectionRead::Built(section)) =
        geom_brep::pair_section_with(&plane, &ridge, Some(geom_brep::SsiLane::marched()), band)
    else {
        panic!("the ridge's section is read")
    };
    let geom_brep::Components::Counted(components) = section.components() else {
        panic!("a spline section is counted: {section:?}")
    };
    let [c] = components.as_slice() else {
        panic!("one oval: {section:?}")
    };
    assert_eq!(c.shape, geom_brep::ComponentShape::Loop, "{c:?}");
    let w = c.witness.expect("the oval carries a witness");
    // The box's edge stands `d` past the witness `wx` toward the ridge's
    // peak, the oval's middle, so the box holds part of the oval and not
    // the witness.
    let block_at = |wx: f64, d: f64| {
        let x = if wx < peak.x {
            (wx + d, peak.x + 0.3)
        } else {
            (peak.x - 0.3, wx - d)
        };
        let block = topo::test_support::finished(
            "a box on the plane",
            sweep::test_support::brick::<f64>(x, (-1.0, 1.0), (h, h + 0.5), tol),
            tol,
        );
        (block, if wx < peak.x { x.0 } else { x.1 })
    };
    let floor = |block: &Body<f64>| {
        block
            .faces()
            .find_map(|(k, f)| match block.get_surface(f.surface) {
                Some(s @ Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && (origin.z - h).abs() < 1e-12 =>
                {
                    Some((k, s.clone()))
                }
                _ => None,
            })
            .expect("the box has a floor in the plane")
    };
    // The boolean reads the ridge against the box's floor, a plane stored
    // as the box stores it, so the witness and pad the edge is set by are
    // that reading's.
    let loop_on = |floor: &Surface<f64>| {
        let Ok(geom_brep::SectionRead::Built(section)) =
            geom_brep::pair_section_with(&ridge, floor, Some(geom_brep::SsiLane::marched()), band)
        else {
            panic!("the ridge's section with the floor is read")
        };
        let geom_brep::Components::Counted(components) = section.components() else {
            panic!("a spline section is counted: {section:?}")
        };
        let [c] = components.as_slice() else {
            panic!("one oval: {section:?}")
        };
        (c.witness.expect("the oval carries a witness"), c.pad)
    };
    let (first, _) = block_at(w.x, band.escalate() + 1.5 * c.pad);
    let (w0, pad0) = loop_on(&floor(&first).1);
    let (near, edge) = block_at(w0.x, band.escalate() + 1.5 * pad0);
    let (wn, pad) = loop_on(&floor(&near).1);
    let gap = (edge - wn.x).abs();
    assert!(
        band.escalate() + pad < gap && gap < band.escalate() + 2.0 * pad,
        "the edge {gap:e} from the witness stands between one pad and two past the band: {pad:e}"
    );
    let (far, _) = block_at(w.x, 10.0 * band.escalate());
    for (block, d, want) in [
        (near, gap, "Err(Undecided)"),
        (far, 10.0 * band.escalate(), "Ok([Out(G)])"),
    ] {
        let bottom = floor(&block).0;
        let got: Vec<String> = topo::test_support::no_event_section_report(&blade, &block, tol)
            .expect("the section pass reads every pair")
            .into_iter()
            .filter(|&(fa, fb, _)| fa == ridge_face && fb == bottom)
            .map(|(_, _, v)| v)
            .collect();
        assert_eq!(
            got,
            vec![want.to_string()],
            "the box's edge {d:e} from the witness"
        );
    }
}
