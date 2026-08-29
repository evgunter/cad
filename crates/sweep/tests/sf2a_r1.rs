//! **SHELLFIX PR-2a reviewer R1 probes (probe branch only).**
//!
//! Written to compile at the MERGE BASE and at the head unchanged, so
//! every row is a before/after measurement rather than an assertion
//! about one side. Print-heavy on purpose: the interesting output is
//! the numbers, not the pass/fail.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;

/// A right prism on a polygon (the PR's own helper, copied).
fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a polygon extrudes")
        .body
}

/// Shoelace area of a CCW polygon.
fn area(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len();
    let mut a = 0.0;
    for i in 0..n {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % n];
        a += x0 * y1 - x1 * y0;
    }
    a * 0.5
}

/// The polygon offset INWARD by `t`, computed independently of the
/// kernel: each edge's supporting line moves in by `t` and adjacent
/// lines are intersected. Valid for a convex polygon whose offset does
/// not degenerate — every fixture here is one.
fn offset_in(pts: &[(f64, f64)], t: f64) -> Vec<(f64, f64)> {
    let n = pts.len();
    // Edge i runs pts[i] -> pts[i+1]; outward normal for CCW is
    // (dy, -dx) normalized. Line: n·X = n·P - t.
    let lines: Vec<(f64, f64, f64)> = (0..n)
        .map(|i| {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[(i + 1) % n];
            let (dx, dy) = (x1 - x0, y1 - y0);
            let l = (dx * dx + dy * dy).sqrt();
            let (nx, ny) = (dy / l, -dx / l);
            (nx, ny, nx * x0 + ny * y0 - t)
        })
        .collect();
    // Vertex i is edge i-1 ∩ edge i.
    (0..n)
        .map(|i| {
            let (a1, b1, c1) = lines[(i + n - 1) % n];
            let (a2, b2, c2) = lines[i];
            let det = a1 * b2 - a2 * b1;
            ((c1 * b2 - c2 * b1) / det, (a1 * c2 - a2 * c1) / det)
        })
        .collect()
}

/// The wall volume of `shell(prism(pts, h), t)` in closed form.
fn wall_volume(pts: &[(f64, f64)], h: f64, t: f64) -> f64 {
    area(pts) * h - area(&offset_in(pts, t)) * (h - 2.0 * t)
}

/// A plane as this file reads it: an origin and a unit normal.
type PlaneFrame = (Point3<f64>, Vec3<f64>);

/// Every distinct plane (origin, unit normal) of a body's faces.
fn planes(body: &Body<f64>) -> Vec<PlaneFrame> {
    let mut out: Vec<PlaneFrame> = Vec::new();
    for (_, f) in body.faces() {
        if let Some(geom::Surface::Plane { origin, normal, .. }) = body.get_surface(f.surface)
            && !out
                .iter()
                .any(|(o, n)| (*n - *normal).norm() < 1e-12 && (n.dot(*o - *origin)).abs() < 1e-12)
        {
            out.push((*origin, *normal));
        }
    }
    out
}

fn points(body: &Body<f64>) -> Vec<Point3<f64>> {
    body.vertices()
        .filter_map(|(k, _)| body.get_vertex(k))
        .filter_map(|v| body.get_point(v.point))
        .copied()
        .collect()
}

fn report(what: &str, r: Result<Body<f64>, topo::ShellError<f64>>) -> Option<Body<f64>> {
    match r {
        Ok(b) => {
            let props = topo::mass_properties(&b, Tol::witness()).expect("props");
            println!(
                "[r1] {what}: BUILDS  shells={} volume={:.17e} tier3={:?}",
                b.shells().count(),
                props.volume,
                topo::validate_geometric(&b, Tol::witness()).is_ok(),
            );
            Some(b)
        }
        Err(e) => {
            println!("[r1] {what}: REFUSES {e}");
            None
        }
    }
}

// =====================================================================
// R1-A. The hexagon's corners, measured the way the spec's falsified
// premise was measured: the signed offset of every cavity corner from
// each of the three ORIGINAL planes meeting it must be exactly -t.
// =====================================================================

