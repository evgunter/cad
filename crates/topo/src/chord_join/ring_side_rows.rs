//! **A ring vertex's containment refusal stands by its own standing**
//! (`vertex_side`, [`PointInLoopError::refusal`]): an exhausted ray
//! schedule (in band), an edge no ray crosses (a limit) or an escalation
//! (its margin) at the first vertex is kept, and a later vertex's `In`
//! answers; a loop that does not walk, or a point off its plane, aborts
//! the ring at once, before the later answer. Red where the side reads a
//! refusal by any table but the error's own, the old "an escalation is
//! kept, anything else aborts" included, and where that table changes.

use geom_brep::ladder::{Answering, Climb, climb};
use geom_core::{Band, Indeterminate, MarginDiag};

use super::{RingSide, vertex_side};
use crate::entity::{EdgeKey, LoopKey};
use crate::splitting::Uncrossable;
use crate::splitting::containment::{LoopContainment, PointInLoopError, UncrossableCarrier};

fn side(first: PointInLoopError) -> Climb<Result<RingSide, PointInLoopError>, PointInLoopError> {
    climb(
        Answering::Any,
        [Err(first), Ok(LoopContainment::In)],
        vertex_side,
    )
}

#[test]
fn a_ring_vertex_refusal_stands_by_its_own_standing() {
    let r#loop = LoopKey::default();
    let kept = [
        PointInLoopError::RayExhausted { r#loop },
        PointInLoopError::Uncrossable(Uncrossable {
            r#loop,
            edge: EdgeKey::default(),
            carrier: UncrossableCarrier::Spline,
        }),
        PointInLoopError::Escalated {
            r#loop,
            decision: crate::splitting::LoopDecision::ALL[0],
            escalation: crate::splitting::containment::Escalation::Margin,
            diag: Indeterminate {
                margin: MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap_or_else(|_| unreachable!()),
                predicate: Some("t"),
                terminal_sliver: false,
            },
        },
    ];
    for refused in kept {
        assert!(
            matches!(
                side(refused.clone()),
                Climb::Answered {
                    answer: Ok(RingSide::In),
                    ..
                }
            ),
            "{refused:?} is kept and the next vertex answers"
        );
    }
    let aborts = PointInLoopError::CorruptLoop { r#loop };
    assert!(
        matches!(side(aborts.clone()), Climb::Answered { answer: Err(_), .. }),
        "{aborts:?} stops the ring before the next vertex answers"
    );
}
