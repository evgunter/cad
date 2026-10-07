//! PR 4246 third-review probes (scratch, not for merge): planar ring
//! re-homing at a ring vertex a hair from the dividing plane, by split
//! and by Boolean, one outcome line per op, written to `$RP_OUT` for a
//! base/head diff. They assert nothing and are `#[ignore]`d.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::fmt::Write as _;

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::splitting::{SplitPart, split};
use topo::{Body, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(plane: SketchPlane<f64>, loops: &[Vec<(f64, f64, f64)>], h: f64) -> Option<Body<f64>> {
    let loops = loops
        .iter()
        .map(|lp| bulge_loop(lp.iter().map(|&(x, y, b)| (Point2::new(x, y), b)).collect()))
        .collect();
    let profile = Profile::new(plane, loops).validate(tol()).ok()?;
    extrude(&profile, Extrusion::Distance { depth: h, side: ExtrudeSide::Along }, tol())
        .ok()
        .map(|e| e.body)
}

/// Polygon ∩ half-plane `n·p >= c` (Sutherland–Hodgman), then area.
fn clip_area(poly: &[(f64, f64)], n: (f64, f64), c: f64) -> f64 {
    let f = |p: (f64, f64)| n.0 * p.0 + n.1 * p.1 - c;
    let mut out = Vec::new();
    for i in 0..poly.len() {
        let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
        let (fp, fq) = (f(p), f(q));
        if fp >= 0.0 {
            out.push(p);
        }
        if (fp >= 0.0) != (fq >= 0.0) {
            let t = fp / (fp - fq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    let mut a = 0.0;
    for i in 0..out.len() {
        let (p, q) = (out[i], out[(i + 1) % out.len()]);
        a += p.0 * q.1 - q.0 * p.1;
    }
    0.5 * a.abs()
}

/// Disc (centre, r) ∩ half-plane `n·p >= c`, `n` unit.
fn disc_area(centre: (f64, f64), r: f64, n: (f64, f64), c: f64) -> f64 {
    let h = n.0 * centre.0 + n.1 * centre.1 - c; // signed distance of centre into the half-plane
    let h = h.clamp(-r, r);
    // area of the part with distance-from-line >= 0: segment on the far side
    let seg = |d: f64| r * r * (d / r).acos() - d * (r * r - d * d).max(0.0).sqrt();
    PI * r * r - seg(h)
}

#[derive(Clone)]
enum Hole {
    Poly(Vec<(f64, f64)>),
    Disc((f64, f64), f64),
}

impl Hole {
    fn area(&self, n: (f64, f64), c: f64) -> f64 {
        match self {
            Hole::Poly(p) => clip_area(p, n, c),
            Hole::Disc(ce, r) => disc_area(*ce, *r, n, c),
        }
    }
    fn profile(&self) -> Vec<(f64, f64, f64)> {
        match self {
            // holes run clockwise
            Hole::Poly(p) => p.iter().rev().map(|&(x, y)| (x, y, 0.0)).collect(),
            Hole::Disc((cx, cy), r) => vec![(cx - r, *cy, 1.0), (cx + r, *cy, 1.0)],
        }
    }
}

const OUTER: [(f64, f64); 4] = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)];

struct Log(String, usize);

fn outcome_body(part: &Body<f64>, want: f64) -> String {
    let t3 = topo::validate_geometric(part, tol()).is_ok();
    match topo::mass_properties(part, tol()) {
        Ok(m) => {
            let ok = (m.volume - want).abs() <= 1e-9 * want.abs().max(1e-12);
            format!(
                "BODY t3={t3} vol={:.10e} {} want={want:.10e}",
                m.volume,
                if ok { "VOK" } else { "VBAD" }
            )
        }
        Err(e) => format!("BODY t3={t3} massERR {}", tag(&e)),
    }
}

fn tag(e: &dyn core::fmt::Debug) -> String {
    let s = format!("{e:?}");
    let mut out = String::new();
    let mut depth = 0;
    for ch in s.chars() {
        match ch {
            '{' | '(' => depth += 1,
            '}' | ')' => depth -= 1,
            _ if depth <= 2 && (ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == ' ') => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().filter(|w| w.chars().next().is_some_and(char::is_uppercase) || w.starts_with("split_") || w.starts_with("pcurve")).take(8).collect::<Vec<_>>().join(" ")
}

/// One pose: the holed square prism cut by the vertical plane through the
/// origin with normal `n`, by split and (axis-aligned only) by Boolean.
fn pose(log: &mut Log, name: &str, holes: &[Hole], n: (f64, f64)) {
    let mut loops = vec![OUTER.iter().map(|&(x, y)| (x, y, 0.0)).collect::<Vec<_>>()];
    loops.extend(holes.iter().map(Hole::profile));
    let Some(raw) = extruded(SketchPlane::xy(), &loops, 1.0) else {
        let _ = writeln!(log.0, "{name} | build | NOPROFILE");
        return;
    };
    let body = match <f64 as topo::AtRestPolicy>::gate_at_rest_kept(raw, tol()) {
        Ok(b) => b,
        Err(_) => {
            let _ = writeln!(log.0, "{name} | build | NOTATREST");
            return;
        }
    };
    let want = |n: (f64, f64)| clip_area(&OUTER, n, 0.0) - holes.iter().map(|h| h.area(n, 0.0)).sum::<f64>();
    let (above_want, below_want) = (want(n), want((-n.0, -n.1)));
    log.1 += 1;
    let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(n.0, n.1, 0.0), tol());
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| split(&body, &plane, tol())));
    let line = match out {
        Err(_) => "PANIC".into(),
        Ok(Err(e)) => format!("ERR {}", tag(&e)),
        Ok(Ok(r)) => {
            let side = |p: &SplitPart<f64>, w: f64| match p {
                SplitPart::Body(b) => outcome_body(b, w),
                _ => "EMPTY".into(),
            };
            format!("above[{}] below[{}]", side(&r.above, above_want), side(&r.below, below_want))
        }
    };
    let _ = writeln!(log.0, "{name} | split | {line}");
    if n.1 != 0.0 {
        return;
    }
    let slab = |x0: f64, x1: f64| {
        let rect = vec![(x0, -3.0, 0.0), (x1, -3.0, 0.0), (x1, 3.0, 0.0), (x0, 3.0, 0.0)];
        finished("slab", extruded(sketch_at(-0.5), &[rect], 2.0).unwrap(), tol())
    };
    let right = slab(0.0, 3.0);
    let rw = want((1.0, 0.0));
    let lw = want((-1.0, 0.0));
    let ops: [(&str, f64, Box<dyn Fn() -> Result<BooleanResult<f64>, topo::BooleanError>>); 4] = [
        ("b∩R", rw, Box::new(|| topo::intersect(&body, &right, tol()))),
        ("R∩b", rw, Box::new(|| topo::intersect(&right, &body, tol()))),
        ("b∖R", lw, Box::new(|| topo::subtract(&body, &right, tol()))),
        ("b∪R", lw + 3.0 * 6.0 * 2.0 - 0.0, Box::new(|| topo::union(&body, &right, tol()))),
    ];
    for (op, w, run) in ops {
        log.1 += 1;
        let line = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(run)) {
            Err(_) => "PANIC".into(),
            Ok(Err(e)) => format!("ERR {}", tag(&e)),
            Ok(Ok(BooleanResult::Body(b))) => {
                // the union's volume: the slab plus the left half's prism part
                outcome_body(&b.body, w)
            }
            Ok(Ok(_)) => "OTHER".into(),
        };
        let _ = writeln!(log.0, "{name} | {op} | {line}");
    }
}

