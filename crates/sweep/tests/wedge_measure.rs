//! MEASUREMENT ONLY (`embed/wedge-measure`, not for merge): item 2, the
//! sliver and the plain plane-cylinder lens as built bodies. Run with
//! `--no-capture`; prints `WM2` rows and asserts nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]
use profile::RawLoop as _;

use geom::Surface;
use geom_brep::EdgeDescription;
use geom_core::{Band, Bounds, Decide, Interval, Point2, Real, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, LeafBody as _};
use topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, Side, embedded};

const R: f64 = 1e-3;
const BUDGETS: Budgets = Budgets {
    max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
    max_cell_pairs: 65_536,
};

/// A lens: the chord from `(0,0)` to `(chord,0)` and back by an arc of
/// radius `R` subtending `delta`, the profile on `xy` extruded 1 m.
fn lens<T: Decide + topo::AtRestPolicy>(delta: f64, bulge_sign: f64) -> Result<Body<T>, String> {
    let chord = 2.0 * R * (delta / 2.0).sin();
    let b = bulge_sign * (delta / 4.0).tan();
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let lp = bulge_loop(vec![(p(0.0, 0.0), T::from_f64(b)), (p(chord, 0.0), T::zero())]);
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol)
        .map_err(|e| format!("profile refused: {e:?}"))?;
    extrude(
        &vp,
        Extrusion::Distance {
            depth: T::from_f64(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|t| t.body.into_body())
    .map_err(|e| format!("extrude refused: {e:?}"))
}

/// The long arc as two arcs through its far point, so no one segment is
/// near a full turn: chord `(0,0)→(c,0)`, then `→ M → (0,0)`.
fn long2<T: Decide + topo::AtRestPolicy>(half: f64, bulge_sign: f64) -> Result<Body<T>, String> {
    let c = 2.0 * R * half.sin();
    let m = (c / 2.0, R * half.cos() + R);
    let b = bulge_sign * ((core::f64::consts::PI - half) / 4.0).tan();
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let lp = bulge_loop(vec![
        (p(0.0, 0.0), T::zero()),
        (p(c, 0.0), T::from_f64(b)),
        (p(m.0, m.1), T::from_f64(b)),
    ]);
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol)
        .map_err(|e| format!("profile refused: {e:?}"))?;
    extrude(
        &vp,
        Extrusion::Distance {
            depth: T::from_f64(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|t| t.body.into_body())
    .map_err(|e| format!("extrude refused: {e:?}"))
}

fn kind<T: Real>(s: &Surface<T>) -> &'static str {
    match s {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cyl",
        _ => "other",
    }
}

fn read<T: Decide + topo::AtRestPolicy + geom_core::Certifies + geom_core::CertifiedBounds + topo::props::CertifiedPolicy + Bounds>(
    name: &str,
    body: &Body<T>,
) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let scalar = std::any::type_name::<T>().rsplit("::").next().unwrap_or("");
    let t3 = topo::validate_geometric(body, tol);
    eprintln!(
        "WM2 {name} [{scalar}] faces={} edges={} tier3={}",
        body.faces().count(),
        body.edges().count(),
        match &t3 {
            Ok(()) => "PASS".to_owned(),
            Err(e) => format!("FAIL {e:?}"),
        }
    );
    for (e, edge) in body.edges() {
        let [fa, fb] = topo::edge_faces(body, e);
        let (sa, sb) = (body.face_surface(fa).unwrap(), body.face_surface(fb).unwrap());
        if !(matches!(sa, Surface::Cylinder { .. }) || matches!(sb, Surface::Cylinder { .. })) {
            continue;
        }
        let curve = body.get_curve_geom(edge.curve).and_then(|c| c.certified());
        let desc = curve.map_or("uncertified", |c| match c.description() {
            EdgeDescription::Intersection { .. } => "Intersection",
            EdgeDescription::TangentIntersection { .. } => "TangentIntersection",
            _ => "other(conventional)",
        });
        let reach = topo::edge_wedge_reach(body, e);
        let fold = match (reach, curve) {
            (Some(r), Some(c)) => {
                let (t0, t1) = c.params();
                let mid = c.carrier().mid_point(t0, t1);
                Some(geom_brep::folded_lever_arm(sa, sb, mid, r).length().hi())
            }
            _ => None,
        };
        let verdict = topo::edge_wedge(body, e, band);
        eprintln!(
            "WM2 {name} [{scalar}]   edge {e:?} {}-{} desc={desc} reach={:?} fold={:?} check4={:?}",
            kind(sa),
            kind(sb),
            reach.map(|r| r.length().hi()),
            fold,
            verdict
        );
    }
    for mode in ["fold", "narrow"] {
        walk(name, body, mode);
    }
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
        excluded: topo::clearance::FacePairs::none(),
    };
    let t = std::time::Instant::now();
    let report = embedded(&side, band, BUDGETS);
    let v = match &report.verdict {
        Embedding::Embedded => "Embedded".to_owned(),
        Embedding::Crosses(c) => format!("Crosses {:?}", c.faces),
        Embedding::Refused(e) => {
            let named = match e {
                topo::clearance::EngineRefusal::Uncertified { a, b, .. } => {
                    format!(" a={} b={}", kind(body.face_surface(*a).unwrap()), kind(body.face_surface(*b).unwrap()))
                }
                _ => String::new(),
            };
            format!("Refused {e:?}{named}")
        }
    };
    eprintln!(
        "WM2 {name} [{scalar}] embedded={v} cand={} disc={} refused={} splits={} {:.2}s",
        report.receipt.candidates,
        report.receipt.discharged,
        report.receipt.refused,
        report.receipt.splits,
        t.elapsed().as_secs_f64()
    );
}

