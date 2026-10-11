//! **The sphere flux arm's Gauss–Bonnet form** (`curved::sphere_face`),
//! held to Girard's theorem: a spherical triangle with three right
//! angles — the octant `x, y, z > 0` — has area `R²·π/2`, on a sphere
//! whose chart axis is `(1, 1, −1)/√3`, so each of its three
//! great-circle arcs is tilted against the chart (neither a rim nor a
//! meridian) and neither chart pole lies in the octant or on its loop.
//!
//! The flux is `σ·R·Area + c·A⃗` with `A⃗` the octant's vector area,
//! `(R²π/4)(1, 1, 1)` — a quarter disc projected on each axis plane — so
//! a centre off the origin checks the anchor term against an
//! independent number too.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face, curved_face_loops};
use geom_core::Vec3;

const R: f64 = 2.0;

fn sphere(c: Vec3<f64>) -> Surface<f64> {
    let k = 1.0 / 3.0_f64.sqrt();
    Surface::Sphere {
        center: p3(c.x, c.y, c.z),
        radius: R,
        axis: v3(k, k, -k),
        u_ref: v3(1.0, -1.0, 0.0) * (1.0 / 2.0_f64.sqrt()),
    }
}

/// The octant's loop, interior on the left under the outward normal:
/// `+x → +y` in `z = 0`, `+y → +z` in `x = 0`, `+z → +x` in `y = 0`.
fn octant(c: Vec3<f64>) -> Vec<LoopEdge<f64>> {
    let arc = |axis: Vec3<f64>, u_ref: Vec3<f64>, s: u32, e: u32| {
        edge(
            Curve3::Circle {
                center: p3(c.x, c.y, c.z),
                axis,
                radius: R,
                u_ref,
            },
            0.0,
            FRAC_PI_2,
            s,
            e,
        )
    };
    let (x, y, z) = (v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0));
    vec![arc(z, x, 0, 1), arc(x, y, 1, 2), arc(y, z, 2, 0)]
}

#[test]
fn the_octant_meets_girard_and_its_vector_area() {
    let area = R * R * FRAC_PI_2;
    for (label, c) in [
        ("centred", v3(0.0, 0.0, 0.0)),
        ("off-centre", v3(1.0, 2.0, 3.0)),
    ] {
        let fc = curved_face(&sphere(c), &octant(c), true, band()).expect("the octant measures");
        let flux = R * area + (c.x + c.y + c.z) * R * R * PI / 4.0;
        assert!(
            (fc.area - area).abs() < 1e-12,
            "{label}: area {} != {area}",
            fc.area
        );
        assert!(
            (fc.flux - flux).abs() < 1e-12,
            "{label}: flux {} != {flux}",
            fc.flux
        );
        // The same loop under the reversed bit claims the rest of the
        // sphere, which holds both chart poles: the loop's pole-free
        // side contradicts the bit, and the face is refused rather than
        // measured as the complement.
        assert_eq!(
            curved_face(&sphere(c), &octant(c), false, band()).map(|_| ()),
            Err(geom_brep::props::PropsError::SenseContradicted),
            "{label}: the reversed bit"
        );
    }
}

/// A loop that turns back on itself at a vertex has no turning angle
/// there, and is refused rather than integrated.
#[test]
fn a_cusp_is_refused() {
    let c = v3(0.0, 0.0, 0.0);
    // The octant's first arc, there and back: the loop closes, and at
    // both ends the departing tangent is the arriving one reversed.
    let mut cusp = octant(c);
    cusp.truncate(1);
    let back = LoopEdge {
        forward: false,
        start: 1,
        end: 0,
        ..cusp[0].clone()
    };
    cusp.push(back);
    assert_eq!(
        curved_face(&sphere(c), &cusp, true, band()).map(|_| ()),
        Err(geom_brep::props::PropsError::SphereLoop {
            what: "props_sphere_loop_cusp",
        })
    );
}