fn write(log: &Log, name: &str) {
    let dir = std::env::var("RP_OUT").unwrap_or_else(|_| "/tmp/rp".into());
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), &log.0).unwrap();
    eprintln!("{name}: {} ops", log.1);
}

fn offsets() -> Vec<f64> {
    let mut v = vec![0.0];
    for k in -16..=-3 {
        for m in [1.0, 3.0] {
            let d = m * 10f64.powi(k);
            v.push(d);
            v.push(-d);
        }
    }
    v
}

/// An n-gon of circumradius `r` turned by `theta`, translated so its
/// minimum `x` is `d` (so it lies on `x >= d`), centred at `y = cy`.
fn ngon(n: usize, r: f64, theta: f64, d: f64, cy: f64) -> Vec<(f64, f64)> {
    let pts: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let a = theta + 2.0 * PI * k as f64 / n as f64;
            (r * a.cos(), r * a.sin())
        })
        .collect();
    let minx = pts.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    pts.into_iter().map(|(x, y)| (x - minx + d, y + cy)).collect()
}

/// A ring vertex a hair from the dividing plane, on either side, by
/// polygon shape and turn; plus discs tangent within the band; plus a
/// mirrored partner hole on the other side.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp3_planar_ring_near_plane() {
    let mut log = Log(String::new(), 0);
    for d in offsets() {
        for (n, theta) in [(3, 0.0), (3, 0.3), (4, 0.2), (4, PI / 4.0), (5, 0.1), (6, 0.05)] {
            for cy in [0.0, 0.9] {
                let hole = Hole::Poly(ngon(n, 0.3, theta, d, cy));
                let partner = Hole::Poly(ngon(4, 0.2, 0.3, -1.2, -1.0));
                for (tag_n, nrm) in [("+x", (1.0, 0.0)), ("-x", (-1.0, 0.0))] {
                    pose(&mut log, &format!("poly n{n} th{theta:.3} cy{cy} d{d:e} {tag_n}"), &[hole.clone(), partner.clone()], nrm);
                }
            }
        }
        for r in [0.3, 0.05] {
            let hole = Hole::Disc((d + r, 0.4), r);
            for (tag_n, nrm) in [("+x", (1.0, 0.0)), ("-x", (-1.0, 0.0))] {
                pose(&mut log, &format!("disc r{r} d{d:e} {tag_n}"), &[hole.clone()], nrm);
            }
        }
    }
    write(&log, "planar_ring_near_plane");
}

