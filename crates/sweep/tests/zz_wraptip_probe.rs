//! PROBE (foreign-wraptip): the consumers a one-face pole (wrap tip)
//! meets. Run with and without `CAD_WRAPTIP=1` (which makes full
//! revolves build their wire walls in one band) and compare. Prints
//! outcomes; asserts nothing beyond what it reports.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use std::f64::consts::{FRAC_PI_4, PI};
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, rim_arcs_at, try_revolved_about_y_at};
use topo::{AtRestBody, Body, BooleanResult, ContactRecords, subtract};

fn tol() -> Tol {
    Tol::witness()
}

fn mode() -> &'static str {
    if std::env::var_os("CAD_WRAPTIP").is_some() {
        "wraptip"
    } else {
        "two-band"
    }
}

fn counts(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// Runs `f`, printing PANIC if it panics.
fn row(name: &str, f: impl FnOnce() -> String) {
    let out = catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|p| {
        let msg = p
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_default();
        format!("PANIC: {}", msg.chars().take(600).collect::<String>())
    });
    println!("ROW [{}] {name}: {out}", mode());
}

fn tiers(b: &AtRestBody<f64>) -> String {
    let t3 = topo::validate_geometric(b, tol());
    let t3p = topo::validate_pseudomanifold(b, &ContactRecords::default(), tol());
    format!(
        "tier3={} tier3'={}",
        if t3.is_ok() { "ok".into() } else { format!("{t3:?}") },
        if t3p.is_ok() { "ok".into() } else { format!("{t3p:?}") }
    )
}

fn vol_check(what: &str, b: &AtRestBody<f64>, want: f64) -> String {
    match topo::mass_properties(b, tol(), topo::test_support::pinned_resolution(tol())) {
        Ok(m) => match m.met() {
            Ok(e) => {
                let got = e.volume;
                let rel = (got - want).abs() / want;
                format!(
                    "{what} vol {got:.15} want {want:.15} rel {rel:.2e} {}",
                    if rel < 1e-6 { "RIGHT" } else { "WRONG" }
                )
            }
            Err(u) => format!("{what} vol unmet: {u}"),
        },
        Err(e) => format!("{what} vol REFUSED: {e}"),
    }
}

fn build(what: &str, verts: Vec<(Point2<f64>, f64)>) -> Result<AtRestBody<f64>, String> {
    let body = try_revolved_about_y_at(verts, Revolution::Full, tol())
        .map_err(|e| format!("{what}: revolve refused {e:?}"))?;
    let c = counts(&body);
    <f64 as topo::AtRestPolicy>::gate_at_rest_kept(body, tol())
        .map_err(|e| format!("{what}: {c:?} not at rest: {e:?}"))
}

fn hemisphere() -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), (FRAC_PI_4 / 2.0).tan()),
        (Point2::new(0.0, 1.0), 0.0),
    ]
}

fn ball() -> Vec<(Point2<f64>, f64)> {
    vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)]
}

fn cone() -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]
}

const HEMI: f64 = 2.0 * PI / 3.0;

fn cap(h: f64) -> f64 {
    PI * h * h * (3.0 - h) / 3.0
}

fn mesh_row(what: &str, b: &Body<f64>) -> String {
    match mesh::tessellate(b, 0.01, tol()) {
        Ok(m) => format!(
            "{what} mesh ok signed_vol={:.6} tris={} check={:?}",
            m.patches
                .iter()
                .flat_map(|p| p.triangles.iter())
                .map(|t| {
                    let [a, b, c] = t.map(|i| m.positions[i as usize] - Point3::origin());
                    a.dot(b.cross(c)) / 6.0
                })
                .sum::<f64>(),
            m.patches.iter().map(|p| p.triangles.len()).sum::<usize>(),
            mesh::validate::check_mesh(&m).map(|_| "watertight")
        ),
        Err(e) => format!("{what} mesh REFUSED {e:?}"),
    }
}

fn boolean_row(
    what: &str,
    a: &AtRestBody<f64>,
    cutter: Body<f64>,
    want: f64,
) -> String {
    let c = finished("cutter", cutter, tol());
    match subtract(a, &c, tol()) {
        Ok(BooleanResult::Body(b)) => {
            format!("{} {:?} {}", vol_check(what, &b.body, want), counts(&b.body), tiers(&b.body))
        }
        other => format!("{what} boolean -> {other:?}"),
    }
}

