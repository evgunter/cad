//! **SHELL-7's measurement and differential instrument.** Prints a
//! deterministic dump — every face's surface, every edge's carrier,
//! parameters and description with the faces and vertices it joins,
//! every vertex's point and the DISTINCT surfaces at it, the volume
//! and the tier-3 verdict — of every axial-door fixture in this
//! crate's own vocabulary and of the tour's two torus vessels, plus
//! the full-period tori the unit is about. The same file compiled at
//! the merge base and at the head is diffed line by line; it asserts
//! nothing beyond "the fixture builds", the diff is the verdict. Kept
//! free of every symbol the unit adds, so it compiles on both trees.
//!
//! Run with `--no-capture`, grep `[dump]`, diff across trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Curve3;
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, FaceKey, VertexKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

fn faces_at(body: &Body<f64>, v: VertexKey) -> Vec<FaceKey> {
    let Some(em) = body.get_vertex(v).unwrap().emanating else {
        return Vec::new();
    };
    let mut out: Vec<FaceKey> = body
        .vertex_orbit(em)
        .unwrap()
        .into_iter()
        .map(|he| face_of_he(body, he))
        .collect();
    out.sort();
    out.dedup();
    out
}

fn dump(label: &str, body: &Body<f64>) {
    println!(
        "[dump] {label}: solids={} shells={} faces={} edges={} vertices={}",
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count()
    );
    for (k, f) in body.faces() {
        println!(
            "[dump] {label}: face {k:?} shell={:?} sense={} rings={} surface_key={:?} surface={:?}",
            f.shell,
            f.sense,
            f.rings.len(),
            f.surface,
            body.get_surface(f.surface)
        );
    }
    for (k, e) in body.edges() {
        let (fa, fb) = (face_of_he(body, e.he_plus), face_of_he(body, e.he_minus));
        let start = body.get_half_edge(e.he_plus).unwrap().start;
        let end = body.half_edge_end(e.he_plus).unwrap();
        let c = body
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .unwrap();
        println!(
            "[dump] {label}: edge {k:?} faces=({fa:?},{fb:?}) verts=({start:?},{end:?}) carrier={:?} params={:?} description={:?}",
            c.carrier(),
            c.params(),
            c.description()
        );
    }
    for (k, v) in body.vertices() {
        let faces = faces_at(body, k);
        let mut surfaces: Vec<_> = faces
            .iter()
            .map(|f| body.get_face(*f).unwrap().surface)
            .collect();
        surfaces.sort();
        surfaces.dedup();
        println!(
            "[dump] {label}: vertex {k:?} point={:?} faces={faces:?} distinct_surfaces={surfaces:?}",
            body.get_point(v.point).unwrap()
        );
    }
    match topo::mass_properties(body, tol()) {
        Ok(p) => println!(
            "[dump] {label}: volume={:?} pad={:?} area={:?}",
            p.volume, p.volume_pad, p.surface_area
        ),
        Err(e) => println!("[dump] {label}: props Err {e:?}"),
    }
    println!(
        "[dump] {label}: tier3={:?}",
        topo::validate_geometric(body, tol())
    );
}

fn shelled(label: &str, body: &Body<f64>, t: f64) -> Option<Body<f64>> {
    match topo::shell(body, t, tol()) {
        Ok(s) => {
            dump(label, &s.body);
            Some(s.body)
        }
        Err(e) => {
            println!("[dump] {label}: shell Err {e}");
            None
        }
    }
}

/// Every chart moved inward by `t` through the simultaneous door.
fn hollow_moves(body: &Body<f64>, t: f64) -> Vec<topo::ChartMove<f64>> {
    let mut charts: Vec<(topo::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = body.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}

fn direct(label: &str, body: &Body<f64>, t: f64) {
    let mut cavity = body.clone();
    let band = Band::linear(tol()).expect("band");
    match topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, tol()) {
        Ok(()) => dump(label, &cavity),
        Err(e) => println!("[dump] {label}: direct door Err {e}"),
    }
}

fn revolved_about(
    lp: ProfileLoop<f64>,
    axis: RevolveAxis<f64>,
    turn: Revolution<f64>,
) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(&profile, axis, turn, tol())
        .expect("the meridian revolves")
        .body
}

fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    revolved_about(
        lp,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
    )
}

fn polyline(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    revolved(
        ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        ),
        turn,
    )
}

