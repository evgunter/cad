//! **Degenerate loops that DO carry a meridian**, so
//! `walk::require_a_meridian` admits them, measured for what
//! `tessellate` then answers. They are the known edge of that guard:
//! it asks whether a meridian traversal exists, and these loops have
//! one.
//!
//! * **Zero-width loops** — meridians only, all on one column: the
//!   one-face sphere whose loop is a single seam walked both ways, and
//!   a torus face bounded by one meridian circle. The walk gives them a
//!   domain of zero width, the face emits nothing, and the cross-face
//!   census reports it. The sphere member has BOTH poles on its loop,
//!   so "no rim and no pole" would not close this class either
//!   (`work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md`).
//! * **A rim-only cap wearing a spur** — a meridian strut from the rim
//!   toward the pole, walked up and back. A spur that reaches the pole
//!   is the one-face seamed statement of the cap; one that stops short
//!   leaves the pole interior, which is the state the guard refuses,
//!   in disguise.
//!
//! Every outcome here is read where debug assertions run, which is
//! every profile this workspace builds; the census is a `debug_assert`,
//! so the rows are gated on it.
#![cfg(debug_assertions)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::witness_bodies::one_circle_cut;
use core::f64::consts::{FRAC_PI_2, PI};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, SurfaceKind};
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MevSite};

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

/// The great circle in the xz plane from the north pole:
/// `eval(t) = (sin t, 0, cos t)`.
fn meridian_from_the_pole() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 0.0, 1.0),
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

/// **The one-face, one-seam sphere** (V2 / E1 / F1): one meridian from
/// pole to pole, walked down one side and back up the other. Both poles
/// are junctions of the loop and every traversal is a meridian, on one
/// column. Tier 3 refuses the body, so it is outside the input
/// `tessellate` is specified on; handed over anyway, the face emits
/// nothing and the cross-face census reports its seam unused.
#[test]
fn the_one_seam_sphere_is_a_zero_width_band_the_census_reports() {
    let tol = Tol::witness();
    let seam = meridian_from_the_pole();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(seam.eval(0.0)).unwrap();
    body.set_face_surface(start.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    body.mev(
        MevSite::Lone {
            r#loop: start.r#loop,
        },
        seam.eval(PI),
        EdgeCurveSpec::arc_of_circle(seam, 0.0, PI).unwrap(),
        tol,
    )
    .unwrap();
    assert!(
        topo::validate_geometric(&body, tol).is_err(),
        "a sphere stated as one face on one seam is not tier-3 valid"
    );
    assert_eq!(
        disposition(&body, SurfaceKind::Sphere, 0.05),
        "panic: chord segment"
    );
}

/// **The torus face bounded by one meridian circle**, and its
/// complement: two half arcs of the minor circle at `u = 0`, nothing
/// else. No pole exists on this chart for a meridian to end on. Tier 3
/// refuses it (the flux lane has no rim to read); both doors the curved
/// lane cites admit it; the walk gives it zero width.
#[test]
fn a_torus_face_bounded_by_one_meridian_circle_is_a_zero_width_band() {
    let torus = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let minor = Curve3::Circle {
        center: p3(2.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let body = one_circle_cut(&minor, torus, None);
    assert!(topo::validate_geometric(&body, Tol::witness()).is_err());
    assert_eq!(
        disposition(&body, SurfaceKind::Torus, 0.05),
        "panic: chord segment"
    );
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
/// rest is the record of what stands behind the guard: a spur to the
/// pole is a statement this lane can mesh or cannot; a spur that stops
/// short leaves the pole interior, and the answer moves with δ.
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
