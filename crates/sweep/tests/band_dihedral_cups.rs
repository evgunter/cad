//! **A cup goes with a plane–plane band at any dihedral, and one whose
//! wall crosses what the band keeps refuses** (Ev's band ruling, PR
//! 4963: a convex band's result is `S − R`, a concave band's `S ∪ F`).
//! The fixtures are a wedge whose edge `(·, 1, 1)` has interior angle
//! `φ` between its top `z = 1` and a slant falling at `φ` below it, and
//! the mirror slot cut through a brick, its crease concave with a void of
//! `φ`; both at 30°, 60°, 120° and 135°. The section `A` is
//! `Verb::section_at(r, φ)`; a cup of radius `a` held `π·a²·h` of the
//! region's air (or fill) already, so `ΔV = ∓(A·L − π·a²·h)`.
//!
//! At an acute dihedral the fillet's arc runs past its extreme point
//! along a support's normal, so a line along that normal can leave the
//! section through the arc and come back in the cusp by the other
//! support: a cup square to its support whose floor lies below the
//! ball's centre can cross the disk the band keeps with every boundary
//! curve inside. Those refuse; cups above the centre build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, UnitVec3, Vec3};
use geom_core::{RigidMotion, Turns};
use profile::SketchPlane;
use sweep::blend::BlendError;
use sweep::test_support::{brick, prism_on};
use topo::{Body, EdgeKey};

use crate::band_concave_bar::{blend, boolean};
use crate::band_one_cycle_members::assert_taken;
use crate::band_planar_consume::VERBS;
use crate::band_planar_cut_off::{Verb, edge, tol, volume};
use crate::band_strip_mouth_cups::{post, post_faces};

/// The four dihedrals, in degrees.
const DIHEDRALS: [f64; 4] = [30.0, 60.0, 120.0, 135.0];

/// The band's size.
const R: f64 = 0.1;

/// The wedge's section `(0, 1), (1 − cos φ, 1 − sin φ), (1, 1)` in
/// `(y, z)`, run along `x ∈ [x0, x1]`.
fn wedge_along(phi: f64, (x0, x1): (f64, f64)) -> Body<f64> {
    let p = |y: f64, z: f64| (Point2::new(y, z), 0.0);
    prism_on(
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(x0, 0.0, 0.0))),
        vec![
            p(0.0, 1.0),
            p(1.0 - phi.cos(), 1.0 - phi.sin()),
            p(1.0, 1.0),
        ],
        x1 - x0,
        tol(),
    )
}

/// The convex wedge, `x ∈ [0, 1]`.
fn wedge(phi: f64) -> Body<f64> {
    wedge_along(phi, (0.0, 1.0))
}

/// The slot: a brick less the wedge's section over `x ∈ [0.2, 0.8]`,
/// opening through the brick's face `y = 0.3`, so its crease ends at
/// all-concave corners against the pocket's end walls.
fn slot(phi: f64) -> Body<f64> {
    boolean(
        false,
        brick((0.0, 1.0), (0.3, 2.5), (0.0, 1.5), tol()),
        wedge_along(phi, (0.2, 0.8)),
    )
}

fn wedge_edge(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0])
}

fn slot_crease(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.2, 1.0, 1.0], [0.8, 1.0, 1.0])
}

/// Which support a strip cup stands on.
#[derive(Clone, Copy, Debug)]
enum On {
    Top,
    Slant,
}

/// The mouth centre `t` from the edge at `x = ½` on `on`, and the unit
/// normal into the wedge's material (the slot's void).
fn mouth(phi: f64, on: On, t: f64) -> (Point3<f64>, Vec3<f64>) {
    let (c, s) = (phi.cos(), phi.sin());
    match on {
        On::Top => (Point3::new(0.5, 1.0 - t, 1.0), Vec3::new(0.0, 0.0, -1.0)),
        On::Slant => (
            Point3::new(0.5, 1.0 - t * c, 1.0 - t * s),
            Vec3::new(0.0, -s, c),
        ),
    }
}

