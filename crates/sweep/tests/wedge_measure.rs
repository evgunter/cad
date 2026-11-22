//! MEASUREMENT ONLY (`embed/wedge-measure`, not for merge): item 2, the
//! sliver and the plain plane-cylinder lens as built bodies. Run with
//! `--no-capture`; prints `WM2` rows and asserts nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

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
