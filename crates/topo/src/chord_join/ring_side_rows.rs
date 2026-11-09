//! **A ring vertex whose containment refuses with a standing is kept, and
//! a later vertex answers** (`vertex_side`): an exhausted ray schedule
//! (in band) or an edge no ray crosses (a limit) at the first vertex,
//! then a decided `In`, answers `In`; a loop that does not walk aborts at
//! once, before the later answer. Red where the side reads every
//! refusal but an escalation as an abort.

use geom_brep::ladder::{Answering, Climb, climb};

use super::{RingSide, vertex_side};
use crate::entity::{EdgeKey, LoopKey};
use crate::splitting::Uncrossable;
use crate::splitting::containment::{LoopContainment, PointInLoopError, UncrossableCarrier};

#[test]
fn a_kept_vertex_refusal_lets_a_later_vertex_answer() {
    let r#loop = LoopKey::default();
    for refused in [
        PointInLoopError::RayExhausted { r#loop },
        PointInLoopError::Uncrossable(Uncrossable {
            r#loop,
            edge: EdgeKey::default(),
            carrier: UncrossableCarrier::Spline,
        }),
    ] {
        let got = climb(
            Answering::Any,
            [Err(refused.clone()), Ok(LoopContainment::In)],
            vertex_side,
        );
        assert!(
            matches!(
                got,
                Climb::Answered {
                    answer: Ok(RingSide::In),
                    ..
                }
            ),
            "{refused:?} is kept and the next vertex answers"
        );
    }
    let got = climb(
        Answering::Any,
        [
            Err(PointInLoopError::CorruptLoop { r#loop }),
            Ok(LoopContainment::In),
        ],
        vertex_side,
    );
    assert!(
        matches!(got, Climb::Answered { answer: Err(_), .. }),
        "a loop that does not walk aborts, never answering past itself"
    );
}
