//! Reviewer r1 probes for PR #3984 (frozen head 8abb6e7931). Included
//! locally as a child of `boolean/reduce/planar_lane_carrier_rows.rs`
//! (`#[cfg(test)] #[path = "<abs>/probes/r1_probes.rs"] mod r1_probes;`)
//! so they drive the sweep's arms directly, as the PR's own harness
//! does, without touching the operand gate. Oracles are closed forms in
//! f64, never the kernel's evaluator.
//!
//! Spiric closed form (geom `Curve3::Spiric` docs): with centre 0,
//! axis ẑ, u_ref x̂ (so m = ŷ), P(v) = (off, √(ρ² − off²), r sin v),
//! ρ = R + r cos v.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::BooleanError;
use crate::boolean::{ContactRecords, Operand};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::test_support_fixtures::brick;
use crate::{Body, EdgeKey};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Tol, Vec3};

fn tol() -> Tol {
    Tol::witness()
}

/// One sweep direction: `x`'s edges against `y`'s faces.
fn sweep(
    x: &Body<f64>,
    y: &Body<f64>,
    x_is: Operand,
) -> Result<ContactRecords, BooleanError> {
    let (mut x, mut y) = (x.clone(), y.clone());
    let mut acc = super::super::ContactAcc::default();
    let band = geom_core::Band::linear(tol()).unwrap();
    super::super::sweep_direction(
        &mut x,
        &mut y,
        x_is,
        &crate::boolean::DeclaredPairs::default(),
        &mut acc,
        band,
        super::super::SweepStrategy::Realized,
        &super::super::SweepKnobs::default(),
        None,
        &mut Vec::new(),
        tol(),
    )?;
    Ok(acc.finish())
}

/// Both directions, `a` as `A`.
fn settle(a: &Body<f64>, b: &Body<f64>) -> Result<ContactRecords, BooleanError> {
    let (mut a, mut b) = (a.clone(), b.clone());
    let mut acc = super::super::ContactAcc::default();
    let band = geom_core::Band::linear(tol()).unwrap();
    super::super::sweep_and_settle(
        &mut a,
        &mut b,
        &crate::boolean::DeclaredPairs::default(),
        &mut acc,
        band,
        super::super::SweepStrategy::Realized,
        [&super::super::SweepKnobs::default(); 2],
        [None, None],
        tol(),
    )?;
    Ok(acc.finish())
}

/// A planar cap in `x = off`, bounded by the spiric arc of the torus
/// `(R, r)` about ẑ over `v ∈ [v0, v1]` and the chord back (census's
/// `spiric_cap`, re-spelled, scaled by `s`).
fn spiric_cap(s: f64, big_r: f64, r: f64, off: f64, (v0, v1): (f64, f64)) -> (Body<f64>, EdgeKey) {
    let spiric = Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: s * big_r,
        minor_radius: s * r,
        offset: s * off,
    };
    let (p0, p1) = (spiric.eval(v0), spiric.eval(v1));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin: Point3::new(s * off, 0.0, 0.0),
                    normal: Vec3::unit_x(),
                    u_ref: Vec3::unit_y(),
                },
                sense: true,
            },
        )
        .unwrap();
    let torus = body.add_surface(Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: s * big_r,
        minor_radius: s * r,
        u_ref: Vec3::unit_x(),
    });
    let arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: torus,
                    s2: plane,
                    witness: spiric.eval(0.5 * (v0 + v1)),
                },
                carrier: spiric,
                param_start: v0,
                param_end: v1,
            },
            tol(),
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: arc.he_minus,
            he2: arc.he_plus,
        },
        EdgeCurveSpec::line_between(p1, p0),
        FaceSurface::Shared {
            key: plane,
            sense: true,
        },
        tol(),
    )
    .unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol()).unwrap();
    (body, arc.edge)
}

/// Oracle: the spiric `(R, r, off)` at minor angle `v`.
fn spiric_at(big_r: f64, r: f64, off: f64, v: f64) -> (f64, f64, f64) {
    let rho = big_r + r * v.cos();
    (off, (rho * rho - off * off).sqrt(), r * v.sin())
}

/// Oracle: `(y, z)` in the cap of the torus `(R, r)` in `x = off`, the
/// part of the section's +y oval on the far side of the chord at
/// `y = y_chord`.
fn in_cap(big_r: f64, r: f64, off: f64, y_chord: f64, y: f64, z: f64) -> bool {
    let rho = (off * off + y * y).sqrt();
    y > y_chord && (rho - big_r).powi(2) + z * z < r * r
}

