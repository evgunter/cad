//! **S58 / #649: the iso-rectangle predicate, on all four curved
//! kinds.**
//!
//! `props`' closed forms integrate `area = r·Δu·(hi − lo)` and its
//! per-kind siblings, whose premise is that the face's domain in
//! `(u, v)` is an iso-parameter RECTANGLE. Until S58 that premise was
//! tested three different ways to three different strengths, and the
//! way three of the four kinds tested it — grouping rims on
//! `(level, direction)` and requiring the per-group span SUMS to agree
//! — is not a shape test. A cross-shaped domain whose arm width is
//! exactly half the u-extent makes every group sum to the same `Δu`
//! and passed: `topo::mass_properties` then certified a **19%-low**
//! volume with `volume_pad = 0.0` (#649).
//!
//! The one named predicate is `props_rim_level`: **every rim sits at
//! one of the face's two extreme `v`-levels**. `w(v)`, the total
//! `u`-measure at height `v`, can change only at a rim level (between
//! levels the boundary is meridians, which move no `u`-endpoint), so
//! the rule forces `w ≡ Δu` — exactly the premise. It was already in
//! the tree on ONE arm, inside `torus()`, for a periodicity reason;
//! this suite is the record that it now governs all four.
//!
//! Every row is built as key-free `LoopEdge`s and run through the
//! public `curved_face`, the same entry `topo::mass_properties` uses
//! per face. The controls are load-bearing: a rectangle of the same
//! extent must still be ACCEPTED and its area must still be EXACT, so
//! a rule that refused everything could not make this suite green.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::{Band, Point3, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// One traversed boundary edge: `a → b` on the carrier, stored as the
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

/// Rim/meridian carrier factories for one surface kind.
type Rim<'a> = &'a dyn Fn(f64, f64, f64, u32, u32) -> LoopEdge<f64>;
type Mer<'a> = &'a dyn Fn(f64, f64, f64, u32, u32) -> LoopEdge<f64>;
/// An owned one, for the kit below to hand back.
type EdgeFn = Box<dyn Fn(f64, f64, f64, u32, u32) -> LoopEdge<f64>>;

/// The cylinder of radius `r` about +Z, with its rim (a coaxial
/// circle at height `v`) and meridian (an axial line at azimuth `u`)
/// carrier factories. Three rows below want the same three things;
/// this is their one home. The sphere, cone and torus rows keep their
/// own factories inline — each is used once, and the geometry IS the
/// row.
fn cylinder_kit(r: f64) -> (Surface<f64>, EdgeFn, EdgeFn) {
    let s = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = move |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, v),
                axis: v3(0.0, 0.0, 1.0),
                radius: r,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = move |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Line {
                origin: p3(r * u.cos(), r * u.sin(), 0.0),
                dir: v3(0.0, 0.0, 1.0),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    (s, Box::new(rim), Box::new(mer))
}

/// The genuine iso-rectangle `[u0,u1] × [v0,v1]`, walked CCW.
fn rect_loop(rim: Rim, mer: Mer, v0: f64, v1: f64, u0: f64, u1: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(v0, u0, u1, 0, 1),
        mer(u1, v0, v1, 1, 2),
        rim(v1, u1, u0, 2, 3),
        mer(u0, v1, v0, 3, 0),
    ]
}

/// The PLUS/CROSS domain: a column `u ∈ [−uc, uc]` over the full
/// `[v0, v3]`, plus two arms out to `±uo` over the INTERIOR band
/// `[v1, v2]`. With `uo − uc = uc` every rim group sums to `2·uc`, so
/// the pre-S58 sum rule accepted it on cylinder, cone and sphere.
#[allow(clippy::too_many_arguments)]
fn plus_loop(
    rim: Rim,
    mer: Mer,
    v0: f64,
    v1: f64,
    v2: f64,
    v3v: f64,
    uc: f64,
    uo: f64,
) -> Vec<LoopEdge<f64>> {
    vec![
        rim(v0, -uc, uc, 0, 1),
        mer(uc, v0, v1, 1, 2),
        rim(v1, uc, uo, 2, 3),
        mer(uo, v1, v2, 3, 4),
        rim(v2, uo, uc, 4, 5),
        mer(uc, v2, v3v, 5, 6),
        rim(v3v, uc, -uc, 6, 7),
        mer(-uc, v3v, v2, 7, 8),
        rim(v2, -uc, -uo, 8, 9),
        mer(-uo, v2, v1, 9, 10),
        rim(v1, -uo, -uc, 10, 11),
        mer(-uc, v1, v0, 11, 0),
    ]
}

/// The control must be ACCEPTED and its area EXACT (relative, since the
/// closed form is a product of stored quantities).
fn accepts_exactly(kind: &str, s: &Surface<f64>, edges: &[LoopEdge<f64>], exact_area: f64) {
    let band = Band::linear().unwrap();
    match curved_face(s, edges, 1.0, band) {
        Ok(fc) => {
            let rel = (fc.area - exact_area).abs() / exact_area;
            assert!(
                rel < 1e-12,
                "{kind}: control rectangle area {:.15e} != exact {exact_area:.15e} (rel {rel:.3e})",
                fc.area
            );
        }
        Err(e) => panic!("{kind}: the control ISO-RECTANGLE was refused: {e:?}"),
    }
}

