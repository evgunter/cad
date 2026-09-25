//! Review probe for the rim-free wedge arm (issue 542): the coplanar
//! arm's coincident pair — meridian half-planes a hair apart decide
//! `props_band_coplanar` Zero — reached through `revolve`. The revolve
//! door's angle headroom is levered at the profile's radial extent
//! `r_max`; the coplanar decide at the sphere radius `R`. A cylinder
//! with a spherical dimple (`r_max ≫ R`) opens a window
//! `10ε/r_max ≤ δ < ε/R` where a `Partial(2π − δ)` revolve is admitted
//! and the dimple's lune reaches the coplanar branch. Adopted by the
//! unit with the window's row inverted: the branch refuses the pair
//! typed (`props_band_opposite`, the loop reversing at the poles) where
//! it measured it at `π`; the ambiguity band escalates; past it the
//! wedge arm reads `2π − δ`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::revolve_common;

use geom_core::Tol;
use profile::{ProfileLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, revolve};

/// The trapezoid `(0, ∓r) – (w, ∓(r + 1))` less the half-disc of radius
/// `r` on the axis (the arc bulges into it; the slanted sides keep the
/// pole joints non-tangent), counterclockwise. Revolved by `α` its
/// volume is `α·(r·w² + (2/3)·w² − (2/3)·r³)` by Pappus.
fn dimple(r: f64, w: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (p2(0.0, -r), 0.0),
        (p2(w, -r - 1.0), 0.0),
        (p2(w, r + 1.0), 0.0),
        (p2(0.0, r), -1.0),
    ])
}

/// What the props door answers for the dimple revolved by `angle`,
/// printed with the tiers and the two closed forms (the exact one and
/// the one a lune measured at `π` would give); `None` when `revolve`
/// itself refuses the angle, with its refusal printed.
fn report(label: &str, r: f64, w: f64, angle: f64) -> Option<Result<f64, topo::MassPropsError>> {
    let vp = validated(vec![dimple(r, w)]);
    let t = match revolve(&vp, axis_y(), Revolution::Partial(angle), Tol::witness()) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "[bool5r1 {label}] angle = 2pi - {:e}: revolve refused {e:?}",
                TAU - angle
            );
            return None;
        }
    };
    let tiers = (
        topo::validate(&t.body),
        topo::validate_closed(&t.body),
        topo::validate_geometric(&t.body, Tol::witness()),
    );
    let solid = r * w * w + 2.0 / 3.0 * w * w;
    let exact = angle * (solid - 2.0 / 3.0 * r * r * r);
    let blind = angle * solid - 2.0 / 3.0 * r * r * r * PI;
    let mp = topo::mass_properties(&t.body, Tol::witness()).map(|m| m.volume);
    println!(
        "[bool5r1 {label}] angle = 2pi - {:e}: tiers = {tiers:?}; volume = {mp:?}; exact = {exact:.15e}; \
         with the lune at pi = {blind:.15e}",
        TAU - angle
    );
    if let Ok(v) = mp {
        let rel_exact = (v - exact).abs() / exact;
        let rel_blind = (v - blind).abs() / blind;
        println!(
            "[bool5r1 {label}] rel to exact {rel_exact:.3e}, rel to the blind answer {rel_blind:.3e}"
        );
    }
    Some(mp)
}

fn dimple_exact(r: f64, w: f64, angle: f64) -> f64 {
    angle * (r * w * w + 2.0 / 3.0 * w * w - 2.0 / 3.0 * r * r * r)
}

fn is_slit_refusal(got: &Result<f64, topo::MassPropsError>) -> bool {
    matches!(
        got,
        Err(topo::MassPropsError::Face {
            source: geom_brep::PropsError::NotIsoRectangle { what },
            ..
        }) if what.starts_with("a rimless sphere face whose coplanar meridians share one half-plane")
    )
}

#[test]
fn probe_the_dimple_lune_through_revolve_across_the_slit_window() {
    let (r, w) = (0.1, 5.0);
    let eps = Tol::witness().get().eps;
    let control = report("control quarter turn", r, w, FRAC_PI_2)
        .expect("the control revolves")
        .expect("the control measures");
    let exact = dimple_exact(r, w, FRAC_PI_2);
    assert!(
        (control - exact).abs() / exact < 1e-12,
        "control {control:.15e} vs {exact:.15e}"
    );
    // Inside the window: admitted by the door (δ·r_max ≥ 10ε), coincident
    // at the coplanar decide (δ·R < ε) — and refused there, typed.
    let lo = 10.0 * eps / w;
    let hi = eps / r;
    let window = report("slit in the blind window", r, w, TAU - 0.5 * (lo + hi))
        .expect("the window's angle is definite at the revolve door");
    assert!(
        is_slit_refusal(&window),
        "the hairline slit refuses typed at the coplanar branch, got {window:?}"
    );
    // The coplanar ambiguity band, ε ≤ δ·R < 10ε: an escalation, at
    // whichever door meets the hairline first — `revolve`'s own pcurve
    // certification (the near-coincident caps' trim containment sits
    // in the same band) or, if the body builds, the coplanar decide.
    match report("slit in the coplanar band", r, w, TAU - 3.0 * eps / r) {
        None => {}
        Some(banded) => assert!(
            matches!(
                banded,
                Err(topo::MassPropsError::Face {
                    source: geom_brep::PropsError::Escalated { .. },
                    ..
                })
            ),
            "the coplanar band escalates, got {banded:?}"
        ),
    }
    // Past the band: the wedge arm reads 2π − δ.
    let angle = TAU - 50.0 * eps / r;
    let past = report("slit past the band", r, w, angle)
        .expect("the angle is definite at the revolve door")
        .expect("the wedge arm measures");
    let exact = dimple_exact(r, w, angle);
    assert!(
        (past - exact).abs() / exact < 1e-9,
        "past the band {past:.15e} vs {exact:.15e}"
    );
}
