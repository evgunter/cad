//! Review probes for PR 4271 (band-dual-4271-r2): polygonal holes in the
//! D-rod's cap that cross, graze or clear the upper crease's cut-off
//! sliver. A hole any of whose boundary lies in the sliver must refuse;
//! a carve must be tier-3 and tier-3′ valid, at the closed form, with
//! sampled membership matching the expected section.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::fillet_edges;
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, rod_chord_at, rod_creases, rod_section_cut,
};
use topo::{
    ContactRecords, SolidContainment, mass_properties, point_in_solid, validate_geometric,
    validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn d_loop() -> ProfileLoop<f64> {
    let c = rod_chord_at(ROD_FLAT);
    bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ])
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

fn ccw(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let a: f64 = (0..pts.len())
        .map(|i| {
            let (p, q) = (pts[i], pts[(i + 1) % pts.len()]);
            p.0 * q.1 - q.0 * p.1
        })
        .sum();
    let mut v = pts.to_vec();
    if a < 0.0 {
        v.reverse();
    }
    v
}

/// The sliver at the upper (y > 0) or lower corner, open set, with a
/// margin `m` (positive shrinks it).
fn in_sliver(x: f64, y: f64, m: f64) -> bool {
    let r = ROD_FILLET;
    let cx = ROD_FLAT - r;
    let cy = ((ROD_R - r).powi(2) - cx * cx).sqrt();
    let y = y.abs();
    let (dx, dy) = (x - cx, y - cy);
    let th = dy.atan2(dx);
    let wall = cy.atan2(cx);
    dx.hypot(dy) > r + m && th > 0.0 && th < wall && x < ROD_FLAT - m && x.hypot(y) < ROD_R - m
}

fn in_poly(pts: &[(f64, f64)], x: f64, y: f64) -> bool {
    let mut inside = false;
    let n = pts.len();
    for i in 0..n {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            inside = !inside;
        }
    }
    inside
}

