//! Review probe for the rim-free wedge arm (issue 542): the coplanar
//! arm's blind spot — coincident meridian half-planes decide
//! `props_band_coplanar` Zero and take `Δu = π` — reached through
//! `revolve`. The revolve door's angle headroom is levered at the
//! profile's radial extent `r_max`; the coplanar decide at the sphere
//! radius `R`. A cylinder with a spherical dimple (`r_max ≫ R`) opens a
//! window `10ε/r_max ≤ δ < ε/R` where a `Partial(2π − δ)` revolve is
//! admitted and the dimple's lune is measured at `π` instead of
//! `2π − δ`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::Tol;
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, revolve};

/// The rectangle `[0, w] × [−r, r]` less the half-disc of radius `r`
/// on the axis (the arc bulges into the rectangle), counterclockwise.
fn dimple(r: f64, w: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 0.0),
        ProfileVertex::new(p2(w, -r), 0.0),
        ProfileVertex::new(p2(w, r), 0.0),
        ProfileVertex::new(p2(0.0, r), -1.0),
    ])
}

fn report(label: &str, r: f64, w: f64, angle: f64) {
    let vp = validated(vec![dimple(r, w)]);
    let t = match revolve(&vp, axis_y(), Revolution::Partial(angle), Tol::witness()) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "[bool5r1 {label}] angle = 2pi - {:e}: revolve refused {e:?}",
                TAU - angle
            );
            return;
        }
    };
    let tiers = (
        topo::validate(&t.body),
        topo::validate_closed(&t.body),
        topo::validate_geometric(&t.body, Tol::witness()),
    );
    let exact = angle * (w * w * r - 2.0 / 3.0 * r * r * r);
    let blind = angle * w * w * r - 2.0 / 3.0 * r * r * r * PI;
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
}

#[test]
fn probe_the_dimple_lune_through_revolve_across_the_slit_window() {
    let (r, w) = (0.1, 5.0);
    let eps = Tol::witness().get().eps;
    report("control quarter turn", r, w, FRAC_PI_2);
    // Inside the window: admitted by the door (δ·r_max ≥ 10ε), coincident
    // at the coplanar decide (δ·R < ε).
    let lo = 10.0 * eps / w;
    let hi = eps / r;
    report("slit in the blind window", r, w, TAU - 0.5 * (lo + hi));
    // The coplanar ambiguity band: ε ≤ δ·R < 10ε.
    report("slit in the coplanar band", r, w, TAU - 3.0 * eps / r);
    // Past the band: the wedge arm reads 2π − δ.
    report("slit past the band", r, w, TAU - 50.0 * eps / r);
}