fn case(name: &str, delta: f64) {
    for sign in [1.0, -1.0] {
        let tag = format!("{name}(bulge{})", if sign > 0.0 { "+" } else { "-" });
        match lens::<f64>(delta, sign) {
            Ok(b) => read(&tag, &b),
            Err(e) => eprintln!("WM2 {tag} [f64] NOT BUILT: {e}"),
        }
        match lens::<Interval>(delta, sign) {
            Ok(b) => read(&tag, &b),
            Err(e) => eprintln!("WM2 {tag} [Interval] NOT BUILT: {e}"),
        }
    }
}

/// The rod: a disc of radius `R` about the origin on `xy`, extruded 1 m.
fn rod<T: Decide + topo::AtRestPolicy>() -> Result<T::Finished, String> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let lp = bulge_loop(vec![(p(-R, 0.0), T::from_f64(1.0)), (p(R, 0.0), T::from_f64(1.0))]);
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(tol)
        .map_err(|e| format!("rod profile refused: {e:?}"))?;
    let body = extrude(
        &vp,
        Extrusion::Distance {
            depth: T::from_f64(1.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map_err(|e| format!("rod extrude refused: {e:?}"))?
    .body
    .into_body();
    topo::finish(body, tol).map_err(|e| format!("rod not finished: {e:?}"))
}

/// The rod cut by the plane `y = −R cos(half)`: intersected with the
/// brick below it (the short arc, a lens) and less it (the long arc).
fn cut<T: Decide + topo::AtRestPolicy + geom_core::Certifies + geom_core::CertifiedBounds + topo::props::CertifiedPolicy + Bounds>(
    name: &str,
    half: f64,
) {
    let tol = Tol::witness();
    let scalar = std::any::type_name::<T>().rsplit("::").next().unwrap_or("");
    let rod = match rod::<T>() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("WM2 bool {name} [{scalar}] NOT BUILT: {e}");
            return;
        }
    };
    let y0 = -R * half.cos();
    let brick = sweep::test_support::brick::<T>((-1.0, 1.0), (-1.0, y0), (-0.5, 1.5), tol);
    let brick = match topo::finish(brick, tol) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("WM2 bool {name} [{scalar}] brick not finished: {e:?}");
            return;
        }
    };
    for (op, r) in [
        ("short=rod∩brick", topo::intersect(&rod, &brick, tol)),
        ("long=rod−brick", topo::subtract(&rod, &brick, tol)),
    ] {
        let tag = format!("bool {name} {op}");
        match r {
            Ok(res) => match res.body() {
                Some(b) => read(&tag, &*b.body),
                None => eprintln!("WM2 {tag} [{scalar}] EMPTY"),
            },
            Err(e) => eprintln!("WM2 {tag} [{scalar}] REFUSED: {e:?}"),
        }
    }
}