/// Does the hole's boundary (or interior) meet the sliver?
fn crosses(pts: &[(f64, f64)], m: f64) -> bool {
    let n = pts.len();
    for i in 0..n {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        for k in 0..=4000 {
            let t = f64::from(k) / 4000.0;
            if in_sliver(a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t, m) {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Carved,
    Refused(String),
    BadSource,
}

/// Runs the fillet, checks a carve fully. Returns the outcome; panics
/// on any wrong body.
fn run(pts: &[(f64, f64)], what: &str) -> Outcome {
    let pts = ccw(pts);
    let pv = pts.clone();
    let Ok(body) = std::panic::catch_unwind(move || {
        extruded(SketchPlane::xy(), vec![d_loop(), poly(&pv)], ROD_L, tol())
    }) else {
        return Outcome::BadSource;
    };
    if validate_geometric(&body, tol()).is_err() {
        return Outcome::BadSource;
    }
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "{what}");
    let vol0 = mass_properties(&body, tol()).unwrap().volume;
    let out = match fillet_edges(&body, &creases, ROD_FILLET, tol()) {
        Ok(o) => o,
        Err(e) => return Outcome::Refused(format!("{:?}", e.error).chars().take(60).collect()),
    };
    let crossing = crosses(&pts, 0.0);
    assert!(
        !crossing,
        "{what}: WRONG BODY — hole {pts:?} meets the sliver yet carved"
    );
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3' {e:?}"));
    let dv = mass_properties(&out.body, tol()).unwrap().volume - vol0;
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
    assert!((dv - want).abs() < 1e-12, "{what}: dV {dv} vs {want}");
    // membership on a grid over the upper corner, mid-length
    let band = Band::linear(tol()).unwrap();
    let mut checked = 0;
    for i in 0..24 {
        for j in 0..24 {
            let x = 0.1 + 0.2 * f64::from(i) / 23.0 + 1e-4;
            let y = 0.2 + 0.25 * f64::from(j) / 23.0 + 1e-4;
            if x.hypot(y) > ROD_R - 1e-3 || x > ROD_FLAT - 1e-3 {
                continue;
            }
            // skip near any boundary: hole, sliver arc
            if crosses_near(&pts, x, y, 2e-3) || (in_sliver(x, y, -2e-3) && !in_sliver(x, y, 2e-3))
            {
                continue;
            }
            let want_in = !in_poly(&pts, x, y) && !in_sliver(x, y, 0.0);
            let got = point_in_solid(&out.body, Point3::new(x, y, 0.5 * ROD_L), band, tol());
            match got {
                Ok(SolidContainment::In) => assert!(want_in, "{what}: ({x},{y}) inside, want out"),
                Ok(SolidContainment::Out) => {
                    assert!(!want_in, "{what}: ({x},{y}) outside, want in")
                }
                other => panic!("{what}: membership at ({x},{y}): {other:?}"),
            }
            checked += 1;
        }
    }
    assert!(checked > 50, "{what}: membership checked {checked}");
    Outcome::Carved
}

fn crosses_near(pts: &[(f64, f64)], x: f64, y: f64, d: f64) -> bool {
    let n = pts.len();
    (0..n).any(|i| {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let (ex, ey) = (b.0 - a.0, b.1 - a.1);
        let t = (((x - a.0) * ex + (y - a.1) * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
        (a.0 + ex * t - x).hypot(a.1 + ey * t - y) < d
    })
}

const CY: f64 = 0.346_410_161_513_775_4;

/// Hand-built edges meeting the sliver in awkward ways: each must refuse.
#[test]
fn hand_built_crossing_holes_refuse() {
    let rows: Vec<(&str, Vec<(f64, f64)>)> = vec![
        // a thin sliver-crossing triangle: enters from inside the ball section, exits beyond the floor
        (
            "chord across the corner",
            vec![(0.22, 0.36), (0.297, 0.44_f64.min(0.399)), (0.21, 0.37)],
        ),
        (
            "long thin edge grazing arc from inside",
            vec![(0.15, 0.30), (0.2985, 0.3985), (0.16, 0.30)],
        ),
        (
            "edge along the flat inside",
            vec![(0.2995, 0.36), (0.2995, 0.39), (0.25, 0.37)],
        ),
        (
            "edge along the wall chord",
            vec![(0.28, 0.40), (0.255, 0.42), (0.24, 0.40)],
        ),
        (
            "tiny triangle at V",
            vec![(0.2990, 0.3985), (0.2995, 0.3985), (0.2992, 0.3988)],
        ),
        (
            "across the foot on the flat",
            vec![(0.29, CY - 0.01), (0.299, CY + 0.004), (0.28, CY + 0.002)],
        ),
        (
            "radial through c to V",
            vec![(0.2, CY), (0.2985, 0.399), (0.2005, CY + 0.001)],
        ),
        (
            "tangent-ish to the arc",
            vec![
                (0.18, 0.4465),
                (0.2995, 0.4465_f64.min(0.399)),
                (0.18, 0.44),
            ],
        ),
    ];
    for (what, pts) in rows {
        let crossing = crosses(&ccw(&pts), 0.0);
        let o = run(&pts, what);
        eprintln!("{what}: crossing {crossing}, {o:?}");
        if crossing {
            assert!(
                matches!(o, Outcome::Refused(_) | Outcome::BadSource),
                "{what}: {o:?}"
            );
        }
    }
}

/// A seeded random scan of triangles near the upper corner.
#[test]
fn random_triangles_near_the_corner_never_carve_wrong() {
    let mut s: u64 = 0x4271_7212;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut carved, mut refused_cross, mut refused_clear, mut bad) = (0, 0, 0, 0);
    let n: usize = std::env::var("R2_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);
    let mut tried = 0;
    while tried < n {
        let mut pts = Vec::new();
        for _ in 0..3 {
            let x = 0.17 + 0.13 * rnd();
            let y = 0.30 + 0.12 * rnd();
            pts.push((x, y));
        }
        if pts
            .iter()
            .any(|&(x, y)| x.hypot(y) > ROD_R - 2e-3 || x > ROD_FLAT - 2e-3)
        {
            continue;
        }
        // thin triangles: pull the third point toward the first edge sometimes
        if rnd() < 0.5 {
            let t = rnd();
            let (a, b) = (pts[0], pts[1]);
            let w = 0.02 * rnd();
            pts[2] = (a.0 + (b.0 - a.0) * t + w, a.1 + (b.1 - a.1) * t - w);
            if pts[2].0 > ROD_FLAT - 2e-3 || pts[2].0.hypot(pts[2].1) > ROD_R - 2e-3 {
                continue;
            }
        }
        tried += 1;
        let crossing = crosses(&ccw(&pts), 0.0);
        let near = crosses(&ccw(&pts), -3e-3);
        match run(&pts, &format!("tri {tried} {pts:?}")) {
            Outcome::Carved => carved += 1,
            Outcome::Refused(_) if crossing => refused_cross += 1,
            Outcome::Refused(e) => {
                refused_clear += 1;
                if !near {
                    eprintln!("over-refusal (≥3e-3 clear): {pts:?} {e}");
                }
            }
            Outcome::BadSource => bad += 1,
        }
    }
    eprintln!(
        "carved {carved}, refused crossing {refused_cross}, refused clear {refused_clear}, bad source {bad}"
    );
}

/// The PR's three rectangular-hole carve rows, and its refusal row, at
/// `Interval`: what does the certified scalar decide?
#[test]
fn rect_holes_at_interval() {
    use geom_core::Interval;
    let iv = |x: f64| Interval::from_bounds(x, x);
    let c = rod_chord_at(ROD_FLAT);
    for (x0, y0, x1, y1, what) in [
        (0.1, 0.35, 0.275, 0.3964, "level with the corner"),
        (0.12, 0.37, 0.27, 0.41, "above the ball centre"),
        (0.24, 0.27, 0.28, 0.39, "beside the flat"),
        (
            0.2,
            0.35,
            0.296,
            0.398,
            "cornered in the sliver (must refuse)",
        ),
    ] {
        let d: ProfileLoop<Interval> = bulge_loop(vec![
            (Point2::new(iv(ROD_FLAT), iv(c.half)), iv(c.wall_bulge)),
            (Point2::new(iv(ROD_FLAT), iv(-c.half)), iv(0.0)),
        ]);
        let hole: ProfileLoop<Interval> = bulge_loop(
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
                .iter()
                .map(|&(x, y)| (Point2::new(iv(x), iv(y)), iv(0.0)))
                .collect(),
        );
        let plane = SketchPlane::<Interval>::xy();
        let body = extruded(plane, vec![d, hole], iv(ROD_L), tol());
        let creases = rod_creases(&body);
        match fillet_edges(&body, &creases, iv(ROD_FILLET), tol()) {
            Ok(out) => {
                let r = validate_geometric(&out.body, tol());
                eprintln!("Interval {what}: carved, tier 3 {:?}", r.is_ok());
            }
            Err(e) => eprintln!(
                "Interval {what}: refused {:?}",
                format!("{:?}", e.error)
                    .chars()
                    .take(140)
                    .collect::<String>()
            ),
        }
    }
}

/// The filed row `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver`:
/// does the witness reproduce?
#[test]
fn filed_face_clearance_witness() {
    for (x, y, a) in [(0.21, 0.36, 0.08), (0.20, 0.36, 0.08)] {
        let c = rod_chord_at(ROD_FLAT);
        let d = bulge_loop(vec![
            (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
            (Point2::new(ROD_FLAT, -c.half), 0.0),
        ]);
        let b = bulge_loop(vec![
            (Point2::new(x + a, y), 1.0),
            (Point2::new(x - a, y), 1.0),
        ]);
        let body = extruded(SketchPlane::xy(), vec![d, b], ROD_L, tol());
        let r = fillet_edges(&body, &rod_creases(&body), ROD_FILLET, tol());
        eprintln!(
            "bore ({x},{y}) a {a}: {:?}",
            r.map(|_| "carved").map_err(|e| format!("{:?}", e.error)
                .chars()
                .take(200)
                .collect::<String>())
        );
    }
}