fn expect_crossing_refusal(got: Result<ContactRecords, BooleanError>, operand: Operand, edge: EdgeKey, what: &str) {
    match got {
        Err(BooleanError::CrossingCarrierUnsupported { operand: o, edge: e, .. }) => {
            assert_eq!(o, operand, "{what}: operand");
            assert_eq!(e, edge, "{what}: the refusal names the curved edge");
        }
        other => panic!("{what}: expected CrossingCarrierUnsupported, got {other:?}"),
    }
}

/// **P1. A spiric dipping through a brick face refuses (planar arm).**
/// Scales ×1e-3, ×1, ×1e3. The arc over `v ∈ [−π/2, π/2]` ends at
/// `y = √3.75 ≈ 1.936` and peaks at `y = √8.75 ≈ 2.958`; the brick face
/// `y = 2.5` (outward −ŷ) is crossed twice, at `z = ±√(1 − (√6.5 − 2)²)`
/// ≈ ±0.836, inside the face (z ∈ [−1.2, 1.2]): same-side ends.
#[test]
fn p1_spiric_dip_through_plane_face_refuses() {
    use core::f64::consts::FRAC_PI_2;
    let (big_r, r, off) = (2.0, 1.0, 0.5);
    let (_, y_end, _) = spiric_at(big_r, r, off, FRAC_PI_2);
    let (_, y_mid, _) = spiric_at(big_r, r, off, 0.0);
    assert!(y_end < 2.5 && y_mid > 2.5, "oracle: a dip, {y_end} {y_mid}");
    let zc = (1.0 - (6.5f64.sqrt() - 2.0).powi(2)).sqrt();
    assert!(zc < 1.2, "oracle: crossings at z = ±{zc} land in the face");
    for s in [1e-3, 1.0, 1e3] {
        let (a, e) = spiric_cap(s, big_r, r, off, (-FRAC_PI_2, FRAC_PI_2));
        let b: Body<f64> = brick((0.0, s), (2.5 * s, 4.0 * s), (-1.2 * s, 1.2 * s), tol());
        expect_crossing_refusal(sweep(&a, &b, Operand::A), Operand::A, e, &format!("P1 s={s}"));
        // Both operand orders: the cap as B.
        expect_crossing_refusal(settle(&b, &a), Operand::B, e, &format!("P1 swapped s={s}"));
    }
}

/// **P2. A spiric crossing a cylinder wall refuses (curved arm).** The
/// arc's ρ runs 2 → 3 → 2; the wall `ρ = 2.5` is crossed twice.
#[test]
fn p2_spiric_vs_cylinder_wall_refuses() {
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    use core::f64::consts::{FRAC_PI_2, PI};
    let (a, e) = spiric_cap(1.0, 2.0, 1.0, 0.5, (-FRAC_PI_2, FRAC_PI_2));
    let mut b = Body::<f64>::new();
    cyl_wall_sheet(&mut b, CylFrame::canonical(2.5), None, (0.0, PI), (-2.0, 2.0), tol());
    expect_crossing_refusal(sweep(&a, &b, Operand::A), Operand::A, e, "P2");
}

/// **P3. The other operand's straight edges piercing a spiric-bounded
/// planar face: the reverse direction the PR's rows do not run.** The
/// brick's four x-parallel edges pierce `x = ½` at `(y, z)` corners of
/// `[y0, y1] × [−zh, zh]`; the oracle decides each corner in or out of
/// the cap. Where every corner is in, four VF contacts are owed; where
/// every corner is out, none. A refusal is acceptable; a wrong count is
/// not.
#[test]
fn p3_brick_edges_pierce_spiric_cap() {
    use core::f64::consts::FRAC_PI_2;
    let (big_r, r, off) = (2.0, 1.0, 0.5);
    let (_, y_chord, _) = spiric_at(big_r, r, off, FRAC_PI_2);
    // (y0, y1, zh): all in; all out beyond the oval; all out below the chord
    // but inside the oval's hull; straddling.
    let cases = [
        (2.2, 2.6, 0.3),
        (2.2, 2.6, 0.95),
        (1.5, 1.8, 0.3),
        (2.9, 3.1, 0.1),
    ];
    for s in [1e-3, 1.0, 1e3] {
        let (cap, _) = spiric_cap(s, big_r, r, off, (-FRAC_PI_2, FRAC_PI_2));
        for &(y0, y1, zh) in &cases {
            let corners = [(y0, -zh), (y0, zh), (y1, -zh), (y1, zh)];
            let owed = corners
                .iter()
                .filter(|&&(y, z)| in_cap(big_r, r, off, y_chord, y, z))
                .count();
            let b: Body<f64> = brick((0.0, s), (y0 * s, y1 * s), (-zh * s, zh * s), tol());
            match sweep(&b, &cap, Operand::B) {
                Ok(c) => {
                    eprintln!("P3 s={s} case=({y0},{y1},{zh}) owed={owed} got b_on_a={} vv={}", c.b_on_a.len(), c.vv.len());
                    assert_eq!(c.b_on_a.len(), 2 * owed, "P3 (sheet: two faces, one per side) s={s} ({y0},{y1},{zh})");
                }
                Err(err) => eprintln!("P3 s={s} case=({y0},{y1},{zh}) owed={owed} REFUSED {err:?}"),
            }
        }
    }
}

