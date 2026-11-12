//! **Probe (ONEDOOR loop lift, PR 6851, probe c): loop-moving doors
//! between faces of opposite sense on one sphere payload.** A
//! measurement, not a product test.
//!
//! `Body::same_chart` answers from the surface KEY (or a shared spline
//! payload) and never reads `sense`, so a loop moved between two faces
//! on one key keeps its rows untouched. This builds, on one unit-sphere
//! key about `z`:
//!
//! - `F0` (sense `true`), the face `mvfs` charts;
//! - `F1`, the upper octant `u ∈ [0, π/2]`, `z > 0`, split off by `mef`
//!   with `FaceSurface::Shared { key, sense: false }` — its loop has a
//!   joint at the north pole;
//! - `Fx`, the lower octant, split off the same way (a joint at the
//!   south pole).
//!
//! Then it runs `kfmrh(F0, F1)` (F1's loop becomes a ring of F0, sense
//! `false` → `true`), `ring_move(ring, Fx)` (→ `false`), and
//! `mfkrh(ring, Shared { key, sense })`, printing at each step every
//! door's answer, the moved loop's stored elements at its pole joints,
//! `face_azimuth_window_traces`, and tier 3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::Curve3;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, LoopBoundary, LoopKey, face_azimuth_window_traces, validate_pcurves};

use crate::zz_probe_looplift_tilted_pole::{band, dump_body, unit_sphere};

/// The great circle through `from` and `to` (orthogonal unit vectors),
/// `eval(t) = cos t · from + sin t · to`.
fn great(from: Vec3<f64>, to: Vec3<f64>) -> Curve3<f64> {
    Curve3::circle(Point3::new(0.0, 0.0, 0.0), from.cross(to), 1.0, from)
}

fn arc(from: Vec3<f64>, to: Vec3<f64>) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(great(from, to), 0.0, FRAC_PI_2).unwrap()
}

fn pt(v: Vec3<f64>) -> Point3<f64> {
    Point3::new(v.x, v.y, v.z)
}

/// The moved loop's pole joints, as stored, with the face it sits on.
fn pole_joints(body: &Body<f64>, lk: LoopKey) -> String {
    let l = body.get_loop(lk).unwrap();
    let face = body.get_face(l.face).unwrap();
    let LoopBoundary::Cycle { first } = l.boundary else {
        return "not a cycle".into();
    };
    let mut out = format!("loop {lk:?} on face {:?} (sense {}):", l.face, face.sense);
    for he in body.loop_cycle(first).unwrap() {
        let h = body.get_half_edge(he).unwrap();
        let p = body
            .vertex_points()
            .find(|(v, _)| *v == h.start)
            .map(|(_, p)| p)
            .unwrap();
        if (p.z.abs() - 1.0).abs() < 1e-9 {
            out += &format!(
                " [{} joint into {he:?}: {:?}]",
                if p.z > 0.0 { "N" } else { "S" },
                body.joint(he)
            );
        }
    }
    out += &format!(
        " | window of its face: {:?} | tier 3: {:?}",
        face_azimuth_window_traces(body, l.face, band()),
        validate_pcurves(body, band())
    );
    out
}

fn shared(body: &Body<f64>, f: FaceKey, sense: bool) -> FaceSurface<f64> {
    FaceSurface::Shared {
        key: body.get_face(f).unwrap().surface,
        sense,
    }
}