/// **The bit against the boundary.** A cap `x > ½` on the `z`-poled
/// sphere — one circle tilted against the chart, holding neither pole —
/// is a face whose loop encodes its side: the arm measures it under
/// the bit the loop agrees with (area `2πR·h`, `h = R − ½R`; centred,
/// so the flux is `R·Area`), refuses it as `SenseContradicted` under
/// the other, and `boundary_material_sign` reads the same side. The
/// same circle traversed the other way bounds the complement, which
/// holds both poles: the face is then the cap again, under the
/// reversed bit.
#[test]
fn a_pole_free_tilted_cap_holds_its_sense_against_its_boundary() {
    use geom_brep::props::{MaterialSign, PropsError, boundary_material_sign};
    use geom_core::Sign;
    let sphere = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: R,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rho = R * 0.75_f64.sqrt();
    let rim = |a: f64, b: f64| {
        edge(
            Curve3::Circle {
                center: p3(0.5 * R, 0.0, 0.0),
                axis: v3(1.0, 0.0, 0.0),
                radius: rho,
                u_ref: v3(0.0, 1.0, 0.0),
            },
            a,
            b,
            0,
            0,
        )
    };
    let cap = vec![rim(0.0, 2.0 * PI)];
    let area = 2.0 * PI * R * (0.5 * R);
    let fc = curved_face(&sphere, &cap, true, band()).expect("the cap measures");
    assert!((fc.area - area).abs() < 1e-12, "area {} != {area}", fc.area);
    assert!((fc.flux - R * area).abs() < 1e-12, "flux {}", fc.flux);
    assert_eq!(
        boundary_material_sign(&sphere, &cap, band()),
        Ok(MaterialSign::Encoded(Sign::Positive))
    );
    assert_eq!(
        curved_face(&sphere, &cap, false, band()).map(|_| ()),
        Err(PropsError::SenseContradicted),
        "the flipped bit is refused, not measured as the complement"
    );
    let reversed = vec![rim(2.0 * PI, 0.0)];
    assert_eq!(
        boundary_material_sign(&sphere, &reversed, band()),
        Ok(MaterialSign::Encoded(Sign::Negative))
    );
    let fc = curved_face(&sphere, &reversed, false, band()).expect("the cap, reversed");
    assert!((fc.area - area).abs() < 1e-12, "area {} != {area}", fc.area);
    assert!((fc.flux + R * area).abs() < 1e-12, "flux {}", fc.flux);
    assert_eq!(
        curved_face(&sphere, &reversed, true, band()).map(|_| ()),
        Err(PropsError::SenseContradicted)
    );
}

/// **Holes that overlap bound no face**, under either bit (review of
/// PR 4573). A cap of angular radius 1 rad with the same 0.8 rad hole
/// stated twice has Gauss–Bonnet area `2π(1 − cos 1) − 2·2π(1 − cos 0.8)`
/// < 0 on the side its traversal claims, and `4π(1 − r)` less that on
/// the other: no side of its loops has positive area. Each hole alone
/// is positive and inside the cap, so only the face's own area,
/// decided once (`props_sphere_face_area`), refuses it; the same loops
/// reversed under the other bit are the same statement and refuse the
/// same way, where the face used to measure a negative area.
#[test]
fn holes_that_overlap_bound_no_face_under_either_bit() {
    let s = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let cap = |th: f64, forward: bool| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p3(th.cos(), 0.0, 0.0),
                axis: v3(1.0, 0.0, 0.0),
                radius: th.sin(),
                u_ref: v3(0.0, 1.0, 0.0),
            },
            0.0,
            core::f64::consts::TAU,
            forward,
            0,
            0,
        )
    };
    for sense in [true, false] {
        let (outer, hole) = (cap(1.0, sense), cap(0.8, !sense));
        assert_eq!(
            curved_face_loops(
                &s,
                &[
                    &[outer][..],
                    core::slice::from_ref(&hole),
                    core::slice::from_ref(&hole)
                ],
                sense,
                band()
            )
            .map(|c| c.area),
            Err(geom_brep::props::PropsError::SphereLoop {
                what: "props_sphere_face_area"
            }),
            "sense {sense}"
        );
    }
}

