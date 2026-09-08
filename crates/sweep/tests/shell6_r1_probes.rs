//! R1 review probes for SHELL-6 (PR #2178). Print-first, assert where
//! the answer is already known. Lane-private.
//!
//! Rows: the consumer's-seat end-to-end (both frustums through
//! `topo::shell`, then the per-chart door on the same faces); the three
//! apex-window cases the brief names (a chart straddling the apex, a
//! chart carrying faces on BOTH nappes, a face whose corner SUM is
//! positive while some corners are negative); and an adversarial
//! reachability attack on the per-chart door — a cone whose two rims
//! stand on spheres chosen so that the transported rims land exactly on
//! their carriers, which is the one neighbour a single-chart cone offset
//! does not push its rim off.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use geom::Surface;
use geom_brep::Nappe;
use geom_core::{Band, Point2, Point3, Tol, Vec2};
use profile::{ArcSweep, Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, bulge_from_center};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ReplaceFaceError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const T: f64 = 1.0 / 128.0;
const H: f64 = 8.0 / 64.0;
const R_WIDE: f64 = 4.0 / 64.0;
const R_NARROW: f64 = 2.0 / 64.0;

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

fn frustum(r0: f64, r1: f64) -> Body<f64> {
    revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r0, 0.0), 0.0),
        ProfileVertex::new(p2(r1, H), 0.0),
        ProfileVertex::new(p2(0.0, H), 0.0),
    ]))
}

fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .collect()
}

fn chart_moves(body: &Body<f64>, d: f64) -> Vec<topo::ChartMove<f64>> {
    let mut moves: Vec<topo::ChartMove<f64>> = Vec::new();
    for (k, f) in body.faces() {
        match moves
            .iter_mut()
            .find(|m| body.get_face(m.faces[0]).unwrap().surface == f.surface)
        {
            Some(m) => m.faces.push(k),
            None => moves.push(topo::ChartMove {
                faces: vec![k],
                distance: d,
            }),
        }
    }
    moves
}

fn cone_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

fn corners(body: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let data = body.get_face(face).unwrap();
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let v = body.get_half_edge(he).unwrap().start;
            out.push(*body.get_point(body.get_vertex(v).unwrap().point).unwrap());
        }
    }
    out
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).expect("props").volume
}

fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// The cavity a frustum hollowed by `t` encloses, re-derived: the caps
/// move in by `t`, and at each moved cap station the cavity's radius is
/// the operand's radius there pulled in by `t / cos α` (the horizontal
/// width of a wall of perpendicular thickness `t`).
fn cavity_closed_form(r0: f64, r1: f64, h: f64, t: f64) -> f64 {
    let alpha = ((r0 - r1).abs() / h).atan();
    let at = |y: f64| r0 + (r1 - r0) * y / h;
    let c0 = at(t) - t / alpha.cos();
    let c1 = at(h - t) - t / alpha.cos();
    frustum_volume(c0, c1, h - 2.0 * t)
}

fn name(e: &ReplaceFaceError<f64>) -> String {
    let s = format!("{e:?}");
    s.split(|c: char| !c.is_alphanumeric()).next().unwrap().to_string()
}

