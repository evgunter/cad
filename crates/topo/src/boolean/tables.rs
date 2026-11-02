//! The three symbolic decision tables of the boolean classifier, TYPED
//! and unit-tested per row.
//!
//! # Eq. 15.3 — on-component lumping (the BOOK's table, adjudicated)
//!
//! The two witnesses disagree on the ∖ row's A side: the TOG 1986 paper
//! (Table I, p. 13) prints `∖: AonB⁺→AoutB, AonB⁻→AinB` — which
//! duplicates its own ∪ row and contradicts the paper's own eq. (5)
//! (`A∖B` keeps `AonB⁻` and discards `AonB⁺`, so `AonB⁻` must lump to
//! the *kept* AoutB). The book (Eq. 15.3, p. 270; Program 15.10's code
//! agrees) prints `∖` sharing the ∩ row: `AonB⁺→AinB, AonB⁻→AoutB` —
//! consistent with eq. (5) and with `A∖B ≡ A∩revert(B)` (Problem 15.9,
//! the reason ∩ and ∖ share a row). **Adjudication: the book's table is
//! adopted; the paper's ∖-row A side is its own misprint.** (Citations:
//! Mäntylä 1988 §15.3 Eq. 15.3; Mäntylä TOG 5(1) 1986 Table I + eq. 5;
//! synthesis §C.)
//!
//! # Edge-sector coincidence (TOG Tables II and III)
//!
//! TOG decides an on-edge of one solid lying in a face of the other by
//! Table II on the side codes of the edge's two flanking sectors
//! against the face, deferring a flanker on the face to Table III. The
//! kernel reads it as any material about a line: the face's half-turn
//! is a line cone, a flanker on the face lies in the other solid's
//! material as it lumps by Eq. 15.3 (`recl::resolve_edge_sector`).
//! TOG's printed Table III carries the ∖-row misprint above in its
//! A-versus-B rows (`∖` Yes where the lump derives No, and No where it
//! derives Yes); its worked example (Fig. 18, ∪) exercises only the ∪
//! column and matches.

use super::{BooleanOp, Operand, SideCode};
use crate::boolean::plane_eq::PlaneRelation;

/// Which 4-way component an on-sector is lumped into, expressed as the
/// side code the sector's bounds are rewritten to.
///
/// Reading: for an A-sector on B, `Out` means "treated as AoutB";
/// for a B-sector on A, `Out` means "treated as BoutA".
///
/// Stated for the glue's kept side ([`super::glue::KEPT`]): where the
/// lump keeps one copy of the region ([`lump_keeps_one`]), the kept
/// side's sector lumps to the side `op` keeps for its operand and the
/// other's to the side `op` drops; elsewhere both lump to the side
/// dropped. With A kept this is the book's table, cell for cell:
/// ∪: AonB⁺→AoutB, AonB⁻→AinB; BonA±→BinA. ∩ and ∖ share a row
/// (A∖B ≡ A∩revert(B)): AonB⁺→AinB, AonB⁻→AoutB; BonA±→BoutA.
pub fn eq15_3_lump(op: BooleanOp, on_side: Operand, relation: PlaneRelation) -> SideCode {
    let kept = super::finish::kept_side(op, on_side);
    if kept_copy(op, relation) == Some(on_side) {
        kept
    } else {
        kept.opposite()
    }
}

/// Whether the Eq. 15.3 lump of a coincident sector pair keeps exactly
/// one operand's copy of the shared region, so that the result holds
/// one face's region through the other face
/// (`BooleanNaming::substituted`): where the two faces' materials lie
/// on the side the op keeps one of, ∪ and ∩ on same-oriented faces and
/// ∖ on opposed ones (B is reverted). The other arms keep neither copy.
pub fn lump_keeps_one(op: BooleanOp, relation: PlaneRelation) -> bool {
    debug_assert!(matches!(
        relation,
        PlaneRelation::SameOriented | PlaneRelation::SameOpposite
    ));
    let aligned = matches!(relation, PlaneRelation::SameOriented);
    match op {
        BooleanOp::Union | BooleanOp::Intersect => aligned,
        BooleanOp::Subtract => !aligned,
    }
}

/// Which operand's copy of a coincident sector pair's region the Eq.
/// 15.3 lump keeps, when it keeps exactly one ([`lump_keeps_one`]): the
/// glue's kept side ([`super::glue::KEPT`]), whose description the
/// overlap carries into the merge.
pub fn kept_copy(op: BooleanOp, relation: PlaneRelation) -> Option<Operand> {
    lump_keeps_one(op, relation).then_some(super::glue::KEPT)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use BooleanOp::{Intersect, Subtract, Union};
    use Operand::{A, B};
    use PlaneRelation::{SameOpposite as Minus, SameOriented as Plus};
    use SideCode::{In, Out};

    /// The lump keeps one copy of a coincident region exactly where the
    /// two faces' materials lie on the side the op keeps one of: ∪ and
    /// ∩ on same-oriented faces, ∖ on opposed ones (B is reverted).
    #[test]
    fn a_lump_keeps_one_copy_on_the_kept_arms_only() {
        let cells = [
            (Union, Plus, true),
            (Union, Minus, false),
            (Intersect, Plus, true),
            (Intersect, Minus, false),
            (Subtract, Plus, false),
            (Subtract, Minus, true),
        ];
        for (op, rel, keeps_one) in cells {
            assert_eq!(lump_keeps_one(op, rel), keeps_one, "{op:?} {rel:?}");
        }
    }

    /// Eq. 15.3, every cell of the BOOK's table (the adjudicated one;
    /// module docs for the ∖-row misprint record): the lump with A's
    /// copy kept, the glue's kept side.
    #[test]
    fn eq15_3_all_rows() {
        // ∪ row.
        assert_eq!(eq15_3_lump(Union, A, Plus), Out); // AonB⁺ → AoutB
        assert_eq!(eq15_3_lump(Union, A, Minus), In); // AonB⁻ → AinB
        assert_eq!(eq15_3_lump(Union, B, Plus), In); // BonA⁺ → BinA
        assert_eq!(eq15_3_lump(Union, B, Minus), In); // BonA⁻ → BinA
        // ∩ and ∖ share a row — asserted cell by cell for BOTH ops.
        for op in [Intersect, Subtract] {
            assert_eq!(eq15_3_lump(op, A, Plus), In); // AonB⁺ → AinB
            assert_eq!(eq15_3_lump(op, A, Minus), Out); // AonB⁻ → AoutB
            assert_eq!(eq15_3_lump(op, B, Plus), Out); // BonA⁺ → BoutA
            assert_eq!(eq15_3_lump(op, B, Minus), Out); // BonA⁻ → BoutA
        }
    }
}
