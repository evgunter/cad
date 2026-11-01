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
//! # Table II — edge-sector coincidence (TOG p. 22, transcribed)
//!
//! For an on-edge of one solid lying in a sector (face) of the other:
//! the two sectors flanking the on-edge are the *test sectors*, keyed
//! [`SideCode`] by the signed distance of each one's noncoplanar
//! bounding vector against the *reference sector*'s plane (bisector if
//! ≥ 180° — our convex subdivision provides it); the table decides per
//! test sector whether it intersects the reference. ON-keyed rows defer
//! to the op/orientation rule (Table III).
//!
//! # Table III — the ON-case rule, DERIVED (not transcribed)
//!
//! TOG's printed Table III carries the same ∖-row misprint consequence
//! in its A-versus-B rows (`∪ Yes, ∖ Yes, ∩ No` — the ∖ entry follows
//! the misprinted Table I; with the corrected lumping ∩ and ∖ must
//! agree everywhere, as `A∖B ≡ A∩revert(B)` forces). We therefore
//! DERIVE the rule from the adjudicated Eq. 15.3: *Rule = "the coplanar
//! (ON) sector is lumped to the OUT side of the reference solid"* —
//! then a test sector keyed IN intersects the reference iff Rule (the
//! IN→lumped-OUT transition crosses at the on-edge), and a test sector
//! keyed OUT intersects iff NOT(Rule). The derived table:
//!
//! | comparison | orientation | ∪  | ∩  | ∖  |
//! |------------|-------------|----|----|----|
//! | A vs B     | identical   | Y  | N  | N  |
//! | A vs B     | opposite    | N  | Y  | Y  |
//! | B vs A     | identical   | N  | Y  | Y  |
//! | B vs A     | opposite    | N  | Y  | Y  |
//!
//! (TOG's print differs only in the ∖ column of the A-vs-B rows —
//! `Y`/`N` where we derive `N`/`Y`; its worked example (Fig. 18, ∪)
//! exercises only the ∪ column and matches.)

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

/// A Table II result cell for one test sector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableIiVerdict {
    /// The test sector does not intersect the reference.
    No,
    /// It does.
    Yes,
    /// Defer to Table III ([`table_iii_rule`]).
    Rule,
    /// The negation of Table III.
    NotRule,
}

/// TOG Table II, row (test-sector-1 key, test-sector-2 key) →
/// (result 1, result 2). Transcribed verbatim (all nine rows).
pub fn table_ii(t1: SideCode, t2: SideCode) -> (TableIiVerdict, TableIiVerdict) {
    use SideCode::{In, On, Out};
    use TableIiVerdict::{No, NotRule, Rule, Yes};
    match (t1, t2) {
        (On, On) => (No, No),
        (On, In) => (No, Rule),
        (On, Out) => (No, NotRule),
        (In, On) => (Rule, No),
        (In, In) => (No, No),
        (In, Out) => (Yes, No),
        (Out, On) => (NotRule, No),
        (Out, In) => (No, Yes),
        (Out, Out) => (No, No),
    }
}

/// Table III, DERIVED (module docs): Rule(op, comparison, orientation)
/// = "the ON sector lumps to OUT" = `eq15_3_lump(..) == Out`.
/// `comparison` names the solid the TEST sectors belong to (A-versus-B
/// = test sectors in A classified against a reference sector of B).
pub fn table_iii_rule(op: BooleanOp, comparison: Operand, relation: PlaneRelation) -> bool {
    eq15_3_lump(op, comparison, relation) == SideCode::Out
}