/// **A face larger than a hemisphere measures under the side its
/// boundary encodes** — the sphere less two caps, one stated as the
/// outer loop and one as a ring, each traversed with the face on its
/// left. A face with holes encodes its side by its area's sign, so the
/// bit that claims the face measures `4π − cap₁ − cap₂` and the other
/// bit refuses as inverted. The chart axis is a point of the outer
/// loop and its antipode the centre of the hole, so the face holds no
/// chart pole either.
#[test]
fn the_sphere_less_two_caps_measures_under_its_encoded_side() {
    let tau = core::f64::consts::TAU;
    let (a, b) = (0.5_f64, 0.4_f64);
    let pole = v3(a.cos(), 0.0, a.sin());
    let s = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: R,
        axis: pole,
        u_ref: v3(-a.sin(), 0.0, a.cos()),
    };
    // A whole circle about `axis` at angular radius `th`, run clockwise
    // about it, so the cap it bounds is on its right.
    let around = |axis: Vec3<f64>, th: f64, u_ref: Vec3<f64>| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p3(0.0, 0.0, 0.0) + axis * (R * th.cos()),
                axis,
                radius: R * th.sin(),
                u_ref,
            },
            0.0,
            tau,
            false,
            0,
            0,
        )
    };
    let outer = around(v3(1.0, 0.0, 0.0), a, v3(0.0, 1.0, 0.0));
    let ring = around(-pole, b, v3(0.0, 1.0, 0.0));
    let loops: [&[LoopEdge<f64>]; 2] = [&[outer][..], &[ring][..]];
    let fc = curved_face_loops(&s, &loops, true, band()).expect("the face measures");
    let exact = R * R * (2.0 * tau - tau * (1.0 - a.cos()) - tau * (1.0 - b.cos()));
    assert!(
        (fc.area - exact).abs() < 1e-12 * exact,
        "area {} != {exact}",
        fc.area
    );
    assert!(
        fc.area > 2.0 * tau * R * R / 2.0,
        "the face is the larger side"
    );
    assert_eq!(
        curved_face_loops(&s, &loops, false, band()).map(|c| c.area),
        Err(geom_brep::props::PropsError::SenseInverted)
    );
}

/// The loop of a cap of half-angle `theta` about `d` on the sphere of
/// radius `r` about `c`, in `n` arcs, run counterclockwise about `d`.
fn general_cap(c: Vec3<f64>, r: f64, d: Vec3<f64>, theta: f64, n: usize) -> Vec<LoopEdge<f64>> {
    let d = d * (1.0 / d.norm());
    let u = d.cross(v3(0.0, 0.0, 1.0));
    let u = u * (1.0 / u.norm());
    let step = core::f64::consts::TAU / n as f64;
    (0..n)
        .map(|k| {
            LoopEdge::hand_built(
                Curve3::Circle {
                    center: p3(c.x, c.y, c.z) + d * (r * theta.cos()),
                    axis: d,
                    radius: r * theta.sin(),
                    u_ref: u,
                },
                k as f64 * step,
                (k + 1) as f64 * step,
                true,
                0,
                0,
            )
        })
        .collect()
}

/// **A thin cap in a general pose keeps the fan form's absolute
/// floor** (review of PR 4573). The fan reads a small region from
/// terms the region's own size, so its error is a few ulps of those
/// terms: relative where the region is symmetric about the fan's apex
/// (an axial cap, `sweep/tests/a_thin_cap_keeps_its_digits.rs`), and
/// an absolute floor of some `1e-17` sr in a general pose, where the
/// arcs' points carry their own rounding. Pinned on an off-centre
/// sphere with the cap's axis oblique to every chart and world axis,
/// over solid angles from a hundred times the band to `1e-2` sr stated in 1, 3 and 7
/// arcs, against `4π·sin²(θ/2)`.
#[test]
fn a_thin_cap_in_a_general_pose_keeps_the_fans_absolute_floor() {
    let c = v3(0.3, -1.1, 0.7);
    let d = v3(0.8, 0.5, 0.33);
    let s = Surface::Sphere {
        center: p3(c.x, c.y, c.z),
        radius: R,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    for n in [1usize, 3, 7] {
        // From well clear of the area decide's band (`props_sphere_loop_area`,
        // levered by `R`) up to a hundredth of a steradian.
        let mut solid = 100.0 * band().escalate() / R;
        while solid < 1e-2 {
            let theta = 2.0 * (solid / (4.0 * PI)).sqrt().asin();
            let exact = 4.0 * PI * (theta / 2.0).sin().powi(2);
            let lp = general_cap(c, R, d, theta, n);
            let fc = curved_face(&s, &lp, true, band()).expect("the thin cap");
            // Measured at most 5.8e-17 sr over this sweep, at ε 1e-9 and
            // 1e-12; the pin is the floor, not a relative bound, so a
            // cancellation that scaled with the sphere reds it.
            let err = (fc.area / (R * R) - exact).abs();
            assert!(err <= 2e-16, "{n} arcs, {exact:e} sr: off by {err:e} sr");
            solid *= 10.0;
        }
    }
}