/// Tilted vertical planes: the plane turned by `phi` about z, and the
/// polygon's nearest vertex placed a hair from it along its normal.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp3_planar_ring_near_tilted_plane() {
    let mut log = Log(String::new(), 0);
    for d in offsets() {
        for phi in [0.01, 0.37, 1.1] {
            let nrm = (f64::cos(phi), f64::sin(phi));
            for (n, theta) in [(3, 0.2), (4, 0.7), (5, 0.0)] {
                // place the polygon so its minimum along nrm is d
                let r = 0.3;
                let pts: Vec<(f64, f64)> = (0..n)
                    .map(|k| {
                        let a = theta + 2.0 * PI * k as f64 / n as f64;
                        (r * a.cos(), r * a.sin())
                    })
                    .collect();
                let m = pts.iter().map(|p| p.0 * nrm.0 + p.1 * nrm.1).fold(f64::INFINITY, f64::min);
                let shift = d - m;
                let along = (-nrm.1, nrm.0);
                let pts: Vec<_> = pts.iter().map(|p| (p.0 + shift * nrm.0 + 0.5 * along.0, p.1 + shift * nrm.1 + 0.5 * along.1)).collect();
                pose(&mut log, &format!("tilt phi{phi} n{n} d{d:e}"), &[Hole::Poly(pts)], nrm);
            }
        }
    }
    write(&log, "planar_ring_near_tilted_plane");
}

