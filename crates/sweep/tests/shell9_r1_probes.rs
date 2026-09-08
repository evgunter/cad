//! **SHELL-9 R1 probes** — the review lane's rows for PR #2223 (the
//! closing pcurve mint in `shell` / `shell_open`).
//!
//! 1. A row dump over a corpus of the reviewer's choosing (the sf2b
//!    frustums and vase, SHELL-8's box beside a vessel, `verbs_shell`'s
//!    hollow operands opened, the tube torus, sphere seam variants),
//!    printed as `[r1rows]` lines for a base/head diff — panic-free, so
//!    the same file runs at the merge base where some bodies refuse.
//! 2. The end-to-end exercise from a consumer's seat.
//! 3. The drum's cause by execution (`validate_geometric` of the
//!    reverted cavity alone) and the sphere's closed forms re-derived.
//! 4. A hunt for a body that reaches `ShellError::Pcurve`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ShellError, ShellRole};

use super::shell7_common::{face_of_he, hollow_moves, point, polyline, tol, tube_torus, tube_torus_hollow};
use super::shell8_common::{beside, cap, outer_and_void_of};
use super::verbs_shell::{boxy, hollow_box, two_void_box, vessel};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// The bulge (`tan(θ/4)`) of the arc from `a` to `b` about `c`.
fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

/// `sf2b_axial`'s sphere-zone vase: a belly on a sphere centred on the
/// axis at `(0, h/2)`, between two caps normal to it.
fn sphere_zone_vase(r: f64, h: f64) -> Body<f64> {
    let c = p2(0.0, h / 2.0);
    revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r, 0.0), bulge(p2(r, 0.0), p2(r, h), c)),
        ProfileVertex::new(p2(r, h), 0.0),
        ProfileVertex::new(p2(0.0, h), 0.0),
    ]))
}

/// `sf2b_axial`'s cone frustum.
fn cone_frustum(r0: f64, r1: f64, h: f64) -> Body<f64> {
    polyline(&[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)], Revolution::Full)
}

/// A sphere of radius `r` authored as cocircular arcs meeting at the
/// latitudes `seams` (each in `(-π/2, π/2)`, ascending).
fn multi_arc_sphere(r: f64, seams: &[f64]) -> Body<f64> {
    let c = p2(0.0, 0.0);
    let mut angles = vec![-FRAC_PI_2];
    angles.extend_from_slice(seams);
    angles.push(FRAC_PI_2);
    let pts: Vec<Point2<f64>> = angles.iter().map(|a| p2(r * a.cos(), r * a.sin())).collect();
    let mut verts = Vec::new();
    for i in 0..pts.len() - 1 {
        verts.push(ProfileVertex::new(pts[i], bulge(pts[i], pts[i + 1], c)));
    }
    verts.push(ProfileVertex::new(pts[pts.len() - 1], 0.0));
    revolved(ProfileLoop::new(verts))
}

/// The two-arc sphere of `shell7_seam_corner` / `shell9_probe`.
fn two_arc_sphere() -> Body<f64> {
    multi_arc_sphere(1.0, &[PI / 4.0])
}

/// A torus of major `big` and minor `small` authored as `n` cocircular
/// arcs — same-surface latitude seams on a torus.
fn n_arc_torus(big: f64, small: f64, n: usize) -> Body<f64> {
    let c = p2(big, 0.0);
    let pts: Vec<Point2<f64>> = (0..n)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / n as f64 + 0.3;
            p2(big + small * a.cos(), small * a.sin())
        })
        .collect();
    let verts: Vec<ProfileVertex<f64>> = (0..n)
        .map(|i| ProfileVertex::new(pts[i], bulge(pts[i], pts[(i + 1) % n], c)))
        .collect();
    revolved(ProfileLoop::new(verts))
}

/// The planar faces whose plane is normal to `y` at height `y`.
fn cap_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.y - y).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.z.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// One line per stored row: half-edge, face, the face's surface kind,
/// the parameter window and the image — `{:?}` is shortest round-trip,
/// so equal text is equal bits.
fn dump_rows(label: &str, body: &Body<f64>) {
    let mut n = 0;
    for (he, cache) in body.pcurves() {
        n += 1;
        let face = face_of_he(body, he);
        let kind = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .map(|s| format!("{s:?}").split(['{', ' ']).next().unwrap_or("?").to_owned())
            .unwrap_or_default();
        println!(
            "[r1rows] {label}: he {he:?} face {face:?} {kind} params {:?} pcurve {:?}",
            cache.params(),
            cache.pcurve()
        );
    }
    let t3 = topo::validate_geometric(body, tol());
    println!("[r1rows] {label}: {n} rows, tier3 {}", if t3.is_ok() { "Ok" } else { "Err" });
}