/// The bulge (`tan(θ/4)`) of the arc from `a` to `b` about `c`.
fn bulge(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> f64 {
    let (u, v) = (a - c, b - c);
    (u.perp_dot(v).atan2(u.dot(v)) / 4.0).tan()
}

// ---- torax_axial's fixtures ----

fn torus_barrel() -> Body<f64> {
    let c = p2(6.0 / 64.0, 1.0 / 16.0);
    let (lo, hi) = (p2(3.0 / 64.0, 0.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

fn torus_belly() -> Body<f64> {
    let c = p2(7.0 / 64.0, 5.0 / 64.0);
    let (lo, hi) = (p2(4.0 / 64.0, 1.0 / 64.0), p2(3.0 / 64.0, 8.0 / 64.0));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(4.0 / 64.0, 0.0), 0.0),
            ProfileVertex::new(lo, bulge(lo, hi, c)),
            ProfileVertex::new(hi, 0.0),
            ProfileVertex::new(p2(0.0, 8.0 / 64.0), 0.0),
        ]),
        Revolution::Full,
    )
}

fn lune(r: f64, turn: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), 0.0),
            ProfileVertex::new(p2(0.0, r), -1.0),
        ]),
        Revolution::Partial(turn),
    )
}

// ---- sf2b_axial's fixtures ----

fn sphere_zone_vase(r: f64, h: f64) -> Body<f64> {
    let c = p2(0.0, h / 2.0);
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), bulge(p2(r, 0.0), p2(r, h), c)),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    )
}

// ---- verbs_shell's klein elbow ----

fn circle_loop(r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(-r, 0.0), 1.0),
        ProfileVertex::new(p2(r, 0.0), 1.0),
    ])
}

fn klein_elbow(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the elbow's cross-section validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(1.20, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        Revolution::Partial(-FRAC_PI_2),
        tol(),
    )
    .expect("the elbow revolves")
    .body
}

// ---- the tour's torus vessels, their meridian spelled with a bulge ----

fn torus_vessel(centre_rho: f64) -> Body<f64> {
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth, h_tube) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0, 8.0 / 64.0);
    let (a, b) = (p2(r_band, y_foot), p2(r_band, y_shoulder));
    revolved(
        RawLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r_foot, 0.0), 0.0),
            ProfileVertex::new(p2(r_foot, y_foot), 0.0),
            ProfileVertex::new(a, bulge(a, b, p2(centre_rho, h_tube))),
            ProfileVertex::new(b, 0.0),
            ProfileVertex::new(p2(r_neck, y_shoulder), 0.0),
            ProfileVertex::new(p2(r_neck, y_mouth), 0.0),
            ProfileVertex::new(p2(0.0, y_mouth), 0.0),
        ]),
        Revolution::Full,
    )
}

// ---- the tube door's tori ----

fn tube_torus(wall: Option<f64>) -> Body<f64> {
    let (c, a, u) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_y(), Vec3::unit_x());
    match wall {
        None => {
            tube_along_arc::<f64>(c, a, u, 2.0, TubeWindow::Full, 0.5, tol())
                .expect("the solid torus builds")
                .body
        }
        Some(w) => {
            tube_along_arc_hollow::<f64>(c, a, u, 2.0, TubeWindow::Full, 0.5, w, tol())
                .expect("the hollow torus builds")
                .body
        }
    }
}

/// The one same-surface generator line at `u_ref`, split at its midpoint.
fn split_seam(body: &mut Body<f64>, on_axis: bool) {
    let seam = body
        .edges()
        .find(|(e, data)| {
            let key = |he| body.get_face(face_of_he(body, he)).unwrap().surface;
            let same = key(data.he_plus) == key(data.he_minus);
            let c = body
                .get_curve_geom(body.get_edge(*e).unwrap().curve)
                .and_then(|g| g.certified())
                .unwrap();
            matches!(c.carrier(), Curve3::Line { origin, dir }
                if dir.y.abs() > 0.5 && (if on_axis { origin.x.abs() <= 1e-15 && origin.z.abs() <= 1e-15 } else { same && origin.x > 0.0 }))
        })
        .map(|(e, _)| e)
        .expect("the seam");
    let c = body
        .get_curve_geom(body.get_edge(seam).unwrap().curve)
        .and_then(|g| g.certified())
        .unwrap();
    let (t0, t1) = c.params();
    body.split_edge(seam, (t0 + t1) * 0.5, tol()).unwrap();
}