/// A two-arc disc of radius 2 extruded to height 1, less the box
/// `[1, 3] × [y0, y1] × [z0, z1]`: a window in the upper wall face
/// bounded by two rulings and two rim arcs (a ring the at-rest gate
/// admits on a cylinder). Then a horizontal split a hair below or above
/// the window's bottom rim, and a vertical split a hair from its
/// low ruling.
#[test]
#[ignore = "reviewer evidence: writes outcome files, asserts nothing"]
fn rp3_cylinder_window_near_plane() {
    let mut log = Log(String::new(), 0);
    let disc = vec![(2.0, 0.0, 1.0), (-2.0, 0.0, 1.0)];
    let Some(prism) = extruded(SketchPlane::xy(), &[disc], 1.0) else {
        panic!("disc");
    };
    let prism = finished("disc prism", prism, tol());
    let area_disc = PI * 4.0;
    for (y0, y1, z0, z1) in [(0.8, 1.2, 0.3, 0.7), (0.5, 0.6, 0.2, 0.9), (1.1, 1.5, 0.45, 0.55)] {
        let rect = vec![(1.0, y0, 0.0), (3.0, y0, 0.0), (3.0, y1, 0.0), (1.0, y1, 0.0)];
        let cutter = finished("cutter", extruded(sketch_at(z0), &[rect], z1 - z0).unwrap(), tol());
        let body = match topo::subtract(&prism, &cutter, tol()) {
            Ok(BooleanResult::Body(b)) => b.body,
            other => {
                let _ = writeln!(log.0, "win {y0} {y1} | build | {}", tag(&other.err()));
                continue;
            }
        };
        let rings = body.faces().map(|(_, f)| f.rings.len()).sum::<usize>();
        let _ = writeln!(log.0, "win {y0} {y1} | build | rings={rings}");
        // the window's xy footprint inside the disc
        let f = |y: f64| 0.5 * (y * (4.0 - y * y).sqrt() + 4.0 * (y / 2.0).asin());
        let a_bd = (f(y1) - f(y0)) - (y1 - y0);
        for d in offsets() {
            for (dir, c) in [("zlo", z0 - d), ("zhi", z1 + d)] {
                log.1 += 1;
                let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, c), Vec3::new(0.0, 0.0, 1.0), tol());
                // volume below c
                let below = area_disc * c - a_bd * (c.min(z1) - z0).max(0.0);
                let total = area_disc - a_bd * (z1 - z0);
                let line = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| split(&body, &plane, tol()))) {
                    Err(_) => "PANIC".into(),
                    Ok(Err(e)) => format!("ERR {}", tag(&e)),
                    Ok(Ok(r)) => {
                        let side = |p: &SplitPart<f64>, w: f64| match p {
                            SplitPart::Body(b) => outcome_body(b, w),
                            _ => "EMPTY".into(),
                        };
                        format!("above[{}] below[{}]", side(&r.above, total - below), side(&r.below, below))
                    }
                };
                let _ = writeln!(log.0, "win {y0} {y1} {dir} d{d:e} | split | {line}");
            }
            for (dir, c) in [("ylo", y0 - d), ("yhi", y1 + d)] {
                log.1 += 1;
                let plane = topo::test_support::split_plane(Point3::new(0.0, c, 0.5), Vec3::new(0.0, 1.0, 0.0), tol());
                let line = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| split(&body, &plane, tol()))) {
                    Err(_) => "PANIC".into(),
                    Ok(Err(e)) => format!("ERR {}", tag(&e)),
                    Ok(Ok(r)) => {
                        let vol = |p: &SplitPart<f64>| match p {
                            SplitPart::Body(b) => topo::mass_properties(b, tol()).map(|m| m.volume).unwrap_or(f64::NAN),
                            _ => 0.0,
                        };
                        let t3 = |p: &SplitPart<f64>| match p {
                            SplitPart::Body(b) => topo::validate_geometric(b, tol()).is_ok(),
                            _ => true,
                        };
                        let sum = vol(&r.above) + vol(&r.below);
                        let total = area_disc - a_bd * (z1 - z0);
                        format!(
                            "t3={}/{} above={:.10e} below={:.10e} {}",
                            t3(&r.above),
                            t3(&r.below),
                            vol(&r.above),
                            vol(&r.below),
                            if (sum - total).abs() < 1e-9 { "SUMOK" } else { "SUMBAD" }
                        )
                    }
                };
                let _ = writeln!(log.0, "win {y0} {y1} {dir} d{d:e} | split | {line}");
            }
        }
    }
    write(&log, "cylinder_window_near_plane");
}