fn dump_shelled(label: &str, body: &Body<f64>, t: f64, open: &[FaceKey]) -> Option<Body<f64>> {
    match topo::shell_open(body, t, open, tol()) {
        Ok(s) => {
            dump_rows(label, &s.body);
            Some(s.body)
        }
        Err(e) => {
            println!("[r1rows] {label}: Err {e}");
            None
        }
    }
}

/// **Row dump, the reviewer's corpus.** Run with `--nocapture`, grep
/// `[r1rows]`, sort, diff across trees.
#[test]
fn r1_rows_corpus() {
    let t = 1.0 / 128.0;
    // sf2b's bodies.
    let vase = sphere_zone_vase(1.0, 2.0);
    dump_rows("operand vase", &vase);
    dump_shelled("vase sealed", &vase, t, &[]);
    dump_shelled("vase opened top", &vase, t, &cap_at_y(&vase, 2.0));
    dump_shelled("vase opened both", &vase, t, &[cap_at_y(&vase, 2.0), cap_at_y(&vase, 0.0)].concat());
    let frustum = cone_frustum(1.0, 0.6, 2.0);
    dump_shelled("frustum sealed", &frustum, t, &[]);
    dump_shelled("frustum opened wide", &frustum, t, &cap_at_y(&frustum, 0.0));
    dump_shelled("frustum opened narrow", &frustum, t, &cap_at_y(&frustum, 2.0));
    // SHELL-8's multi-solid bodies.
    let pair = beside(&boxy(2.0, 3.0, 4.0), &vessel(1.0, 2.0), 10.0);
    dump_shelled("box beside vessel sealed", &pair, 0.1, &[]);
    let hollow_vessel = topo::shell(&vessel(1.0, 2.0), 0.1, tol()).expect("hollows").body;
    let pair_h = beside(&boxy(2.0, 3.0, 4.0), &hollow_vessel, 10.0);
    dump_rows("operand box beside hollow vessel", &pair_h);
    let vessel_solid = pair_h
        .solids()
        .find(|(k, _)| pair_h.get_solid(*k).unwrap().shells.len() == 2)
        .map(|(k, _)| k)
        .expect("the hollow solid");
    let (_, void) = outer_and_void_of(&pair_h, vessel_solid);
    let ceiling = cap(&pair_h, void, Vec3::new(0.0, 1.0, 0.0), 1.9);
    dump_shelled("box beside hollow vessel sealed", &pair_h, 0.02, &[]);
    dump_shelled("box beside hollow vessel opened void ceiling", &pair_h, 0.02, &ceiling);
    // verbs_shell's hollow operands.
    let hb = hollow_box();
    dump_shelled("hollow box sealed", &hb, 0.05, &[]);
    let (two, gap_t) = two_void_box();
    dump_shelled("two-void box sealed", &two, gap_t / 2.0 * 0.75, &[]);
    // The torus family.
    dump_shelled("tube torus sealed", &tube_torus(2.0, 0.5), 0.05, &[]);
    dump_shelled("tube torus hollow sealed", &tube_torus_hollow(2.0, 0.5, 0.2), 0.05, &[]);
    dump_shelled("three-arc torus sealed", &n_arc_torus(2.0, 0.5, 3), 0.05, &[]);
    // Sphere seam variants.
    dump_shelled("one-arc sphere sealed", &multi_arc_sphere(1.0, &[]), 0.05, &[]);
    dump_shelled("two-arc sphere sealed", &two_arc_sphere(), 0.05, &[]);
    dump_shelled("two-arc sphere seam -pi/4", &multi_arc_sphere(1.0, &[-PI / 4.0]), 0.05, &[]);
    dump_shelled("two-arc sphere seam 0", &multi_arc_sphere(1.0, &[0.0]), 0.05, &[]);
    dump_shelled("four-arc sphere", &multi_arc_sphere(1.0, &[-PI / 4.0, 0.0, PI / 4.0]), 0.05, &[]);
    // A vase whose belly is two cocircular arcs.
    let c = p2(0.0, 1.0);
    let m = p2(2.0f64.sqrt(), 1.0);
    let two_arc_vase = revolved(RawLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), bulge(p2(1.0, 0.0), m, c)),
        ProfileVertex::new(m, bulge(m, p2(1.0, 2.0), c)),
        ProfileVertex::new(p2(1.0, 2.0), 0.0),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
    ]));
    dump_shelled("two-arc vase sealed", &two_arc_vase, t, &[]);
    dump_shelled("two-arc vase opened top", &two_arc_vase, t, &cap_at_y(&two_arc_vase, 2.0));
}