#[test]
fn wm2_rows() {
    let (s, phi) = (1e-4_f64, 2e-8_f64);
    // The sliver: azimuth s - phi from the shared ruling, its far ruling
    // shared with the chord plane (the only plane a kernel door puts
    // through both rulings).
    case("sliver", s - phi);
    // The plain case: short arc 2s, long arc 2pi - 2s, chord 2e-7.
    case("short", 2.0 * s);
    case("long", core::f64::consts::TAU - 2.0 * s);
    // The plane moved 5e-12 toward the axis: depth below the plane 1e-11.
    let moved = 2.0 * (1.0 - 1e-11 / R).acos();
    case("short_moved_in", moved);
    case("long_moved_in", core::f64::consts::TAU - moved);
    // Moved 4.9e-12 away (depth 1e-13): the cut nearly tangent.
    let away = 2.0 * (1.0 - 1e-13 / R).acos();
    case("short_moved_out", away);
    case("long_moved_out", core::f64::consts::TAU - away);
    for (name, half) in [("long2_plain", s), ("long2_moved_in", moved / 2.0), ("long2_moved_out", away / 2.0)] {
        for sign in [1.0, -1.0] {
            let tag = format!("{name}(bulge{})", if sign > 0.0 { "+" } else { "-" });
            match long2::<f64>(half, sign) {
                Ok(b) => read(&tag, &b),
                Err(e) => eprintln!("WM2 {tag} [f64] NOT BUILT: {e}"),
            }
            match long2::<Interval>(half, sign) {
                Ok(b) => read(&tag, &b),
                Err(e) => eprintln!("WM2 {tag} [Interval] NOT BUILT: {e}"),
            }
        }
    }
    for (name, half) in [
        ("sliver_chord", (s - phi) / 2.0),
        ("plain", s),
        ("moved_in", moved / 2.0),
        ("moved_out", away / 2.0),
    ] {
        cut::<f64>(name, half);
        cut::<Interval>(name, half);
    }
}

// ---------------------------------------------------------------- the table

/// A margin's lower end, read off its `Debug`.
fn margin_lo(d: &geom_core::MarginDiag) -> f64 {
    let s = format!("{d:?}");
    let grab = |key: &str| {
        s.find(key).and_then(|i| {
            let t = &s[i + key.len()..];
            let e = t.find(|c: char| c == ',' || c == ' ' || c == ')').unwrap_or(t.len());
            t[..e].parse::<f64>().ok()
        })
    };
    grab("lo: ").or_else(|| grab("Value(")).unwrap_or(f64::NAN)
}

/// `embedded` on the whole body under one walk mode of the CAD_WM switch.
fn walk<T: geom_core::Certifies + geom_core::CertifiedBounds + Real>(
    name: &str,
    body: &Body<T>,
    mode: &str,
) {
    let dir = std::env::temp_dir().join("wm-table-log");
    std::fs::create_dir_all(&dir).unwrap();
    let flags = match mode {
        "narrow" => vec!["CAD_WM_NARROW"],
        "fold" => vec!["CAD_WM_FOLD"],
        _ => vec![],
    };
    topo::wedge_measure_modes(Some((dir.to_string_lossy().into_owned(), flags)));
    let band = Band::linear(Tol::witness()).unwrap();
    let side = Side {
        body,
        faces: body.faces().map(|(k, _)| k).collect(),
        excluded: topo::clearance::FacePairs::none(),
    };
    let r = embedded(&side, band, BUDGETS);
    topo::wedge_measure_modes(None);
    let scalar = std::any::type_name::<T>().rsplit("::").next().unwrap_or("");
    eprintln!(
        "WM6 {name} [{scalar}] embedded under {mode}: {:?} | cand {} disc {} refused {} splits {}",
        r.verdict, r.receipt.candidates, r.receipt.discharged, r.receipt.refused, r.receipt.splits
    );
}

