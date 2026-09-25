//! Review probes for the rim-free spherical wedge arm (issue 542):
//! the structural arc rule under re-statements of the same loop, a
//! tilted off-origin wedge, the coplanar arm's coincident-plane ladder
//! at the band, a split meridian on a wedge, and a full-circle pair.
//! Adopted by the unit with the blind-spot rows re-aimed at the typed
//! refusals the fix pass added (`props_band_opposite` for a coincident
//! pair, the pole-to-pole premise for a full-circle pair).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::{band, eps};
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{CarrierId, LoopEdge, PropsError, curved_face};
use geom_core::{Point3, Real, Vec3};

const RS: f64 = 0.010;
const H: f64 = FRAC_PI_2;

fn sphere<T: Real>() -> Surface<T> {
    surf::sphere(RS)
}

fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

/// The same meridian half-plane at azimuth `u`, stated on the REVERSED
/// carrier (axis `−n`): its parameter is MINUS the latitude, so the
/// stored `t0` sits at the other pole. Traversed from latitude `lat0`
/// to `lat1`.
fn great_flipped(u: f64, lat0: f64, lat1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(-u.sin(), u.cos(), 0.0),
            radius: RS,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        -lat0,
        -lat1,
        a,
        b,
    )
}

fn expect_lune(kind: &str, fc: geom_brep::props::FaceContribution<f64>, du: f64, sense: bool) {
    let area = 2.0 * RS * RS * du;
    let rel = (fc.area - area).abs() / area;
    assert!(rel < 1e-12, "{kind}: area {:.15e} != {area:.15e}", fc.area);
    let s_f = if sense { 1.0 } else { -1.0 };
    let volume = s_f * (2.0 / 3.0) * RS.powi(3) * du;
    let rel = (fc.flux / 3.0 - volume).abs() / volume.abs();
    assert!(
        rel < 1e-12,
        "{kind}: flux/3 {:.15e} != {volume:.15e}",
        fc.flux / 3.0
    );
}

/// The arc rule under four re-statements of one wedge: the arcs listed
/// in the other order, arc A on the reversed carrier (both arcs then
/// `forward = true`), arc B on the reversed carrier (both `false`), and
/// both reversed. Angles include 3π/2 and 7π/4, both senses.
#[test]
fn probe_the_arc_rule_is_invariant_under_loop_restatement() {
    let band = band();
    for theta in [FRAC_PI_6, FRAC_PI_2, 1.5 * PI, 1.75 * PI] {
        let variants: [(&str, Vec<LoopEdge<f64>>); 5] = [
            (
                "natural",
                vec![great(0.0, H, -H, 0, 1), great(theta, -H, H, 1, 0)],
            ),
            (
                "rotated",
                vec![great(theta, -H, H, 1, 0), great(0.0, H, -H, 0, 1)],
            ),
            (
                "A flipped (both forward)",
                vec![great_flipped(0.0, H, -H, 0, 1), great(theta, -H, H, 1, 0)],
            ),
            (
                "B flipped (both reversed)",
                vec![great(0.0, H, -H, 0, 1), great_flipped(theta, -H, H, 1, 0)],
            ),
            (
                "both flipped",
                vec![
                    great_flipped(0.0, H, -H, 0, 1),
                    great_flipped(theta, -H, H, 1, 0),
                ],
            ),
        ];
        for (name, edges) in &variants {
            let fwd: Vec<bool> = edges.iter().map(|e| e.forward).collect();
            for sense in [true, false] {
                let du = if sense { theta } else { TAU - theta };
                let kind =
                    format!("theta = {theta:.4}, {name}, forward = {fwd:?}, sense = {sense}");
                let fc = curved_face(&sphere::<f64>(), edges, sense, band)
                    .unwrap_or_else(|e| panic!("{kind}: refused: {e:?}"));
                expect_lune(&kind, fc, du, sense);
            }
        }
    }
}

