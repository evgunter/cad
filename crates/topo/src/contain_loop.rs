//! **The containment door's reading of a planar loop walk's refusal**:
//! the boolean's [`ContainError`] for each [`PointInLoopError`] the
//! splitting engine's loop walk answers. It names both engines, so it
//! lives at the root (`[topo.homes]`).

use crate::contain::ContainError;
use crate::splitting::PointInLoopError;

impl From<PointInLoopError> for ContainError {
    fn from(e: PointInLoopError) -> Self {
        match e {
            PointInLoopError::Escalated {
                decision,
                escalation,
                diag,
                ..
            } => Self::Escalated {
                decision: Some(decision.into()),
                escalation,
                diag,
            },
            PointInLoopError::RayExhausted { .. } => Self::RayExhausted,
            PointInLoopError::CorruptLoop { r#loop } => Self::LoopUnreadable(r#loop),
            PointInLoopError::LoneLoop { r#loop } => Self::EmptyLoop(r#loop),
            PointInLoopError::Uncrossable(u) => Self::Uncrossable(u),
            PointInLoopError::OffPlane(_) => {
                unreachable!("contfp reads loops through carrier_loop, which certifies no plane")
            }
        }
    }
}
