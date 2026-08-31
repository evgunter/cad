//! R1 review probes for CERT-6 — the certified perimeter lower bound.
//! NOT for merge; reviewer's probe branch only.
//!
//! `boundary_chord_perimeter_lo` takes its evaluator as a closure, so
//! it can be driven with ANALYTIC surfaces whose true perimeter is
//! known in closed form. That isolates the bound's own mathematics
//! from the kernel's evaluation: soundness (is it really a LOWER
//! bound) and tightness (how far under the truth can it sit, which is
//! what the gauge's 52x margin is actually spending).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::quad::{RVec3, boundary_chord_perimeter_lo};
use geom_core::RingInterval;

fn p(x: f64, y: f64, z: f64) -> RVec3 {
    [
        RingInterval::point(x),
        RingInterval::point(y),
        RingInterval::point(z),
    ]
}

/// Sanity: the flat unit square. True perimeter 4; the chord polygon
/// on straight edges is exact, so the bound should be 4 to rounding.
#[test]
fn r1_flat_square_bound_is_sound_and_tight() {
    let lo = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), 16, (&[], &[]), |u, v| p(u, v, 0.0));
    println!("R1PERIM flat_square p_lo={lo} true=4");
    assert!(lo <= 4.0 + 1e-12, "must not OVERstate: {lo}");
    assert!(lo > 3.999_999, "should be tight on straight edges: {lo}");
}

/// A full-revolution wrap: the u0 and u1 edges map to one seam. The
/// PR's stated reason for sampling rather than corner-only.
#[test]
fn r1_wrapped_cylinder_bound() {
    let f = |u: f64, v: f64| {
        let t = std::f64::consts::TAU * u;
        p(t.cos(), t.sin(), v)
    };
    for n in [1usize, 4, 16, 64, 256] {
        let lo = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), n, (&[], &[]), f);
        println!("R1PERIM wrapped n={n} p_lo={lo}");
    }
    // The true traversal is two unit circles plus the seam TWICE
    // (the polygon walks u=u1 up and u=u0 back down, and those are the
    // same physical curve on a wrap).
    let truth = 2.0 * std::f64::consts::TAU + 2.0;
    let lo16 = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), 16, (&[], &[]), f);
    println!(
        "R1PERIM wrapped truth(with seam counted twice)={truth} ratio={}",
        lo16 / truth
    );
    assert!(lo16 <= truth, "must not OVERstate");
}

/// **The aliasing attack.** The chord sampling is UNIFORM in the
/// parameter and fixed at 16 per side, with no coupling to the
/// patch's knot structure — while the area grid this bound is
/// compared against is `knot_aligned_cuts`. A boundary whose
/// oscillation is commensurate with 16 is therefore invisible to the
/// bound: every sample lands on a zero crossing and the chord polygon
/// reads a straight line.
#[test]
fn r1_oscillatory_boundary_aliases_the_sixteen_chord_bound() {
    for k in [
        1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 16.0, 17.0, 24.0, 32.0, 34.0,
    ] {
        let f = move |u: f64, v: f64| {
            // z oscillates on the v0 and v1 edges only.
            let bump = if v <= 0.0 || v >= 1.0 {
                (std::f64::consts::TAU * k * u).sin()
            } else {
                0.0
            };
            p(u, v, bump)
        };
        let lo16 = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), 16, (&[], &[]), f);
        let lo4096 = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), 4096, (&[], &[]), f);
        println!(
            "R1PERIM osc k={k} p_lo(16)={lo16:.6} p_lo(4096)={lo4096:.6} understatement={:.3}x \
             gauge-margin-consumed={:.1}x",
            lo4096 / lo16,
            (lo4096 / lo16).powi(2)
        );
    }
}

/// A small face far from the origin: the chord differences are formed
/// in the ring, so at large coordinates the enclosure noise can swamp
/// a short chord and every `lo()` clamps to zero. If the bound can
/// reach 0 this way, the gauge silently downgrades to the 1e3
/// relative fallback with nothing said.
#[test]
fn r1_small_face_far_from_origin() {
    for (size, offset) in [
        (1e-3, 0.0f64),
        (1e-6, 0.0),
        (1e-9, 1e6),
        (1e-12, 1e6),
        (1e-12, 1e9),
        (1e-15, 1e9),
    ] {
        let f = move |u: f64, v: f64| p(offset + size * u, offset + size * v, 0.0);
        let lo = boundary_chord_perimeter_lo((0.0, 1.0, 0.0, 1.0), 16, (&[], &[]), f);
        println!(
            "R1PERIM tiny size={size:e} offset={offset:e} p_lo={lo:e} true={:e} positive={}",
            4.0 * size,
            lo > 0.0
        );
    }
}
