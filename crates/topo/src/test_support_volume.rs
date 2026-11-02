//! **The boolean's volume relations, a test oracle.** Whatever a
//! boolean of two bounded solids builds, its volume stands in fixed
//! relations to theirs: vol(A ∪ B) lies between max(vol A, vol B) and
//! vol A + vol B, vol(A ∩ B) between 0 and min(vol A, vol B), and
//! vol(A ∖ B) between max(0, vol A − vol B) and vol A. The kernel does
//! not check them on its own results; the suites check them on theirs,
//! through [`VolumeOracle`] (or [`volume_relations`] for one op).
//!
//! **What it is for.** Gross wrong components: a piece wrongly kept or
//! dropped, a result that is the wrong side of a cut, an inside-out
//! shell. It is not an accuracy check, and a closed form is the
//! stronger oracle wherever a row has one.
//!
//! **What it forgives.** The relations hold exactly for the true set
//! operation, and a correct result is the true one to within about ε
//! of boundary displacement ([tolerance.measure]). The true result's
//! boundary lies on the operands' boundaries, so to first order a
//! correct result's volume is the true one to within ε·(area A + area
//! B); a Zero-band glue moves it by the gap times the glued area, which
//! that slack holds. A relation is flagged only where the certified
//! volumes miss it by more than that slack, so any wrong volume up to
//! ε·(area A + area B) passes: at ε = 1e-9, a stray 2.5 mm cube beside
//! two 2 m × 2 m × 0.1 m plates (17.6 m² of boundary, 1.76e-8 m³ of
//! slack) passes, and at ε = 1e-6 a 2.5 cm one does.
//!
//! **Certified volumes.** Each body is measured through the certified
//! measurement door ([`crate::test_support::measure_to`]), first at
//! [`crate::test_support::pinned_resolution`], and again at ε where
//! that leaves a relation open. A relation is broken where it fails at
//! its enclosures' least favourable ends, and held where it holds at
//! their most. Both operands must measure certified positive, as every
//! operand a boolean admits does. Nothing passes vacuously: an operand
//! not certified positive, a relation the finest enclosures leave open,
//! or a body the door cannot measure panics, naming it. A closed-form
//! face's enclosure is its value alone, its rounding unstated
//! (`work/nearpoint/a-planar-face-sums-its-area-about-a-far-carrier-origin`),
//! so on a body far from its carriers' origins a flag may be that
//! rounding's.

use std::cell::RefCell;

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
    Open(String),
}

/// The relations `op` holds over two bounded operands and its result.
fn relations(op: BooleanOp) -> [Relation; 3] {
    use Term::{A, B, Result as R};
    let rel = |which, small, large| Relation {
        which,
        small,
        large,
    };
    match op {
        BooleanOp::Union => [
            rel("vol(A ∪ B) ≥ vol(A)", &[A], &[R]),
            rel("vol(A ∪ B) ≥ vol(B)", &[B], &[R]),
            rel("vol(A ∪ B) ≤ vol(A) + vol(B)", &[R], &[A, B]),
        ],
        BooleanOp::Intersect => [
            rel("vol(A ∩ B) ≤ vol(A)", &[R], &[A]),
            rel("vol(A ∩ B) ≤ vol(B)", &[R], &[B]),
            rel("vol(A ∩ B) ≥ 0", &[], &[R]),
        ],
        BooleanOp::Subtract => [
            rel("vol(A ∖ B) ≤ vol(A)", &[R], &[A]),
            rel("vol(A ∖ B) ≥ 0", &[], &[R]),
            rel("vol(A ∖ B) ≥ vol(A) − vol(B)", &[A], &[R, B]),
        ],
    }
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
    let text = format!(
        "{}: the larger side measures [{large_lo:e}, {large_hi:e}] and the smaller \
         [{small_lo:e}, {small_hi:e}], against ε·(area A + area B) = {slack:e}, about the \
         most a correct result's volume moves",
        relation.which
    );
    if large_hi - small_lo < -slack {
        Reading::Broken(text)
    } else if large_lo - small_hi >= -slack {
        Reading::Held
    } else {
        Reading::Open(text)
    }
}

/// `body` measured at a resolution of `len` metres of mean boundary
/// displacement; whatever the door's stop, its enclosure is certified,
/// and an open one leaves the relation open.
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

/// **The volume relations over a pair of operands**, each operand
/// measured once at each resolution however many ops and orders the
/// pair is checked under.
pub struct VolumeOracle<'b> {
    bodies: [&'b Body<f64>; 2],
    tol: Tol,
    /// Each body's measure at the pinned resolution and at ε.
    cache: RefCell<[[Option<Measured>; 2]; 2]>,
}

impl<'b> VolumeOracle<'b> {
    /// The oracle over `x` and `y`, either of which may be either
    /// operand of an op it checks.
    #[must_use]
    pub fn new(x: &'b Body<f64>, y: &'b Body<f64>, tol: Tol) -> Self {
        Self {
            bodies: [x, y],
            tol,
            cache: RefCell::new([[None; 2]; 2]),
        }
    }

    /// The resolutions the oracle measures at, coarse first.
    fn resolutions(&self) -> [f64; 2] {
        [crate::test_support::pinned_len(self.tol), self.tol.eps()]
    }

    /// Which of the pair `operand` is.
    #[allow(clippy::panic)]
    fn index(&self, operand: &Body<f64>) -> usize {
        self.bodies
            .iter()
            .position(|&b| core::ptr::eq(b, operand))
            .unwrap_or_else(|| panic!("the volume oracle holds no such operand"))
    }

