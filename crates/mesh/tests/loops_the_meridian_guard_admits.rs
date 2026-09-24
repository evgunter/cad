//! **Degenerate loops that DO carry a meridian**, so
//! `walk::require_a_meridian` admits them, measured for what
//! `tessellate` then answers. They are the known edge of that guard:
//! it asks whether a meridian traversal exists, and these loops have
//! one.
//!
//! The zero-width loops that used to be measured here — meridians only,
//! all on one column — are refused typed by the walk's other premise, on
//! the EDGES their iso sides open
//! ([`mesh::TessellateError::SingleColumnCurvedFace`]), and their rows
//! live with that guard's class in `loops_with_no_rim.rs`. What is left
//! here is the one shape the meridian guard admits and nothing else
//! refuses structurally:
//!
//! * **A rim-only cap wearing a spur** — a meridian strut from the rim
//!   toward the pole, walked up and back. A spur that reaches the pole
//!   is the one-face seamed statement of the cap, and meshes; one that
//!   stops short leaves the pole interior — the state the guard
//!   refuses, in disguise — and is caught only by the deviation
//!   certificate.
//!
//! Every outcome here is read where debug assertions run, which is
//! every profile this workspace builds; the census is a `debug_assert`,
//! so the rows are gated on it.
#![cfg(debug_assertions)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::witness_bodies::one_circle_cut;
use core::f64::consts::FRAC_PI_2;
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, SurfaceKind};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The same great circle by latitude: `eval(t) = (cos t, 0, sin t)`.
fn meridian_by_latitude() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// What `tessellate` answers for `body`, reduced to the part that is a
/// disposition: the refusal's variant, the panic's opening words, or —
/// for a mesh — whether the first face on a `kind` surface emitted
/// anything and which `check_mesh` verdict the whole mesh gets. Payload
/// numbers are dropped on purpose: they move with δ and ε, and the rows
/// pin dispositions.
fn disposition(body: &Body<f64>, kind: SurfaceKind, delta: f64) -> String {
    let tol = Tol::witness();
    let answered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, tol)
    }));
    let word = |debug: String| {
        debug
            .split([' ', '(', '{'])
            .next()
            .unwrap_or_default()
            .to_owned()
    };
    match answered {
        Err(payload) => {
            let said = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_default();
            format!("panic: {}", said.chars().take(13).collect::<String>())
        }
        Ok(Err(refusal)) => format!("refused: {}", word(format!("{refusal:?}"))),
        Ok(Ok(mesh)) => {
            let emitted = mesh
                .patches
                .iter()
                .find(|p| {
                    let face = body.get_face(p.face).unwrap();
                    SurfaceKind::of(body.get_surface(face.surface).unwrap()) == kind
                })
                .is_some_and(|p| !p.triangles.is_empty());
            let verdict = match mesh::validate::check_mesh(&mesh) {
                Ok(()) => "watertight".to_owned(),
                Err(e) => word(format!("{e:?}")),
            };
            format!(
                "mesh: {kind:?} face {}, {verdict}",
                if emitted { "emitted" } else { "EMPTY" }
            )
        }
    }
}

/// The rim-only cap above `z = ½` closed by its disc, with a meridian
/// strut added from the rim's `t = 0` vertex up to latitude `tip`,
/// inside the sphere face — so that face's loop is the two rim arcs and
/// the strut walked up and back.
fn cap_with_a_spur(tip: f64) -> Body<f64> {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: (1.0 - z * z).sqrt(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let disc = Surface::Plane {
        origin: p3(0.0, 0.0, z),
        normal: Vec3::new(0.0, 0.0, -1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut body = one_circle_cut(&rim, unit_sphere(), Some(disc));
    let foot = rim.eval(0.0);
    let (cap, _) = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .unwrap();
    let outer = body.get_face(cap).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the cap's loop is a cycle")
    };
    let at = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .find(|&h| {
            let he = body.get_half_edge(h).unwrap();
            let v = body.get_vertex(he.start).unwrap();
            (*body.get_point(v.point).unwrap() - foot).norm() < 1e-9
        })
        .unwrap();
    let strut = meridian_by_latitude();
    body.mev(
        MevSite::Fan { he1: at, he2: at },
        strut.eval(tip),
        EdgeCurveSpec::arc_of_circle(strut, z.asin(), tip).unwrap(),
        tol,
    )
    .unwrap();
    body
}

/// **What the spur buys, measured at two δ.** The guard admits both
/// bodies — each loop has a meridian — so neither answer below is
/// `MeridianFreeCurvedFace`, and that is the row's first claim. The
/// rest is the record of what stands behind the guard:
///
/// * a spur that REACHES the pole is the one-face seamed statement of
///   the cap, and it meshes watertight at both δ;
/// * a spur that stops SHORT leaves the pole interior — the state the
///   guard refuses, wearing a meridian — and is refused
///   `CertificateExceeded` at both δ tried. That is a typed refusal and
///   not a hole, but it is a comparison of a deviation bound against δ
///   and not a statement about the loop: nothing structural refuses
///   this face, and the row does not claim the answer holds at every δ.
#[test]
fn a_spur_on_a_rim_only_cap_gets_past_the_guard_and_this_is_what_answers() {
    let measured: Vec<(&str, f64, String)> = [
        ("short of the pole", FRAC_PI_2 - 0.3),
        ("to the pole", FRAC_PI_2),
    ]
    .into_iter()
    .flat_map(|(name, tip)| {
        let body = cap_with_a_spur(tip);
        [0.01, 0.1].map(|delta| (name, delta, disposition(&body, SurfaceKind::Sphere, delta)))
    })
    .collect();
    for (name, delta, got) in &measured {
        assert_ne!(
            got.as_str(),
            "refused: MeridianFreeCurvedFace",
            "{name}, δ = {delta}: the loop carries a meridian"
        );
    }
    let want = [
        ("short of the pole", 0.01, "refused: CertificateExceeded"),
        ("short of the pole", 0.1, "refused: CertificateExceeded"),
        ("to the pole", 0.01, "mesh: Sphere face emitted, watertight"),
        ("to the pole", 0.1, "mesh: Sphere face emitted, watertight"),
    ];
    let got: Vec<(&str, f64, &str)> = measured
        .iter()
        .map(|(name, delta, d)| (*name, *delta, d.as_str()))
        .collect();
    assert_eq!(got, want);
}
