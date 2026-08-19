//! The fillet **selection** family — the S8 branch rule and its lift to
//! the PATHS algebra's joint corner×candidate space.
//!
//! # Why this is its own module
//!
//! Selection is the one step in the fillet story that reads a bracket
//! (`Bounds`), and it is shared by two doors that otherwise have no
//! business knowing about each other: the arc-carrier construction
//! (candidates at an AUTHORED corner) and the PATHS lowering's
//! arc-carrier resolution (candidates across DERIVED corners). Giving
//! the rule one home means the ladder is stated once, and the CI
//! discipline allowlist can carry one purpose-matched justification for
//! it instead of the same paragraph twice.
//!
//! The justification, in one line: the S8 pick compares the f64
//! **diagnostic channel** and is a plain deterministic selection rule,
//! not a Q1 predicate — a representation-level choice between
//! already-classified constructions, never a re-decision of geometry.
//!
//! Everything here is evaluation code. There is no `decide` call in
//! this module, deliberately: no funnel entry, no escalation arm, no
//! error (the M5 S8 amendment).
//!
//! # The lift
//!
//! The PATHS algebra chooses over a larger domain than the builder
//! door: it forbids authoring the corner, so a line×circle or
//! circle×circle carrier pair hands it 0, 1 or 2 corners, each with its
//! own candidates, and the choice is over PAIRS. That lift is
//! [`nearest_joint`] — the same ladder applied to the flattened pair
//! list — and it landed here together with its caller, the path-side
//! arc-fillet boundary [`crate::path::arc_fillet`] (LIB-G2 §3b).
//!
//! Why the dominance argument lifts, recorded now so the mechanism is
//! not re-derived later: the two levels factor. WITHIN one corner it is
//! S8's argument verbatim — the survivors are mirror-symmetric about a
//! line through the offset centres, so the near candidate is nearer on
//! BOTH carriers at once. ACROSS corners it is not a dominance claim at
//! all and does not need to be: the advance/reach gates discard every
//! corner that is not ahead on the incoming side AND behind on the
//! arrival side, so each surviving pair is a valid fillet tangent to
//! both authored carriers, and ranking valid fillets by total setback
//! asserts nothing about geometric truth — which is precisely why S8 is
//! a selection rule and not a predicate, and why the lift inherits that
//! status unchanged.

use crate::sugar::ArcFilletCandidate;
use geom_core::Bounds;

