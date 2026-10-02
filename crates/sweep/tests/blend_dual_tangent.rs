//! **The blend's tangent channel**: a fillet and a chamfer built at
//! `Dual64` carry the derivative of the body's volume, not only its
//! value.
//!
//! The blend doors stay generic so that a blend is differentiable; a
//! value-channel comparison cannot see a tangent that the build drops
//! (a size read through its value into an `f64` and lifted back as a
//! constant gives the right body with a zero derivative). These rows
//! read the volume's `deriv` and compare it with central differences
//! of the `f64` build and, for the filleted cube, with the closed form.
//!
//! Two seeds, because a frozen tangent can sit on either side of the
//! door: the blend size itself, and an upstream size (the cube's side,
//! seeded through the profile and the extrusion distance) that reaches
//! the blend only through the faces it consumes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Decide, Dual, Dual64, Point2, Tol};
use sweep::blend::{chamfer_edges, fillet_edges};
use sweep::test_support::{cube, prism};
use topo::{Body, query};

/// The blend's verb, so one driver serves both doors.
#[derive(Clone, Copy, Debug)]
enum Verb {
    Fillet,
    Chamfer,
}

/// Every edge of `body` blended by `verb` at `size`.
fn blended<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    body: &Body<T>,
    verb: Verb,
    size: T,
) -> Body<T> {
    let edges = query::all_edges(body);
    assert_eq!(edges.len(), 12, "a box has twelve edges");
    match verb {
        Verb::Fillet => {
            fillet_edges(body, &edges, size, Tol::witness())
                .expect("the filleted cube")
                .body
        }
        Verb::Chamfer => {
            chamfer_edges(body, &edges, size, Tol::witness())
                .expect("the chamfered cube")
                .body
        }
    }
}

/// The volume through the closed-form door, the one a dual can take.
fn volume<T: Decide>(body: &Body<T>) -> T {
    topo::mass_properties_structural(body, Tol::witness())
        .expect("a planar/cylindrical/spherical blended cube measures in closed form")
        .volume
}

/// A cube of side `l` built through the profile and the extrusion, so
/// a seeded `l` reaches every face of the body.
fn extruded_cube<T: Decide>(l: T) -> Body<T> {
    let z = T::zero();
    let verts = vec![
        (Point2::new(z, z), z),
        (Point2::new(l, z), z),
        (Point2::new(l, l), z),
        (Point2::new(z, l), z),
    ];
    prism(verts, l, Tol::witness())
}

/// The central-difference step.
const H: f64 = 1e-6;

/// The agreement the tangent owes the central difference. The volume
/// is a cubic polynomial in either size, so the truncation error
/// `h²/6 · |V'''|` is below `1e-11`; the rounding error is
/// `ε·|V| / h ≈ 1e-10`, and the measured disagreement is at most
/// `2.6e-10` over every row here. `1e-7` sits nearly three decades
/// above that and several below the `O(1)` error of a dropped tangent.
const FD_TOL: f64 = 1e-7;

/// The agreement the tangent owes a closed form: the dual's arithmetic
/// is the closed form's up to rounding (measured at most `9e-16`).
const CF_TOL: f64 = 1e-12;

/// Central difference of `f` at `x`.
fn central(f: impl Fn(f64) -> f64, x: f64) -> f64 {
    (f(x + H) - f(x - H)) / (2.0 * H)
}

/// `dV/dr` of the unit cube with all twelve edges filleted at `r`:
/// `V = 1 − (12 − 3π) r² + (16 − 14π/3) r³`, from core + six slabs +
/// twelve quarter-cylinders + eight octants (`m5_pr12_die_body`).
fn filleted_unit_cube_dvdr(r: f64) -> f64 {
    -12.0 * (1.0 - PI / 4.0) * (2.0 * r - 6.0 * r * r) - 24.0 * (1.0 - PI / 6.0) * r * r
}

/// `dV/dl` of the cube of side `l` with every edge filleted at a
/// fixed `r`: `3c² + 12rc + 3πr²` with `c = l − 2r`.
fn filleted_cube_dvdl(l: f64, r: f64) -> f64 {
    let c = l - 2.0 * r;
    3.0 * c * c + 12.0 * r * c + 3.0 * PI * r * r
}

const RADII: [f64; 3] = [0.1, 0.15, 0.3];

/// Fails with every disagreement at once, so a mutant's reach over
/// both verbs and every radius reads off one run.
fn assert_none(misses: &[String]) {
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

#[test]
fn the_blend_size_tangent_matches_central_differences() {
    let mut misses = Vec::new();
    for verb in [Verb::Fillet, Verb::Chamfer] {
        for &s in &RADII {
            let v = volume(&blended(
                &cube::<Dual64>(1.0, Tol::witness()),
                verb,
                Dual::variable(s),
            ));
            let v_f = volume(&blended(&cube::<f64>(1.0, Tol::witness()), verb, s));
            assert_eq!(
                v.value.to_bits(),
                v_f.to_bits(),
                "{verb:?} at {s}: the value channel"
            );
            let fd = central(
                |x| volume(&blended(&cube::<f64>(1.0, Tol::witness()), verb, x)),
                s,
            );
            assert!(
                fd.abs() > 1e-3,
                "{verb:?} at {s}: a pin needs a nonzero slope"
            );
            if (v.deriv - fd).abs() > FD_TOL {
                misses.push(format!(
                    "{verb:?} at {s}: dV/ds = {} vs central difference {fd}",
                    v.deriv
                ));
            }
        }
    }
    assert_none(&misses);
}

#[test]
fn the_filleted_cube_tangent_matches_the_closed_form() {
    let mut misses = Vec::new();
    for &r in &RADII {
        let v = volume(&blended(
            &cube::<Dual64>(1.0, Tol::witness()),
            Verb::Fillet,
            Dual::variable(r),
        ));
        let want = filleted_unit_cube_dvdr(r);
        if (v.deriv - want).abs() > CF_TOL {
            misses.push(format!("dV/dr at {r}: {} vs closed form {want}", v.deriv));
        }
    }
    assert_none(&misses);
}

#[test]
fn an_upstream_size_tangent_reaches_through_the_blend() {
    let l = 1.0;
    let mut misses = Vec::new();
    for verb in [Verb::Fillet, Verb::Chamfer] {
        for &s in &RADII {
            let v = volume(&blended(
                &extruded_cube(Dual::variable(l)),
                verb,
                Dual::constant(s),
            ));
            let fd = central(|x| volume(&blended(&extruded_cube(x), verb, s)), l);
            if (v.deriv - fd).abs() > FD_TOL {
                misses.push(format!(
                    "{verb:?} at {s}: dV/dl = {} vs central difference {fd}",
                    v.deriv
                ));
            }
            if let Verb::Fillet = verb {
                let want = filleted_cube_dvdl(l, s);
                if (v.deriv - want).abs() > CF_TOL {
                    misses.push(format!(
                        "Fillet at {s}: dV/dl = {} vs closed form {want}",
                        v.deriv
                    ));
                }
            }
        }
    }
    assert_none(&misses);
}
