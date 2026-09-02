//! **R2 review probes for MESH-7** — independent rows against the new
//! `require_iso_rectangle` door, written to falsify rather than to
//! confirm.
//!
//! The unit's own rows exercise the door on a cylinder rectangle, a
//! keyway, a rimless lune, an oblique sphere section and a plane. The
//! rows here ask what those leave open:
//!
//! * the sphere is not the only surface carrying a circle that is
//!   neither a rim nor a meridian — a torus carries **Villarceau
//!   circles**, and two of them bound a real lens-shaped region whose
//!   walk would classify BOTH as rims (`|n · axis| = cos 30° > 0.5`,
//!   `mesh::walk::classify`) and collapse them onto one `v`, which is
//!   the defeat class the oblique lens witnesses on the sphere. The
//!   door has to refuse it too, or the qualification the unit closes
//!   is closed only for one surface kind;
//! * a **zero-extent** cylinder face (both rims at one level) is
//!   vacuously "every rim at an extreme": what does the shape door say
//!   about a face the flux lane refuses `DegenerateFace`?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::print_stdout)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face, require_iso_rectangle};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// One traversed boundary edge `a → b`, as `topo`'s half-edge
/// flattening stores it (certified forward interval + traversal bool).
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge {
        carrier_id: None,
        carrier,
        t0,
        t1,
        forward,
        start,
        end,
    }
}

/// **The Villarceau lens.** The torus `R = 1`, `r = 0.5` about `+Z`;
/// its bitangent plane `z = y·tan α`, `sin α = r/R`, meets it in two
/// circles of radius `R` centred at `(±r, 0, 0)`, crossing at
/// `(0, ±0.75, ±0.433)`. The two arcs between those crossings bound a
/// lens-shaped region of the torus whose every boundary point is on
/// the surface and whose boundary is iso in neither chart direction.
fn villarceau_loop() -> (Surface<f64>, Vec<LoopEdge<f64>>) {
    let (major, minor) = (1.0_f64, 0.5_f64);
    let (s, c) = (
        minor / major,
        (1.0 - (minor / major) * (minor / major)).sqrt(),
    );
    let n = v3(0.0, -s, c);
    let circle = |cx: f64| Curve3::Circle {
        center: p3(cx, 0.0, 0.0),
        axis: n,
        radius: major,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let third = core::f64::consts::PI / 3.0;
    let surface = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: major,
        minor_radius: minor,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    // A: t ∈ [2π/3, 4π/3] runs P1 → P2; B: t ∈ [−π/3, π/3] runs P2 → P1.
    let loop_edges = vec![
        edge(circle(minor), 2.0 * third, 4.0 * third, 0, 1),
        edge(circle(-minor), -third, third, 1, 0),
    ];
    (surface, loop_edges)
}

/// The construction itself, checked before anything is concluded from
/// it: every sampled point of both carriers lies ON the torus, and the
/// two arcs share their endpoints.
#[test]
fn the_villarceau_arcs_lie_on_the_torus_and_close_a_loop() {
    let (surface, edges) = villarceau_loop();
    let Surface::Torus {
        major_radius,
        minor_radius,
        ..
    } = surface
    else {
        panic!("torus")
    };
    for e in &edges {
        for i in 0..=16 {
            let t = e.t0 + (e.t1 - e.t0) * f64::from(i) / 16.0;
            let p = e.carrier.eval(t);
            let rho = (p.x * p.x + p.y * p.y).sqrt();
            let d = (rho - major_radius).powi(2) + p.z * p.z - minor_radius * minor_radius;
            assert!(d.abs() < 1e-12, "off the torus by {d:e} at t = {t}");
        }
    }
    let a1 = edges[0].carrier.eval(edges[0].t1);
    let b0 = edges[1].carrier.eval(edges[1].t0);
    assert!((a1 - b0).norm() < 1e-12, "the arcs meet");
}

/// **The door has to refuse the Villarceau lens.** Its boundary is on
/// the torus and is not iso; `mesh::walk::classify` would call both
/// arcs rims (`|n·axis| = cos 30° ≈ 0.866 > 0.5`) and `iso_side_starts`
/// would collapse them onto one `v` — the same shape as the oblique
/// sphere lens the unit closes. A pass here would mean the closure is
/// per-surface-kind rather than structural.
#[test]
fn the_villarceau_lens_is_refused_by_the_shape_door() {
    let (surface, edges) = villarceau_loop();
    let got = require_iso_rectangle(&surface, &edges, band());
    println!("villarceau lens: door = {got:?}");
    println!(
        "villarceau lens: flux = {:?}",
        curved_face(&surface, &edges, 1.0, band())
    );
    assert!(
        matches!(
            got,
            Err(PropsError::NotIsoRectangle { .. } | PropsError::Escalated { .. })
        ),
        "a non-iso loop on the torus must not pass the shape door; got {got:?}"
    );
}

/// **A zero-extent cylinder face**: two rims at ONE level, joined by
/// two meridians of zero length. Every rim is trivially at an extreme
/// (`lo == hi`), so the shape door passes it; the flux lane refuses
/// `DegenerateFace` at its own `require_extent`. Recorded, not judged:
/// the point is that the door's "shape only" is strictly weaker than
/// the flux arm's precondition in a second place besides the lune, and
/// only the lune is documented.
#[test]
fn a_zero_extent_cylinder_face_passes_the_shape_door_and_not_the_flux_lane() {
    let surface = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, v),
                axis: v3(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Line {
                origin: p3(u.cos(), u.sin(), 0.0),
                dir: v3(0.0, 0.0, 1.0),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let edges = vec![
        rim(0.0, 0.0, 1.0, 0, 1),
        mer(1.0, 0.0, 0.0, 1, 2),
        rim(0.0, 1.0, 0.0, 2, 3),
        mer(0.0, 0.0, 0.0, 3, 0),
    ];
    let door = require_iso_rectangle(&surface, &edges, band());
    let flux = curved_face(&surface, &edges, 1.0, band());
    println!("zero-extent cylinder face: door = {door:?}, flux = {flux:?}");
    assert_eq!(door, Ok(()), "the shape door is vacuous on a zero extent");
    assert!(
        matches!(flux, Err(PropsError::DegenerateFace)),
        "the flux lane refuses the same face; got {flux:?}"
    );
}