/// The non-rectangular domain must be refused by the ONE named
/// predicate — not by some other check that happens to fire, which is
/// the whole point of S58 (the torus was already protected, by a
/// private rule, and that is what made the fragmentation invisible).
fn refuses_on_rim_level(kind: &str, s: &Surface<f64>, edges: &[LoopEdge<f64>]) {
    let band = Band::linear().unwrap();
    match curved_face(s, edges, 1.0, band) {
        Err(PropsError::NotIsoRectangle {
            what: "props_rim_level",
        }) => {}
        other => panic!(
            "{kind}: a non-rectangular iso domain was not refused by props_rim_level: {other:?}"
        ),
    }
}

// ---------------------------------------------------------------------
// The four kinds, each with its own control
// ---------------------------------------------------------------------

/// Shared plus geometry in `(u, v)`: `uo − uc == uc`, so every rim
/// group sums to `2·uc` and the span-sum rule alone cannot see it.
const UC: f64 = 0.5;
const UO: f64 = 1.0;

#[test]
fn cylinder_plus_domain_refuses_and_the_rectangle_still_measures() {
    let r = 0.010;
    let (s, rim, mer) = cylinder_kit(r);
    let (a0, a1, a2, a3) = (0.0, 0.006, 0.014, 0.020);
    accepts_exactly(
        "cylinder",
        &s,
        &rect_loop(&rim, &mer, a0, a3, -UO, UO),
        r * 2.0 * UO * (a3 - a0),
    );
    refuses_on_rim_level(
        "cylinder",
        &s,
        &plus_loop(&rim, &mer, a0, a1, a2, a3, UC, UO),
    );
}

#[test]
fn cone_plus_domain_refuses_and_the_rectangle_still_measures() {
    let alpha: f64 = 0.5;
    let (sa, ca) = alpha.sin_cos();
    let s = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: alpha,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = move |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, v * ca),
                axis: v3(0.0, 0.0, 1.0),
                radius: v * sa,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = move |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Line {
                origin: p3(0.0, 0.0, 0.0),
                dir: v3(sa * u.cos(), sa * u.sin(), ca),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let (a0, a1, a2, a3) = (0.010, 0.014, 0.018, 0.020);
    accepts_exactly(
        "cone",
        &s,
        &rect_loop(&rim, &mer, a0, a3, -UO, UO),
        sa * 2.0 * UO * (a3 * a3 - a0 * a0) / 2.0,
    );
    refuses_on_rim_level("cone", &s, &plus_loop(&rim, &mer, a0, a1, a2, a3, UC, UO));
}

#[test]
fn sphere_plus_domain_refuses_and_the_rectangle_still_measures() {
    let rs = 0.010;
    let s = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: rs,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = move |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, rs * v.sin()),
                axis: v3(0.0, 0.0, 1.0),
                radius: rs * v.cos(),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = move |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, 0.0),
                axis: v3(u.sin(), -u.cos(), 0.0),
                radius: rs,
                u_ref: v3(u.cos(), u.sin(), 0.0),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let (a0, a1, a2, a3) = (0.2f64, 0.5f64, 0.9f64, 1.2f64);
    accepts_exactly(
        "sphere",
        &s,
        &rect_loop(&rim, &mer, a0, a3, -UO, UO),
        rs * rs * 2.0 * UO * (a3.sin() - a0.sin()),
    );
    refuses_on_rim_level("sphere", &s, &plus_loop(&rim, &mer, a0, a1, a2, a3, UC, UO));
}

#[test]
fn torus_plus_domain_still_refuses_by_the_shared_predicate() {
    let (rr, r0) = (0.020f64, 0.005f64);
    let s = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: rr,
        minor_radius: r0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = move |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, r0 * v.sin()),
                axis: v3(0.0, 0.0, 1.0),
                radius: rr + r0 * v.cos(),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = move |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(rr * u.cos(), rr * u.sin(), 0.0),
                axis: v3(u.sin(), -u.cos(), 0.0),
                radius: r0,
                u_ref: v3(u.cos(), u.sin(), 0.0),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let (a0, a1, a2, a3) = (0.2f64, 0.5f64, 0.9f64, 1.2f64);
    let band_area = |du: f64, x: f64, y: f64| r0 * du * (rr * (y - x) + r0 * (y.sin() - x.sin()));
    accepts_exactly(
        "torus",
        &s,
        &rect_loop(&rim, &mer, a0, a3, -UO, UO),
        band_area(2.0 * UO, a0, a3),
    );
    // The torus refused this before S58 too — by its own private rule.
    // The claim of this row is that it still refuses, and now by the
    // SHARED predicate: same name, same metering, one home.
    refuses_on_rim_level("torus", &s, &plus_loop(&rim, &mer, a0, a1, a2, a3, UC, UO));
}

