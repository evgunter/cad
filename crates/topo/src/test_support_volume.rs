//! **The boolean's volume relations, a test oracle.** Whatever a
//! boolean of two bounded solids builds, its volume stands in fixed
//! relations to theirs: vol(A ∪ B) lies between max(vol A, vol B) and
//! vol A + vol B, vol(A ∩ B) between 0 and min(vol A, vol B), and
//! vol(A ∖ B) between max(0, vol A − vol B) and vol A. The kernel does
//! not check them on its own results; the suites check them on theirs,
//! through [`volume_relations`].
//!
//! **What counts as broken.** The relations hold exactly for the true
//! set operation, and a correct result is the true one to within ε of
//! boundary displacement ([tolerance.measure]: the model defines its
//! measures only to that). The true result's boundary lies on the
//! operands' boundaries, so a correct result's volume is the true one
//! to within ε·(area A + area B), and a relation is broken only where
//! the certified volumes miss it by more than that: no ε-correct body
//! measures so. A glue that closes a Zero-band gap moves a correct
//! result's volume by the gap times the glued area, which that slack
//! holds.
//!
//! **Certified volumes.** Each body is measured through the
//! certified measurement door ([`crate::test_support::measure_to`]),
//! first at [`crate::test_support::pinned_resolution`], and again at
//! ε where that leaves a relation or an operand's sign open. A relation
//! is broken where it fails at its enclosures' least favourable ends;
//! one the finest enclosures leave open passes, since nothing here
//! certifies it broken. A relation over an operand applies only where
//! that operand's volume is certified positive: against a complement
//! (a negative volume) the bound says nothing. A closed-form face's
//! enclosure is its value alone, its rounding unstated
//! (`work/fluxtail/a-planar-face-sums-its-area-about-a-far-carrier-origin`),
//! so on a body far from its carriers' origins a flag may be that
//! rounding's.

use geom_core::Tol;

use crate::{Body, BooleanOp};

/// A body's certified volume enclosure and its surface area.
#[derive(Clone, Copy, Debug)]
struct Measured {
    lo: f64,
    hi: f64,
    area: f64,
}

impl Measured {
    /// An empty result: no material, no boundary.
    const EMPTY: Self = Self {
        lo: 0.0,
        hi: 0.0,
        area: 0.0,
    };

    /// Certified positive: a bounded solid.
    fn bounded(self) -> bool {
        self.lo > 0.0
    }

    /// The sign still open.
    fn open(self) -> bool {
        self.lo <= 0.0 && self.hi > 0.0
    }
}

/// Which body a relation's term reads.
#[derive(Clone, Copy)]
enum Term {
    A,
    B,
    Result,
}

/// One relation, `Σ small ≤ Σ large` over the bodies' volumes.
struct Relation {
    which: &'static str,
    small: &'static [Term],
    large: &'static [Term],
}

/// What one relation reads at a set of enclosures.
enum Reading {
    Held,
    Broken(String),
    Open,
}

/// The relations `op` holds over `[a, b, result]`'s measures, those
/// whose operands are certified bounded.
fn relations(op: BooleanOp, a: Measured, b: Measured) -> Vec<Relation> {
    use Term::{A, B, Result as R};
    let rel = |which, small, large| Relation {
        which,
        small,
        large,
    };
    let (pa, pb) = (a.bounded(), b.bounded());
    let mut out = Vec::new();
    match op {
        BooleanOp::Union => {
            if pa {
                out.push(rel("vol(A ∪ B) ≥ vol(A)", &[A], &[R]));
            }
            if pb {
                out.push(rel("vol(A ∪ B) ≥ vol(B)", &[B], &[R]));
            }
            if pa && pb {
                out.push(rel("vol(A ∪ B) ≤ vol(A) + vol(B)", &[R], &[A, B]));
            }
        }
        BooleanOp::Intersect => {
            if pa {
                out.push(rel("vol(A ∩ B) ≤ vol(A)", &[R], &[A]));
            }
            if pb {
                out.push(rel("vol(A ∩ B) ≤ vol(B)", &[R], &[B]));
            }
            if pa || pb {
                out.push(rel("vol(A ∩ B) ≥ 0", &[], &[R]));
            }
        }
        BooleanOp::Subtract => {
            if pa {
                out.push(rel("vol(A ∖ B) ≤ vol(A)", &[R], &[A]));
                out.push(rel("vol(A ∖ B) ≥ 0", &[], &[R]));
            }
            if pa && pb {
                out.push(rel("vol(A ∖ B) ≥ vol(A) − vol(B)", &[A], &[R, B]));
            }
        }
    }
    out
}

