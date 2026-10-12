//! Review probes for PR 4815 (TANG: `split_edge` routes a tangent child
//! through the must-carry rule). Scratch, not for merge.
//!
//! Matrix: scale × pose × profile turn × c × both cuts × both normals,
//! with an independent oracle (volume, the stored image against the
//! analytic ruling), then downstream consumers on the cut pieces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::Profile;
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn tol() -> Tol {
    Tol::witness()
}

fn rot2(phi: f64, p: (f64, f64)) -> Point2<f64> {
    let (c, s) = (phi.cos(), phi.sin());
    Point2::new(c * p.0 - s * p.1, s * p.0 + c * p.1)
}

/// The shoulder of `wedge_end_doors.rs`, scaled by `s` (height `s`),
/// its profile turned by `phi` about the origin.
fn shoulder(s: f64, phi: f64) -> topo::AtRestBody<f64> {
    let bulge = (std::f64::consts::PI / 8.0).tan();
    let pts = [(-1.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 1.0)];
    let bul = [0.0, 0.0, 0.0, 0.0, bulge];
    let outline = bulge_loop(
        pts.iter()
            .zip(bul)
            .map(|(&(x, y), b)| (rot2(phi, (s * x, s * y)), b))
            .collect(),
    );
    let profile = Profile::new(sketch_at(0.0), vec![outline])
        .validate(tol())
        .expect("profile validates");
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: s,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("extrudes")
    .body;
    finished("the shoulder", body, tol())
}

struct Case {
    s: f64,
    phi: f64,
    pose_name: &'static str,
    pose: Affine3<f64>,
}

fn poses(s: f64) -> Vec<(&'static str, Affine3<f64>)> {
    let off = Affine3::translation(Vec3::new(3.0 * s, -7.0 * s, 11.0 * s));
    let tilt = Affine3::rotation_about_axis(
        Point3::new(-0.2 * s, 0.1 * s, 0.4 * s),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        1.1,
    );
    vec![
        ("identity", Affine3::identity()),
        ("offset", off),
        ("tilted", tilt),
        ("offset∘tilted", off * tilt),
    ]
}

/// Distance of `p` from the line through `a` along unit `u`, and the
/// coordinate of `p` along it.
fn line_dist(p: Point3<f64>, a: Point3<f64>, u: Vec3<f64>) -> (f64, f64) {
    let d = p - a;
    let along = d.dot(u);
    ((d - u * along).norm(), along)
}

#[derive(Default)]
struct Tally {
    rows: usize,
    chart_on_cyl: usize,
    chart_on_plane: usize,
    tangent: usize,
    refused_in_band: usize,
    max_image_off: f64,
    downstream: usize,
}

fn kind_of(
    piece: &topo::Body<f64>,
    he: topo::HalfEdgeKey,
) -> (topo::SurfaceKey, geom::SurfaceKind) {
    let face = piece.face_of_half_edge(he).unwrap();
    let sk = piece.get_face(face).unwrap().surface;
    (sk, piece.get_surface(sk).unwrap().kind())
}