/// Nearest-the-authored-corner branch selection (M5 S8; Evan's ruling,
/// in-chat 2026-07-30): the index of the surviving candidate to build,
/// given each survivor's `[incoming, outgoing]` tangent setbacks in
/// meters (arc lengths `R·Δθ` on circular legs).
///
/// This is a **plain deterministic selection rule, not a Q1 predicate**
/// (amended per the follow-up ruling, same day): every surviving
/// candidate is a valid fillet, exactly tangent to both authored legs,
/// so choosing between them asserts nothing about geometric truth — and
/// below ε_input the author cannot have meant a distinguishable
/// preference (D4 ¶1). A within-ε pick is therefore
/// arbitrary-but-deterministic and both-valid; an author who wants the
/// other pocket forces it by authoring that pocket's own corner (the
/// far circle is always the near fillet of the second carrier
/// intersection — the S8 reachability rows pin this). No K-funnel
/// entry, no escalation arm, no error. It compares the f64
/// **diagnostic channel** (the enclosure's lower bound at interval
/// scalars — the error payloads' messages-only channel):
/// a representation-level choice between two already-classified
/// constructions, never a re-decision of geometry. Each lane's pick is
/// deterministic, and the lanes agree whenever the survivors' setback
/// gap exceeds the interval channel's enclosure width — the macroscopic
/// case the cross-lane vesica row pins. In a hairline-asymmetric lens
/// (the corner an ulp off the configuration's mirror line) the
/// strict-but-tiny gap can sit inside that width, and the lanes may
/// then legally pick DIFFERENT pockets — harmless per the ruling, since
/// both candidates are valid fillets of the authored legs; the
/// perturbed-lens rows pin each lane's own determinism there, not
/// cross-lane agreement.
///
/// # The ladder (fixed order)
///
/// 1. strict `<` on **total setback** (incoming + outgoing) — the near
///    pocket sets both tangent points back a little, the far pocket a
///    lot;
/// 2. exact tie ⇒ strict `<` on the **incoming** setback alone;
/// 3. both tied ⇒ the **first-classified** candidate (enumeration
///    order, incoming-leg-first construction order).
///
/// # Equivariance (`memories/equivariance-principle.md`)
///
/// Rungs 1–2 compare arc lengths, which are isometry-invariant, so in ℝ
/// the selection commutes with every rigid motion AND reflection of the
/// sketch (bitwise f64 equivariance is already forgone by D9's fixed
/// evaluation orders). Rung 3 cannot be equivariant: candidates carrying
/// identical per-leg setback pairs are exactly the class where a
/// candidate-swapping symmetry would have to swap the pick, so it falls
/// to enumeration order — the kernel's first knowingly-designed
/// non-equivariant residual, recorded as such.
///
/// # Why sum (and when the later rungs can even fire)
///
/// Among survivors of the corner-side extent gates, the nearer candidate
/// is nearer on BOTH legs, so every monotone combination (sum, max, …)
/// agrees and sum is the symmetric spelling. The argument, for
/// non-enclosing tangencies (offset radius ρ > 0): the two candidates
/// are mirror-symmetric about a line L — arc×arc, the line through the
/// two offset-circle centers; line×arc, the line through the single
/// offset circle's center perpendicular to the offset line (re-derived
/// and confirmed by the S8 review) — and each tangent point is its
/// candidate's same-side radial projection (resp. same-side foot on the
/// straight leg), so the candidate on the corner's side of L is nearer
/// the corner on both carriers at once: by symmetric distances along
/// the line, and angularly about each arc center. The corner strictly
/// off L and distinct candidates are guaranteed by the
/// `fillet_corner_turn` and offset-clearance gates, so the dominance is
/// strict and rungs 2–3 are unreachable through this constructor in
/// exact arithmetic — they exist as the total, documented fallback for
/// computed-value collisions on the diagnostic channel. Enclosing
/// tangencies (ρ < 0, the tangent point antipodally flipped) never
/// participate in a two-survivor corner: the S8 review's hand proof
/// rules the mixed enclosing/non-enclosing pair out outright and shows
/// even a both-enclosing pair (never reached by any search) would still
/// be componentwise dominated. Committed evidence:
/// `tests/review_s8_probe.rs` (the constructor cross-check on
/// fuzz-found two-survivor corners, plus the trimmed independent
/// dominance fuzz over the arc×arc and line×arc classes); the
/// full-scale runs — the review's 5M-trial phases, targeted
/// both-enclosing construction and hill-climb search, and this unit's
/// ~27M-corner exploration — are uncommitted review artifacts. The
/// two-survivor situation therefore arises only in the non-enclosing
/// lens class, where the dominance argument holds.
pub(crate) fn nearest_candidate(setbacks: &[[f64; 2]]) -> usize {
    let mut best = 0;
    for (i, pair) in setbacks.iter().enumerate().skip(1) {
        let (total, best_total) = (pair[0] + pair[1], setbacks[best][0] + setbacks[best][1]);
        if total < best_total || (total == best_total && pair[0] < setbacks[best][0]) {
            best = i;
        }
    }
    best
}

