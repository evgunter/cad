//! **A partial-revolve wedge through a full-turn collar**, under every
//! boolean.
//!
//! The collar is the rectangle `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved a
//! full turn about `y`: its bore and its outer wall are each ONE face,
//! a whole turn wide. The wedge is a rectangle `ρ ∈ [ρ₀, ρ₁]`,
//! `y ∈ [y₀, y₁]` revolved a partial turn about the same axis, so every
//! section it cuts in the collar's walls is a rim arc, and every chord
//! a wall face takes divides a face whose azimuth extent is a full
//! period. The chord takes the arc the germs it joins name; no window
//! of the divided face enters.
//!
//! The oracle is closed form: the two solids share the annular sector
//! `θ/2 · (b² − a²) · (d − c)`, with `[a, b]` and `[c, d]` the overlaps of
//! their `ρ` and `y` ranges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

/// The rectangle `ρ ∈ [r0, r1]`, `y ∈ [y0, y1]` in the profile plane.
fn rect(r0: f64, r1: f64, y0: f64, y1: f64) -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(r0, y0), 0.0),
        (Point2::new(r1, y0), 0.0),
        (Point2::new(r1, y1), 0.0),
        (Point2::new(r0, y1), 0.0),
    ]
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let tol = Tol::witness();
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, tol),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, tol),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, tol),
    }
    .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"));
    out.body()
        .unwrap_or_else(|| panic!("{op:?} came back empty"))
        .body
        .clone()
}

/// Every wedge of the matrix — radial ranges inside the bore, across
/// the bore, across the collar and past both walls; height ranges
/// inside the collar, across either cap and past both; three angles —
/// under ∪, ∩, collar ∖ wedge and wedge ∖ collar, each held to tiers 2
/// and 3 and to the closed form.
#[test]
fn every_wedge_through_a_full_turn_collar_builds_under_every_boolean() {
    let tol = Tol::witness();
    let collar = revolved_about_y(rect(0.5, 1.5, 1.0, 2.0), Revolution::Full, tol);
    let vc = PI * (1.5f64.powi(2) - 0.5f64.powi(2));
    for (r0, r1) in [(0.3, 0.9), (0.2, 0.7), (0.4, 1.2), (0.3, 1.8)] {
        for (y0, y1) in [(1.2, 1.8), (0.5, 1.5), (1.5, 2.5), (0.5, 2.5)] {
            for angle in [PI / 2.0, 1.0, 4.0] {
                let wedge = revolved_about_y(rect(r0, r1, y0, y1), Revolution::Partial(angle), tol);
                let vw = angle / 2.0 * (r1 * r1 - r0 * r0) * (y1 - y0);
                let (a, b) = (r0.max(0.5), r1.min(1.5));
                let (c, d) = (y0.max(1.0), y1.min(2.0));
                let shared = angle / 2.0 * (b * b - a * a) * (d - c);
                for (op_name, op, x, y, want) in [
                    ("∪", BooleanOp::Union, &collar, &wedge, vc + vw - shared),
                    ("∩", BooleanOp::Intersect, &collar, &wedge, shared),
                    (
                        "collar ∖ wedge",
                        BooleanOp::Subtract,
                        &collar,
                        &wedge,
                        vc - shared,
                    ),
                    (
                        "wedge ∖ collar",
                        BooleanOp::Subtract,
                        &wedge,
                        &collar,
                        vw - shared,
                    ),
                ] {
                    let what = format!("ρ ({r0}, {r1}), y ({y0}, {y1}), θ {angle:.3}: {op_name}");
                    let body = run(op, x, y);
                    assert_eq!(topo::validate_closed(&body), Ok(()), "{what}: tier 2");
                    assert_eq!(
                        topo::validate_geometric(&body, tol),
                        Ok(()),
                        "{what}: tier 3"
                    );
                    let got = topo::mass_properties(&body, tol)
                        .unwrap_or_else(|e| panic!("{what}: mass properties, got {e:?}"))
                        .volume;
                    assert!(
                        (got - want).abs() <= 1e-9 * want.max(1.0),
                        "{what}: volume {got} against the closed form {want}"
                    );
                }
            }
        }
    }
}
