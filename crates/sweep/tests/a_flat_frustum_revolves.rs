//! **A flat frustum revolves at every ε.** The frustum of half-angle `α`
//! between radii `r0 = S/2 + S·tan α` and `S/2`, `S` high, revolved a
//! full turn about `+y`: its cone wall's seam is a ruling about `S/cos α`
//! long whose anchor sits `S/2` from the cone's apex in radius but only
//! `S/(2·tan α)` in height. The rows are
//! `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`'s table,
//! each of which refused the seam's chart image at ε = 1e-12.
//!
//! Three rows reach past the 1 km session box — `(1.5, 100)`,
//! `(1.55, 30)` and `(1.55, 100)`, their base rims at about 1.46, 1.46
//! and 4.86 km — and assert its refusal (`OutsideSessionBox`) at every ε:
//! their edges' certification asks the box before any residual is read,
//! so `(1.55, 100)`'s return-half rim, which misses its start vertex past
//! the band at ε = 1e-12
//! (`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns`),
//! is refused by the box first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::{validate, validate_closed};

#[test]
fn a_flat_frustum_revolves_at_every_eps() {
    let tol = Tol::witness();
    let mut wrong = Vec::new();
    // (α, S)
    for (alpha, scale) in [
        (1.45, 100.0),
        (1.5, 30.0),
        (1.5, 100.0),
        (1.55, 10.0),
        (1.55, 30.0),
        (1.55, 100.0),
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
        // A frustum whose largest coordinate, `max(r0, S)` about `+y`, is
        // past the 1 km session box is refused there, by the box and
        // nothing else (D4 ¶4), at every ε: its edges' certification asks
        // the box before any residual is read.
        if r0.max(scale) > geom_core::tolerance::SESSION_BOX_EXTENT {
            match got {
                Err(e) if format!("{e:?}").contains("OutsideSessionBox") => {}
                other => wrong.push(format!(
                    "{what}: its rim at {r0:.0} m is past the session box, but: {other:?}"
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