#[allow(clippy::too_many_lines)]
fn run_case(case: &Case, cs: &[f64], downstream: bool, fails: &mut Vec<String>, tally: &mut Tally) {
    let eps = tol().eps();
    let k = tol().k();
    let s = case.s;
    let unit = (2.0 * eps * s).sqrt();
    let area = (2.0 + std::f64::consts::FRAC_PI_4) * s * s;
    let tp = rot2(case.phi, (0.0, s));
    let tn = rot2(case.phi, (0.0, -1.0));
    let first = topo::split(
        &shoulder(s, case.phi),
        &topo::test_support::split_plane(
            Point3::new(tp.x, tp.y, 0.0),
            Vec3::new(tn.x, tn.y, 0.0),
            tol(),
        ),
        tol(),
    );
    let tag0 = format!("s={s:e} phi={:.4} pose={}", case.phi, case.pose_name);
    let wall = match first {
        Ok(r) => match r.above {
            topo::SplitPart::Body(b) => b,
            topo::SplitPart::Empty => {
                fails.push(format!("{tag0}: first cut empty above"));
                return;
            }
        },
        Err(e) => {
            fails.push(format!("{tag0}: first cut refused {e:?}"));
            return;
        }
    };
    let wall = match topo::transform_rigid(&wall, &case.pose, tol()) {
        Ok(b) => finished("posed wall", b, tol()),
        Err(e) => {
            fails.push(format!("{tag0}: pose refused {e:?}"));
            return;
        }
    };
    let m = case.pose;
    let a = m.transform_point(Point3::new(tp.x, tp.y, 0.0));
    let u = m.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    for &c in cs {
        let d = c * unit;
        for (zc, up) in [(s - d, 1.0), (s - d, -1.0), (d, 1.0), (d, -1.0)] {
            let tag = format!("{tag0} c={c} d={d:e} z={zc:e} up={up}");
            tally.rows += 1;
            let plane = topo::test_support::split_plane(
                m.transform_point(Point3::new(0.0, 0.0, zc)),
                m.transform_vec(Vec3::new(0.0, 0.0, up)),
                tol(),
            );
            let result = topo::split(&wall, &plane, tol());
            let in_band = c * c > 1.0 && c * c < k;
            if in_band {
                match &result {
                    Err(topo::SplitError::Reduce(topo::SplitReduceError::CrossingInsertion {
                        source,
                        ..
                    })) if matches!(
                        **source,
                        topo::EulerOpError::SplitTangentBendEscalated { .. }
                    ) =>
                    {
                        tally.refused_in_band += 1;
                    }
                    other => fails.push(format!("{tag}: in band, got {other:?}")),
                }
                continue;
            }
            let halves = match result {
                Ok(h) => h,
                Err(e) => {
                    fails.push(format!("{tag}: refused {e:?}"));
                    continue;
                }
            };
            let heights = if up > 0.0 { [s - zc, zc] } else { [zc, s - zc] };
            let mut pieces = Vec::new();
            for (height, part) in heights.into_iter().zip([&halves.above, &halves.below]) {
                let Some(piece) = part.body() else {
                    fails.push(format!("{tag}: empty side"));
                    continue;
                };
                if let Err(e) = topo::validate_geometric(piece, tol()) {
                    fails.push(format!("{tag}: piece {height:e} tier3 {e:?}"));
                    continue;
                }
                match topo::mass_properties(piece, tol()) {
                    Ok(mp) => {
                        let want = area * height;
                        if (mp.volume - want).abs() > (1e-9 * want).max(area * eps) {
                            fails.push(format!(
                                "{tag}: piece {height:e} volume {:e} want {want:e}",
                                mp.volume
                            ));
                        }
                    }
                    Err(e) => fails.push(format!("{tag}: piece {height:e} mass {e:?}")),
                }
                let point = |v| *piece.get_point(piece.get_vertex(v).unwrap().point).unwrap();
                let on = |p: Point3<f64>| line_dist(p, a, u).0 <= 4.0 * eps;
                let seams: Vec<_> = piece
                    .edges()
                    .filter(|(_, e)| {
                        let st = piece.get_half_edge(e.he_plus).unwrap().start;
                        let en = piece.half_edge_end(e.he_plus).unwrap();
                        on(point(st))
                            && on(point(en))
                            && [e.he_plus, e.he_minus]
                                .into_iter()
                                .any(|he| kind_of(piece, he).1 == geom::SurfaceKind::Cylinder)
                    })
                    .map(|(_, e)| e.clone())
                    .collect();
                let [seam] = &seams[..] else {
                    fails.push(format!("{tag}: piece {height:e} seams {}", seams.len()));
                    continue;
                };
                let curve = piece
                    .get_curve_geom(seam.curve)
                    .and_then(|g| g.certified())
                    .unwrap();
                let (lo, hi) = if height == zc { (0.0, zc) } else { (zc, s) };
                // The carrier itself, independent of description.
                let (t0, t1) = curve.params();
                for i in 0..=8 {
                    let t = t0 + (t1 - t0) * f64::from(i) / 8.0;
                    let (off, _) = line_dist(curve.carrier().eval(t), a, u);
                    if off > eps {
                        fails.push(format!("{tag}: carrier off ruling by {off:e}"));
                    }
                }
                let short = height < 0.5 * s;
                match curve.description() {
                    geom_brep::EdgeDescription::TangentIntersection { .. } => {
                        tally.tangent += 1;
                        if short && c * c <= 1.0 {
                            fails
                                .push(format!("{tag}: short under-determined piece kept tangency"));
                        }
                    }
                    geom_brep::EdgeDescription::Chart(chart) => {
                        if !(short && c * c <= 1.0) {
                            fails.push(format!("{tag}: piece {height:e} stores a chart"));
                        }
                        let (kp, kindp) = kind_of(piece, seam.he_plus);
                        let (km, kindm) = kind_of(piece, seam.he_minus);
                        if chart.surface != kp && chart.surface != km {
                            fails.push(format!("{tag}: chart not adjacent"));
                        }
                        let kind = if chart.surface == kp { kindp } else { kindm };
                        match kind {
                            geom::SurfaceKind::Cylinder => tally.chart_on_cyl += 1,
                            _ => tally.chart_on_plane += 1,
                        }
                        if !matches!(curve.authority(), geom_brep::EdgeAuthority::Derived) {
                            fails.push(format!("{tag}: not derived"));
                        }
                        let surface = piece.get_surface(chart.surface).unwrap();
                        if c == 0.25 && up > 0.0 {
                            let u0 = chart.pcurve.eval(t0);
                            let u1 = chart.pcurve.eval(t1);
                            eprintln!(
                                "  uv {tag}: {kind:?} image ({:.17},{:.6e}) .. ({:.17},{:.6e}) wrap={}",
                                u0.x, u0.y, u1.x, u1.y, chart.wrap
                            );
                        }
                        for i in 0..=16 {
                            let t = t0 + (t1 - t0) * f64::from(i) / 16.0;
                            let uv = chart.pcurve.eval(t);
                            let p = surface.eval(uv.x, uv.y);
                            let (off, along) = line_dist(p, a, u);
                            tally.max_image_off = tally.max_image_off.max(off / eps);
                            if off > eps || along < lo - eps || along > hi + eps {
                                fails.push(format!(
                                    "{tag}: image at t={t} uv={uv:?} off {off:e} along {along:e} not in [{lo:e},{hi:e}]"
                                ));
                            }
                        }
                    }
                    other => fails.push(format!("{tag}: seam stores {other:?}")),
                }
                pieces.push((height, piece.clone()));
            }
            if downstream && pieces.len() == 2 {
                downstream_probes(&tag, s, area, m, &pieces, fails);
                tally.downstream += 1;
            }
        }
    }
}

