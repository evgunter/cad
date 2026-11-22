//! STAGE-1 MEASUREMENT PROBE — does not ship. Env-gated log lines and
//! cure switches for `work/tesslane/a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`.
#![allow(dead_code, clippy::all, clippy::pedantic, missing_docs)]

use std::io::Write as _;

/// The log path, when armed.
fn path() -> Option<String> {
    std::env::var("ISO_PROBE_LOG").ok()
}

pub(crate) fn armed() -> bool {
    path().is_some()
}

/// The test this process runs (nextest passes it as an argument).
fn test_name() -> String {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| args.first().cloned().unwrap_or_default())
}

pub(crate) fn log(line: &str) {
    let Some(p) = path() else { return };
    let exe = std::env::args()
        .next()
        .and_then(|a| a.rsplit('/').next().map(str::to_owned))
        .unwrap_or_default();
    let full = format!("{exe}\t{}\t{line}\n", test_name());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = f.write_all(full.as_bytes());
    }
}

/// A cure switch: `ISO_CURE` names one.
pub(crate) fn cure() -> String {
    std::env::var("ISO_CURE").unwrap_or_default()
}

/// The polygon's corners: the points where the walked edge changes.
pub(crate) fn corners(points: &[(f64, f64, u32)], edges: &[topo::EdgeKey], ends: &[usize]) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let mut start = 0;
    for &end in ends {
        for i in start..end {
            let prev = if i == start { end - 1 } else { i - 1 };
            if edges[i] != edges[prev] {
                out.push((points[i].0, points[i].1));
            }
        }
        start = end;
    }
    out
}

fn uniq(mut xs: Vec<f64>, scale: f64) -> Vec<f64> {
    xs.sort_by(f64::total_cmp);
    let mut out: Vec<f64> = Vec::new();
    for x in xs {
        if out.last().is_none_or(|&l| (x - l).abs() > 1e-9 * scale.max(1.0)) {
            out.push(x);
        }
    }
    out
}

/// Even-odd point in polygon over every loop.
pub(crate) fn inside(points: &[(f64, f64, u32)], ends: &[usize], (x, y): (f64, f64)) -> bool {
    let mut c = false;
    let mut start = 0;
    for &end in ends {
        for i in start..end {
            let j = if i + 1 == end { start } else { i + 1 };
            let (xi, yi) = (points[i].0, points[i].1);
            let (xj, yj) = (points[j].0, points[j].1);
            if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
                c = !c;
            }
        }
        start = end;
    }
    c
}

/// The face's iso-rectangles: vertical slabs between consecutive corner
/// `u`s, each cut into maximal `v` intervals inside the polygon. Answers
/// `(u0, u1, v0, v1)` per rectangle. `None` if the polygon is not
/// rectilinear (a corner set that leaves a cell half in).
pub(crate) fn slabs(points: &[(f64, f64, u32)], edges: &[topo::EdgeKey], ends: &[usize]) -> Vec<(f64, f64, f64, f64)> {
    let cs = corners(points, edges, ends);
    let us = uniq(cs.iter().map(|c| c.0).collect(), 1.0);
    let vs = uniq(cs.iter().map(|c| c.1).collect(), 1.0);
    let mut rects = Vec::new();
    for w in us.windows(2) {
        let um = 0.5 * (w[0] + w[1]);
        let mut open: Option<f64> = None;
        for k in 0..vs.len().saturating_sub(1) {
            let vm = 0.5 * (vs[k] + vs[k + 1]);
            let ins = inside(points, ends, (um, vm));
            match (ins, open) {
                (true, None) => open = Some(vs[k]),
                (false, Some(v0)) => {
                    rects.push((w[0], w[1], v0, vs[k]));
                    open = None;
                }
                _ => {}
            }
        }
        if let Some(v0) = open {
            rects.push((w[0], w[1], v0, *vs.last().unwrap()));
        }
    }
    rects
}

/// The same, slabs horizontal.
pub(crate) fn slabs_v(points: &[(f64, f64, u32)], edges: &[topo::EdgeKey], ends: &[usize]) -> Vec<(f64, f64, f64, f64)> {
    let sw: Vec<(f64, f64, u32)> = points.iter().map(|&(u, v, i)| (v, u, i)).collect();
    slabs(&sw, edges, ends).into_iter().map(|(a, b, c, d)| (c, d, a, b)).collect()
}