/// A strip cup inside `verb`'s region with its floor on the support's
/// side of the fillet ball's centre: its distance from the edge, its
/// radius and its depth, half the least depth under its mouth of the
/// band's surface and of the other support. The isosceles section serves
/// either support.
fn strip_cup(verb: Verb, phi: f64) -> (f64, f64, f64) {
    let (t, a, surface) = match verb {
        Verb::Fillet => {
            // The trimline `s` from the edge; the ball's centre `R` under it.
            let s = R / (phi / 2.0).tan();
            let (t, a) = if s > R {
                (s - 0.5 * R, 0.1 * R)
            } else {
                (0.5 * s, 0.1 * s)
            };
            let off = s - t - a;
            (t, a, R - (R * R - off * off).sqrt())
        }
        Verb::Chamfer => {
            let (t, a) = (0.5 * R, 0.1 * R);
            (t, a, (R - t - a) * ((PI - phi) / 2.0).tan())
        }
    };
    let other = if phi < PI / 2.0 {
        (t - a) * phi.tan()
    } else {
        f64::INFINITY
    };
    (t, a, 0.5 * surface.min(other))
}

/// `verb` at `R` on `edges` refuses as `want` reads it.
fn refuses(
    verb: Verb,
    body: &Body<f64>,
    edges: &[EdgeKey],
    r: f64,
    want: impl Fn(&BlendError) -> bool,
    what: &str,
) {
    match blend(verb, body, edges, r) {
        Err(e) => assert!(want(&e), "{what} ({verb:?}): refused, but {e:?}"),
        Ok(out) => panic!(
            "{what} ({verb:?}): refuses, but built ΔV = {:e}",
            volume(&out.body) - volume(body)
        ),
    }
}

fn bounded(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { bounded: true, .. })
}

fn ring(e: &BlendError) -> bool {
    matches!(e, BlendError::RingClearance { .. })
}

fn crossing(e: &BlendError) -> bool {
    matches!(e, BlendError::FaceClearance { bounded: false, .. })
}

/// **A dimple and a boss on either support's strip go at every
/// dihedral under both verbs**, where the cup's floor lies on the
/// support's side of the fillet ball's centre.
#[test]
fn strip_cups_go_at_acute_and_obtuse_dihedrals() {
    for deg in DIHEDRALS {
        let phi = deg.to_radians();
        for verb in VERBS {
            let (t, a, h) = strip_cup(verb, phi);
            let held = PI * a * a * h;
            let area = verb.section_at(R, phi);
            for on in [On::Top, On::Slant] {
                let (o, dir) = mouth(phi, on, t);
                let cup = || post(o - dir * 0.05, dir, 0.05 + h, a);
                let what = format!("a dimple on the {on:?} at {deg}°");
                let body = boolean(false, wedge(phi), cup());
                let cups = post_faces(&body, (o, dir), a, (0.0, h));
                assert_eq!(cups.len(), 3, "{what}: two half-walls and a floor");
                let e = wedge_edge(&body);
                assert_taken(
                    verb,
                    &body,
                    &[e],
                    R,
                    (&cups, 1, 0),
                    (-(area - held), 1.0),
                    &what,
                );
                let what = format!("a boss on the {on:?} at {deg}°");
                let body = boolean(true, slot(phi), cup());
                let cups = post_faces(&body, (o, dir), a, (0.0, h));
                assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
                let e = slot_crease(&body);
                assert_taken(
                    verb,
                    &body,
                    &[e],
                    R,
                    (&cups, 1, 0),
                    (0.6 * area - held, 1.0),
                    &what,
                );
            }
        }
    }
}

