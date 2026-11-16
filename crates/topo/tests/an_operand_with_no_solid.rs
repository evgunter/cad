//! **A body with no solid, handed to an operation.** The split refuses
//! it as the caller's, with a recourse and no kernel-defect ending: it
//! cut nothing, so nothing of its own failed. The Boolean refuses it at
//! its containment gate, before any of its surgery reads a solid.

#![allow(clippy::expect_used, clippy::panic)]

use geom_brep::recourse::{Reading, defect_ending};
use geom_core::{Point3, Tol, Vec3};
use topo::test_support::{brick, finished, split_plane};
use topo::{Body, BooleanErrorKind, SplitError, SplitFinishError};

#[test]
fn the_split_refuses_an_empty_body_as_the_callers() {
    let tol = Tol::witness();
    let empty = finished("an empty body", Body::<f64>::new(), tol);
    let plane = split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), tol);
    let Err(error) = topo::split(&empty, &plane, tol) else {
        panic!("an empty body has no sides");
    };
    assert!(
        matches!(error, SplitError::Finish(SplitFinishError::EmptyOperand)),
        "{error:?}"
    );
    let said = error.render(Reading::Build);
    assert!(
        said.contains("Recourse:") && !said.contains(defect_ending(Reading::Build)),
        "{said}"
    );
}

#[test]
fn the_boolean_refuses_an_empty_operand_before_its_surgery() {
    let tol = Tol::witness();
    let empty = finished("an empty body", Body::<f64>::new(), tol);
    let block = finished(
        "a block",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let ops = [topo::union, topo::intersect, topo::subtract];
    for (i, op) in ops.into_iter().enumerate() {
        for (a, b, one_empty) in [
            (&empty, &block, true),
            (&block, &empty, true),
            (&empty, &empty, false),
        ] {
            let built = op(a, b, tol);
            assert!(
                built.is_err() || !one_empty,
                "op {i}: an empty operand beside a block builds nothing"
            );
            if let Err(error) = built {
                assert!(
                    !matches!(
                        error.kind(),
                        BooleanErrorKind::Carve | BooleanErrorKind::JoinDesync
                    ),
                    "op {i}: {error:?}"
                );
            }
        }
    }
}
