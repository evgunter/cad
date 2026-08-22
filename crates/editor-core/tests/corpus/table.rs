//! Corpus document **corner_table** — the corner-aligned four-leg
//! table, the capability M4 PR 5's Declare opened (`#71`; the kernel
//! pins live in `topo/tests/corner_table.rs`). Here it is a RECIPE:
//! every flush contact is declared by NAME through a `Declare` node,
//! never inferred from values.
//!
//! **Authored through the LIB-SEL2 detect/declare protocol**
//! (`docs/SELECT-DESIGN.md` §3): each leg's flush declarations are no
//! longer a hand-maintained segment table plus hand-tracked
//! `Merged`/`FromA` name wrapping — the author EVALUATES the document
//! so far, runs `find_flush_candidates` between the accumulated body
//! and the new leg, inspects the findings (the counts asserted below
//! are that inspection), and turns them into the `Declare` node
//! through `declare_node`. The findings carry exactly the names the
//! evaluation's own table speaks — including the `Merged` rows earlier
//! unions minted — so the N3 name-tracking discipline this file used
//! to demonstrate by hand is now what the detector DOES. Detection
//! decides through the C4 verifier's own doors, so detect-then-declare
//! cannot disagree with the boolean's verify-at-use.
//!
//! Two authoring-visible consequences, both stated:
//!
//! - the detector also reports the COPLANAR-BUT-DISJOINT pairs: each
//!   new leg's floor against the earlier legs' floors at z = 0, and
//!   its inner-wall planes against same-side earlier legs' (all
//!   `SameOriented`). Declaring them is recording true intent — those
//!   faces ARE flush by construction — and a declared pair whose
//!   faces never meet in the op is a SILENT no-op at the op (C4's
//!   verified-at-use posture, CONTACT-DESIGN: verification happens
//!   where declaration meets geometry, and these never meet), so the
//!   pins below are unchanged;
//! - the `Declare` nodes therefore carry 2/4/5/7 pairs (the
//!   inspection comment at the assert derives the inventory) rather
//!   than the hand-picked 2 the old segment table wrote down. The
//!   vocabulary exercised is identical.
//!
//! Vocabulary: Profile, Extrude, Declare, Boolean (Union),
//! `InsertNode`, `SetParam`.
//!
//! Geometry (dyadic): top `[0,4] × [0,3] × [1,1.25]`; four legs
//! `0.5 × 0.5`, `z ∈ [0,1.125]`, each aligned to a corner so TWO of
//! its side faces are coplanar with the top's sides.
//!
//! Exact oracles, derived here:
//!
//! - volume = top `4·3·0.25 = 3` + per leg `0.5·0.5·1.125 = 0.28125`
//!   minus the slab overlap `0.5·0.5·0.125 = 0.03125` ⇒ `+0.25` each
//!   ⇒ **4.0**
//! - area: top alone `2·12 + 2·1 + 2·0.75 = 27.5`; a leg alone
//!   `2·0.25 + 4·(0.5·1.125) = 2.75`. Each leg's union removes
//!   `0.625` of INTERIOR boundary — the top's bottom face under the
//!   leg footprint `0.25`, the leg's top cap `0.25`, and the leg's two
//!   inner side strips inside the slab `2·(0.5·0.125) = 0.125` — plus
//!   `0.125` of DOUBLE-COUNTED coplanar overlap (each of the leg's two
//!   outer walls overlaps the top's own side face on
//!   `0.5 × 0.125 = 0.0625`; that region is one merged face, counted
//!   once). So `27.5 + 4·2.75 − 4·0.625 − 4·0.125` =
//!   `27.5 + 11 − 2.5 − 0.5` = **35.5**
//!
//! D2 bump: the first leg's `Distance` (mid-DAG — its cone is that
//! extrude plus all four unions, everything else reused).

use editor_core::{
    BooleanOp, CancelToken, Dimension, DocEdit, EvalOptions, Evaluation, Expr, Node, RoleSeg,
    SlotId, declare_node, evaluate, find_flush_candidates,
};
use topo::PlaneRelation;

