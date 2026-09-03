//! **The two iso-bounded witnesses no public construction mints**,
//! assembled through the Euler doors — the one route the walk's own
//! docs name as fronted by no certification. Shared by the integration
//! suite (`tests/iso_rectangle_door.rs`, through the public
//! `tessellate`) and by `curved`'s in-crate rows (through the walk
//! itself), so the bodies both sides measure are one definition. Uses
//! nothing from `mesh`, which is what lets one file compile in both.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, FaceKey, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The arc of `circle` from parameter `t0` to `t1`, as the scaffolding
/// spec the Euler doors certify against the arc's own endpoints.
fn arc(circle: Curve3<f64>, t0: f64, t1: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(circle, t0, t1).expect("a circle carrier has an arc spec")
}

/// **The keyway**: the unit cylinder about +Z carrying ONE face whose
/// loop is the U-shaped iso domain below, and a second face (the
/// complement, on the same surface) closing the two-manifold. Vertices
/// in `(u, v)`:
///
/// ```text
///   (0,1) ── (0.5,1)      (1,1) ── (1.5,1)
///     │         │            │        │
///     │      (0.5,0.6) ── (1,0.6)     │
///     │                               │
///   (0,0) ─────────────────────── (1.5,0)
/// ```
///
/// Rims at `v = 1` and the notch floor at `v = 0.6`: the interior rim
/// is what makes it a U and not a rectangle. Returns the body and the
/// U-domain face.
pub fn keyway() -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let on = |u: f64, v: f64| p3(u.cos(), u.sin(), v);
    // A rim at height `v`, walked in DECREASING `u`: the circle about
    // −Z has `u = −t`, so the certified forward interval is increasing.
    let rim_back = |v: f64| Curve3::Circle {
        center: p3(0.0, 0.0, v),
        axis: v3(0.0, 0.0, -1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim_fwd = |v: f64| Curve3::Circle {
        center: p3(0.0, 0.0, v),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let line = EdgeCurveSpec::line_between;
    let (v0, v1, v2, v3_, v4, v5, v6, v7) = (
        on(0.0, 0.0),
        on(1.5, 0.0),
        on(1.5, 1.0),
        on(1.0, 1.0),
        on(1.0, 0.6),
        on(0.5, 0.6),
        on(0.5, 1.0),
        on(0.0, 1.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(v0).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e01 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            v1,
            arc(rim_fwd(0.0), 0.0, 1.5),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e12 = strut(&mut body, e01.he_minus, v2, line(v1, v2));
    let e23 = strut(&mut body, e12.he_minus, v3_, arc(rim_back(1.0), -1.5, -1.0));
    let e34 = strut(&mut body, e23.he_minus, v4, line(v3_, v4));
    let e45 = strut(&mut body, e34.he_minus, v5, arc(rim_back(0.6), -1.0, -0.5));
    let e56 = strut(&mut body, e45.he_minus, v6, line(v5, v6));
    let e67 = strut(&mut body, e56.he_minus, v7, arc(rim_back(1.0), -0.5, 0.0));
    body.mef(
        MefSite::Chords {
            he1: e67.he_minus,
            he2: e01.he_plus,
        },
        line(v7, v0),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    (body, seed.face)
}

/// **The oblique lens**: the unit sphere about +Z split into two faces
/// by a loop of two plane-section arcs whose planes are tilted 0.6 rad
/// off the polar axis and offset 0.3 from the centre — so each circle
/// is neither a coaxial rim nor a great circle, and the two meet at
/// `(0, ±y0, z0)`, well off the axis. Returns the body and the seed
/// face (the arcs' `A → B` side).
pub fn oblique_lens() -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let (a, d) = (0.6_f64, 0.3_f64);
    let z0 = d / a.cos();
    let y0 = (1.0 - z0 * z0).sqrt();
    let (p, q) = (p3(0.0, y0, z0), p3(0.0, -y0, z0));
    let r = (1.0 - d * d).sqrt();
    // The section circle through `from` and `to` in the plane with
    // unit normal `n`, oriented so the arc `from → to` has increasing
    // parameter, with `from` at parameter 0.
    let section = |n: Vec3<f64>, from: Point3<f64>, to: Point3<f64>| {
        let center = p3(0.0, 0.0, 0.0) + n * d;
        let u_ref = (from - center) * (1.0 / r);
        let mut circle = Curve3::Circle {
            center,
            axis: n,
            radius: r,
            u_ref,
        };
        let mut t1 = circle.param_near(to, 0.0).unwrap();
        if t1 < 0.0 {
            circle = Curve3::Circle {
                center,
                axis: n * -1.0,
                radius: r,
                u_ref,
            };
            t1 = -t1;
        }
        arc(circle, 0.0, t1)
    };
    let n_a = v3(a.sin(), 0.0, a.cos());
    let n_b = v3(-a.sin(), 0.0, a.cos());
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let ea = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            q,
            section(n_a, p, q),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: ea.he_minus,
            he2: ea.he_plus,
        },
        section(n_b, q, p),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    (body, seed.face)
}

/// **The zero-width slit**: the cylinder rectangle `[0, 1.5] × [0, 1]`
/// with a spur edge from the bottom rim's midpoint `(0.75, 0)` up one
/// column to `(0.75, 0.5)`, traversed up and back down by the one
/// loop. Every rim sits at an extreme, every edge's carrier is a rim
/// circle or an axial line, so the shape door admits it — and the
/// walk's polygon carries the slit's tip a feature width inside its own
/// box, which is the case the walk-consistency check keeps. Returns
/// the body and the slit face.
pub fn slit() -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let on = |u: f64, v: f64| p3(u.cos(), u.sin(), v);
    let rim = |v: f64, axis_z: f64| Curve3::Circle {
        center: p3(0.0, 0.0, v),
        axis: v3(0.0, 0.0, axis_z),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let line = EdgeCurveSpec::line_between;
    let (v0, v1, v3_, v2, v7, tip) = (
        on(0.0, 0.0),
        on(0.75, 0.0),
        on(1.5, 0.0),
        on(1.5, 1.0),
        on(0.0, 1.0),
        on(0.75, 0.5),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(v0).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e01 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            v1,
            arc(rim(0.0, 1.0), 0.0, 0.75),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e13 = strut(&mut body, e01.he_minus, v3_, arc(rim(0.0, 1.0), 0.75, 1.5));
    let e32 = strut(&mut body, e13.he_minus, v2, line(v3_, v2));
    let e27 = strut(&mut body, e32.he_minus, v7, arc(rim(1.0, -1.0), -1.5, 0.0));
    body.mef(
        MefSite::Chords {
            he1: e27.he_minus,
            he2: e01.he_plus,
        },
        line(v7, v0),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    // The spur, inside the seed face's loop: `e13.he_plus` starts at
    // the bottom rim's midpoint and is the only half-edge of that loop
    // to do so, so the spur's two half-edges are spliced before it.
    strut(&mut body, e13.he_plus, tip, line(v1, tip));
    (body, seed.face)
}

/// **The pole-crossing half-cap**: the unit sphere about +Z split into
/// two faces by a rim half-circle at latitude `asin 0.5` and ONE
/// great-circle arc that runs from the rim's `u = π` end OVER THE
/// NORTH POLE to its `u = 0` end. The seed face is the half-cap
/// `[0, π] × [asin 0.5, π/2]`; the second is the rest of the sphere.
///
/// Both faces' boundaries are the SAME two edges traversed opposite
/// ways, which is what makes this body the two things it witnesses:
/// the traversed arc lies on two chart meridians (`u` jumps by π at
/// the pole), and the parse hands both faces the same levels, so their
/// fluxes are equal and opposite and `mass_properties` answers 0.0 on
/// a closed sphere (issue 1598).
///
/// Returns the body, the half-cap face and the complement face.
pub fn pole_crossing_half_cap() -> (Body<f64>, FaceKey, FaceKey) {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = p3(r, 0.0, z);
    let b = p3(-r, 0.0, z);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    // The great circle in the plane y = 0 anchored at `b`, oriented so
    // the arc `b → a` climbs over the north pole.
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: v3(-r, 0.0, z),
    };
    let mut g = great(v3(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(v3(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).expect("the arc's far end");
    // The arc's own shape, stated where it is built: `b → a` over the
    // pole is the long way round, 120° of the great circle.
    assert!(
        t_end > 0.0 && (t_end - 2.0 * core::f64::consts::FRAC_PI_3).abs() < 1e-9,
        "the over-the-pole arc spans 120 degrees, got {t_end}"
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            arc(rim, 0.0, core::f64::consts::PI),
            tol,
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            arc(g, 0.0, t_end),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    (body, seed.face, made.face)
}

/// **The apex-crossing bow tie**: the 45° cone about +Z with its apex
/// at the origin, cut into two faces by rims at signed slant ∓1 and
/// two generator segments that run THROUGH the apex. Every carrier is
/// a certified generator or coaxial rim and both rims sit at the
/// extremes, so the shape door admits it; the traversed segments leave
/// their chart branch at the apex, where `u` jumps to the mirror
/// nappe.
///
/// The cone's sibling of [`pole_crossing_half_cap`], and the reason
/// this unit's class sweep does not read "the cone is immune".
pub fn apex_crossing_bowtie() -> (Body<f64>, FaceKey, FaceKey) {
    let tol = Tol::witness();
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let (a, b, c, d) = (
        p3(s, 0.0, -s),
        p3(-s, 0.0, -s),
        p3(-s, 0.0, s),
        p3(s, 0.0, s),
    );
    let rim = |z: f64| Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: s,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let line = EdgeCurveSpec::line_between;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cone {
            apex: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            arc(rim(-s), 0.0, core::f64::consts::PI),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    // `b → d` and `c → a` are the two lines through the apex.
    let e_bd = strut(&mut body, e_ab.he_minus, d, line(b, d));
    let e_dc = strut(
        &mut body,
        e_bd.he_minus,
        c,
        arc(rim(s), 0.0, core::f64::consts::PI),
    );
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_dc.he_minus,
                he2: e_ab.he_plus,
            },
            line(c, a),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    (body, seed.face, made.face)
}
