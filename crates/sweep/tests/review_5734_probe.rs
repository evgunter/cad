//! Review probe (PR 5734): split volumes against exact oracles, no slack.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::split;

fn cylinder(tol: Tol) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(
        sweep::test_support::prism(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
            1.0,
            tol,
        ),
        tol,
    )
    .expect("the unit cylinder")
}

/// Plane through (x0, 0, d) with normal (sx, sy, 1): z = d - sx (x - x0) - sy y.
fn run(x0: f64, d: f64, sx: f64, sy: f64, tol: Tol, exact_lower: Option<f64>) -> String {
    let normal = Vec3::new(sx, sy, 1.0);
    let plane = topo::test_support::split_plane(Point3::new(x0, 0.0, d), normal / normal.norm(), tol);
    let parts = match split(&cylinder(tol), &plane, tol) {
        Ok(p) => p,
        Err(e) => return format!("REFUSED {e:?}").chars().take(300).collect(),
    };
    let mut out = String::new();
    let (mut v, mut p) = (0.0, 0.0);
    for (name, part) in [("above", &parts.above), ("below", &parts.below)] {
        let Some(body) = part.body() else { out += &format!(" {name}=empty"); continue };
        match topo::mass_properties(body, tol) {
            Ok(props) => {
                v += props.volume;
                p += props.volume_pad;
                if name == "below" {
                    if let Some(ex) = exact_lower {
                        let r = (props.volume - ex).abs() / props.volume_pad;
                        out += &format!(" below_err/pad={r:.3e} pad={:.3e}", props.volume_pad);
                        if r > 1.0 { out += " ESCAPE_BELOW"; }
                    }
                }
            }
            Err(e) => { out += &format!(" {name}:ERR {e}"); return out; }
        }
    }
    let r = (v - core::f64::consts::PI).abs() / p;
    out += &format!(" sum_err/pad={r:.3e} pad={p:.3e}");
    if r > 1.0 { out += " ESCAPE_SUM"; }
    out
}

#[test]
fn review_5734_split_volumes_hold_their_oracles() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut escapes = vec![];
    // Sliver family (the PR's): through (1,0,d), normal (-1e-2,0,1).
    for k in [50.0, 120.0, 500.0, 5000.0, 5e4] {
        for s in [-1e-2, -3e-2, -1e-3] {
            let d = k * eps;
            let r = run(1.0, d, s, 0.0, tol, None);
            eprintln!("sliver d={d:e} s={s:e}: {r}");
            if r.contains("ESCAPE") { escapes.push(format!("sliver d={d:e} s={s:e}: {r}")); }
        }
    }
    // Full crossings: z = h - sx x - sy y, lower volume = pi * h exactly.
    for (h, sx, sy) in [(0.5, 0.3, 0.1), (0.5, -0.45, 0.2), (0.37, 1e-3, -2e-3), (0.6, 0.39, 0.0), (0.5, 0.0, 0.49), (0.5, 1e-7, 0.0)] {
        let r = run(0.0, h, sx, sy, tol, Some(core::f64::consts::PI * h));
        eprintln!("cross h={h} s=({sx},{sy}): {r}");
        if r.contains("ESCAPE") { escapes.push(format!("cross h={h} s=({sx},{sy}): {r}")); }
    }
    assert!(escapes.is_empty(), "{escapes:#?}");
}