fn split_row(
    what: &str,
    a: &AtRestBody<f64>,
    o: Point3<f64>,
    n: Vec3<f64>,
    above: f64,
    below: f64,
) -> String {
    let plane = topo::test_support::split_plane(o, n.normalize(), tol());
    match topo::split(a, &plane, tol()) {
        Ok(r) => {
            let mut s = String::new();
            for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
                match part.body() {
                    Some(b) => s += &format!(
                        "[{side}: {} {:?} {}] ",
                        vol_check(what, b, want),
                        counts(b),
                        tiers(b)
                    ),
                    None => s += &format!("[{side}: empty, want {want}] "),
                }
            }
            s
        }
        Err(e) => format!("{what} split REFUSED {e:?}"),
    }
}

#[test]
fn zz_wraptip_probe() {
    // ---- builds ----
    let hemi = build("hemisphere", hemisphere());
    let ball_b = build("ball", ball());
    let cone_b = build("cone", cone());
    for (n, b, want) in [
        ("hemisphere", &hemi, HEMI),
        ("ball", &ball_b, 4.0 * PI / 3.0),
        ("cone", &cone_b, PI / 3.0),
    ] {
        row(&format!("build {n}"), || match b {
            Ok(b) => format!("{:?} {} {}", counts(b), tiers(b), vol_check(n, b, want)),
            Err(e) => e.clone(),
        });
    }
    // ---- mesh ----
    for (n, b) in [("hemisphere", &hemi), ("ball", &ball_b), ("cone", &cone_b)] {
        if let Ok(b) = b {
            row(&format!("mesh {n}"), || mesh_row(n, b));
        }
    }
    if let Ok(h) = &hemi {
        // ---- Boolean: a slab across the cap; a half-space through the pole ----
        row("boolean hemi minus slab y>0.5", || {
            boolean_row("hemi∖slab", h, brick((-2.0, 2.0), (0.5, 2.0), (-2.0, 2.0), tol()), HEMI - cap(0.5))
        });
        row("boolean hemi minus x>0 (through pole, across meridian plane)", || {
            boolean_row("hemi∖x>0", h, brick((0.0, 2.0), (-0.5, 2.0), (-2.0, 2.0), tol()), HEMI / 2.0)
        });
        row("boolean hemi minus z>0 (through pole, along the seam meridian)", || {
            boolean_row("hemi∖z>0", h, brick((-2.0, 2.0), (-0.5, 2.0), (0.0, 2.0), tol()), HEMI / 2.0)
        });
        row("boolean hemi minus z<0 (seam side, other half)", || {
            boolean_row("hemi∖z<0", h, brick((-2.0, 2.0), (-0.5, 2.0), (-2.0, 0.0), tol()), HEMI / 2.0)
        });
        row("boolean hemi minus x<0", || {
            boolean_row("hemi∖x<0", h, brick((-2.0, 0.0), (-0.5, 2.0), (-2.0, 2.0), tol()), HEMI / 2.0)
        });
        row("boolean hemi minus quadrant x>0,z>0 (pole on the cutter's edge)", || {
            boolean_row("hemi∖q", h, brick((0.0, 2.0), (-0.5, 2.0), (0.0, 2.0), tol()), 0.75 * HEMI)
        });
        row("boolean hemi minus quadrant x<0,z<0", || {
            boolean_row("hemi∖q'", h, brick((-2.0, 0.0), (-0.5, 2.0), (-2.0, 0.0), tol()), 0.75 * HEMI)
        });
        row("boolean hemi minus x<0.5 (kept piece crossed by the wrap meridian rim to cut)", || {
            boolean_row("hemi∖x<.5", h, brick((-2.0, 0.5), (-0.5, 2.0), (-2.0, 2.0), tol()), cap(0.5) / 2.0)
        });
        row("boolean hemi minus x>0.3 (off pole)", || {
            boolean_row("hemi∖x>0.3", h, brick((0.3, 2.0), (-0.5, 2.0), (-2.0, 2.0), tol()), HEMI - cap(0.7) / 2.0)
        });
        // ---- split ----
        let y = Vec3::new(0.0, 1.0, 0.0);
        row("split hemi y=0.5 (across)", || {
            split_row("hemi|y=.5", h, Point3::new(0.0, 0.5, 0.0), y, cap(0.5), HEMI - cap(0.5))
        });
        row("split hemi x=0 (section chords end at the tip)", || {
            split_row("hemi|x=0", h, Point3::origin(), Vec3::new(1.0, 0.0, 0.0), HEMI / 2.0, HEMI / 2.0)
        });
        row("split hemi z=0 (section along the seam meridian)", || {
            split_row("hemi|z=0", h, Point3::origin(), Vec3::new(0.0, 0.0, 1.0), HEMI / 2.0, HEMI / 2.0)
        });
        let hc = 1.0 - 1.0 / 2f64.sqrt();
        row("split hemi x+y=1 (tilted, through pole)", || {
            split_row("hemi|x+y=1", h, Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0), cap(hc), HEMI - cap(hc))
        });
        row("split hemi y+z=1 (tilted, through pole, seam side)", || {
            split_row("hemi|y+z=1", h, Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 1.0), cap(hc), HEMI - cap(hc))
        });
        // ---- fillet along the cap's rim ----
        for r in [0.1, 0.3] {
            row(&format!("fillet hemi rim r={r}"), || {
                let rim = rim_arcs_at(h, 1.0, 0.0);
                match sweep::blend::fillet_edges(h, &rim, r, tol()) {
                    Ok(out) => {
                        let fb = <f64 as topo::AtRestPolicy>::gate_at_rest_kept(out.body.clone(), tol());
                        match fb {
                            Ok(fb) => format!(
                                "rim edges {} {} {:?} {}",
                                rim.len(),
                                vol_check("fillet", &fb, HEMI - fillet_removed(r)),
                                counts(&fb),
                                tiers(&fb)
                            ),
                            Err(e) => format!("fillet built but not at rest: {e:?}"),
                        }
                    }
                    Err(e) => format!("fillet REFUSED {e:?}"),
                }
            });
        }
    }
    if let Ok(b) = &ball_b {
        row("split ball y=0.5", || {
            split_row("ball|y=.5", b, Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), cap(0.5), 4.0 * PI / 3.0 - cap(0.5))
        });
        row("boolean ball minus x<0 (kept half holds the wrap meridian, both poles on the cut)", || {
            boolean_row("ball∖x<0", b, brick((-2.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol()), 2.0 * PI / 3.0)
        });
        row("boolean ball minus x<0.5 (kept cap off both poles, meridian crosses it)", || {
            boolean_row("ball∖x<.5", b, brick((-2.0, 0.5), (-2.0, 2.0), (-2.0, 2.0), tol()), cap(0.5))
        });
        row("boolean ball minus x>0", || {
            boolean_row("ball∖x>0", b, brick((0.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol()), 2.0 * PI / 3.0)
        });
    }
    row("boolean slab minus y-poled ball (the die's pip; the Boolean re-cuts the ball)", || {
        let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol()), tol());
        let ball = sweep::test_support::ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.2), tol());
        let h = 0.3;
        boolean_row("slab∖pip", &slab, ball, 16.0 - PI * h * h * (1.5 - h) / 3.0)
    });
    for (pose, z) in [("z>0", (0.0, 2.0)), ("z<0", (-2.0, 0.0)), ("both", (-2.0, 2.0))] {
        let capv = PI * 0.25 * (3.0 - 0.5) / 3.0;
        let shared = if pose == "both" { capv } else { capv / 2.0 };
        let boxv = 2.5 * 4.0 * (z.1 - z.0);
        let ballv = 4.0 * PI / 3.0;
        for (op, want) in [("union", boxv + ballv - shared), ("intersect", shared), ("subtract", boxv - shared)] {
            row(&format!("tilted_sphere_pair box x>0.5 {pose} {op} ball"), || {
                let bx = finished("box", brick((0.5, 3.0), (-2.0, 2.0), z, tol()), tol());
                let ball = finished("ball", topo::transform_rigid(&try_revolved_about_y_at(ball(), Revolution::Full, tol()).unwrap(), &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, 0.0)).unwrap(), tol()).unwrap(), tol());
                let r = match op {
                    "union" => topo::union(&bx, &ball, tol()),
                    "intersect" => topo::intersect(&bx, &ball, tol()),
                    _ => topo::subtract(&bx, &ball, tol()),
                };
                match r {
                    Ok(BooleanResult::Body(b)) => format!("{} {:?} {}", vol_check(op, &b.body, want), counts(&b.body), tiers(&b.body)),
                    other => format!("{other:?}"),
                }
            });
        }
    }
    row("boolean box minus a ball inside it (a one-face sphere void, sense false)", || {
        let bx = finished("box", brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol()), tol());
        let ball = sweep::test_support::ball_poled_y(0.5, Vec3::new(0.1, 0.2, 0.3), tol());
        boolean_row("box∖ball", &bx, ball, 64.0 - 4.0 * PI * 0.125 / 3.0)
    });
    if let Ok(c) = &cone_b {
        row("split cone x=0 (through apex)", || {
            split_row("cone|x=0", c, Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI / 6.0, PI / 6.0)
        });
        row("split cone z=0 (along the seam meridian, through apex)", || {
            split_row("cone|z=0", c, Point3::origin(), Vec3::new(0.0, 0.0, 1.0), PI / 6.0, PI / 6.0)
        });
        for (n, nn) in [("tilted a", Vec3::new(1.0, 0.3, 0.2)), ("tilted b", Vec3::new(0.2, 0.3, 1.0)), ("tilted c", Vec3::new(-1.0, 0.5, -0.4))] {
            row(&format!("split cone through apex {n} (volumes printed; compare modes)"), || {
                split_row("cone|tilt", c, Point3::new(0.0, 1.0, 0.0), nn, 1.0, 1.0)
            });
        }
        for (psi, phi, qy) in [(0.0f64, 0.2f64, 0.15), (PI, 0.2, 0.15), (0.53, 0.2, 0.15), (0.0, 0.5, 0.4)] {
            row(&format!("split cone psi {psi} phi {phi} y {qy} (compare modes)"), || {
                let n = Vec3::new(phi.sin() * psi.cos(), phi.cos(), phi.sin() * psi.sin());
                split_row("cone|reach", c, Point3::new(0.0, qy, 0.0), n, 1.0, 1.0)
            });
        }
        let tip = PI * 0.25 * 0.5 / 3.0;
        row("split cone y=0.5", || {
            split_row("cone|y=.5", c, Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), tip, PI / 3.0 - tip)
        });
        row("boolean cone minus slab y>0.5", || {
            boolean_row("cone∖slab", c, brick((-2.0, 2.0), (0.5, 2.0), (-2.0, 2.0), tol()), PI / 3.0 - tip)
        });
        row("boolean cone minus x>0 (through apex)", || {
            boolean_row("cone∖x>0", c, brick((0.0, 2.0), (-0.5, 2.0), (-2.0, 2.0), tol()), PI / 6.0)
        });
    }
}

