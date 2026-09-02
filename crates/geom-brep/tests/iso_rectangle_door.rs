//! **The iso-rectangle SHAPE door** (`require_iso_rectangle`): the S58
//! predicate and the per-kind boundary classification, public and
//! flux-free, for a consumer whose lane rests on the premise without
//! wanting a volume.
//!
//! Every row is built as key-free `LoopEdge`s and run through the door
//! AND through `curved_face`, so the rows state where the two agree
//! (a rectangle passes both, a notch refuses both by `props_rim_level`,
//! an oblique sphere section refuses both by the same incidence name)
//! and the ONE place they part: the rimless lune, a chart rectangle
//! the door admits and the flux lane refuses on its own `Δu = π`
//! premise. That divergence is the door's contract, not a gap in it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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

/// One traversed boundary edge `a → b` on the carrier, stored as the
/// certified forward interval plus the traversal bool, exactly as
/// `topo`'s half-edge flattening does it.
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge {
        carrier,
        t0,
        t1,
        forward,
        start,
        end,
    }
}

/// The unit cylinder about +Z with its rim (coaxial circle at height
/// `v`) and meridian (axial line at azimuth `u`) edge factories.
fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
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
}
fn mer(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
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
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
/// The great circle whose plane contains the axis at azimuth `u`; its
/// parameter IS the latitude.
fn great(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: 1.0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        a,
        b,
    )
}

/// A cylinder rectangle passes the door and measures; the U-shaped
/// keyway (a notch cut into the top rim) refuses at the door AND at the
/// flux lane, both by `props_rim_level` — one predicate, two callers.
#[test]
fn a_keyway_refuses_at_the_door_by_the_same_name_the_flux_lane_uses() {
    let rect = vec![
        rim(0.0, 0.0, 1.5, 0, 1),
        mer(1.5, 0.0, 1.0, 1, 2),
        rim(1.0, 1.5, 0.0, 2, 3),
        mer(0.0, 1.0, 0.0, 3, 0),
    ];
    assert_eq!(require_iso_rectangle(&cylinder(), &rect, band()), Ok(()));
    assert!(curved_face(&cylinder(), &rect, 1.0, band()).is_ok());
    let keyway = vec![
        rim(0.0, 0.0, 1.5, 0, 1),
        mer(1.5, 0.0, 1.0, 1, 2),
        rim(1.0, 1.5, 1.0, 2, 3),
        mer(1.0, 1.0, 0.6, 3, 4),
        rim(0.6, 1.0, 0.5, 4, 5),
        mer(0.5, 0.6, 1.0, 5, 6),
        rim(1.0, 0.5, 0.0, 6, 7),
        mer(0.0, 1.0, 0.0, 7, 0),
    ];
    let want = Err(PropsError::NotIsoRectangle {
        what: "props_rim_level",
    });
    assert_eq!(require_iso_rectangle(&cylinder(), &keyway, band()), want);
    assert_eq!(
        curved_face(&cylinder(), &keyway, 1.0, band()).map(|_| ()),
        want
    );
}

/// **The divergence, pinned.** A lune between two great circles a
/// quarter turn apart is `[0, π/2] × [−π/2, π/2]` — a chart rectangle
/// — so the door admits it; the flux lane refuses it on
/// `props_band_coplanar`, its own `Δu = π` premise. Goes red if either
/// side is folded onto the other.
#[test]
fn a_rimless_lune_passes_the_door_and_fails_the_flux_lane() {
    let half = core::f64::consts::FRAC_PI_2;
    let lune = vec![
        great(0.0, -half, half, 0, 1),
        great(half, half, -half, 1, 0),
    ];
    assert_eq!(require_iso_rectangle(&sphere(), &lune, band()), Ok(()));
    assert_eq!(
        curved_face(&sphere(), &lune, 1.0, band()).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "props_band_coplanar"
        })
    );
}

/// An oblique plane section of the sphere — tilted 0.6 rad off the
/// polar axis, offset 0.3 from the centre — is neither a coaxial rim
/// nor a great circle. The door classifies it a rim (`n·â` definite)
/// and refuses its incidence: the circle's axis is not the sphere's.
/// This is the `walk::iso_side_starts` qualification's face, refused
/// on rim structure before any walk could collapse it.
#[test]
fn an_oblique_sphere_section_is_refused_on_rim_incidence() {
    let (a, d) = (0.6_f64, 0.3_f64);
    let r = (1.0 - d * d).sqrt();
    let n = v3(a.sin(), 0.0, a.cos());
    let section = |t0: f64, t1: f64, s: u32, e: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, 0.0) + n * d,
                axis: n,
                radius: r,
                u_ref: v3(a.cos(), 0.0, -a.sin()),
            },
            t0,
            t1,
            s,
            e,
        )
    };
    // Two arcs of the one oblique circle closing a loop on their own:
    // the door refuses on the FIRST edge's incidence, so the loop's
    // closure is not what is under test here.
    let lens = vec![section(0.0, 3.0, 0, 1), section(3.0, 6.0, 1, 0)];
    let want = Err(PropsError::NotIsoRectangle {
        what: "props_rim_axis_parallel",
    });
    assert_eq!(require_iso_rectangle(&sphere(), &lens, band()), want);
    assert_eq!(curved_face(&sphere(), &lens, 1.0, band()).map(|_| ()), want);
}

/// A plane is not the door's question and refuses typed, as
/// `curved_face` does.
#[test]
fn a_plane_is_refused_typed_not_answered() {
    let plane = Surface::Plane {
        origin: p3(0.0, 0.0, 0.0),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    assert_eq!(
        require_iso_rectangle(&plane, &[], band()),
        Err(PropsError::NotIsoRectangle {
            what: "require_iso_rectangle called on a plane (a planar loop is not a chart rectangle)",
        })
    );
}