/// **A cup whose floor lies past the fillet ball's centre at an acute
/// dihedral, its wall crossing the disk the band keeps, refuses as a
/// bound**, though every curve of its boundary lies inside the section:
/// at 30° a dimple `2.62·r` from the edge, `1.4·r` deep, seamed on the
/// edge's side, and the mirror boss in the slot (`r = 0.1`); its
/// control `0.5·r` deep builds. The chamfer, its section convex, takes
/// neither past its plane: the deep cup crosses it.
#[test]
fn an_acute_strip_cup_whose_wall_crosses_the_kept_disk_refuses_as_a_bound() {
    let phi = 30f64.to_radians();
    let a = 0.15 * R;
    let t = 2.62 * R;
    for (h, builds) in [(1.4 * R, false), (0.5 * R, true)] {
        let (o, dir) = mouth(phi, On::Top, t);
        let cup = || seamed_post(o - dir * 0.05, 0.05 + h, a);
        let held = PI * a * a * h;
        let area = Verb::Fillet.section_at(R, phi);
        let what = format!("a seamed dimple {h} deep at 30°");
        let body = boolean(false, wedge(phi), cup());
        let cups = post_faces(&body, (o, dir), a, (0.0, h));
        assert_eq!(cups.len(), 3, "{what}: the dimple's three faces");
        let e = wedge_edge(&body);
        if builds {
            let dv = -(area - held);
            assert_taken(
                Verb::Fillet,
                &body,
                &[e],
                R,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        } else {
            refuses(Verb::Fillet, &body, &[e], R, bounded, &what);
        }
        let what = format!("a seamed boss {h} deep at 30°");
        let body = boolean(true, slot(phi), cup());
        let cups = post_faces(&body, (o, dir), a, (0.0, h));
        assert_eq!(cups.len(), 3, "{what}: the boss's three faces");
        let e = slot_crease(&body);
        if builds {
            let dv = 0.6 * area - held;
            assert_taken(
                Verb::Fillet,
                &body,
                &[e],
                R,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        } else {
            refuses(Verb::Fillet, &body, &[e], R, bounded, &what);
        }
    }
}

/// A round post of radius `a` from `from` along `−z` for `len`, its wall
/// split into two faces at section angles `240°` and `300°`: its seams
/// lie on the `+y` side, towards the wedge's edge.
fn seamed_post(from: Point3<f64>, len: f64, a: f64) -> Body<f64> {
    let at = |deg: f64| {
        let t = deg.to_radians();
        Point2::new(a * t.cos(), a * t.sin())
    };
    prism_on(
        SketchPlane::from_frame(RigidMotion::turned(
            from,
            UnitVec3::exact_x(),
            Turns::quarters(2),
        )),
        vec![
            (at(240.0), 15f64.to_radians().tan()),
            (at(300.0), 75f64.to_radians().tan()),
        ],
        len,
        tol(),
    )
}

/// The equilateral wedge of side 3, its apex edge `(·, 0, 0)` a 60°
/// dihedral between its top `z = 0` and the slant `z = √3·y`.
fn equilateral(len: f64) -> Body<f64> {
    let s3 = 3f64.sqrt();
    let p = |y: f64, z: f64| (Point2::new(y, z), 0.0);
    prism_on(
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(0.0, 0.0, 0.0))),
        vec![p(0.0, 0.0), p(-3.0, 0.0), p(-1.5, -1.5 * s3)],
        len,
        tol(),
    )
}

/// The 60° groove: a brick less the prism over `(0, 0), (−√3, 3),
/// (−4, 3), (−4, 0)` run `x ∈ [0, 1]`, open at the top, its crease
/// `(·, 0, 0)` concave between the floor `z = 0` and the wall
/// `z = −√3·y`.
fn groove() -> Body<f64> {
    let p = |y: f64, z: f64| (Point2::new(y, z), 0.0);
    let void = prism_on(
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(0.0, 0.0, 0.0))),
        vec![
            p(0.0, 0.0),
            p(-(3f64.sqrt()), 3.0),
            p(-4.0, 3.0),
            p(-4.0, 0.0),
        ],
        1.0,
        tol(),
    );
    boolean(
        false,
        brick((-1.0, 2.0), (-5.0, 2.0), (-2.0, 2.5), tol()),
        void,
    )
}