use super::super::fixture::{desc, len};
use super::{CorpusDoc, MassPin, Recorder};
use geom_core::Tol;

/// The four leg footprints, corner order `(0,0) → (4,0) → (4,3) →
/// (0,3)`. Which faces are flush is no longer written down here —
/// that is the detector's job.
fn legs() -> [[(f64, f64); 4]; 4] {
    [
        [(0.0, 0.0), (0.5, 0.0), (0.5, 0.5), (0.0, 0.5)],
        [(3.5, 0.0), (4.0, 0.0), (4.0, 0.5), (3.5, 0.5)],
        [(3.5, 2.5), (4.0, 2.5), (4.0, 3.0), (3.5, 3.0)],
        [(0.0, 2.5), (0.5, 2.5), (0.5, 3.0), (0.0, 3.0)],
    ]
}

/// The corner-table corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    let top_profile = r.insert(Node::Profile(desc(
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)]],
    )));
    let top = r.insert(Node::Extrude {
        profile: top_profile,
        distance: len(0.25),
    });

    let mut acc = top;
    let mut first_leg = top; // overwritten by the first leg's extrude
    let mut prior: Option<Evaluation<f64>> = None;
    for (i, poly) in legs().into_iter().enumerate() {
        let prof = r.insert(Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![poly.to_vec()],
        )));
        let ext = r.insert(Node::Extrude {
            profile: prof,
            distance: len(1.125),
        });
        if i == 0 {
            first_leg = ext;
        }
        // The protocol (SELECT-DESIGN §3): evaluate, detect, INSPECT,
        // declare — findings pass through the author's hands as
        // values; nothing is fused.
        let ev = evaluate::<f64>(
            &r.doc,
            prior.as_ref(),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let findings = find_flush_candidates(&ev, acc, ext, Tol::witness())
            .expect("corner-table flush pairs are definite");
        // The inspection. Each leg shares planes with the accumulated
        // body in three families: its two corner wall planes (against
        // the top's — by leg 2, the already-MERGED — side faces), the
        // floor plane of every earlier leg (disjoint coplanar faces),
        // and the inner-wall plane it shares with each same-side
        // earlier leg (leg 2's y = 0.5 with leg 1; leg 3's x = 3.5
        // with leg 2; leg 4's y = 2.5 with leg 3 AND x = 0.5 with
        // leg 1). Hence 2, then 2+1+1, then 2+2+1, then 2+3+2 — and
        // every one a flush wall, the merge-stage SameOriented flavor.
        assert_eq!(findings.len(), [2, 4, 5, 7][i], "leg {i}: {findings:#?}");
        assert!(
            findings
                .iter()
                .all(|f| f.evidence.relation == PlaneRelation::SameOriented),
            "leg {i}: {findings:#?}"
        );
        // The N3 demonstration the hand-tracking used to carry: once a
        // wall pair has GLUED (leg 1's union), the accumulated body's
        // side of the next wall finding is the boolean's `Merged` row
        // — the detector answers with the N3 merge-lane name itself.
        if i > 0 {
            assert!(
                findings
                    .iter()
                    .any(|f| matches!(f.pair.0.path.first(), Some(RoleSeg::Merged(_)))),
                "leg {i}: no Merged-named wall in {findings:#?}"
            );
        }
        let decl = r.insert(declare_node(&findings).expect("nonempty findings"));
        let uni = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: acc,
            b: ext,
            declare: Some(decl),
        });
        acc = uni;
        prior = Some(ev);
    }
    CorpusDoc {
        name: "corner_table",
        about: "corner-aligned four-leg table (declared flush contacts)",
        edits: r.edits,
        doc: r.doc,
        result: Some(acc),
        pin: Some(MassPin {
            volume: 4.0,
            area: Some(35.5),
        }),
        bump: DocEdit::SetParam {
            node: first_leg,
            slot: SlotId::Distance,
            expr: Expr::literal(1.0625, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: first_leg,
    }
}
