//! **An inside-out thin tube is refused as an operand.** A tube `t`
//! thick, the profile `[r, r + t] × [0, 1]` turned a full turn about the
//! sketch `y` axis, reverted: at `t = 20ε` its `V/A` is in band, so
//! check 7 exempts it and it finishes at rest. Against the brick
//! `(−20, 20)³`, which holds it, every op in both orders refuses rather
//! than answering for its complement (`brick ∖ revtube` built the tube's
//! own volume). At ε = 1e-12 a ray reads the tube's side at infinity
//! inside its material and the refusal names it as the inside-out
//! operand; at the wider rows its side reads in band, from the Boolean's
//! one point outside it or from the brick's witnesses, and the operation
//! refuses on that reading.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, Operand, PointInSolidError, ValidationError,
};

fn tol() -> Tol {
    Tol::witness()
}

fn tube(r: f64, t: f64) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    revolved_about_y(
        vec![p(r, 0.0), p(r + t, 0.0), p(r + t, 1.0), p(r, 1.0)],
        Revolution::Full,
        tol(),
    )
}

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

const OPS: [(&str, Op); 3] = [
    ("∪", topo::union),
    ("∩", topo::intersect),
    ("∖", topo::subtract),
];

#[test]
fn an_inside_out_thin_tube_is_refused_as_an_operand() {
    let t = 20.0 * tol().eps();
    let big = finished(
        "the brick",
        brick((-20.0, 20.0), (-20.0, 20.0), (-20.0, 20.0), tol()),
        tol(),
    );
    for r in [0.5, 1.0, 2.0] {
        let rev = AtRestBody::validate(tube(r, t).revert(), tol())
            .unwrap_or_else(|e| panic!("r {r}: the reverted tube finishes at rest, got {e:?}"));
        for (name, op) in OPS {
            for (a, b, inside_out) in [(&rev, &big, Operand::A), (&big, &rev, Operand::B)] {
                let got = op(a, b, tol());
                match &got {
                    Err(BooleanError::InsideOutOperand { operand, errors }) => {
                        assert_eq!(*operand, inside_out, "r {r} {name}: the operand");
                        assert!(
                            matches!(errors[..], [ValidationError::SliverInsideOut { .. }]),
                            "r {r} {name}: {errors:?}"
                        );
                    }
                    Err(
                        BooleanError::Containment(PointInSolidError::Escalated { .. })
                        | BooleanError::ShellWitnessExhausted { .. },
                    ) if tol().eps() > 1e-12 => {}
                    _ => panic!(
                        "r {r} {name} ({inside_out:?} inside-out): want InsideOutOperand, got {:?}",
                        got.map(|r| r
                            .body()
                            .map(|b| topo::mass_properties(&b.body, tol()).map(|m| m.volume)))
                    ),
                }
            }
        }
    }
}