// ---------------------------------------------------------------------
// The end-to-end exercise
// ---------------------------------------------------------------------

/// Sphere zone of radius `rho` centred at height `c`, between `a` and `b`.
fn zone(rho: f64, c: f64, a: f64, b: f64) -> f64 {
    PI * (rho * rho * (b - a) - ((b - c).powi(3) - (a - c).powi(3)) / 3.0)
}

fn check_result(what: &str, body: &Body<f64>, want: f64, tol_abs: f64) {
    assert_eq!(topo::validate_geometric(body, tol()), Ok(()), "{what}: tier 3");
    let roles = topo::classify_shells(body, tol()).expect("classifies");
    let outers = roles.iter().filter(|c| c.role == ShellRole::Outer).count();
    let voids = roles.iter().filter(|c| c.role == ShellRole::Void).count();
    let props = topo::mass_properties(body, tol()).expect("props");
    let rows = body.pcurves().count();
    println!(
        "[r1e2e] {what}: solids {} shells {} (outer {outers}, void {voids}) faces {} rows {rows} volume {} pad {} want {want} diff {:e}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        props.volume,
        props.volume_pad,
        props.volume - want
    );
    assert!(
        (props.volume - want).abs() <= tol_abs + props.volume_pad,
        "{what}: volume {} vs {want}",
        props.volume
    );
    let mesh = mesh::tessellate(body, 1e-3, tol()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **From the consumer's seat**: the two-arc sphere hollowed; the
/// sphere-zone vase hollowed then opened at its cap; SHELL-8's box
/// beside a (hollow) vessel, opened on the vessel's void ceiling.
#[test]
fn r1_end_to_end() {
    // 1. The two-arc sphere.
    let (r, t): (f64, f64) = (1.0, 0.05);
    let sphere = two_arc_sphere();
    let out = topo::shell(&sphere, t, tol()).expect("the two-arc sphere shells");
    let want = 4.0 / 3.0 * PI * (r.powi(3) - (r - t).powi(3));
    check_result("two-arc sphere", &out.body, want, 1e-12);
    // Every inner vertex — poles and seam ring alike — is concentric at r − t.
    for &(new, old) in &out.naming.inner_vertices {
        let p = point(&out.body, new);
        let n = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
        assert!((n - (r - t)).abs() <= 1e-15, "{old:?} -> {new:?} at radius {n}");
    }
    assert_eq!(out.naming.inner_vertices.len(), sphere.vertices().count());
    let cavity = 4.0 / 3.0 * PI * (r - t).powi(3);
    println!(
        "[r1e2e] closed forms: cavity {cavity} (spec 3.591364001828733), thin solid {want} (spec 0.5974262029576595)"
    );
    assert!((cavity - 3.591_364_001_828_733).abs() < 1e-14, "closed form re-derived: {cavity}");
    assert!((want - 0.597_426_202_957_659_5).abs() < 1e-15);

    // 2. The sphere-zone vase, hollowed then opened at its top cap.
    let (rv, h, tv) = (1.0, 2.0, 1.0 / 128.0);
    let vase = sphere_zone_vase(rv, h);
    let big = (rv * rv + (h / 2.0) * (h / 2.0)).sqrt();
    let outer = zone(big, h / 2.0, 0.0, h);
    let sealed = topo::shell(&vase, tv, tol()).expect("the vase hollows");
    check_result(
        "vase sealed",
        &sealed.body,
        outer - zone(big - tv, h / 2.0, tv, h - tv),
        1e-11,
    );
    let opened = topo::shell_open(&vase, tv, &cap_at_y(&vase, h), tol()).expect("the vase opens");
    check_result(
        "vase opened top",
        &opened.body,
        outer - zone(big - tv, h / 2.0, tv, h),
        1e-11,
    );
    assert_eq!(opened.naming.rims.len(), 1);

    // 3. SHELL-8: a box beside a hollow vessel, opened on the void ceiling.
    let hv = topo::shell(&vessel(1.0, 2.0), 0.1, tol()).expect("the vessel hollows").body;
    let pair = beside(&boxy(2.0, 3.0, 4.0), &hv, 10.0);
    let vessel_solid = pair
        .solids()
        .find(|(k, _)| pair.get_solid(*k).unwrap().shells.len() == 2)
        .map(|(k, _)| k)
        .expect("the hollow solid");
    let (_, void) = outer_and_void_of(&pair, vessel_solid);
    let ceiling = cap(&pair, void, Vec3::new(0.0, 1.0, 0.0), 1.9);
    let t8 = 0.02;
    let out8 = topo::shell_open(&pair, t8, &ceiling, tol()).expect("opens on the void ceiling");
    let cyl = |rr: f64, hh: f64| PI * rr * rr * hh;
    let box_wall = 2.0 * 3.0 * 4.0 - (2.0 - 2.0 * t8) * (3.0 - 2.0 * t8) * (4.0 - 2.0 * t8);
    let outer_wall = cyl(1.0, 2.0) - cyl(1.0 - t8, 2.0 - 2.0 * t8);
    // The void: r 0.9, y in [0.1, 1.9]; its twin dilated by t8 (r 0.92,
    // y in [0.08, 1.92]) with the ceiling lifted back onto the mouth at
    // 1.9, minus the void itself.
    let inner_wall = cyl(0.9 + t8, 1.9 - (0.1 - t8)) - cyl(0.9, 1.8);
    check_result(
        "box beside hollow vessel, opened void ceiling",
        &out8.body,
        box_wall + outer_wall + inner_wall,
        1e-10,
    );
    assert_eq!(out8.body.solids().count(), 3);
    assert_eq!(out8.naming.rims.len(), 1);
}

// ---------------------------------------------------------------------
// Claim 3: the drum's cause by execution; claim 4: hunting the arm
// ---------------------------------------------------------------------

fn door_cavity(body: &Body<f64>, t: f64) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol())
        .expect("the door takes it");
    cavity
}