/// Every edge's check-4 reading, and stations along the edges between a
/// curved face and a plane.
fn edges_and_stations<T: Decide + topo::AtRestPolicy + geom_core::Certifies + geom_core::CertifiedBounds + topo::props::CertifiedPolicy + Bounds>(
    name: &str,
    body: &Body<T>,
) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let scalar = std::any::type_name::<T>().rsplit("::").next().unwrap_or("");
    eprintln!(
        "WM6 {name} [{scalar}] faces={} tier3={:?}",
        body.faces().count(),
        topo::validate_geometric(body, tol).map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
    );
    let home = topo::WedgeReach::new(body);
    for (e, edge) in body.edges() {
        let [fa, fb] = topo::edge_faces(body, e);
        let (sa, sb) = (body.face_surface(fa).unwrap(), body.face_surface(fb).unwrap());
        let Some(curve) = body.get_curve_geom(edge.curve).and_then(|c| c.certified()) else { continue };
        let Some(reach) = home.edge(e) else { continue };
        let (t0, t1) = curve.params();
        let verdict = topo::edge_wedge(body, e, band);
        let desc = match curve.description() {
            EdgeDescription::Intersection { .. } => "Intersection",
            EdgeDescription::TangentIntersection { .. } => "TangentIntersection",
            _ => "conventional",
        };
        let class = format!("{verdict:?}");
        eprintln!(
            "WM6 {name} [{scalar}]  edge {e:?} {}-{} {desc} reach={:.4e} check4={}",
            format!("{:?}", sa.kind()),
            format!("{:?}", sb.kind()),
            reach.length().hi(),
            class.chars().take(140).collect::<String>()
        );
        if matches!(sa, Surface::Plane { .. }) && matches!(sb, Surface::Plane { .. }) {
            continue;
        }
        let mut least_prod = f64::INFINITY;
        let (mut least_sine, mut least_fold) = (f64::INFINITY, f64::INFINITY);
        for k in 0..=10 {
            let t = t0 + (t1 - t0) * T::from_f64(f64::from(k) / 10.0);
            let p = curve.carrier().eval(t);
            let fold = geom_brep::folded_lever_arm(sa, sb, p, reach).length().hi();
            let read = geom_brep::classify_dihedral_reported(sa, sb, p, reach, band);
            let (cls, m) = match &read {
                Ok((d, m)) => (format!("{:?}", d.class()), margin_lo(m)),
                Err(e) => (format!("in band {}", format!("{e:?}").chars().take(60).collect::<String>()), f64::NAN),
            };
            let sine = m / fold;
            if m.is_finite() {
                least_prod = least_prod.min(m);
                least_sine = least_sine.min(sine);
            }
            least_fold = least_fold.min(fold);
            eprintln!(
                "WM6 {name} [{scalar}]    t={:.2} p=({:.4},{:.4},{:.4}) fold={fold:.4e} reach={:.4e} sine~{sine:.4e} sine*fold={m:.4e} {cls}",
                f64::from(k) / 10.0,
                p.x.hi(),
                p.y.hi(),
                p.z.hi(),
                reach.length().hi()
            );
        }
        eprintln!(
            "WM6 {name} [{scalar}]    least(sine*fold)={least_prod:.4e} least sine x least fold={:.4e}",
            least_sine * least_fold
        );
    }
    for mode in ["reach", "fold", "narrow"] {
        walk(name, body, mode);
    }
}

fn apex_cone<T: Decide + topo::AtRestPolicy>(rev: sweep::Revolution<T>) -> Result<Body<T>, String> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    sweep::test_support::try_revolved_about_y_at(
        vec![(p(0.0, 0.0), T::zero()), (p(0.5, 0.0), T::zero()), (p(0.0, 1.0), T::zero())],
        rev,
        Tol::witness(),
    )
    .map_err(|e| format!("{e:?}"))
}