fn downstream_probes(
    tag: &str,
    s: f64,
    area: f64,
    m: Affine3<f64>,
    pieces: &[(f64, topo::Body<f64>)],
    fails: &mut Vec<String>,
) {
    let (short_h, short) = pieces.iter().find(|(h, _)| *h < 0.5 * s).cloned().unwrap();
    let (_, long) = pieces.iter().find(|(h, _)| *h >= 0.5 * s).cloned().unwrap();
    // Mesh.
    if let Err(e) = mesh::tessellate(&short, 1e-2 * s, tol()) {
        fails.push(format!("{tag}: mesh short {e:?}"));
    }
    // Rigid map.
    let again = Affine3::rotation_about_axis(
        Point3::new(0.25 * s, -0.5 * s, 0.125 * s),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
    match topo::transform_rigid(&short, &again, tol()) {
        Ok(b) => {
            if let Err(e) = topo::validate_geometric(&b, tol()) {
                fails.push(format!("{tag}: transformed short tier3 {e:?}"));
            }
        }
        Err(e) => fails.push(format!("{tag}: transform short {e:?}")),
    }
    let short_rest = finished("short", short.clone(), tol());
    let long_rest = finished("long", long.clone(), tol());
    // Lowest z of the short piece in local frame.
    let inv = m.inverse();
    let zs: Vec<f64> = short
        .vertices()
        .map(|(_, v)| inv.transform_point(*short.get_point(v.point).unwrap()).z)
        .collect();
    let zlo = zs.iter().copied().fold(f64::INFINITY, f64::min);
    // A further split across the piece: z at mid-height, and an oblique
    // plane through the seam's midpoint.
    for (name, n) in [
        ("mid z", Vec3::new(0.0, 0.0, 1.0)),
        ("oblique", Vec3::new(0.2, 0.3, 1.0)),
    ] {
        let origin = m.transform_point(Point3::new(0.0, 0.0, zlo + 0.5 * short_h));
        let plane = topo::test_support::split_plane(origin, m.transform_vec(n), tol());
        match topo::split(&short_rest, &plane, tol()) {
            Ok(h) => {
                let mut vol = 0.0;
                for part in [&h.above, &h.below] {
                    if let Some(b) = part.body() {
                        if let Err(e) = topo::validate_geometric(b, tol()) {
                            fails.push(format!("{tag}: further split {name} tier3 {e:?}"));
                        }
                        vol += topo::mass_properties(b, tol()).map_or(f64::NAN, |p| p.volume);
                    }
                }
                let want = area * short_h;
                if (vol - want).abs() > 1e-8 * want {
                    fails.push(format!(
                        "{tag}: further split {name} volume {vol:e} want {want:e}"
                    ));
                }
            }
            Err(topo::SplitError::Reduce(topo::SplitReduceError::CrossingInsertion {
                source,
                ..
            })) if matches!(
                *source,
                topo::EulerOpError::SplitTangentBendEscalated { .. }
            ) =>
            {
                eprintln!("  note {tag}: further split {name} refuses in band (rule over a half)");
            }
            Err(e) => fails.push(format!("{tag}: further split {name} refused {e:?}")),
        }
    }
    // Boolean: re-join the two pieces, and a union with a far brick.
    match topo::union(&short_rest, &long_rest, tol()) {
        Ok(r) => match r.body() {
            Some(b) => {
                if let Err(e) = topo::validate_geometric(&b.body, tol()) {
                    fails.push(format!("{tag}: rejoin tier3 {e:?}"));
                }
                let v = topo::mass_properties(&b.body, tol()).map_or(f64::NAN, |p| p.volume);
                let want = area * s;
                if (v - want).abs() > 1e-8 * want {
                    fails.push(format!("{tag}: rejoin volume {v:e} want {want:e}"));
                }
            }
            None => fails.push(format!("{tag}: rejoin empty")),
        },
        Err(e) => fails.push(format!("{tag}: rejoin refused {e:?}")),
    }
    let far = topo::test_support::brick(
        (50.0 * s, 51.0 * s),
        (50.0 * s, 51.0 * s),
        (50.0 * s, 51.0 * s),
        tol(),
    );
    let far = finished(
        "far",
        topo::transform_rigid(&far, &m, tol()).unwrap(),
        tol(),
    );
    match topo::union(&short_rest, &far, tol()) {
        Ok(r) => {
            if let Some(b) = r.body()
                && let Err(e) = topo::validate_geometric(&b.body, tol())
            {
                fails.push(format!("{tag}: far union tier3 {e:?}"));
            }
        }
        Err(e) => fails.push(format!("{tag}: far union refused {e:?}")),
    }
}

fn report(name: &str, fails: &[String], tally: &Tally) {
    eprintln!(
        "{name}: eps={:e} rows={} chart_on_cyl={} chart_on_plane={} tangent={} in_band={} downstream={} max_image_off/eps={:.3e} fails={}",
        tol().eps(),
        tally.rows,
        tally.chart_on_cyl,
        tally.chart_on_plane,
        tally.tangent,
        tally.refused_in_band,
        tally.downstream,
        tally.max_image_off,
        fails.len()
    );
    for f in fails {
        eprintln!("  FAIL {f}");
    }
    assert!(fails.is_empty(), "{name}: {} failures", fails.len());
}

const CS: [f64; 6] = [0.25, 0.99, 1.01, 3.0, 4.0, 8.0];

fn cs() -> Vec<f64> {
    let rk = tol().k().sqrt();
    let mut v = CS.to_vec();
    v.extend([0.99 * rk, 1.01 * rk]);
    v
}

fn scale_rows(s: f64) {
    let mut fails = Vec::new();
    let mut tally = Tally::default();
    for (pose_name, pose) in poses(s) {
        let case = Case {
            s,
            phi: 0.0,
            pose_name,
            pose,
        };
        let down = pose_name == "identity" || pose_name == "offset∘tilted";
        run_case(&case, &cs(), down, &mut fails, &mut tally);
    }
    report(&format!("scale {s:e}"), &fails, &tally);
}

#[test]
fn probe_scale_milli() {
    scale_rows(1e-3);
}

#[test]
fn probe_scale_unit() {
    scale_rows(1.0);
}

#[test]
fn probe_scale_kilo() {
    scale_rows(1e3);
}

/// The profile turned so the tangent point sits at other azimuths of the
/// wall's chart (including its period seam, wherever the extrude puts it).
#[test]
fn probe_profile_turns() {
    let mut fails = Vec::new();
    let mut tally = Tally::default();
    for phi in [
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
        -std::f64::consts::PI,
        0.3,
        2.0,
        -2.5,
    ] {
        let case = Case {
            s: 1.0,
            phi,
            pose_name: "identity",
            pose: Affine3::identity(),
        };
        run_case(&case, &[0.25, 0.99, 4.0], true, &mut fails, &mut tally);
    }
    report("profile_turns", &fails, &tally);
}

/// Both faces curved: a profile whose quarter arcs of radius 1 and 2 meet
/// tangent at `(4, 1)` (`κ_rel = 1/2`), with plane–cylinder joints at
/// `(3, 0)` (`κ_rel = 1`) and `(2, 3)` (`κ_rel = 1/2`), extruded by 1 and
/// cut by `z = d` (both normals) and `z = 1 − d`.
#[test]
fn probe_both_faces_curved() {
    let eps = tol().eps();
    let k = tol().k();
    let q = (std::f64::consts::PI / 8.0).tan();
    let outline = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(3.0, 0.0), q),
        (Point2::new(4.0, 1.0), q),
        (Point2::new(2.0, 3.0), 0.0),
        (Point2::new(0.0, 3.0), 0.0),
    ]);
    let profile = Profile::new(sketch_at(0.0), vec![outline])
        .validate(tol())
        .expect("profile validates");
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("extrudes")
    .body;
    let body = finished("the two-arc prism", body, tol());
    let full = topo::mass_properties(&body, tol()).unwrap().volume;
    // The three rulings: (x, y, κ_rel).
    let rulings = [(3.0, 0.0, 1.0), (4.0, 1.0, 0.5), (2.0, 3.0, 0.5)];
    let describe = |b: &topo::Body<f64>| -> Vec<String> {
        b.edges()
            .filter_map(|(_, e)| {
                let c = b.get_curve_geom(e.curve)?.certified()?;
                let kp = kind_of(b, e.he_plus);
                let km = kind_of(b, e.he_minus);
                (kp.0 != km.0
                    && (kp.1 != geom::SurfaceKind::Plane || km.1 != geom::SurfaceKind::Plane))
                    .then(|| {
                        format!(
                            "{:?}|{:?}: {:?}",
                            kp.1,
                            km.1,
                            match c.description() {
                                geom_brep::EdgeDescription::Chart(ch) => format!(
                                    "Chart on {:?}",
                                    b.get_surface(ch.surface).unwrap().kind()
                                ),
                                geom_brep::EdgeDescription::TangentIntersection { .. } =>
                                    "Tangent".into(),
                                geom_brep::EdgeDescription::Intersection { .. } =>
                                    "Intersection".into(),
                                _ => "other".into(),
                            }
                        )
                    })
            })
            .collect()
    };
    eprintln!("parent seams: {:?}", describe(&body));
    let mut fails = Vec::new();
    for c in [0.25, 0.99, 1.01, 1.5, 4.0] {
        // In units of the κ = 1/2 seams' threshold 2√ε.
        let d = c * 2.0 * eps.sqrt();
        for (zc, up) in [(d, 1.0), (d, -1.0), (1.0 - d, 1.0)] {
            let tag = format!("c={c} d={d:e} z={zc:e} up={up}");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 0.0, zc),
                Vec3::new(0.0, 0.0, up),
                tol(),
            );
            let halves = match topo::split(&body, &plane, tol()) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("  {tag}: refused {e:?}");
                    continue;
                }
            };
            let heights = if up > 0.0 {
                [1.0 - zc, zc]
            } else {
                [zc, 1.0 - zc]
            };
            for (height, part) in heights.into_iter().zip([&halves.above, &halves.below]) {
                let piece = part.body().unwrap();
                if let Err(e) = topo::validate_geometric(piece, tol()) {
                    fails.push(format!("{tag}: piece {height:e} tier3 {e:?}"));
                }
                let v = topo::mass_properties(piece, tol()).map_or(f64::NAN, |p| p.volume);
                if (v - full * height).abs() > (1e-9 * full * height).max(full * eps) {
                    fails.push(format!(
                        "{tag}: piece {height:e} volume {v:e} want {:e}",
                        full * height
                    ));
                }
                for (_, e) in piece.edges() {
                    let Some(curve) = piece.get_curve_geom(e.curve).and_then(|g| g.certified())
                    else {
                        continue;
                    };
                    let geom_brep::EdgeDescription::Chart(chart) = curve.description() else {
                        continue;
                    };
                    let kp = kind_of(piece, e.he_plus);
                    let km = kind_of(piece, e.he_minus);
                    if kp.0 == km.0 {
                        continue;
                    }
                    let surface = piece.get_surface(chart.surface).unwrap();
                    let (t0, t1) = curve.params();
                    let mid = curve.carrier().eval(0.5 * (t0 + t1));
                    let Some(&(rx, ry, kappa)) = rulings
                        .iter()
                        .find(|(x, y, _)| (mid.x - x).abs() < 1e-6 && (mid.y - y).abs() < 1e-6)
                    else {
                        continue;
                    };
                    let sag = kappa * height * height / 2.0;
                    if sag > eps * (1.0 + 1e-9) {
                        fails.push(format!("{tag}: ruling ({rx},{ry}) piece {height:e} sag {sag:e} > eps stores a chart"));
                    }
                    for i in 0..=16 {
                        let t = t0 + (t1 - t0) * f64::from(i) / 16.0;
                        let uv = chart.pcurve.eval(t);
                        let p = surface.eval(uv.x, uv.y);
                        let off = ((p.x - rx).powi(2) + (p.y - ry).powi(2)).sqrt();
                        if off > eps {
                            fails.push(format!("{tag}: ruling ({rx},{ry}) image off {off:e}"));
                        }
                    }
                    let _ = k;
                }
                eprintln!("  {tag} piece {height:e}: {:?}", describe(piece));
            }
        }
    }
    for f in &fails {
        eprintln!("  FAIL {f}");
    }
    assert!(fails.is_empty());
}

/// How the new refusals render to a user (D4: one message, one recourse).
#[test]
fn probe_refusal_rendering() {
    let eps = tol().eps();
    let unit = (2.0 * eps).sqrt();
    let wall = match topo::split(
        &shoulder(1.0, 0.0),
        &topo::test_support::split_plane(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), tol()),
        tol(),
    )
    .unwrap()
    .above
    {
        topo::SplitPart::Body(b) => finished("wall", b, tol()),
        topo::SplitPart::Empty => panic!(),
    };
    for c in [1.01, 1.5, 3.0] {
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, c * unit),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        );
        match topo::split(&wall, &plane, tol()) {
            Err(e) => eprintln!("RENDER c={c}: {e}"),
            Ok(_) => eprintln!("RENDER c={c}: cut"),
        }
    }
}
