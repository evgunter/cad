//! **The containment door's reading of a planar loop walk's refusal**:
//! the root's [`ContainError`] for each [`PointInLoopError`] the
//! splitting engine's loop walk answers. A reading of the engine by the
//! root lives at the root (`[topo.homes]`).

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
            PointInLoopError::RayExhausted { r#loop } => Self::RayExhausted(r#loop),
            PointInLoopError::CorruptLoop { r#loop } => Self::LoopUnreadable(r#loop),
            PointInLoopError::LoneLoop { r#loop } => Self::EmptyLoop(r#loop),
            PointInLoopError::Uncrossable(u) => Self::Uncrossable(u),
            PointInLoopError::OffPlane(_) => {
                unreachable!(
                    "the door's walks read loops through carrier_loop, which certifies no plane"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::contain::ContainError;
    use crate::entity::{EdgeKey, LoopKey};
    use crate::splitting::{PointInLoopError, Uncrossable, UncrossableCarrier};

    /// **A loop walk's refusal keeps the loop it names, and the door's
    /// text names it**: the walk's ray schedule, its unwalkable loop, its
    /// lone-vertex loop and its uncrossable edge each reach the door's
    /// refusal with their loop, and the door's sentence spells that loop's
    /// key (and the uncrossable edge's).
    #[test]
    fn a_loop_walks_refusal_keeps_and_names_its_loop() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let (lk, ek) = (LoopKey::from(key(9)), EdgeKey::from(key(3)));
        let u = Uncrossable {
            r#loop: lk,
            edge: ek,
            carrier: UncrossableCarrier::Spline,
        };
        for (walk, door) in [
            (
                PointInLoopError::RayExhausted { r#loop: lk },
                ContainError::RayExhausted(lk),
            ),
            (
                PointInLoopError::CorruptLoop { r#loop: lk },
                ContainError::LoopUnreadable(lk),
            ),
            (
                PointInLoopError::LoneLoop { r#loop: lk },
                ContainError::EmptyLoop(lk),
            ),
            (
                PointInLoopError::Uncrossable(u),
                ContainError::Uncrossable(u),
            ),
        ] {
            let got = ContainError::from(walk.clone());
            assert_eq!(got, door, "{walk:?}: the door's refusal");
            let text = got.to_string();
            assert!(
                text.contains(&format!("loop {lk:?} of the face")),
                "{walk:?}: the door's text names its loop: {text}"
            );
        }
        let text = ContainError::Uncrossable(u).to_string();
        assert!(
            text.contains(&format!("edge {ek:?}")),
            "the door's text names the uncrossable edge: {text}"
        );
    }
}
