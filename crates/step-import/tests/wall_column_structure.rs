//! **A wall's boundary column is not a recognizer's negative answer.**
//!
//! `adopt.rs`'s two iso rungs — the `IsoCurve` candidate rung and the
//! ARC-rim residual gate — each extract a described NURBS wall's own
//! boundary column through `geom_brep::boundary_iso_u` /
//! `boundary_iso_v`. Those doors are control-net COPIES whose only
//! refusal is a net that disagrees with the knot vector it is indexed
//! by, and `geom::NurbsSurface::new` refuses exactly that net at
//! construction — so the refusal is unreachable from any body this
//! reader assembles, and it says nothing about whether the edge is the
//! shape the rung is looking for.
//!
//! This suite pins the three halves of that reading, each executed
//! rather than asserted in prose:
//!
//! - every described NURBS wall the reader actually produces extracts
//!   both columns at both ends;
//! - the nets that WOULD break extraction are refused one layer up, at
//!   the surface door, so no wall reaches the rungs in that state;
//! - and when the refusal is carried anyway, it names WHICH invariant
//!   broke, which is the whole reason it is carried rather than
//!   discarded.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{SOLID_FIXTURES, import_body};
use geom::{NurbsSurface, Surface};
use geom_core::Point3;
use geom_core::spline::{KnotVector, SplineError};
use step_import::StepImportError;

/// A clamped knot vector with `count` control points at `degree`,
/// uniform interior — the shape every wall in the written vocabulary
/// has.
fn kv(count: usize, degree: usize) -> KnotVector {
    let spans = count - degree;
    let mut knots = vec![0.0; degree + 1];
    #[allow(clippy::cast_precision_loss)]
    for k in 1..spans {
        knots.push(k as f64 / spans as f64);
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    KnotVector::clamped(knots, degree).expect("a uniform clamped vector forms")
}

/// A described wall with `nu × nv` control points at the given
/// degrees, weights `w` (cycled) — deliberately NON-square so a
/// row/column mix-up in either door shows up as a count mismatch
/// rather than passing by symmetry.
fn wall(nu: usize, nv: usize, du: usize, dv: usize, w: &[f64]) -> NurbsSurface<f64> {
    let knots_u = kv(nu, du);
    let knots_v = kv(nv, dv);
    let mut control = Vec::with_capacity(nu * nv);
    let mut weights = Vec::with_capacity(nu * nv);
    for iu in 0..nu {
        for iv in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            control.push(Point3::new(iu as f64, iv as f64, (iu * iv) as f64 * 0.25));
            weights.push(w[(iu * nv + iv) % w.len()]);
        }
    }
    NurbsSurface::new(knots_u, knots_v, control, weights).expect("the wall constructs")
}

/// The claim the two `adopt.rs` rungs stand on, executed: a wall that
/// exists hands over both of its columns, at both ends. Run over the
/// hand-built shapes AND over every described NURBS surface the reader
/// produces from the committed corpus.
#[test]
fn a_validated_wall_never_refuses_its_own_boundary_column() {
    fn check(what: &str, s: &NurbsSurface<f64>) {
        for end in [false, true] {
            geom_brep::boundary_iso_u(s, end)
                .unwrap_or_else(|e| panic!("{what}: boundary_iso_u(end={end}) refused: {e}"));
            geom_brep::boundary_iso_v(s, end)
                .unwrap_or_else(|e| panic!("{what}: boundary_iso_v(end={end}) refused: {e}"));
        }
    }

    // Non-square nets, both degree orders, polynomial and rational —
    // and the placeholder, which the rungs skip but which is still a
    // surface the body can hold.
    check("bilinear", &wall(2, 2, 1, 1, &[1.0]));
    check("2x5 linear-by-cubic", &wall(2, 5, 1, 3, &[1.0]));
    check("5x2 cubic-by-linear", &wall(5, 2, 3, 1, &[1.0]));
    check(
        "rational 3x4",
        &wall(3, 4, 2, 3, &[1.0, 0.5, 2.0, 0.25, 7.0]),
    );
    check("placeholder", &NurbsSurface::<f64>::placeholder());

    // The real reader's own output: every described NURBS wall the
    // committed corpus lands in a body.
    let mut walls = 0usize;
    for name in SOLID_FIXTURES {
        let (body, _) = import_body(name);
        for (key, surface) in body.surfaces() {
            if let Surface::Nurbs(p) = surface {
                check(&format!("{name} surface {key:?}"), p.as_ref());
                walls += 1;
            }
        }
    }
    assert!(
        walls > 0,
        "the corpus rows found no NURBS wall — this row would then be \
         about the hand-built nets alone"
    );
}

/// The other half of the same claim: the nets that WOULD break a
/// column extraction never become a wall, because the surface door
/// refuses them. This is what makes the carried refusal unreachable
/// rather than merely unobserved — weaken the door's validation and
/// this row reddens.
#[test]
fn a_net_that_would_break_a_column_is_refused_at_the_surface_door() {
    let knots_u = kv(3, 2);
    let knots_v = kv(4, 3);
    let ok_control = vec![Point3::new(0.0, 0.0, 0.0); 12];

    // One control point short: `boundary_iso_u` would slice a row of
    // four out of eleven and `NurbsCurve3::new` would answer
    // ControlCountMismatch — if the wall existed.
    let short = vec![Point3::new(0.0, 0.0, 0.0); 11];
    assert!(
        matches!(
            NurbsSurface::new(knots_u.clone(), knots_v.clone(), short, vec![1.0; 11]),
            Err(SplineError::ControlCountMismatch { .. })
        ),
        "a net that disagrees with its knot vectors must refuse at construction"
    );

    // Weight counts and weight values, the door's other two clauses —
    // a column copies the wall's weights verbatim, so a wall that
    // holds a bad one hands it to `NurbsCurve3::new`.
    assert!(
        matches!(
            NurbsSurface::new(
                knots_u.clone(),
                knots_v.clone(),
                ok_control.clone(),
                vec![1.0; 11]
            ),
            Err(SplineError::WeightCountMismatch { .. })
        ),
        "a weight count that disagrees with the net must refuse at construction"
    );
    for (index, bad) in [0.0, -1.0, f64::NAN, f64::INFINITY].into_iter().enumerate() {
        let mut weights = vec![1.0; 12];
        weights[index] = bad;
        assert!(
            NurbsSurface::new(
                knots_u.clone(),
                knots_v.clone(),
                ok_control.clone(),
                weights
            )
            .is_err(),
            "weight {bad} must refuse at construction"
        );
    }
}

/// Why the refusal is carried rather than discarded: the rendered
/// message names WHICH structural invariant the wall broke, so a
/// kernel-bug report says more than that a kernel bug happened.
/// Dropping the `{source}` interpolation reddens this immediately.
#[test]
fn the_refusal_names_which_invariant_the_wall_broke() {
    let source = SplineError::ControlCountMismatch {
        control: 11,
        expected: 12,
    };
    // Built from its own type, never read back out of the enum that
    // wraps it.
    let inner = format!("{source}");
    let rendered = format!(
        "{}",
        StepImportError::WallColumnStructure { id: 4271, source }
    );
    assert!(
        rendered.contains("#4271"),
        "the refusal must name the edge: {rendered}"
    );
    assert!(
        rendered.contains(&inner),
        "the refusal must carry the extraction door's own words \
         ({inner}), got: {rendered}"
    );
}