/// **At 60° with `r = 1`, a round dimple whose floor lies in the cusp
/// by the slant refuses as a bound, and the mirror boss in the groove
/// too**: the ball's centre is `1` under the top, and a cup `0.025`
/// round at `y = −0.725` reaching `1.2` through it crosses the disk the
/// band keeps with its seams clear of the ball; the same cup `0.2` deep
/// builds. A square dimple reaching as deep has its corner edges through
/// the ball and refuses as a crossing; `0.2` deep it builds.
#[test]
fn a_cup_reaching_into_the_cusp_at_60_degrees_refuses_and_a_shallow_one_goes() {
    let r = 1.0;
    let phi = 60f64.to_radians();
    let area = Verb::Fillet.section_at(r, phi);
    let (o, a) = (Point3::new(0.5, -0.725, 0.0), 0.025);
    let down = Vec3::new(0.0, 0.0, -1.0);
    let apex = |b: &Body<f64>| edge(b, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    for (h, builds) in [(0.2, true), (1.2, false)] {
        let what = format!("a round dimple {h} deep at 60°");
        let body = boolean(
            false,
            equilateral(1.0),
            post(o - down * 0.2, down, 0.2 + h, a),
        );
        if builds {
            let cups = post_faces(&body, (o, down), a, (0.0, h));
            let dv = -(area - PI * a * a * h);
            assert_taken(
                Verb::Fillet,
                &body,
                &[apex(&body)],
                r,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        } else {
            refuses(Verb::Fillet, &body, &[apex(&body)], r, bounded, &what);
        }
        let what = format!("a round boss {h} tall at 60°");
        let up = -down;
        let body = boolean(true, groove(), post(o - up * 0.2, up, 0.2 + h, a));
        if builds {
            let cups = post_faces(&body, (o, up), a, (0.0, h));
            let dv = area - PI * a * a * h;
            assert_taken(
                Verb::Fillet,
                &body,
                &[apex(&body)],
                r,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        } else {
            refuses(Verb::Fillet, &body, &[apex(&body)], r, bounded, &what);
        }
        let h = if builds { h } else { 1.3 };
        let what = format!("a square dimple {h} deep at 60°");
        let (y, b) = (-0.764, 0.011);
        let square = brick((0.5 - b, 0.5 + b), (y - b, y + b), (-h, 0.2), tol());
        let body = boolean(false, equilateral(1.0), square);
        if builds {
            let cups = post_faces(&body, (Point3::new(0.5, y, 0.0), down), 2.0 * b, (0.0, h));
            assert_eq!(cups.len(), 5, "{what}: four walls and a floor");
            let dv = -(area - 4.0 * b * b * h);
            assert_taken(
                Verb::Fillet,
                &body,
                &[apex(&body)],
                r,
                (&cups, 1, 0),
                (dv, 1.0),
                &what,
            );
        } else {
            refuses(Verb::Fillet, &body, &[apex(&body)], r, crossing, &what);
        }
    }
}

/// **A round dimple whose seams run through the disk the band keeps
/// refuses as a crossing** at 30°: `2.743·r` from the edge, `1.488·r`
/// deep, its boundary itself crossing the arc.
#[test]
fn an_acute_round_dimple_with_its_seams_through_the_kept_disk_crosses() {
    let phi = 30f64.to_radians();
    let (t, h, a) = (2.743 * R, 1.488 * R, 0.004);
    let (o, dir) = mouth(phi, On::Top, t);
    let body = boolean(false, wedge(phi), post(o - dir * 0.05, dir, 0.05 + h, a));
    let e = wedge_edge(&body);
    refuses(
        Verb::Fillet,
        &body,
        &[e],
        R,
        crossing,
        "a round dimple through the disk at 30°",
    );
}

/// **A strip dimple whose mouth is requested with the band's edge
/// refuses**: the two bands would overlap, so the fillet's screen reads
/// the ring (`FaceClearanceUncertified`) and the chamfer's arm refuses
/// the mouth's circle (`ChamferArmUnsupported`).
#[test]
fn a_requested_mouth_on_the_strip_refuses() {
    let (u, a, h) = (0.04, 0.01, 0.03);
    let o = Point3::new(0.5, 1.0 - u, 1.0);
    let dir = Vec3::new(0.0, 0.0, -1.0);
    let body = boolean(
        false,
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        post(o - dir * 0.2, dir, 0.2 + h, a),
    );
    let point = |v| {
        *body
            .get_point(body.get_vertex(v).expect("v").point)
            .expect("p")
    };
    let mut edges = vec![edge(&body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0])];
    edges.extend(topo::query::all_edges(&body).into_iter().filter(|&e| {
        let he = body.get_edge(e).expect("an edge").he_plus;
        let p: Point3<f64> = point(body.get_half_edge(he).expect("a half").start);
        let q: Point3<f64> = point(body.half_edge_end(he).expect("an end"));
        (p.z - 1.0).abs() < 1e-12 && (q.z - 1.0).abs() < 1e-12 && ((p - o).norm() - a).abs() < 1e-9
    }));
    assert!(edges.len() > 1, "the mouth's edges");
    let what = "a requested mouth on the strip";
    refuses(
        Verb::Fillet,
        &body,
        &edges,
        0.1,
        |e| matches!(e, BlendError::FaceClearanceUncertified { .. }),
        what,
    );
    refuses(
        Verb::Chamfer,
        &body,
        &edges,
        0.1,
        |e| matches!(e, BlendError::ChamferArmUnsupported { .. }),
        what,
    );
}

/// **A blind bore from a cut-off's cap into a convex wedge, and a pin
/// from a cap into a concave slot's fill, go at every dihedral under
/// both verbs**: a cup along the spine, on the section's bisector
/// halfway from the edge to the band's surface, `0.4` deep (a bore) or
/// `0.2` long (a pin).
#[test]
fn cap_cups_go_at_acute_and_obtuse_dihedrals() {
    for deg in DIHEDRALS {
        let phi = deg.to_radians();
        // Into the section from the edge `(1, 1)` in `(y, z)`, halfway
        // between the supports.
        let (c, s) = (phi.cos(), phi.sin());
        let bis = Vec3::new(0.0, -1.0 - c, -s).normalize();
        for verb in VERBS {
            // How far the band's surface stands from the edge along the
            // bisector.
            let surface = match verb {
                Verb::Fillet => R / (phi / 2.0).sin() - R,
                Verb::Chamfer => R * (phi / 2.0).cos(),
            };
            let d = 0.5 * surface;
            let a = 0.3 * (d * (phi / 2.0).sin()).min(surface - d);
            let at = Point3::new(0.0, 1.0, 1.0) + bis * d;
            let along = Vec3::new(1.0, 0.0, 0.0);
            let area = verb.section_at(R, phi);
            let h = 0.4;
            let what = format!("a blind bore from the cap at {deg}°");
            let o = Point3::new(0.0, at.y, at.z);
            let body = boolean(
                false,
                wedge(phi),
                post(o - along * 0.05, along, 0.05 + h, a),
            );
            let cups = post_faces(&body, (o, along), a, (0.0, h));
            assert_eq!(cups.len(), 3, "{what}: two half-walls and a floor");
            let e = wedge_edge(&body);
            let dv = -(area - PI * a * a * h);
            assert_taken(verb, &body, &[e], R, (&cups, 1, 0), (dv, 1.0), &what);
            let h = 0.2;
            let what = format!("a pin from the slot's cap at {deg}°");
            let o = Point3::new(0.2, at.y, at.z);
            let body = boolean(true, slot(phi), post(o - along * 0.05, along, 0.05 + h, a));
            let cups = post_faces(&body, (o, along), a, (0.0, h));
            assert_eq!(cups.len(), 3, "{what}: two half-walls and an end");
            let e = slot_crease(&body);
            let dv = 0.6 * area - PI * a * a * h;
            assert_taken(verb, &body, &[e], R, (&cups, 1, 0), (dv, 1.0), &what);
        }
    }
}

/// **A cap cup whose mouth straddles the band's surface refuses at
/// every dihedral under both verbs**, at the ring pass: the bore of
/// [`cap_cups_go_at_acute_and_obtuse_dihedrals`] centred on the surface
/// itself, and the pin likewise.
#[test]
fn a_cap_cup_straddling_the_band_surface_refuses_at_every_dihedral() {
    for deg in DIHEDRALS {
        let phi = deg.to_radians();
        let (c, s) = (phi.cos(), phi.sin());
        let bis = Vec3::new(0.0, -1.0 - c, -s).normalize();
        for verb in VERBS {
            let surface = match verb {
                Verb::Fillet => R / (phi / 2.0).sin() - R,
                Verb::Chamfer => R * (phi / 2.0).cos(),
            };
            let a = 0.15 * surface;
            let at = Point3::new(0.0, 1.0, 1.0) + bis * surface;
            let along = Vec3::new(1.0, 0.0, 0.0);
            let what = format!("a bore straddling the band's surface at {deg}°");
            let o = Point3::new(0.0, at.y, at.z);
            let body = boolean(false, wedge(phi), post(o - along * 0.05, along, 0.45, a));
            let e = wedge_edge(&body);
            refuses(verb, &body, &[e], R, ring, &what);
            let what = format!("a pin straddling the band's surface at {deg}°");
            let o = Point3::new(0.2, at.y, at.z);
            let body = boolean(true, slot(phi), post(o - along * 0.05, along, 0.25, a));
            let e = slot_crease(&body);
            refuses(verb, &body, &[e], R, ring, &what);
        }
    }
}
