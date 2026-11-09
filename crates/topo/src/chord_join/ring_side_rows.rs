//! **A ring vertex whose reading refuses with a standing is kept, and a
//! later vertex answers** (`vertex_side`): a refusal kept in band or as a
//! limit at the first vertex, then a decided `In`, answers `In`; one that
//! aborts stops the ring at once, before the later answer. Red where the
//! side reads every refusal but an escalation as an abort.

use geom_brep::ladder::{Aborts, Answering, Climb, Refusal, climb};

use super::{RingSide, vertex_side};

/// A refusal standing as the row says.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Refused(Result<Refusal, Aborts>);

#[test]
fn a_kept_vertex_refusal_lets_a_later_vertex_answer() {
    let side = |first: Refused| {
        climb(
            Answering::Any,
            [Err(first), Ok(Some(RingSide::In))],
            |reading| vertex_side(reading, |r: &Refused| r.0),
        )
    };
    for kept in [Refusal::InBand, Refusal::Limit] {
        assert!(
            matches!(
                side(Refused(Ok(kept))),
                Climb::Answered {
                    answer: Ok(RingSide::In),
                    ..
                }
            ),
            "{kept:?} is kept and the next vertex answers"
        );
    }
    assert!(
        matches!(
            side(Refused(Err(Aborts))),
            Climb::Answered { answer: Err(_), .. }
        ),
        "an abort stops the ring before the next vertex answers"
    );
}
