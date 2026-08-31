//! Issue 555: the far point's engineered exact-zero v-coordinate goes
//! sub-floor, and an ordinary annular cap refuses `Triangulation`.
//!
//! The consumer that found it is the tour's Klein bottle (`klein.rs`,
//! wall 7): the bulb's inner-tube top rim is a plain slit annulus at
//! plain coordinates on a tier-3-valid body, and it refused at every δ
//! — whether it refused turning on the bulb's flare angle and bottom
//! rim radius, parameters the cap's own geometry does not depend on.
//! That e2e witness lives in the demo; these rows carry the same
//! signature in-crate, where the mechanism can be stated exactly:
//!
//! `chart_frame` builds `u` as the far point's own rejection from the
//! normal, so `far` lies in span{normal, u} and `far · (normal × u)`
//! is a determinant with a repeated row — zero in exact arithmetic.
//! With off-plane position noise ν the float residue is ~ν², which for
//! ν ≲ √(2⁻¹⁴²) ≈ 4e-22 is nonzero below spade's coordinate floor.
//! Below the floor spade's `insert` refuses, and the face refuses with
//! it, at every δ.
//!
//! These rows exercise the fix through the FULL projection path
//! (`chart_frame` + `tessellate_planar`'s per-point projection), which
//! is where the write is sited; a stored-chart replay would not reach
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::Body;

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .expect("profile validation")
}

/// A prism whose sketch frame is skewed by ν, so every boundary
/// position carries off-plane noise `z = x·ν` and no exactly-equal
/// input column exists — the signature's minimal carrier.
fn skewed_prism(nu: f64, poly: &[(f64, f64)], h: f64) -> Body<f64> {
    skewed_prism_ringed(nu, poly, &[], h)
}

/// The same, with an optional inner ring — the ringed/concentric
/// geometry that seats boundary vertices on anchor→through-chord
/// diagonals by symmetry rather than by accident.
fn skewed_prism_ringed(nu: f64, outer: &[(f64, f64)], ring: &[(f64, f64)], h: f64) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, nu),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let mut loops = vec![lp(outer)];
    if !ring.is_empty() {
        loops.push(lp(ring));
    }
    extrude(
        &validated(plane, loops),
        Extrusion::Distance(h),
        Tol::witness(),
    )
    .expect("extrude")
    .body
}

/// The rectangle whose diagonal is the far point: anchor (0,0), far
/// (2,1), so `u` is built from the diagonal and the far point's v is
/// the engineered zero.
const RECT: [(f64, f64); 4] = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)];

/// The class, across the whole refusal band and out both sides of it.
///
/// Every ν here used to refuse below the floor and tessellate above
/// it; the band is now closed and the volume oracle holds throughout,
/// so the row is a statement about the fix rather than about where the
/// floor happens to sit. δ is swept alongside because the defect was
/// δ-independent — a tolerance the caller could not turn — and a fix
/// that only worked at one δ would be a different fix.
#[test]
fn subfloor_far_point_v_no_longer_refuses_at_any_noise_scale_or_delta() {
    for exp in [-15i32, -20, -22, -25, -30, -40, -43, -45, -50, -60] {
        for delta in [2e-2, 1e-2, 5e-3, 2e-3] {
            let body = skewed_prism(10.0f64.powi(exp), &RECT, 1.0);
            let label = format!("noise-1e{exp} delta-{delta}");
            let m = tessellate(&body, delta, Tol::witness())
                .unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
            assert_eq!(check_mesh(&m), Ok(()), "{label}: not watertight");
            let v = signed_volume(&m);
            assert!((v - 2.0).abs() < 1e-9, "{label}: volume {v}");
        }
    }
}