/// The volume a fillet of radius `r` removes at the hemisphere's rim
/// (unit sphere on the plane y = 0): Pappus over the profile region
/// between the corner (1, 0), the line, the unit circle and the
/// fillet circle, integrated numerically.
fn fillet_removed(r: f64) -> f64 {
    let cx = ((1.0 - r) * (1.0 - r) - r * r).sqrt();
    let c = Point2::new(cx, r);
    let t2 = Point2::new(cx / (1.0 - r), r / (1.0 - r));
    // Boundary, CCW: (cx,0) -> (1,0) along the line; (1,0) -> t2 on the
    // unit circle; t2 -> (cx,0) on the fillet circle (clockwise about c).
    let mut pts = vec![Point2::new(cx, 0.0), Point2::new(1.0, 0.0)];
    let n = 200_000;
    let a1 = t2.y.atan2(t2.x);
    for i in 1..=n {
        let a = a1 * f64::from(i) / f64::from(n);
        pts.push(Point2::new(a.cos(), a.sin()));
    }
    let b0 = (t2.y - c.y).atan2(t2.x - c.x);
    let b1 = -PI / 2.0;
    for i in 1..=n {
        let b = b0 + (b1 - b0) * f64::from(i) / f64::from(n);
        pts.push(Point2::new(c.x + r * b.cos(), c.y + r * b.sin()));
    }
    // Pappus: V = 2π ∫∫ x dA = 2π · Σ over edges of the polygon moment.
    let mut m = 0.0;
    for w in pts.windows(2) {
        let (p, q) = (w[0], w[1]);
        let cross = p.x * q.y - q.x * p.y;
        m += cross * (p.x + q.x) / 6.0;
    }
    (2.0 * PI * m).abs()
}