/// Resolves a Table II verdict cell to a concrete intersect decision,
/// given Table III's rule for the cell's on-sector: whether it lumps
/// `Out` ([`table_iii_rule`] for a coincident pair).
pub fn resolve_verdict(v: TableIiVerdict, lumps_out: bool) -> bool {
    match v {
        TableIiVerdict::No => false,
        TableIiVerdict::Yes => true,
        TableIiVerdict::Rule => lumps_out,
        TableIiVerdict::NotRule => !lumps_out,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use BooleanOp::{Intersect, Subtract, Union};
    use Operand::{A, B};
    use PlaneRelation::{SameOpposite as Minus, SameOriented as Plus};
    use SideCode::{In, On, Out};
    use TableIiVerdict::{No, NotRule, Rule, Yes};

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

    /// Where the lump keeps one copy, the lumped sectors keep the glue's
    /// kept side's and drop the other's; elsewhere they keep neither. So
    /// a containment fallback that cuts a dropped copy cuts the other
    /// side's (`ops::fallback_fates`).
    #[test]
    fn the_lump_keeps_the_glues_kept_copy_where_it_keeps_one() {
        let kept = super::super::glue::KEPT;
        for op in [Union, Intersect, Subtract] {
            for rel in [Plus, Minus] {
                let held = |o| eq15_3_lump(op, o, rel) == super::super::finish::kept_side(op, o);
                let one = lump_keeps_one(op, rel);
                assert_eq!(held(kept), one, "{op:?} {rel:?}: the kept side's copy");
                assert!(!held(kept.other()), "{op:?} {rel:?}: the other side's copy");
            }
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

    /// Table II, one assertion per printed row (TOG p. 22).
    #[test]
    fn table_ii_all_nine_rows() {
        assert_eq!(table_ii(On, On), (No, No)); // row 1
        assert_eq!(table_ii(On, In), (No, Rule)); // row 2
        assert_eq!(table_ii(On, Out), (No, NotRule)); // row 3
        assert_eq!(table_ii(In, On), (Rule, No)); // row 4
        assert_eq!(table_ii(In, In), (No, No)); // row 5
        assert_eq!(table_ii(In, Out), (Yes, No)); // row 6
        assert_eq!(table_ii(Out, On), (NotRule, No)); // row 7
        assert_eq!(table_ii(Out, In), (No, Yes)); // row 8
        assert_eq!(table_ii(Out, Out), (No, No)); // row 9
    }

    /// Table III, all twelve cells of the DERIVED table (module docs);
    /// the two cells where the derivation corrects TOG's print are
    /// called out.
    #[test]
    fn table_iii_all_cells() {
        // A versus B, identical: ∪ Yes, ∩ No, ∖ No (TOG prints ∖ Yes —
        // the propagated Table I misprint; corrected by derivation).
        assert!(table_iii_rule(Union, A, Plus));
        assert!(!table_iii_rule(Intersect, A, Plus));
        assert!(!table_iii_rule(Subtract, A, Plus), "corrected vs TOG print");
        // A versus B, opposite: ∪ No, ∩ Yes, ∖ Yes (TOG prints ∖ No).
        assert!(!table_iii_rule(Union, A, Minus));
        assert!(table_iii_rule(Intersect, A, Minus));
        assert!(table_iii_rule(Subtract, A, Minus), "corrected vs TOG print");
        // B versus A, identical and opposite: ∪ No, ∩ Yes, ∖ Yes —
        // matches TOG's print.
        for rel in [Plus, Minus] {
            assert!(!table_iii_rule(Union, B, rel));
            assert!(table_iii_rule(Intersect, B, rel));
            assert!(table_iii_rule(Subtract, B, rel));
        }
        // Coherence: ∩ and ∖ agree everywhere (A∖B ≡ A∩revert(B)).
        for side in [A, B] {
            for rel in [Plus, Minus] {
                assert_eq!(
                    table_iii_rule(Intersect, side, rel),
                    table_iii_rule(Subtract, side, rel)
                );
            }
        }
    }

    /// TOG's worked example (Fig. 18, ∪, identical orientation): test
    /// sectors (IN, ON) of A against reference B ⇒ row 4 ⇒ Rule ⇒ the
    /// IN sector intersects; test sectors (OUT, ON) of B against
    /// reference A ⇒ row 7 ⇒ NOT(rule) ⇒ the OUT sector intersects.
    #[test]
    fn tog_fig18_worked_example() {
        let (r1, _) = table_ii(In, On);
        assert!(resolve_verdict(r1, table_iii_rule(Union, A, Plus)));
        let (r1, _) = table_ii(Out, On);
        assert!(resolve_verdict(r1, table_iii_rule(Union, B, Plus)));
    }
}
