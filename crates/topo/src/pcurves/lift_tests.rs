use super::lift_decks;
use crate::joint::{Deck, JointElement, Winding};

/// **The lift from every start is one chain.** A loop whose joints
/// carry twins and second-channel periods and compose to the
/// identity, rotated to start at each member: each member's lift
/// differs from its lift in the original order by ONE deck
/// transformation applied to the whole chain.
#[test]
fn a_closed_loop_lifts_to_one_chain_from_every_start() {
    let [a, b, c] = [
        Deck {
            u: 0,
            v: 0,
            twin: true,
        },
        Deck {
            u: 1,
            v: 0,
            twin: false,
        },
        Deck {
            u: -2,
            v: 1,
            twin: true,
        },
    ];
    let elements = [a, b, c, a.compose(b).compose(c).inverse()].map(JointElement);
    assert!(
        Winding::of(elements).is_zero(),
        "the premise: the loop closes on its start"
    );
    let n = elements.len();
    let base = lift_decks(&elements);
    for start in 1..n {
        let rotated: Vec<JointElement> = (0..n).map(|k| elements[(start + k) % n]).collect();
        let decks = lift_decks(&rotated);
        // `decks[k]` is member `start + k`'s lift from this start.
        let lift = |i: usize| decks[(i + n - start) % n];
        let offset = base[0].compose(lift(0).inverse());
        for i in 0..n {
            assert_eq!(
                offset.compose(lift(i)),
                base[i],
                "start {start}, member {i}: {decks:?} against {base:?}"
            );
        }
    }
}