fn rodrigues(k: Vec3<f64>, ang: f64, v: Vec3<f64>) -> Vec3<f64> {
    let (s, c) = ang.sin_cos();
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// A wedge of a ball with R = 0.37 about a tilted axis, centred off the
/// origin: area `2R²Δu`, flux `s_f·R·area + c·V` with `V` the covered
/// lune's vector area, rotated. Both senses, both directions, four
/// angles.
#[test]
fn probe_tilted_off_origin_wedge_measures_by_the_closed_form() {
    let band = band();
    let r = 0.37;
    let k = Vec3::new(1.0, 2.0, -1.0) * (1.0 / 6.0_f64.sqrt());
    let ang = 0.9;
    let rot = |v: Vec3<f64>| rodrigues(k, ang, v);
    let c = Point3::new(0.3, -0.2, 0.7);
    let axis = rot(Vec3::new(0.0, 0.0, 1.0));
    let u_ref = rot(Vec3::new(1.0, 0.0, 0.0));
    let s = Surface::Sphere {
        center: c,
        radius: r,
        axis,
        u_ref,
    };
    let meridian = |u: f64, t0: f64, t1: f64, a: u32, b: u32| {
        topo::edge(
            Curve3::Circle {
                center: c,
                axis: rot(Vec3::new(u.sin(), -u.cos(), 0.0)),
                radius: r,
                u_ref: rot(Vec3::new(u.cos(), u.sin(), 0.0)),
            },
            t0,
            t1,
            a,
            b,
        )
    };
    for theta in [FRAC_PI_6, FRAC_PI_2, 1.5 * PI, 1.75 * PI] {
        for reversed in [false, true] {
            for sense in [true, false] {
                let edges = if reversed {
                    vec![meridian(theta, H, -H, 0, 1), meridian(0.0, -H, H, 1, 0)]
                } else {
                    vec![meridian(0.0, H, -H, 0, 1), meridian(theta, -H, H, 1, 0)]
                };
                let du = if reversed == sense {
                    TAU - theta
                } else {
                    theta
                };
                let start = if reversed { theta } else { 0.0 };
                let (u0, u1) = if sense {
                    (start, start + du)
                } else {
                    (start - du, start)
                };
                let kind = format!("theta = {theta:.4}, reversed = {reversed}, sense = {sense}");
                let fc = curved_face(&s, &edges, sense, band)
                    .unwrap_or_else(|e| panic!("{kind}: refused: {e:?}"));
                let area = 2.0 * r * r * du;
                assert!(
                    (fc.area - area).abs() / area < 1e-12,
                    "{kind}: area {:.15e} != {area:.15e}",
                    fc.area
                );
                let s_f = if sense { 1.0 } else { -1.0 };
                let va = rot(Vec3::new(u1.sin() - u0.sin(), u0.cos() - u1.cos(), 0.0))
                    * (s_f * 0.5 * PI * r * r);
                let flux = s_f * r * area + (c - Point3::origin()).dot(va);
                assert!(
                    (fc.flux - flux).abs() / flux.abs() < 1e-11,
                    "{kind}: flux {:.15e} != {flux:.15e}",
                    fc.flux
                );
            }
        }
    }
}

/// A wedge whose `u = 0` meridian is a genuinely SPLIT edge (two pieces
/// carrying one `CarrierId`, abutting at the equator): geometrically the
/// quarter wedge, refused as unfolded. The torus arm folds such pieces
/// by lineage; the sphere wedge arm reads a two-edge boundary and says
/// so.
#[test]
fn probe_a_split_meridian_wedge_refuses_as_unfolded() {
    let band = band();
    let mut a1: LoopEdge<f64> = great(0.0, H, 0.0, 0, 1);
    let mut a2: LoopEdge<f64> = great(0.0, 0.0, -H, 1, 2);
    a1.carrier_id = Some(CarrierId::minted(7));
    a2.carrier_id = Some(CarrierId::minted(7));
    let b = great(FRAC_PI_2, -H, H, 2, 0);
    let got = curved_face(&sphere::<f64>(), &[a1, a2, b], true, band).map(|fc| fc.area);
    println!("[bool5r1] split-meridian wedge: {got:?}");
    assert!(
        matches!(got, Err(PropsError::NotIsoRectangle { what }) if what.starts_with("the wedge arm reads a two-edge boundary")),
        "{got:?}"
    );
}

/// Two FULL great circles (span exactly 2π, pole back to the same pole)
/// on two planes, stated as a two-edge loop: the wedge arm's premise
/// that each arc runs pole to pole is decided, not inherited — a full
/// circle has the far pole INSIDE its span (`props_meridian_pole`
/// Positive) and refuses under the premise's name. The same pair on
/// ONE plane is coplanar and runs its circle there and back, which
/// `props_band_opposite` refuses as the slit it is.
#[test]
fn probe_two_full_circle_meridians_refuse_the_pole_to_pole_premise() {
    let band = band();
    let edges: Vec<LoopEdge<f64>> = vec![
        great(0.0, -H, 3.0 * H, 0, 1),
        great(FRAC_PI_2, 3.0 * H, -H, 1, 0),
    ];
    assert_eq!(
        curved_face(&sphere::<f64>(), &edges, true, band).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "a rimless wedge's meridians run pole to pole"
        })
    );
    let coplanar: Vec<LoopEdge<f64>> =
        vec![great(0.0, -H, 3.0 * H, 0, 1), great(0.0, 3.0 * H, -H, 1, 0)];
    assert_eq!(
        curved_face(&sphere::<f64>(), &coplanar, true, band).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "a rimless sphere face whose coplanar meridians share one half-plane — a \
                   slit the flux lane does not measure"
        })
    );
}