/// The corpus, and the unit's own tori: operands and their shells.
#[test]
fn shell7_dump_corpus() {
    let t128 = 1.0 / 128.0;
    // torax_axial
    shelled("torus barrel", &torus_barrel(), t128);
    shelled(
        "torus belly (the teapot's wall-1 belly)",
        &torus_belly(),
        t128,
    );
    direct("sphere lune, direct door", &lune(0.3, FRAC_PI_2), 0.05);
    // verbs_shell
    shelled(
        "klein elbow disc",
        &klein_elbow(vec![circle_loop(0.25)]),
        0.05,
    );
    shelled(
        "klein elbow by hand",
        &klein_elbow(vec![circle_loop(0.25 + 0.025), circle_loop(0.25 - 0.025)]),
        0.01,
    );
    let vessel = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)],
        Revolution::Full,
    );
    if let Some(hollow) = shelled("vessel", &vessel, 0.2) {
        shelled("hollow vessel shelled again", &hollow, 0.05);
    }
    shelled(
        "tube",
        &polyline(
            &[(0.6, 0.0), (1.0, 0.0), (1.0, 2.0), (0.6, 2.0)],
            Revolution::Full,
        ),
        0.1,
    );
    // sf2b_axial
    let (r, h) = (3.0 / 64.0, 8.0 / 64.0);
    shelled("sphere-zone vase", &sphere_zone_vase(r, h), t128);
    shelled(
        "cone frustum",
        &polyline(
            &[(0.0, 0.0), (4.0 / 64.0, 0.0), (2.0 / 64.0, h), (0.0, h)],
            Revolution::Full,
        ),
        t128,
    );
    let rect = [(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)];
    shelled(
        "wedge",
        &polyline(&rect, Revolution::Partial(FRAC_PI_2)),
        t128,
    );
    shelled("drum", &polyline(&rect, Revolution::Full), t128);
    // the tour's torus vessels
    shelled("torus vessel, bellied", &torus_vessel(6.0 / 64.0), t128);
    shelled("torus vessel, waisted", &torus_vessel(12.0 / 64.0), t128);
    // the unit's own
    let solid = tube_torus(None);
    dump("tube torus solid, operand", &solid);
    shelled("tube torus solid", &solid, 0.05);
    let hollow = tube_torus(Some(0.125));
    dump("tube torus hollow, operand", &hollow);
    shelled("tube torus hollow", &hollow, 0.05);
    for v in [0.0, PI / 2.0] {
        let (s, c) = v.sin_cos();
        let a = p2(2.0 + 0.5 * c, 0.5 * s);
        let b = p2(2.0 - 0.5 * c, -0.5 * s);
        let body = revolved(
            RawLoop::new(vec![ProfileVertex::new(a, 1.0), ProfileVertex::new(b, 1.0)]),
            Revolution::Full,
        );
        shelled(&format!("revolved torus, v = {v}"), &body, 0.05);
    }
    let mut wedge = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)],
        Revolution::Partial(FRAC_PI_2),
    );
    split_seam(&mut wedge, true);
    shelled("wedge, axis edge split", &wedge, 0.05);
    let mut drum = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)],
        Revolution::Full,
    );
    split_seam(&mut drum, false);
    println!(
        "[dump] drum, seam split, operand: tier3={:?}",
        topo::validate_geometric(&drum, tol())
    );
    match topo::shell(&drum, 0.05, tol()) {
        Ok(s) => dump("drum, seam split, unminted", &s.body),
        Err(topo::ShellError::NotValid { errors }) => {
            println!("[dump] drum, seam split, unminted: shell NotValid {errors:?}");
        }
        Err(e) => println!("[dump] drum, seam split, unminted: shell Err {e}"),
    }
    topo::mint_pcurves(&mut drum, tol()).expect("the split operand's pcurves mint");
    println!(
        "[dump] drum, seam split, operand minted: tier3={:?}",
        topo::validate_geometric(&drum, tol())
    );
    match topo::shell(&drum, 0.05, tol()) {
        Ok(s) => dump("drum, seam split", &s.body),
        Err(topo::ShellError::NotValid { errors }) => {
            println!("[dump] drum, seam split: shell NotValid {errors:?}");
        }
        Err(e) => println!("[dump] drum, seam split: shell Err {e}"),
    }
    direct("drum, seam split, direct door", &drum, 0.05);
    // The seam-posture class past the torus: the two shapes that pass
    // the door and stop later, measured.
    let cap = polyline(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.5, 2.0), (0.0, 2.0)],
        Revolution::Full,
    );
    dump("collinear cap, operand", &cap);
    match topo::shell(&cap, 0.05, tol()) {
        Ok(s) => dump("collinear cap", &s.body),
        Err(topo::ShellError::NotValid { errors }) => {
            println!("[dump] collinear cap: shell NotValid {errors:?}")
        }
        Err(e) => println!("[dump] collinear cap: shell Err {e}"),
    }
    direct("collinear cap, direct door", &cap, 0.05);
    let v = PI / 4.0;
    let (sn, cs) = v.sin_cos();
    let ball = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(cs, sn), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]),
        Revolution::Full,
    );
    dump("two-arc sphere, operand", &ball);
    match topo::shell(&ball, 0.05, tol()) {
        Ok(s) => dump("two-arc sphere", &s.body),
        Err(topo::ShellError::NotValid { errors }) => {
            println!("[dump] two-arc sphere: shell NotValid {errors:?}")
        }
        Err(e) => println!("[dump] two-arc sphere: shell Err {e}"),
    }
    direct("two-arc sphere, direct door", &ball, 0.05);
}
