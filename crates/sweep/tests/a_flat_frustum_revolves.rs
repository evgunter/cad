//! **A flat frustum revolves at every ε.** The frustum of half-angle `α`
//! between radii `r0 = S/2 + S·tan α` and `S/2`, `S` high, revolved a
//! full turn about `+y`: its cone wall's seam is a ruling about `S/cos α`
//! long whose anchor sits `S/2` from the cone's apex in radius but only
//! `S/(2·tan α)` in height. The rows are
//! `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`'s table,
//! each of which refused the seam's chart image at ε = 1e-12.
//!
//! One row is not this door's: at `(1.55, 100)` the far cap rim is 4858
//! across, and its return half ends `r·2·(π − fl(π))` from its start
//! vertex, past the band at ε = 1e-12
//! (`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns`).
//! Below its least ε the row asserts that refusal, the rim's
//! `EndpointEnd` escalation, so it goes red when that item lands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::{validate, validate_closed};

#[test]
fn a_flat_frustum_revolves_at_every_eps() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let mut wrong = Vec::new();
    // (α, S, the least ε the operand revolves at)
    for (alpha, scale, builds_from) in [
        (1.45, 100.0, 0.0),
        (1.5, 30.0, 0.0),
        (1.5, 100.0, 0.0),
        (1.55, 10.0, 0.0),
        (1.55, 30.0, 0.0),
        (1.55, 100.0, 1e-9),
    ] {
        let what = format!("alpha {alpha}, S {scale}");
        let r1 = 0.5 * scale;
        let r0 = r1 + scale * f64::tan(alpha);
        let lp = ProfileLoop::polygon(
            [(0.0, 0.0), (r0, 0.0), (r1, scale), (0.0, scale)]
                .into_iter()
                .map(|(x, y)| Point2::new(x, y)),
        );
        let got = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol);
        if eps < builds_from {
            // The return-half rim's own refusal, and no other.
            match got {
                Err(e) if format!("{e:?}").contains("EndpointEnd") => {}
                Err(e) => wrong.push(format!(
                    "{what}: at ε {eps:e} refuses other than its return-half rim: {e:?}"
                )),
                Ok(_) => wrong.push(format!(
                    "{what}: revolves at ε {eps:e}, below {builds_from:e}: lower its least ε"
                )),
            }
            continue;
        }
        match got {
            Ok(r) => {
                if let Err(e) = validate(&r.body).and_then(|()| validate_closed(&r.body)) {
                    wrong.push(format!("{what}: {e:?}"));
                }
            }
            Err(e) => wrong.push(format!("{what}: {e:?}")),
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