/// **The drum**: the reverted cavity alone, through `validate_geometric`.
#[test]
fn r1_drum_reverted_cavity_alone() {
    let drum = polyline(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.5, 2.0), (0.0, 2.0)], Revolution::Full);
    let cavity = door_cavity(&drum, 0.05);
    assert_eq!(topo::validate_geometric(&cavity, tol()), Ok(()), "cavity tier 3");
    let reverted = cavity.revert().expect("revert");
    let v = topo::validate_geometric(&reverted, tol());
    println!("[r1drum] reverted cavity alone: {v:?}");
    let e = topo::shell(&drum, 0.05, tol()).expect_err("the drum refuses");
    println!("[r1drum] shell: {e}");
    assert!(matches!(e, ShellError::Insert { .. }));
    // The sphere's reverted cavity for contrast.
    let cavity = door_cavity(&two_arc_sphere(), 0.05);
    let reverted = cavity.revert().expect("revert");
    let v = topo::validate_geometric(&reverted, tol());
    println!("[r1drum] sphere reverted cavity alone: {v:?}");
}

/// **Hunting `ShellError::Pcurve`**: bodies every earlier gate takes.
#[test]
fn r1_hunt_the_pcurve_arm() {
    let cases: Vec<(&str, Body<f64>, f64)> = vec![
        ("four-arc sphere", multi_arc_sphere(1.0, &[-PI / 4.0, 0.0, PI / 4.0]), 0.05),
        ("three-arc torus", n_arc_torus(2.0, 0.5, 3), 0.05),
        ("five-arc torus", n_arc_torus(2.0, 0.5, 5), 0.05),
        ("sphere seam near pole", multi_arc_sphere(1.0, &[FRAC_PI_2 - 0.05]), 0.02),
        ("sphere seams both near poles", multi_arc_sphere(1.0, &[-FRAC_PI_2 + 0.05, FRAC_PI_2 - 0.05]), 0.02),
    ];
    for (name, body, t) in cases {
        let v = topo::validate_geometric(&body, tol());
        let out = topo::shell(&body, t, tol());
        match &out {
            Ok(s) => println!(
                "[r1hunt] {name}: operand tier3 {} -> shells, rows {}, tier3 {:?}",
                v.is_ok(),
                s.body.pcurves().count(),
                topo::validate_geometric(&s.body, tol()).is_ok()
            ),
            Err(e) => println!("[r1hunt] {name}: operand tier3 {} -> Err {e}", v.is_ok()),
        }
        assert!(!matches!(out, Err(ShellError::Pcurve { .. })), "{name}: the arm is reached");
    }
}