/// **E2E, the consumer's seat.** Build a frustum below its apex and one
/// above it, hollow each with `topo::shell`, read the cavity against
/// the closed form; then ask the per-chart door for the same faces.
#[test]
fn r1_e2e_hollow_both_frustums_from_the_consumers_seat() {
    for (what, r0, r1, want_nappe) in [
        ("narrowing upward (below apex)", R_WIDE, R_NARROW, Nappe::Mirror),
        ("widening upward (above apex)", R_NARROW, R_WIDE, Nappe::Opening),
    ] {
        let body = frustum(r0, r1);
        let group = cone_faces(&body);
        for &f in &group {
            assert_eq!(topo::face_nappe(&body, f, band()).unwrap(), want_nappe, "{what}");
        }
        let v0 = volume(&body);
        assert!((v0 - frustum_volume(r0, r1, H)).abs() <= 1e-15, "{what}: operand volume");
        let hollow = topo::shell(&body, T, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: shell refused: {e}"))
            .body;
        assert_eq!(topo::validate_geometric(&hollow, Tol::witness()), Ok(()), "{what}: tier 3");
        let cavity = v0 - volume(&hollow);
        let want = cavity_closed_form(r0, r1, H, T);
        println!("[r1] {what}: operand {v0}, cavity {cavity}, closed form {want}, delta {}", cavity - want);
        assert!((cavity - want).abs() <= 1e-12, "{what}: cavity {cavity} vs {want}");

        // A thick request: what does a user get when the wall would
        // reach its own apex?
        for t in [0.03, 0.04] {
            match topo::shell(&body, t, Tol::witness()) {
                Ok(s) => println!(
                    "[r1] {what} t={t}: shell BUILT, cavity {} (closed form {})",
                    v0 - volume(&s.body),
                    cavity_closed_form(r0, r1, H, t)
                ),
                Err(e) => println!("[r1] {what} t={t}: shell refused: {e}"),
            }
        }

        // The per-chart door on the same faces, both signs.
        for d in [-T, T] {
            let mut work = body.clone();
            let got = topo::replace_faces_offset(&mut work, &group, d, band(), Tol::witness());
            match &got {
                Ok(()) => println!("[r1] {what} per-chart d={d}: BUILT, volume {}", volume(&work)),
                Err(e) => println!("[r1] {what} per-chart d={d}: {}: {e}", name(e)),
            }
            assert!(matches!(got, Err(ReplaceFaceError::ReanchorOffCarrier { .. })), "{what} d={d}: {got:?}");
        }
        // A single band of the two-band chart: the door names the sharer.
        let mut work = body.clone();
        let got = topo::replace_face_offset(&mut work, group[0], -T, band(), Tol::witness());
        println!("[r1] {what} single band: {}", got.as_ref().err().map(name).unwrap_or_default());
        assert!(matches!(got, Err(ReplaceFaceError::SharedSurfaceKey { .. })));
    }
}

/// Re-attach a body's cone chart to the same cone with its apex moved
/// to `apex_y` on the axis; every face of the old chart shares the key.
fn reanchor_cone(body: &mut Body<f64>, group: &[FaceKey], apex_y: f64) -> Surface<f64> {
    let Surface::Cone {
        axis,
        half_angle,
        u_ref,
        ..
    } = cone_of(body, group[0])
    else {
        panic!("a cone chart");
    };
    let surface = Surface::Cone {
        apex: Point3::new(0.0, apex_y, 0.0),
        axis,
        half_angle,
        u_ref,
    };
    let key = body
        .set_face_surface(group[0], topo::FaceSurface::New(surface.clone()))
        .expect("re-anchor");
    for &f in &group[1..] {
        body.set_face_surface(f, topo::FaceSurface::Shared(key)).expect("share");
    }
    surface
}

/// **A face whose corner SUM is positive while some corners are
/// negative.** The narrowing frustum's wall re-anchored to a cone whose
/// apex sits a quarter of the way up: per band two corners at station
/// `−H/4` and two at `+3H/4`, sum `+H` — the home says `Opening` — while
/// the window's low end is below the apex. Both doors, and the variant.
#[test]
fn r1_sum_positive_with_negative_corners() {
    let mut body = frustum(R_WIDE, R_NARROW);
    let group = cone_faces(&body);
    reanchor_cone(&mut body, &group, H / 4.0);
    let apex = Point3::new(0.0, H / 4.0, 0.0);
    for &f in &group {
        let st: Vec<f64> = corners(&body, f).iter().map(|p| p.y - apex.y).collect();
        let sum: f64 = st.iter().sum();
        let nappe = topo::face_nappe(&body, f, band());
        println!("[r1] stations {st:?} sum {sum} -> {nappe:?}");
        assert!(st.iter().any(|s| *s < 0.0) && sum > 0.0);
        assert_eq!(nappe.unwrap(), Nappe::Opening);
    }
    for d in [-T, T] {
        let mut work = body.clone();
        let got = topo::replace_faces_offset(&mut work, &group, d, band(), Tol::witness());
        println!("[r1] per-chart d={d}: {}", got.as_ref().err().map(name).unwrap_or("BUILT".into()));
        assert!(matches!(got, Err(ReplaceFaceError::ApexWindow { .. })), "{got:?}");
        let mut work = body.clone();
        let moves = chart_moves(&work, d);
        let got = topo::offset_charts_together(&mut work, &moves, band(), Tol::witness());
        println!("[r1] axial d={d}: {}", got.as_ref().err().map(name).unwrap_or("BUILT".into()));
    }
}

/// **A chart carrying faces on BOTH nappes** (a double cone stored as
/// one surface). A bi-cone (widening then narrowing, equal half-angles)
/// has two cone charts; re-anchoring all four bands to one cone whose
/// apex is the kink makes the lower bands `Mirror` and the upper ones
/// `Opening`. The per-chart door in both group orders, and the axial
/// door, which has no group-level gate of its own.
#[test]
fn r1_a_chart_on_both_nappes() {
    let mut body = revolved(ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(R_NARROW, 0.0), 0.0),
        ProfileVertex::new(p2(R_WIDE, H), 0.0),
        ProfileVertex::new(p2(R_NARROW, 2.0 * H), 0.0),
        ProfileVertex::new(p2(0.0, 2.0 * H), 0.0),
    ]));
    let group = cone_faces(&body);
    assert_eq!(group.len(), 4, "two cone charts, two bands each");
    reanchor_cone(&mut body, &group, H);
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    for &f in &group {
        match topo::face_nappe(&body, f, band()).unwrap() {
            Nappe::Mirror => lower.push(f),
            Nappe::Opening => upper.push(f),
        }
    }
    assert_eq!((lower.len(), upper.len()), (2, 2));
    for (order, faces) in [
        ("mirror first", [lower.clone(), upper.clone()].concat()),
        ("opening first", [upper.clone(), lower.clone()].concat()),
    ] {
        for d in [-T, T] {
            let mut work = body.clone();
            let got = topo::replace_faces_offset(&mut work, &faces, d, band(), Tol::witness());
            println!("[r1] both nappes, {order}, per-chart d={d}: {}", got.as_ref().err().map(name).unwrap_or("BUILT".into()));
            assert!(matches!(got, Err(ReplaceFaceError::ApexWindow { .. })), "{got:?}");
        }
    }
    for d in [-T, T] {
        let mut work = body.clone();
        let moves = chart_moves(&work, d);
        let got = topo::offset_charts_together(&mut work, &moves, band(), Tol::witness());
        match &got {
            Ok(()) => {
                let cones: Vec<_> = cone_faces(&work).iter().map(|&f| cone_of(&work, f)).collect();
                println!("[r1] both nappes, axial d={d}: BUILT; cone surfaces {cones:?}; tier3 {:?}", topo::validate_geometric(&work, Tol::witness()));
            }
            Err(e) => println!("[r1] both nappes, axial d={d}: {}: {e}", name(e)),
        }
    }
}