/// `relation` at the enclosures `[a, b, result]`, against `slack`.
fn read(relation: &Relation, at: [Measured; 3], slack: f64) -> Reading {
    let of = |t: Term| match t {
        Term::A => at[0],
        Term::B => at[1],
        Term::Result => at[2],
    };
    let sum = |terms: &[Term], end: fn(Measured) -> f64| terms.iter().map(|&t| end(of(t))).sum();
    let (large_lo, large_hi): (f64, f64) =
        (sum(relation.large, |m| m.lo), sum(relation.large, |m| m.hi));
    let (small_lo, small_hi): (f64, f64) =
        (sum(relation.small, |m| m.lo), sum(relation.small, |m| m.hi));
    if large_hi - small_lo < -slack {
        Reading::Broken(format!(
            "{}: the larger side measures at most {large_hi:e} and the smaller at least \
             {small_lo:e}, short by more than ε·(area A + area B) = {slack:e}, the most a \
             correct result's volume can move",
            relation.which
        ))
    } else if large_lo - small_hi >= -slack {
        Reading::Held
    } else {
        Reading::Open
    }
}

/// `body` measured at a resolution of `len` metres of mean boundary
/// displacement.
#[allow(clippy::panic)]
fn measured(what: &str, body: &Body<f64>, len: f64, tol: Tol) -> Measured {
    let settle = crate::Settle::Resolution(quantity::Length::from_meters(len));
    match crate::test_support::measure_to(body, settle, tol) {
        Ok((_, e)) => Measured {
            lo: e.volume_lo,
            hi: e.volume_hi,
            area: e.surface_area,
        },
        Err(refusal) => panic!("the volume oracle cannot measure {what}: {refusal}"),
    }
}

/// **The boolean's volume relations over `op`'s operands `a`, `b` and
/// its `result`** (`None` an empty one), read as the module docs say:
/// the first relation the certified volumes break, as text naming it
/// and its measures, or `Ok`.
///
/// # Errors
///
/// The relation broken.
///
/// # Panics
///
/// Where a body cannot be measured: the oracle then says nothing about
/// it, and a row that asked it must hear so.
pub fn volume_relations(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    result: Option<&Body<f64>>,
    tol: Tol,
) -> Result<(), String> {
    for len in [crate::test_support::pinned_len(tol), tol.eps()] {
        let at = [
            measured("the first operand", a, len, tol),
            measured("the second operand", b, len, tol),
            result.map_or(Measured::EMPTY, |r| measured("the result", r, len, tol)),
        ];
        let slack = tol.eps() * (at[0].area + at[1].area);
        let mut open = at[0].open() || at[1].open();
        for relation in relations(op, at[0], at[1]) {
            match read(&relation, at, slack) {
                Reading::Broken(why) => return Err(format!("{op:?}: {why}")),
                Reading::Open => open = true,
                Reading::Held => {}
            }
        }
        if !open {
            break;
        }
    }
    Ok(())
}