/// The flat twin as a triangle prism: the 1 m floor, the strip 1e-7
/// wide off it at sine `s`, and the closing face from the strip's far
/// edge back to the floor's.
fn flat_twin<T: Decide + topo::AtRestPolicy>(s: f64) -> Result<Body<T>, String> {
    let c = (1.0 - s * s).sqrt();
    let w = 1e-7;
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let lp = profile::ProfileLoop::polygon([p(0.0, 0.0), p(1.0, 0.0), p(w * c, w * s)]);
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::<T>::xy(), vec![lp]).validate(tol).map_err(|e| format!("profile refused: {e:?}"))?;
    extrude(&vp, Extrusion::Distance { depth: T::from_f64(1.0), side: ExtrudeSide::Along }, tol)
        .map(|t| t.body.into_body())
        .map_err(|e| format!("extrude refused: {e:?}"))
}

/// The flat twin with its far edge carried up a wall: floor 1 m, the
/// strip 1e-7 wide at sine `s`, a wall from the strip's far edge to
/// y = 1, a roof, the floor's far wall.
fn flat_notch<T: Decide + topo::AtRestPolicy>(s: f64) -> Result<Body<T>, String> {
    let c = (1.0 - s * s).sqrt();
    let w = 1e-7;
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    let lp = profile::ProfileLoop::polygon([p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(w * c, 1.0), p(w * c, w * s)]);
    let tol = Tol::witness();
    let vp = Profile::new(SketchPlane::<T>::xy(), vec![lp]).validate(tol).map_err(|e| format!("profile refused: {e:?}"))?;
    extrude(&vp, Extrusion::Distance { depth: T::from_f64(1.0), side: ExtrudeSide::Along }, tol)
        .map(|t| t.body.into_body())
        .map_err(|e| format!("extrude refused: {e:?}"))
}

#[test]
fn wm6_table_bodies() {
    use core::f64::consts::PI;
    for (name, rev) in [("apex_half_cone", sweep::Revolution::Partial(PI)), ("apex_quarter_cone", sweep::Revolution::Partial(PI / 2.0))] {
        match apex_cone::<f64>(rev) {
            Ok(b) => edges_and_stations(name, &b),
            Err(e) => eprintln!("WM6 {name} [f64] NOT BUILT: {e}"),
        }
        let rev_i = match rev {
            sweep::Revolution::Partial(a) => sweep::Revolution::Partial(Interval::from_f64(a)),
            sweep::Revolution::Full => sweep::Revolution::Full,
        };
        match apex_cone::<Interval>(rev_i) {
            Ok(b) => edges_and_stations(name, &b),
            Err(e) => eprintln!("WM6 {name} [Interval] NOT BUILT: {e}"),
        }
    }
    // The oblique cut: the full cone less a half-space whose plane is
    // tilted 30° off the base and crosses the axis at y = 0.4.
    let tol = Tol::witness();
    let cone = apex_cone::<f64>(sweep::Revolution::Full).unwrap();
    let cone = topo::finish(cone, tol).unwrap();
    let b = topo::test_support::brick((-4.0, 4.0), (0.0, 8.0), (-4.0, 4.0), tol);
    let m = geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 0.4, 0.0))
        * geom_core::Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            PI / 6.0,
        );
    let moved = topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol)).unwrap();
    let half = topo::finish(moved, tol).unwrap();
    match topo::subtract(&cone, &half, tol) {
        Ok(r) => match r.body() {
            Some(b) => edges_and_stations("oblique_cone", &*b.body),
            None => eprintln!("WM6 oblique_cone EMPTY"),
        },
        Err(e) => eprintln!("WM6 oblique_cone REFUSED {e:?}"),
    }
    for notch in [true] {
        match flat_notch::<f64>(5e-5) {
            Ok(b) => edges_and_stations("flat_twin_notch", &b),
            Err(e) => eprintln!("WM6 flat_twin_notch [f64] NOT BUILT: {e} {notch}"),
        }
    }
    for s in [5e-5] {
        match flat_twin::<f64>(s) {
            Ok(b) => edges_and_stations("flat_twin", &b),
            Err(e) => eprintln!("WM6 flat_twin [f64] NOT BUILT: {e}"),
        }
        match flat_twin::<Interval>(s) {
            Ok(b) => edges_and_stations("flat_twin", &b),
            Err(e) => eprintln!("WM6 flat_twin [Interval] NOT BUILT: {e}"),
        }
    }
}