    /// Operand `i` at resolution `r`, measured on first ask.
    #[allow(clippy::panic)]
    fn operand(&self, i: usize, r: usize) -> Measured {
        if let Some(m) = self.cache.borrow()[i][r] {
            return m;
        }
        let what = ["the first operand", "the second operand"][i];
        let m = measured(what, self.bodies[i], self.resolutions()[r], self.tol);
        if m.lo <= 0.0 {
            panic!(
                "the volume oracle holds {what} to bounded solids, and its volume measures \
                 [{:e}, {:e}]",
                m.lo, m.hi
            );
        }
        self.cache.borrow_mut()[i][r] = Some(m);
        m
    }

    /// **`op` on `a` and `b`** (each one of the pair) **built `result`**
    /// (`None` an empty one): the first relation the certified volumes
    /// break, as text naming it and its measures, or `Ok`.
    ///
    /// # Errors
    ///
    /// The relation broken.
    ///
    /// # Panics
    ///
    /// Where `a` or `b` is not one of the pair or not certified
    /// positive, a body cannot be measured, or a relation is still open
    /// at the finest enclosures: the oracle then says nothing, and a row
    /// that asked it must hear so.
    #[allow(clippy::panic)]
    pub fn relations(
        &self,
        op: BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        result: Option<&Body<f64>>,
    ) -> Result<(), String> {
        let (ia, ib) = (self.index(a), self.index(b));
        let mut open = Vec::new();
        for (r, len) in self.resolutions().into_iter().enumerate() {
            let at = [
                self.operand(ia, r),
                self.operand(ib, r),
                result.map_or(Measured::EMPTY, |body| {
                    measured("the result", body, len, self.tol)
                }),
            ];
            let slack = self.tol.eps() * (at[0].area + at[1].area);
            open.clear();
            for relation in relations(op) {
                match read(&relation, at, slack) {
                    Reading::Broken(why) => return Err(format!("{op:?}: {why}")),
                    Reading::Open(why) => open.push(why),
                    Reading::Held => {}
                }
            }
            if open.is_empty() {
                return Ok(());
            }
        }
        panic!(
            "{op:?}: the volume oracle's finest enclosures leave {} open, so it can say \
             nothing of this result",
            open.join("; ")
        );
    }

    /// [`Self::relations`], panicking on a broken relation with `what`
    /// naming the case.
    ///
    /// # Panics
    ///
    /// Where a relation is broken, and as [`Self::relations`].
    #[allow(clippy::panic)]
    pub fn assert(
        &self,
        what: &str,
        op: BooleanOp,
        a: &Body<f64>,
        b: &Body<f64>,
        result: Option<&Body<f64>>,
    ) {
        if let Err(why) = self.relations(op, a, b, result) {
            panic!("{what}: the result breaks a volume relation: {why}");
        }
    }
}

/// [`VolumeOracle::relations`] for one op.
///
/// # Errors
///
/// The relation broken.
///
/// # Panics
///
/// As [`VolumeOracle::relations`].
pub fn volume_relations(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    result: Option<&Body<f64>>,
    tol: Tol,
) -> Result<(), String> {
    VolumeOracle::new(a, b, tol).relations(op, a, b, result)
}

/// [`VolumeOracle::assert`] for one op.
///
/// # Panics
///
/// As [`VolumeOracle::assert`].
pub fn assert_volume_relations(
    what: &str,
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    result: Option<&Body<f64>>,
    tol: Tol,
) {
    VolumeOracle::new(a, b, tol).assert(what, op, a, b, result);
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

    /// **The slack is ε·(area A + area B), pinned at the run's ε.** Two
    /// unit cubes (12 m² of boundary) and `A ∖ B` planted as the cube
    /// lifted by `h`, which breaks `vol(A ∖ B) ≤ vol(A)` by `h` m³:
    /// 0.9 of the slack passes, 1.1 of it is flagged, and equal volumes
    /// hold every relation.
    #[test]
    fn the_slack_is_eps_over_the_operands_boundary() {
        let tol = Tol::witness();
        let slack = 12.0 * tol.eps();
        let cube = unit_footprint(1.0);
        let under = unit_footprint(1.0 + 0.9 * slack);
        let got = volume_relations(BooleanOp::Subtract, &cube, &cube, Some(&under), tol);
        assert!(got.is_ok(), "0.9 of the slack passes: {got:?}");
        let over = unit_footprint(1.0 + 1.1 * slack);
        breaks(
            BooleanOp::Subtract,
            &cube,
            &cube,
            Some(&over),
            "vol(A ∖ B) ≤ vol(A)",
        );
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let got = volume_relations(op, &cube, &cube, Some(&cube), tol);
            assert!(got.is_ok(), "{op:?}: equal volumes hold: {got:?}");
        }
    }

    /// **An operand that is not a bounded solid is no question the
    /// oracle answers**: a complement operand panics rather than drop
    /// its relations.
    #[test]
    #[should_panic(expected = "the volume oracle holds the second operand to bounded solids")]
    fn a_complement_operand_panics() {
        let cube = unit_footprint(1.0);
        let complement = unit_footprint(0.5).revert().unwrap();
        let _ = volume_relations(
            BooleanOp::Intersect,
            &cube,
            &complement,
            Some(&cube),
            Tol::witness(),
        );
    }
}