/// The coincident-plane ladder: the second meridian at azimuth
/// `k·zero/R` for k across the band. The coplanar decide runs first,
/// so a slit narrower than `zero/R` reaches the coplanar branch — where
/// `props_band_opposite` refuses it typed, the loop reversing at the
/// poles — the ambiguity band escalates, and past `escalate` the wedge
/// arm reads the slit's own width.
fn ladder<T: Real + geom_core::Decide>(label: &str) -> Vec<(f64, Result<T, PropsError>)> {
    let band = band();
    let zero_over_r = eps() / RS;
    let mut out = Vec::new();
    for k in [0.0, 0.5, 1.0, 2.0, 5.0, 9.99, 10.0, 10.01, 20.0] {
        let delta = k * zero_over_r;
        let edges: Vec<LoopEdge<T>> = vec![great(0.0, H, -H, 0, 1), great(delta, -H, H, 1, 0)];
        let got = curved_face(&sphere::<T>(), &edges, true, band).map(|fc| fc.area);
        println!(
            "[bool5r1 {label}] eps = {:e}, k = {k}: delta = {delta:e}, area = {got:?}",
            eps()
        );
        out.push((k, got));
    }
    out
}

#[test]
fn probe_the_coincident_plane_ladder_at_f64() {
    let rows = ladder::<f64>("f64");
    for (k, got) in rows {
        match k {
            k if k <= 0.5 => assert!(
                matches!(
                    got,
                    Err(PropsError::NotIsoRectangle { what })
                        if what.starts_with("a rimless sphere face whose coplanar meridians share one half-plane")
                ),
                "k = {k}: a slit narrower than zero/R refuses typed, got {got:?}"
            ),
            2.0 | 5.0 => assert!(
                matches!(got, Err(PropsError::Escalated { .. })),
                "k = {k}: {got:?}"
            ),
            20.0 => {
                let area = got.expect("past escalate the wedge arm measures the slit");
                let exact = 2.0 * RS * RS * (k * eps() / RS);
                assert!(
                    (area - exact).abs() / exact < 1e-9,
                    "k = {k}: {area} vs {exact}"
                );
            }
            _ => {}
        }
    }
}

#[test]
fn probe_the_coincident_plane_ladder_at_interval() {
    use geom_core::{Bounds, Interval};
    let rows = ladder::<Interval>("interval");
    for (k, got) in rows {
        match k {
            k if k <= 0.5 => assert!(
                matches!(
                    got,
                    Err(PropsError::NotIsoRectangle { what })
                        if what.starts_with("a rimless sphere face whose coplanar meridians share one half-plane")
                ),
                "k = {k}: a slit narrower than zero/R refuses typed, got {got:?}"
            ),
            2.0 | 5.0 => assert!(
                matches!(got, Err(PropsError::Escalated { .. })),
                "k = {k}: {got:?}"
            ),
            20.0 => {
                let area = got.expect("past escalate the wedge arm measures the slit");
                let exact = 2.0 * RS * RS * (k * eps() / RS);
                assert!(
                    area.lo() <= exact && exact <= area.hi(),
                    "k = {k}: {area:?} vs {exact}"
                );
            }
            _ => {}
        }
    }
}