#[test]
fn r1a_hexagon_cavity_corners_sit_at_minus_t_on_all_three_planes() {
    let t = 0.02;
    let r = 0.2;
    let pts: Vec<(f64, f64)> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            (r * a.cos(), r * a.sin())
        })
        .collect();
    let body = prism(&pts, 0.25);
    let outer = planes(&body);
    let Some(hollow) = report(
        "hexagonal prism t=0.02",
        topo::shell(&body, t, FIT_TOL, band(), Tol::witness()),
    ) else {
        return;
    };
    println!(
        "[r1] closed form wall volume = {:.17e}",
        wall_volume(&pts, 0.25, t)
    );

    // Every point of the shelled body: its signed distances to the
    // original planes. A cavity corner is one lying at -t on exactly
    // three of them; an outer corner lies at 0 on three.
    let mut worst: f64 = 0.0;
    let mut cavity_corners = 0;
    for p in points(&hollow) {
        let mut d: Vec<f64> = outer.iter().map(|(o, n)| n.dot(p - *o)).collect();
        d.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
        let near: Vec<f64> = d.into_iter().filter(|x| x.abs() < 0.5 * r).collect();
        // the three (or more) planes this corner is "on"
        let on: Vec<f64> = near.iter().copied().filter(|x| x.abs() < 1.5 * t).collect();
        if on.len() >= 3 && on.iter().all(|x| *x < -0.5 * t) {
            cavity_corners += 1;
            for x in &on {
                worst = worst.max((x + t).abs());
            }
        }
    }
    println!(
        "[r1a] cavity corners found = {cavity_corners}; worst |signed offset - (-t)| = {worst:.3e}"
    );
    assert!(cavity_corners > 0, "no cavity corners identified");
    assert!(
        worst < 1e-14,
        "a cavity corner is not at -t on one of its own planes: {worst:e}"
    );
}

// =====================================================================
// R1-B. The three fixtures the PR hollows WITHOUT a closed form
// (bevel, kite, triangle), checked against one.
// =====================================================================

#[test]
fn r1b_the_unpinned_oblique_fixtures_against_their_closed_forms() {
    let t = 0.02;
    let h = 0.25;
    for (what, pts) in [
        (
            "bevelled box",
            vec![(0.0, 0.0), (0.4, 0.0), (0.3, 0.3), (0.0, 0.3)],
        ),
        (
            "kite",
            vec![(0.0, 0.0), (0.2, -0.1), (0.4, 0.0), (0.2, 0.3)],
        ),
        (
            "triangle 58/58/64",
            vec![(0.0, 0.0), (0.3, 0.0), (0.15, 0.26)],
        ),
    ] {
        let want = wall_volume(&pts, h, t);
        let body = prism(&pts, h);
        match topo::shell(&body, t, FIT_TOL, band(), Tol::witness()) {
            Ok(b) => {
                let got = topo::mass_properties(&b, Tol::witness())
                    .expect("props")
                    .volume;
                println!(
                    "[r1b] {what}: BUILDS volume={got:.17e} closed form={want:.17e} \
                     delta={:.3e} rel={:.3e}",
                    got - want,
                    (got - want).abs() / want
                );
                assert!(
                    (got - want).abs() <= 1e-12,
                    "{what}: the wall volume misses its closed form by {:e}",
                    got - want
                );
            }
            Err(e) => println!("[r1b] {what}: REFUSES {e}"),
        }
    }
}

// =====================================================================
// R1-C. A VALENCE-4 planar corner: the chamfered cube. Every vertex of
// it has four distinct planes (one cube face, two strips, one corner
// patch), and they do NOT concur under a uniform inward offset — so
// the door must refuse typed and must never build.
// =====================================================================