/// [`volume_relations`], panicking on a broken relation with `what`
/// naming the case.
///
/// # Panics
///
/// Where a relation is broken, or a body cannot be measured.
#[allow(clippy::panic)]
pub fn assert_volume_relations(
    what: &str,
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    result: Option<&Body<f64>>,
    tol: Tol,
) {
    if let Err(why) = volume_relations(op, a, b, result, tol) {
        panic!("{what}: the result breaks a volume relation: {why}");
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use geom_core::Tol;

    use super::volume_relations;
    use crate::test_support_fixtures::brick;
    use crate::{Body, BooleanOp};

    fn unit_footprint(z: f64) -> Body<f64> {
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, z), Tol::witness())
    }

    /// `result` breaks `which` and the oracle names it.
    fn breaks(
        op: BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        result: Option<&Body<f64>>,
        which: &str,
    ) {
        let got = volume_relations(op, a, b, result, Tol::witness());
        assert!(
            got.as_ref().is_err_and(|why| why.contains(which)),
            "{op:?}: wanted {which} broken, got {got:?}"
        );
    }

    /// **Each relation flags a planted result that breaks it**: a union
    /// smaller than an operand or larger than both together, an
    /// intersect larger than an operand or inside out, a subtract
    /// larger than its first operand, smaller than the difference of
    /// the two, or inside out, and an empty union of bounded operands.
    #[test]
    fn a_planted_result_breaks_its_relation() {
        let (cube, half, quarter) = (
            unit_footprint(1.0),
            unit_footprint(0.5),
            unit_footprint(0.25),
        );
        let tall = unit_footprint(1.5);
        let inside_out = cube.revert().unwrap();
        breaks(
            BooleanOp::Union,
            &cube,
            &cube,
            Some(&half),
            "vol(A ∪ B) ≥ vol(A)",
        );
        breaks(
            BooleanOp::Union,
            &half,
            &half,
            Some(&tall),
            "vol(A ∪ B) ≤ vol(A) + vol(B)",
        );
        breaks(BooleanOp::Union, &cube, &half, None, "vol(A ∪ B) ≥ vol(A)");
        breaks(
            BooleanOp::Intersect,
            &half,
            &cube,
            Some(&cube),
            "vol(A ∩ B) ≤ vol(A)",
        );
        breaks(
            BooleanOp::Intersect,
            &cube,
            &cube,
            Some(&inside_out),
            "vol(A ∩ B) ≥ 0",
        );
        breaks(
            BooleanOp::Subtract,
            &half,
            &cube,
            Some(&cube),
            "vol(A ∖ B) ≤ vol(A)",
        );
        breaks(
            BooleanOp::Subtract,
            &cube,
            &half,
            Some(&quarter),
            "vol(A ∖ B) ≥ vol(A) − vol(B)",
        );
        breaks(
            BooleanOp::Subtract,
            &cube,
            &half,
            Some(&inside_out),
            "vol(A ∖ B) ≥ 0",
        );
    }

    /// **Equal volumes hold every relation**, each being non-strict, and
    /// a complement operand bounds nothing: `A ∩ revert(B)` may exceed
    /// `vol(revert B)`, while the bound by `A` still applies.
    #[test]
    fn equal_volumes_and_a_complement_operand_hold() {
        let (cube, half) = (unit_footprint(1.0), unit_footprint(0.5));
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let got = volume_relations(op, &cube, &cube, Some(&cube), Tol::witness());
            assert!(got.is_ok(), "{op:?}: {got:?}");
        }
        let complement = half.revert().unwrap();
        let got = volume_relations(
            BooleanOp::Intersect,
            &cube,
            &complement,
            Some(&cube),
            Tol::witness(),
        );
        assert!(got.is_ok(), "the complement bounds nothing: {got:?}");
        breaks(
            BooleanOp::Intersect,
            &half,
            &complement,
            Some(&cube),
            "vol(A ∩ B) ≤ vol(A)",
        );
    }

    /// **A wrong component hidden by a large area is flagged**: a
    /// 2 m × 2 m × 0.1 m plate's `plate ∖ plate`
    /// "result" that wrongly kept `3·ε·(area A + area B)` of material,
    /// about 17.6 m² of boundary over which it is a mean displacement of
    /// `3ε` (a 3.75 mm cube at the default ε), as a lift of the whole
    /// top. A lift inside ε of one face, a correct result's own freedom,
    /// passes.
    #[test]
    fn a_wrong_component_on_a_large_plate_is_flagged() {
        let tol = Tol::witness();
        let plate = |z: f64| brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, z), tol);
        let kept = 3.0 * tol.eps() * 2.0 * (8.0 + 0.8);
        let wrong = plate(0.1 + kept / 4.0);
        breaks(
            BooleanOp::Subtract,
            &plate(0.1),
            &plate(0.1),
            Some(&wrong),
            "vol(A ∖ B) ≤ vol(A)",
        );
        let within = plate(0.1 + 0.5 * tol.eps());
        let got = volume_relations(
            BooleanOp::Subtract,
            &plate(0.1),
            &plate(0.1),
            Some(&within),
            tol,
        );
        assert!(
            got.is_ok(),
            "a lift inside ε is a correct result's: {got:?}"
        );
    }
}
