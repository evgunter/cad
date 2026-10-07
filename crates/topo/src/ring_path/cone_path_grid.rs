//! **An adversarial grid of cone paths never reads a wrong parity**: path
//! pairs on the cone islands ([`super::cone_islands`]) between points at
//! the islands' exact corners, their rulings' azimuths, the top
//! parallel's height and a hair either side of it, the ellipses' height
//! extremes, across the axis, by the apex, and on the other nappe.

use super::cone_islands::{band, cone, frames, lune, on_nappe, sector};
use super::*;
use core::f64::consts::PI;

/// The stride through the pairs: every pair's end is still asked.
const STEP: usize = 7;

/// Every decided path with both ends clear of the islands' edges reads
/// the oracle's parity; no pair across the apex is offered a path; and
/// most paths decide.
#[test]
fn an_adversarial_grid_of_cone_paths_never_reads_a_wrong_parity() {
    let mut on_some: Vec<String> = Vec::new();
    let (mut wrong, mut decided, mut none, mut esc, mut asked, mut cross_nappe, mut boundary_some) =
        (Vec::new(), 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for f in frames() {
        for mirror in [false, true] {
            let cone = cone(f, mirror);
            let quadric = Quadric::of(&cone).unwrap();
            for (name, island) in [
                ("lune", lune(f, &cone, 1.0)),
                ("lune by the apex", lune(f, &cone, 0.1)),
                ("sector", sector(f, &cone)),
            ] {
                let arcs: Vec<_> = island
                    .edges
                    .iter()
                    .map(|e| LoopArc::of(&e.carrier, e.params).unwrap())
                    .collect();
                // Boundary samples, heights and azimuths of interest.
                let mut samples = Vec::new();
                let mut heights = vec![
                    1e-3,
                    0.01,
                    0.05,
                    0.13,
                    0.15,
                    0.3,
                    0.9,
                    1.3,
                    1.5,
                    2.2,
                    2.2 + 1e-7,
                    2.2 - 1e-7,
                    2.5,
                    2.9,
                ];
                let mut azis: Vec<f64> = (0..24).map(|k| -PI + f64::from(k) * PI / 12.0).collect();
                azis.extend([-0.6, 0.9, PI - 1e-9, -0.6 + 1e-8, 0.9 - 1e-8, 0.0]);
                let az = |p: Point3<f64>| {
                    let w = p - f.o;
                    w.dot(f.y).atan2(w.dot(f.x))
                };
                let ht = |p: Point3<f64>| (p - f.o).dot(f.z);
                for e in &island.edges {
                    let (t0, t1) = e.params;
                    let (mut lo, mut hi) = (f64::MAX, f64::MIN);
                    for i in 0..=400 {
                        let p = e.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 400.0);
                        samples.push(p);
                        lo = lo.min(ht(p));
                        hi = hi.max(ht(p));
                    }
                    heights.extend([lo, hi]);
                }
                for c in &island.corners {
                    heights.push(ht(*c));
                    azis.push(az(*c));
                }
                let mut points = Vec::new();
                for &h in &heights {
                    for &t in &azis {
                        points.push(on_nappe(f, h, t));
                    }
                }
                points.extend(island.corners.iter().copied());
                // The other nappe.
                points.push(on_nappe(f, -1.0, 0.3));
                points.push(on_nappe(f, -0.01, 2.0));
                let clear: Vec<bool> = points
                    .iter()
                    .map(|&p| samples.iter().all(|q| (*q - p).norm() > 1e-5))
                    .collect();
                let on: Vec<bool> = points
                    .iter()
                    .map(|&p| {
                        island.corners.iter().any(|c| (*c - p).norm() < 1e-12)
                            || (name == "sector"
                                && (ht(p) - 2.2).abs() < 1e-12
                                && az(p) > -0.6
                                && az(p) < 0.9)
                    })
                    .collect();
                let inside: Vec<bool> = points.iter().map(|&p| (island.inside)(p)).collect();
                for (i, &a) in points.iter().enumerate() {
                    for (j, &b) in points.iter().enumerate().skip(i + 1).step_by(STEP) {
                        let paths = match quadric.paths((a, b), band()) {
                            Ok(p) => p,
                            Err(_) => {
                                esc += 1;
                                continue;
                            }
                        };
                        if ht(a) * ht(b) < 0.0 {
                            cross_nappe += 1;
                            if !paths.is_empty() {
                                wrong.push("a path across the apex".into());
                            }
                        }
                        let both_clear = clear[i] && clear[j];
                        let want = inside[i] != inside[j];
                        for path in paths {
                            asked += 1;
                            match path_parity(&path, &arcs, None, band()) {
                                Ok(Some(got)) => {
                                    if on[i] || on[j] {
                                        on_some.push(format!(
                                            "{name} mirror {mirror}: {a:?} -> {b:?}"
                                        ));
                                    }
                                    if !both_clear {
                                        boundary_some += 1;
                                    } else if got != want {
                                        wrong.push(format!("{name} mirror {mirror}: {a:?} -> {b:?}: got {got} want {want}"));
                                    } else {
                                        decided += 1;
                                    }
                                }
                                Ok(None) => none += 1,
                                Err(_) => esc += 1,
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty() && decided * 2 > asked && cross_nappe > 0,
        "asked {asked}, decided right {decided}, undecided {none}, escalated {esc}, across \
         the apex {cross_nappe}, ended on an edge {boundary_some} ({} on a corner): wrong {:?}",
        on_some.len(),
        &wrong[..wrong.len().min(10)]
    );
}