/// **P4. The NURBS sheet as B under a brick whose edges pierce it** —
/// the PR's own Bézier fixture (`mx = 1, h = 1`, so `y = x(2 − x)`),
/// read in the direction its rows skip. The chord is `y = 0`, so the
/// region read as a chord-polygon has no interior at all.
#[test]
fn p4_brick_edges_pierce_nurbs_sheet() {
    let cases = [
        // (x0, x1, y0, y1): inside; outside above the arc; straddling the arc
        ((0.9, 1.1), (0.3, 0.5)),
        ((0.9, 1.1), (1.2, 1.4)),
        ((0.15, 0.25), (0.5, 0.7)),
        ((0.4, 0.6), (0.6, 0.8)),
    ];
    let (sheet, _) = super::arc_sheet(1.0, 1.0);
    for &((x0, x1), (y0, y1)) in &cases {
        let corners = [(x0, y0), (x0, y1), (x1, y0), (x1, y1)];
        let owed = corners
            .iter()
            .filter(|&&(x, y)| y > 0.0 && y < x * (2.0 - x))
            .count();
        let b: Body<f64> = brick((x0, x1), (y0, y1), (-1.0, 1.0), tol());
        match sweep(&b, &sheet, Operand::B) {
            Ok(c) => {
                eprintln!("P4 case=({x0},{x1},{y0},{y1}) owed={owed} got b_on_a={}", c.b_on_a.len());
                assert_eq!(c.b_on_a.len(), owed, "P4 ({x0},{x1},{y0},{y1})");
            }
            Err(err) => eprintln!("P4 case=({x0},{x1},{y0},{y1}) owed={owed} REFUSED {err:?}"),
        }
        // Full two-direction sweep, both orders: must refuse, naming the arc.
        assert!(
            matches!(settle(&b, &sheet), Err(BooleanError::CrossingCarrierUnsupported { operand: Operand::B, .. } | BooleanError::ArcLoopContainmentUnsupported { operand: Operand::A, .. })),
            "P4 settle brick-first: {:?}",
            settle(&b, &sheet).err()
        );
        assert!(
            matches!(settle(&sheet, &b), Err(BooleanError::CrossingCarrierUnsupported { operand: Operand::A, .. })),
            "P4 settle sheet-first: {:?}",
            settle(&sheet, &b).err()
        );
    }
}

/// **P5. A spiric whose box clears every face of the other operand
/// passes the sweep silently** — the over-refusal is box-level, so
/// this is the case that reaches the downstream (join, ring, section)
/// sites. The brick lies at y ∈ [−4, −3], far from the +y oval.
#[test]
fn p5_far_spiric_passes_the_sweep() {
    use core::f64::consts::FRAC_PI_2;
    let (cap, _) = spiric_cap(1.0, 2.0, 1.0, 0.5, (-FRAC_PI_2, FRAC_PI_2));
    let b: Body<f64> = brick((0.0, 1.0), (-4.0, -3.0), (-0.5, 0.5), tol());
    let got = settle(&cap, &b);
    eprintln!("P5 far spiric: {got:?}");
    assert!(got.is_ok(), "P5: {got:?}");
}

/// **P6. Rotated pose**: P1 with both bodies turned about ẑ by 0.7 rad
/// and moved; the refusal must survive the re-pose.
#[test]
fn p6_rotated_spiric_dip_refuses() {
    use core::f64::consts::FRAC_PI_2;
    let (a, e) = spiric_cap(1.0, 2.0, 1.0, 0.5, (-FRAC_PI_2, FRAC_PI_2));
    let b: Body<f64> = brick((0.0, 1.0), (2.5, 4.0), (-1.2, 1.2), tol());
    let m = geom_core::Affine3::rotation_about_axis(Point3::new(3.0, -2.0, 5.0), Vec3::unit_z(), 0.7);
    let (ra, rb) = (
        crate::transform_rigid(&a, &m, tol()).expect("pose A"),
        crate::transform_rigid(&b, &m, tol()).expect("pose B"),
    );
    let got = sweep(&ra, &rb, Operand::A);
    eprintln!("P6: {got:?}");
    match got {
        Err(BooleanError::CrossingCarrierUnsupported { .. }) => {}
        other => panic!("P6: {other:?}"),
    }
    let _ = e;
}