#[test]
fn probe_loop_moves_between_opposite_sense_faces() {
    let tol = Tol::witness();
    let (z, x, y) = (
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let mut body = Body::<f64>::new();
    let start = body.mvfs(pt(z), true).unwrap();
    let f0 = start.face;
    body.set_face_surface(
        f0,
        FaceSurface::New {
            surface: unit_sphere(),
            sense: true,
        },
    )
    .unwrap();
    // N → A = +x, then a strut A → B = +y along the equator.
    let m1 = body
        .mev(
            MevSite::Lone {
                r#loop: start.r#loop,
            },
            pt(x),
            arc(z, x),
            tol,
        )
        .unwrap();
    let m2 = body
        .mev(
            MevSite::Fan {
                he1: m1.he_minus,
                he2: m1.he_minus,
            },
            pt(y),
            arc(x, y),
            tol,
        )
        .unwrap();
    // The chord B → N splits off F1 = (B → A → N → B), sense false.
    let spec = shared(&body, f0, false);
    let split = body.mef(
        MefSite::Chords {
            he1: m2.he_minus,
            he2: m1.he_plus,
        },
        arc(y, z),
        spec,
        tol,
    );
    println!("mef F1 with Shared {{ sense: false }}: {:?}", split.as_ref().map(|_| ()));
    let f1 = match split {
        Ok(c) => c.face,
        Err(e) => {
            println!("   refused: {e:?}; falling back to Inherit + set_face_sense");
            let c = body
                .mef(
                    MefSite::Chords {
                        he1: m2.he_minus,
                        he2: m1.he_plus,
                    },
                    arc(y, z),
                    FaceSurface::Inherit,
                    tol,
                )
                .unwrap();
            println!("   set_face_sense(F1, false): {:?}", body.set_face_sense(c.face, false));
            c.face
        }
    };
    // A strut A → S, then the chord S → B splits off Fx = the lower
    // octant, sense false.
    let mz = Vec3::new(0.0, 0.0, -1.0);
    let f0_outer = body.get_face(f0).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(f0_outer).unwrap().boundary else {
        panic!()
    };
    let cyc = body.loop_cycle(first).unwrap();
    let at = |body: &Body<f64>, p: Vec3<f64>| {
        cyc.iter()
            .copied()
            .find(|&he| {
                let s = body.get_half_edge(he).unwrap().start;
                body.vertex_points()
                    .any(|(v, q)| v == s && (q - pt(p)).norm() < 1e-12)
            })
            .unwrap()
    };
    let from_a = at(&body, x);
    let from_b = at(&body, y);
    let m3 = body
        .mev(
            MevSite::Fan {
                he1: from_a,
                he2: from_a,
            },
            pt(mz),
            arc(x, mz),
            tol,
        )
        .unwrap();
    let spec = shared(&body, f0, false);
    let split = body.mef(
        MefSite::Chords {
            he1: m3.he_minus,
            he2: from_b,
        },
        arc(mz, y),
        spec,
        tol,
    );
    println!("mef Fx with Shared {{ sense: false }}: {:?}", split.as_ref().map(|_| ()));
    let fx = split.unwrap().face;
    dump_body("built: F0 true, F1 false, Fx false", &body);
    let f1_loop = body.get_face(f1).unwrap().outer;
    println!("BEFORE kfmrh: {}", pole_joints(&body, f1_loop));

    // kfmrh(F0, F1): F1's loop becomes a ring of F0.
    let mut k = body.clone();
    let got = k.kfmrh(f0, f1);
    println!("kfmrh(F0 true, F1 false): {:?}", got.as_ref().map(|_| ()));
    if got.is_ok() {
        println!("AFTER kfmrh: {}", pole_joints(&k, f1_loop));
        let mut m = k.clone();
        println!(
            "   re-mint after kfmrh (mint_pcurves): {:?}",
            topo::mint_pcurves(&mut m, tol)
        );
        if m.get_loop(f1_loop).is_some() {
            println!("   RE-MINTED: {}", pole_joints(&m, f1_loop));
        }
        // ring_move(ring, Fx): back onto a sense-false face.
        let mut r = k.clone();
        let got = r.ring_move(f1_loop, fx);
        println!("ring_move(ring, Fx false): {:?}", got);
        if got.is_ok() {
            println!("AFTER ring_move: {}", pole_joints(&r, f1_loop));
        }
        // mfkrh(ring, Shared): promote it to a face of each sense.
        for sense in [false, true] {
            let mut p = k.clone();
            let spec = shared(&p, f0, sense);
            let got = p.mfkrh(f1_loop, spec);
            println!(
                "mfkrh(ring, Shared {{ sense: {sense} }}): {:?}",
                got.as_ref().map(|c| c.face)
            );
            if got.is_ok() {
                println!("AFTER mfkrh: {}", pole_joints(&p, f1_loop));
            }
        }
    }
    // A same-sense control: Fx's loop onto F1, both sense false.
    let mut k2 = body.clone();
    let got = k2.kfmrh(f1, fx);
    println!("kfmrh(F1 false, Fx false) (same sense, control): {:?}", got.as_ref().map(|_| ()));
    if got.is_ok() {
        let lk = body.get_face(fx).unwrap().outer;
        println!("AFTER control kfmrh: {}", pole_joints(&k2, lk));
    }
    // set_face_sense on a face holding a pole joint, for comparison.
    let mut s = body.clone();
    println!("set_face_sense(F1, true): {:?}", s.set_face_sense(f1, true));
    println!("AFTER set_face_sense: {}", pole_joints(&s, f1_loop));
}