/// The **lifted** ladder (LIB-G2 §3b): the same S8 rule over the PATHS
/// algebra's joint corner×candidate space, plus the bracket read that
/// feeds it.
///
/// The algebra forbids authoring the corner, so a ray×circle or
/// circle×circle carrier pair admits 0, 1 or 2 corners, each with its
/// own surviving candidates. The caller gates the corners (advance on
/// the incoming side, reach on the arrival side), flattens the
/// survivors of every corner that passed into ONE list in corner-then-
/// candidate enumeration order, and hands it here; the returned index
/// is into that flattened list.
///
/// The rule is [`nearest_candidate`] verbatim — same channel, same
/// three rungs, same non-equivariant rung-3 residual — and this
/// function exists to state that identity in code rather than in prose,
/// and to keep the `.lo()` bracket read (the S8 diagnostic channel) in
/// the one module the discipline lets take a **sole-bound**
/// `T: Bounds`. Its dominance story is the module docs' two-level
/// factoring: within a corner it is S8's argument verbatim; across
/// corners it is not a dominance claim at all, because every gated
/// survivor is already a valid fillet tangent to both authored
/// carriers, so ranking them asserts nothing about geometric truth.
///
/// `joints` is never empty at the call site (the caller refuses
/// `NoCornerForFillet` first); an empty slice returns 0, matching
/// [`nearest_candidate`]'s own total shape.
pub(crate) fn nearest_joint<T: Bounds>(joints: &[ArcFilletCandidate<T>]) -> usize {
    let setbacks: Vec<[f64; 2]> = joints
        .iter()
        .map(|c| [c.setbacks[0].lo(), c.setbacks[1].lo()])
        .collect();
    nearest_candidate(&setbacks)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// The S8 selection ladder, rung by rung (`nearest_candidate` docs):
    /// rungs 2–3 are unreachable through `fillet_corner` in exact
    /// arithmetic (strict dominance among survivors — see the fn docs),
    /// so the total fallback rule is pinned here at the unit level, in
    /// both enumeration orders where an order exists to swap.
    #[test]
    fn nearest_candidate_ladder() {
        // Rung 1: strictly smaller total setback wins, either side.
        assert_eq!(nearest_candidate(&[[2.0, 2.0], [1.0, 1.5]]), 1);
        assert_eq!(nearest_candidate(&[[1.0, 1.5], [2.0, 2.0]]), 0);
        // Rung 1 sees the sum, not the parts: a crossed pair with the
        // strictly smaller total wins even with the larger incoming.
        assert_eq!(nearest_candidate(&[[1.0, 4.0], [3.0, 1.0]]), 1);
        // Rung 2: exactly tied totals — the smaller INCOMING setback
        // wins, either side.
        assert_eq!(nearest_candidate(&[[2.0, 1.0], [1.0, 2.0]]), 1);
        assert_eq!(nearest_candidate(&[[1.0, 2.0], [2.0, 1.0]]), 0);
        // Rung 3: identical pairs — the first-classified candidate (the
        // sole designed non-equivariant residual).
        assert_eq!(nearest_candidate(&[[1.5, 2.5], [1.5, 2.5]]), 0);
        // The single-survivor path funnels through the same rule.
        assert_eq!(nearest_candidate(&[[3.0, 4.0]]), 0);
    }

    /// The LIFT is the same ladder, and this pins that identity rather
    /// than restating it: `nearest_joint` over candidates carrying a
    /// given setback list agrees with `nearest_candidate` over that
    /// list, index for index, including the cross-corner case the
    /// builder door can never produce (a joint space whose winner sits
    /// at a LATER corner).
    #[test]
    fn nearest_joint_is_the_same_ladder_over_the_flattened_pairs() {
        use crate::sugar::ArcFilletCandidate;
        use geom_core::{Point2, Sign};
        let mk = |sb: [f64; 2]| ArcFilletCandidate {
            t1: Point2::new(0.0, 0.0),
            t2: Point2::new(0.0, 0.0),
            bulge: 0.0,
            center: Point2::new(0.0, 0.0),
            fit_in: Sign::Positive,
            fit_out: Sign::Positive,
            setbacks: sb,
        };
        // Corner A's two survivors, then corner B's one: the winner is
        // across the corner boundary, which is the whole point of the
        // lift.
        let rows = [[2.0, 2.0], [3.0, 1.5], [0.5, 0.5]];
        let joints: Vec<_> = rows.iter().map(|r| mk(*r)).collect();
        assert_eq!(nearest_joint(&joints), 2);
        assert_eq!(nearest_joint(&joints), nearest_candidate(&rows));
        // Rung 3's non-equivariant residual survives the lift verbatim:
        // identical pairs keep the first-classified joint.
        let tied = [[1.5, 2.5], [1.5, 2.5]];
        let joints: Vec<_> = tied.iter().map(|r| mk(*r)).collect();
        assert_eq!(nearest_joint(&joints), 0);
    }
}