/// A cone from `(r0, z0)` to `(r1, z1)` whose two rims stand on spheres
/// centred on the axis, each chosen so that the rim's image under the
/// cone's offset action at MINT distance `d_mint` lies on the same
/// sphere — the one unmoved neighbour the per-chart door's re-anchor
/// gate cannot refuse. Returns the body and the cone's apex height.
fn sphere_capped_cone(r0: f64, z0: f64, r1: f64, z1: f64, d_mint: f64) -> (Body<f64>, f64) {
    let tan_a = (r1 - r0) / (z1 - z0);
    let apex_y = z0 - r0 / tan_a;
    let alpha = tan_a.abs().atan();
    let (sin_a, cos_a) = alpha.sin_cos();
    let image = |r: f64, z: f64| {
        let v = (z - apex_y) / cos_a;
        let v2 = v + d_mint * cos_a / sin_a;
        let apex2 = apex_y - d_mint / sin_a;
        (v2.abs() * sin_a, apex2 + v2 * cos_a, r)
    };
    let center = |r: f64, z: f64| {
        let (r2, z2, _) = image(r, z);
        (r2 * r2 + z2 * z2 - r * r - z * z) / (2.0 * (z2 - z))
    };
    let (c0, c1) = (center(r0, z0), center(r1, z1));
    let big0 = (r0 * r0 + (z0 - c0).powi(2)).sqrt();
    let big1 = (r1 * r1 + (z1 - c1).powi(2)).sqrt();
    let bottom = p2(0.0, c0 - big0);
    let top = p2(0.0, c1 + big1);
    println!(
        "[r1] sphere-capped cone: apex_y {apex_y}, images {:?} {:?}, spheres c0={c0} R0={big0} c1={c1} R1={big1}",
        image(r0, z0),
        image(r1, z1)
    );
    let body = revolved(RawLoop::new(vec![
        ProfileVertex::new(bottom, bulge_from_center(bottom, p2(r0, z0), p2(0.0, c0), ArcSweep::Ccw)),
        ProfileVertex::new(p2(r0, z0), 0.0),
        ProfileVertex::new(p2(r1, z1), bulge_from_center(p2(r1, z1), top, p2(0.0, c1), ArcSweep::Ccw)),
        ProfileVertex::new(top, 0.0),
    ]));
    (body, apex_y)
}