#[test]
fn r1c_chamfered_cube_is_a_valence_four_planar_corner() {
    let body = sweep::test_support::cube(1.0, Tol::witness());
    let edges: Vec<topo::EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let chamfered = sweep::chamfer::chamfer_edges(&body, &edges, 0.1, band(), Tol::witness())
        .expect("a cube chamfers")
        .body;
    println!(
        "[r1c] chamfered cube: V={} E={} F={}",
        chamfered.vertices().count(),
        chamfered.edges().count(),
        chamfered.faces().count()
    );
    // The valence census: distinct planes at each vertex.
    let mut hist: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    for (vk, v) in chamfered.vertices() {
        let Some(em) = v.emanating else { continue };
        let orbit = chamfered.vertex_orbit(em).expect("orbit");
        let mut ns: Vec<Vec3<f64>> = Vec::new();
        for he in orbit {
            let lk = chamfered.get_half_edge(he).unwrap().parent_loop;
            let fk = chamfered.get_loop(lk).unwrap().face;
            let f = chamfered.get_face(fk).unwrap();
            if let Some(geom::Surface::Plane { normal, .. }) = chamfered.get_surface(f.surface)
                && !ns.iter().any(|n| (*n - *normal).norm() < 1e-12)
            {
                ns.push(*normal);
            }
        }
        *hist.entry(ns.len()).or_default() += 1;
        if ns.len() == 4 {
            // Solve n2 = a*n1 + b*n3 + c*n4 for the affine sum: the
            // four offset planes concur iff d2 = a*d1 + b*d3 + c*d4,
            // so under a UNIFORM offset iff a+b+c == 1.
            let (n1, n2, n3, n4) = (ns[0], ns[1], ns[2], ns[3]);
            let det = n1.dot(n3.cross(n4));
            if det.abs() > 1e-9 {
                let a = n2.dot(n3.cross(n4)) / det;
                let b = n1.dot(n2.cross(n4)) / det;
                let c = n1.dot(n3.cross(n2)) / det;
                println!(
                    "[r1c] vertex {vk:?}: 4 planes, affine sum a+b+c = {:.17} (concur under a \
                     uniform offset iff 1)",
                    a + b + c
                );
            }
        }
    }
    println!("[r1c] distinct-plane valence histogram: {hist:?}");
    report(
        "chamfered cube t=0.05",
        topo::shell(&chamfered, 0.05, FIT_TOL, band(), Tol::witness()),
    );
}

// =====================================================================
// R1-D. The coplanar-adjacent corner BEFORE and AFTER, through `shell`
// (the PR's own row goes through the raw door instead). The footprint
// is a unit square with a STRAIGHT extra vertex on one side, so the
// correct wall volume is the square's.
// =====================================================================

#[test]
fn r1d_the_straight_footprint_vertex_through_shell() {
    let t = 0.05;
    let h = 0.4;
    let pts = vec![(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let square = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let want = wall_volume(&square, h, t);
    println!("[r1d] the correct wall volume is the square's: {want:.17e}");
    let body = prism(&pts, h);
    if let Some(b) = report(
        "straight-footprint prism t=0.05",
        topo::shell(&body, t, FIT_TOL, band(), Tol::witness()),
    ) {
        let got = topo::mass_properties(&b, Tol::witness())
            .expect("props")
            .volume;
        println!(
            "[r1d] volume={got:.17e} want={want:.17e} delta={:.3e}",
            got - want
        );
    }
}

// =====================================================================
// R1-E. The conditioning meter: the SAME geometry, different offsets.
// The gate is `|det| * (sum of |d| over charts)` against a band in
// meters, so its verdict is a function of the offset asked for and of
// how many charts the call names — neither of which is a property of
// the corner's conditioning.
// =====================================================================

#[test]
fn r1e_conditioning_verdict_moves_with_the_offset_alone() {
    let h = 0.4;
    for delta in [1e-2, 1e-4, 1e-6, 1e-8] {
        let pts = vec![
            (0.0, 0.0),
            (0.5, -delta),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];
        // the near-straight vertex at (0.5, -delta) bulges OUTWARD, so
        // the loop stays convex and CCW.
        for t in [0.05, 1e-3, 1e-5, 1e-7] {
            let body = prism(&pts, h);
            let want = wall_volume(&pts, h, t);
            match topo::shell(&body, t, FIT_TOL, band(), Tol::witness()) {
                Ok(b) => {
                    let got = topo::mass_properties(&b, Tol::witness())
                        .expect("props")
                        .volume;
                    println!(
                        "[r1e] delta={delta:e} t={t:e}: BUILDS rel_err={:.3e}",
                        (got - want).abs() / want.abs()
                    );
                }
                Err(e) => println!("[r1e] delta={delta:e} t={t:e}: REFUSES {e}"),
            }
        }
    }
}

// =====================================================================
// R1-F. The split's boundary: ONE curved face among planars, with an
// oblique PLANAR junction elsewhere on the same body.
// =====================================================================

#[test]
fn r1f_one_curved_face_among_planars() {
    let h = 0.25;
    let r = 0.2;
    // A hexagon with ONE side replaced by an arc (bulge on that edge).
    let vs: Vec<ProfileVertex<f64>> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            let bulge = if i == 0 { 0.2 } else { 0.0 };
            ProfileVertex::new(p2(r * a.cos(), r * a.sin()), bulge)
        })
        .collect();
    let profile = Profile::new(SketchPlane::xy(), vec![ProfileLoop::new(vs)])
        .validate(Tol::witness())
        .expect("the bulged hexagon validates");
    let body = extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("extrudes")
        .body;
    let curved = body
        .faces()
        .filter(|(_, f)| {
            !matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .count();
    println!(
        "[r1f] bulged hexagon: {curved} non-planar face(s) of {}",
        body.faces().count()
    );
    report(
        "hexagon with ONE arc side t=0.02",
        topo::shell(&body, 0.02, FIT_TOL, band(), Tol::witness()),
    );
}

// =====================================================================
// R1-G. The box corpus control (the bit-identity claim's own shape,
// measured as a volume rather than a byte dump).
// =====================================================================

#[test]
fn r1g_box_control() {
    let pts = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)];
    let want = wall_volume(&pts, 4.0, 0.25);
    let body = prism(&pts, 4.0);
    if let Some(b) = report(
        "box 2x3x4 t=0.25",
        topo::shell(&body, 0.25, FIT_TOL, band(), Tol::witness()),
    ) {
        let got = topo::mass_properties(&b, Tol::witness())
            .expect("props")
            .volume;
        println!(
            "[r1g] volume bits={:x} closed form={want:.17e} delta={:.3e}",
            got.to_bits(),
            got - want
        );
    }
}