/// The RINGED half of the class, which the sited far-point write does
/// NOT reach and the `mitigate_underflow` floor does.
///
/// A point lying on an anchor→through-chord diagonal has an exact-zero
/// v by GEOMETRY rather than by the frame's construction, so it carries
/// the same ~ν² residue and lands in the same sub-floor band — but the
/// frame does not know which point that is, and the write is scoped to
/// the one point it can name. On concentric geometry those points are
/// seated by SYMMETRY, not by accident: both fixtures below put ring
/// vertices on such diagonals, and both refused across the ν ladder
/// with the far-point write alone.
///
/// Same ν² law and same ~1e-20…1e-22 threshold as the far-point defect,
/// which is the evidence that it is one class and not two.
#[test]
fn ringed_and_concentric_charts_no_longer_refuse_sub_floor() {
    // A 2x1 plate with a rectangular hole, and a square annulus.
    let plate_ring = [(0.5, 0.25), (1.5, 0.25), (1.5, 0.75), (0.5, 0.75)];
    let sq_outer = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sq_ring = [(-0.4, -0.4), (-0.4, 0.4), (0.4, 0.4), (0.4, -0.4)];
    for exp in [-15i32, -20, -22, -25, -30, -45, -50, -60] {
        let nu = 10.0f64.powi(exp);
        for (name, body, want) in [
            (
                "plate-with-hole",
                skewed_prism_ringed(nu, &RECT, &plate_ring, 1.0),
                2.0 - 0.5,
            ),
            (
                "square-annulus",
                skewed_prism_ringed(nu, &sq_outer, &sq_ring, 1.0),
                4.0 - 0.64,
            ),
        ] {
            let label = format!("{name} noise-1e{exp}");
            let m = tessellate(&body, 1e-2, Tol::witness())
                .unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
            assert_eq!(check_mesh(&m), Ok(()), "{label}: not watertight");
            let v = signed_volume(&m);
            assert!((v - want).abs() < 1e-9, "{label}: volume {v}, want {want}");
        }
    }
}

/// The slit annulus the Klein consumer actually refuses on, met
/// through the door that produces one: a full-2π revolve, whose cap
/// loop traverses its seam twice. The write is keyed on POSITION BITS
/// precisely so a 3-D point met twice stays one chart point, and this
/// row is the end-to-end statement that the cancellation still holds
/// under the write — watertight, and the annulus's own volume.
///
/// The unit rows in `mesh::planar`'s test module pin the bit-keying
/// itself, on a frame whose far point is a repeated position; this row
/// cannot choose which loop point comes out farthest, so it asserts
/// the consequence rather than the mechanism.
#[test]
fn the_slit_annulus_cap_survives_the_written_zero() {
    use geom_core::Vec2;
    use sweep::{Revolution, RevolveAxis, revolve};
    let (r_in, r_out, h) = (0.225f64, 0.275f64, 0.4f64);
    let profile = validated(
        SketchPlane::xy(),
        vec![lp(&[(r_in, 0.0), (r_out, 0.0), (r_out, h), (r_in, h)])],
    );
    let body = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("revolve")
    .body;
    let m = tessellate(&body, 1e-2, Tol::witness()).expect("slit annulus tessellates");
    assert_eq!(check_mesh(&m), Ok(()), "slit annulus not watertight");
    // A chorded annulus under-fills the true volume; accept the chord
    // deficit, reject a seam that leaked or double-counted.
    let exact = core::f64::consts::PI * (r_out * r_out - r_in * r_in) * h;
    let v = signed_volume(&m);
    assert!(
        v > 0.985 * exact && v <= exact,
        "slit annulus volume {v} against exact {exact}"
    );
}

/// Determinism (D9) across the closed band: the write must not make
/// the projection depend on anything but the point, so a rebuild is
/// bit-identical.
#[test]
fn the_written_zero_keeps_the_rebuild_bit_identical() {
    fn dump(m: &mesh::Mesh) -> String {
        let mut s = String::new();
        for p in &m.positions {
            s.push_str(&format!(
                "{:016x}{:016x}{:016x}",
                p.x.to_bits(),
                p.y.to_bits(),
                p.z.to_bits()
            ));
        }
        for patch in &m.patches {
            for t in &patch.triangles {
                s.push_str(&format!("{};{};{}|", t[0], t[1], t[2]));
            }
        }
        s
    }
    for exp in [-30i32, -45] {
        let body = skewed_prism(10.0f64.powi(exp), &RECT, 1.0);
        let a = tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
        let b = tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
        assert_eq!(dump(&a), dump(&b), "noise-1e{exp}: rebuild not identical");
    }
}