/// **Is the per-chart cone offset reachable?** On both nappes: a cone
/// between two spheres that hold its transported rims at exactly one
/// inward `d`. If the door BUILDS, the minted cone must be the home's
/// turn of the request and the body must shrink; the opposite sign, for
/// which the spheres were not chosen, is the control and must refuse.
#[test]
fn r1_per_chart_cone_offset_reachability_attack() {
    for (what, r0, r1, want_nappe) in [
        ("widening upward (above apex)", R_NARROW, R_WIDE, Nappe::Opening),
        ("narrowing upward (below apex)", R_WIDE, R_NARROW, Nappe::Mirror),
    ] {
        let d_door = -T; // inward at the face
        let d_mint = want_nappe.turn(d_door);
        let (body, _apex_y) = sphere_capped_cone(r0, 0.0, r1, H, d_mint);
        assert_eq!(topo::validate_geometric(&body, Tol::witness()), Ok(()), "{what}: the operand validates");
        let group = cone_faces(&body);
        assert!(!group.is_empty());
        for &f in &group {
            assert_eq!(topo::face_nappe(&body, f, band()).unwrap(), want_nappe, "{what}");
        }
        let old = cone_of(&body, group[0]);
        let v0 = volume(&body);
        let kinds: Vec<String> = body
            .faces()
            .map(|(_, f)| format!("{:?}", body.get_surface(f.surface)).split(|c: char| !c.is_alphanumeric()).nth(1).unwrap_or("?").to_string())
            .collect();
        println!("[r1] {what}: faces {kinds:?}");
        for (d, chosen) in [(d_door, true), (-d_door, false)] {
            let mut work = body.clone();
            let got = topo::replace_faces_offset(&mut work, &group, d, band(), Tol::witness());
            match &got {
                Ok(()) => {
                    let v1 = volume(&work);
                    let tier3 = topo::validate_geometric(&work, Tol::witness());
                    let minted = cone_of(&work, cone_faces(&work)[0]);
                    let want = geom_brep::offset_surface(&old, want_nappe.turn(d), band()).unwrap();
                    println!("[r1] {what} d={d} chosen={chosen}: BUILT volume {v0} -> {v1}, tier3 {tier3:?}");
                    println!("[r1]   minted {minted:?}\n[r1]   want   {want:?}");
                    assert!(chosen, "{what}: the control sign built");
                    assert_eq!(format!("{minted:?}"), format!("{want:?}"), "{what}: the door's mint is the home's turn");
                    assert!(v1 < v0, "{what}: an inward per-chart offset must shrink the body");
                    assert_eq!(tier3, Ok(()), "{what}: tier 3");
                }
                Err(e) => println!("[r1] {what} d={d} chosen={chosen}: {}: {e}", name(e)),
            }
        }
    }
}