// =====================================================================
// R1-H. The bit-identity harness, WIDENED. PR-1's adopted dump prints
// carriers, vertices, loops and the mass-property bits — but not an
// edge's PARAMETER RANGE, not its DESCRIPTION, and not the pcurve
// rows. All three are conventional data of exactly the kind the PR's
// own "conventional data is data" finding is about, so a dump that
// cannot see them cannot decide the claim it is quoted for. This runs
// the same five fixtures with those three columns added.
// =====================================================================

fn wide_dump(body: &Body<f64>) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={} L={} S={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        body.loops().count(),
        body.shells().count(),
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(
            s,
            "v {k:?} {:x} {:x} {:x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    for (k, f) in body.faces() {
        let surf = body
            .get_surface(f.surface)
            .map(|x| format!("{x:?}"))
            .unwrap_or_else(|| "?".into());
        let _ = writeln!(
            s,
            "f {k:?} sense={} rings={} {surf}",
            f.sense,
            f.rings.len()
        );
    }
    for (k, e) in body.edges() {
        let g = body
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified);
        match g {
            Some(g) => {
                let (t0, t1) = g.params();
                let _ = writeln!(
                    s,
                    "e {k:?} t0={:x} t1={:x} carrier={:?} desc={:?}",
                    t0.to_bits(),
                    t1.to_bits(),
                    g.carrier(),
                    g.description()
                );
            }
            None => {
                let _ = writeln!(s, "e {k:?} null");
            }
        }
    }
    let mut pc: Vec<String> = body
        .pcurves()
        .map(|(he, c)| format!("pc {he:?} {c:?}"))
        .collect();
    pc.sort();
    for line in pc {
        let _ = writeln!(s, "{line}");
    }
    let props = topo::mass_properties(body, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "props V={:x} A={:x} pad={:x}",
        props.volume.to_bits(),
        props.surface_area.to_bits(),
        props.volume_pad.to_bits()
    );
    s
}

#[test]
fn r1h_widened_bit_identity_dump() {
    let Some(dir) = std::env::var_os("SF2A_R1_DUMP_DIR").map(std::path::PathBuf::from) else {
        println!("[r1h] SF2A_R1_DUMP_DIR unset; clean skip");
        return;
    };
    std::fs::create_dir_all(&dir).unwrap();
    let tol = Tol::witness();
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = prism(&[(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)], h);
    let plane_face_at_z = |body: &Body<f64>, z: f64| {
        body.faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Plane { origin, normal, .. })
                        if (origin.z - z).abs() < 1e-9
                            && normal.x.abs() < 1e-9
                            && normal.y.abs() < 1e-9
                )
            })
            .map(|(k, _)| k)
            .unwrap()
    };
    let top = plane_face_at_z(&body, h);
    let bottom = plane_face_at_z(&body, 0.0);
    for (name, b) in [
        (
            "sealed_box",
            topo::shell(&body, t, FIT_TOL, band(), tol).unwrap(),
        ),
        (
            "box_cup",
            topo::shell_open(&body, t, &[top], FIT_TOL, band(), tol).unwrap(),
        ),
        (
            "box_tube",
            topo::shell_open(&body, t, &[top, bottom], FIT_TOL, band(), tol).unwrap(),
        ),
    ] {
        std::fs::write(dir.join(format!("{name}.wide.txt")), wide_dump(&b)).unwrap();
    }
    println!("[r1h] wrote widened dumps to {}", dir.display());
}