// ---------------------------------------------------------------------
// The rest of #649's adversarial family, on the cylinder
// ---------------------------------------------------------------------

/// The cylinder rows of #649's probe that the SUM rule accepted:
/// tall-armed plus (−49.98%), double-plus (−47.37%), Z-staircase
/// (−28.57%). All three have interior rim levels; all three now refuse
/// on the one predicate. The L-shape is the row the sum rule DID catch
/// — it must keep refusing, and it is allowed to refuse on either
/// predicate, since it violates both.
#[test]
fn the_whole_649_family_refuses() {
    let r = 0.010;
    let (s, rim, mer) = cylinder_kit(r);
    let vtop = 0.020;

    // Tall arms: the undercount approaches −50%.
    for (a, b) in [(0.001, 0.019), (1e-5, vtop - 1e-5)] {
        refuses_on_rim_level(
            "cylinder tall-arm plus",
            &s,
            &plus_loop(&rim, &mer, 0.0, a, b, vtop, UC, UO),
        );
    }

    // Double plus: two arm bands, four interior levels.
    let (z1, z2, z3, z4) = (0.0005, 0.0095, 0.0105, 0.0195);
    let dbl = vec![
        rim(0.0, -UC, UC, 0, 1),
        mer(UC, 0.0, z1, 1, 2),
        rim(z1, UC, UO, 2, 3),
        mer(UO, z1, z2, 3, 4),
        rim(z2, UO, UC, 4, 5),
        mer(UC, z2, z3, 5, 6),
        rim(z3, UC, UO, 6, 7),
        mer(UO, z3, z4, 7, 8),
        rim(z4, UO, UC, 8, 9),
        mer(UC, z4, vtop, 9, 10),
        rim(vtop, UC, -UC, 10, 11),
        mer(-UC, vtop, z4, 11, 12),
        rim(z4, -UC, -UO, 12, 13),
        mer(-UO, z4, z3, 13, 14),
        rim(z3, -UO, -UC, 14, 15),
        mer(-UC, z3, z2, 15, 16),
        rim(z2, -UC, -UO, 16, 17),
        mer(-UO, z2, z1, 17, 18),
        rim(z1, -UO, -UC, 18, 19),
        mer(-UC, z1, 0.0, 19, 0),
    ];
    refuses_on_rim_level("cylinder double-plus", &s, &dbl);

    // Z-staircase: two offset blocks, every group sum 1.0.
    let (v1, v2) = (0.006, 0.014);
    let stair = vec![
        rim(0.0, -1.0, 0.0, 0, 1),
        mer(0.0, 0.0, v1, 1, 2),
        rim(v1, 0.0, 1.0, 2, 3),
        mer(1.0, v1, vtop, 3, 4),
        rim(vtop, 1.0, 0.0, 4, 5),
        mer(0.0, vtop, v2, 5, 6),
        rim(v2, 0.0, -1.0, 6, 7),
        mer(-1.0, v2, 0.0, 7, 0),
    ];
    refuses_on_rim_level("cylinder Z-staircase", &s, &stair);

    // The L-shape: unequal group sums AND an interior level. It was
    // already refused before S58; either predicate may claim it.
    let l_shape = vec![
        rim(0.0, -1.0, 0.0, 0, 1),
        mer(0.0, 0.0, v1, 1, 2),
        rim(v1, 0.0, 1.0, 2, 3),
        mer(1.0, v1, vtop, 3, 4),
        rim(vtop, 1.0, -1.0, 4, 5),
        mer(-1.0, vtop, 0.0, 5, 0),
    ];
    assert!(
        matches!(
            curved_face(&s, &l_shape, 1.0, Band::linear().unwrap()),
            Err(PropsError::NotIsoRectangle { .. })
        ),
        "the L-shape must stay refused"
    );
}

/// **Multi-arc rims at the two extremes still measure.** M5 PR 9
/// introduced the span-SUM rule for exactly this shape — a boolean-cut
/// or re-merged wall whose rim arrives as several arcs at one level —
/// and the level predicate must not take it away. The domain is the
/// same rectangle as the cylinder control, with each rim split in two
/// at an interior `u`; the area must be identical.
#[test]
fn multi_arc_rims_at_the_extremes_still_measure() {
    let r = 0.010;
    let (s, rim, mer) = cylinder_kit(r);
    let (v0, v1) = (0.0, 0.020);
    // Bottom rim as two arcs [-1, 0.25] and [0.25, 1]; top likewise.
    let split = vec![
        rim(v0, -1.0, 0.25, 0, 1),
        rim(v0, 0.25, 1.0, 1, 2),
        mer(1.0, v0, v1, 2, 3),
        rim(v1, 1.0, -0.4, 3, 4),
        rim(v1, -0.4, -1.0, 4, 5),
        mer(-1.0, v1, v0, 5, 0),
    ];
    accepts_exactly("cylinder multi-arc rim", &s, &split, r * 2.0 * (v1 - v0));
}
