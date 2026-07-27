//! Corpus document **heat_sink** — the parametric fin strip (#91 C5,
//! the tour's M4 showcase) as a self-contained recipe.
//!
//! The document carries BOTH shapes of "many fins":
//!
//! - a `Pattern` node (Linear, count driven by the `fins` Count
//!   document parameter through `SetStructuralParam`) whose value is
//!   the `Instances` payload — the A8/N1 instance substrate; and
//! - the explicit Transform + Union chain that makes ONE solid, since
//!   a Boolean node cannot consume a Pattern's `Instances` payload in
//!   v1 (the F4 note the demo records honestly). The exact oracle
//!   below is the chain's.
//!
//! Vocabulary: Profile, Extrude, Transform, Pattern (Linear),
//! Boolean (Union), `InsertNode`, `SetDocParam`,
//! `SetStructuralParam`, `SetParam`.
//!
//! Geometry (dyadic): base `[0,3] × [0,1] × [0,0.25]`; fin footprint
//! `0.1875 × 0.75` sketched at `z = 0.1875` — 1/16 INSIDE the base,
//! because a flush fin base would be an undeclared coincidence — and
//! extruded `0.8125` (to `z = 1`); five fins at `x`-spacing `0.3125`.
//!
//! Exact oracle, derived here: base `3 · 1 · 0.25 = 0.75`; each fin
//! adds only the material above the base, `0.1875 · 0.75 · 0.75 =
//! 0.10546875`; five fins ⇒ `0.75 + 0.52734375` = **1.27734375**.
//!
//! D2 bump: the fin master's `Distance` (mid-DAG — its cone is the
//! master extrude, the pattern, and all five Transform+Union pairs;
//! the base half of the DAG is reused).

use editor_core::{
    BooleanOp, Dimension, DocEdit, DocParam, Expr, Node, ParamName, PatternKind, SlotId,
};

use super::{CorpusDoc, MassPin, Recorder};
use crate::fixture::{ang, desc, len, scl};

/// The fin count the document starts at.
const FINS: i64 = 5;
/// The fin pitch.
const PITCH: f64 = 0.3125;

/// The heat-sink corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("fins"),
        value: DocParam::Count { value: FINS },
    });
    let base_p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)]],
    )));
    let base = r.insert(Node::Extrude {
        profile: base_p,
        distance: len(0.25),
    });
    let fin_p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.1875],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ]],
    )));
    let fin = r.insert(Node::Extrude {
        profile: fin_p,
        distance: len(0.8125),
    });
    // The instance-payload half of the document.
    let pattern = r.insert(Node::Pattern {
        input: fin,
        count: Expr::count(FINS),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(PITCH),
        },
    });
    // …now driven by the document's Count parameter (the STRUCTURAL
    // edit arm, distinct from `SetParam` by construction).
    r.push(DocEdit::SetStructuralParam {
        node: pattern,
        slot: SlotId::Count,
        expr: Expr::param(ParamName::new("fins"), Dimension::Count),
    });

    // The explicit one-solid chain. Fin i sits at x = i·PITCH; every
    // fin overlaps the base by 1/16, so no union has a coincidence.
    let mut acc = base;
    for i in 0..FINS {
        let tr = r.insert(Node::Transform {
            input: fin,
            translation: [len(i as f64 * PITCH), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        });
        acc = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: acc,
            b: tr,
            declare: None,
        });
    }

    CorpusDoc {
        name: "heat_sink",
        about: "parametric fin strip: Pattern instances + explicit union chain",
        edits: r.edits,
        doc: r.doc,
        result: Some(acc),
        pin: Some(MassPin {
            volume: 1.27734375,
            area: None, // fin/base seams make the area laborious; volume is the oracle
        }),
        bump: DocEdit::SetParam {
            node: fin,
            slot: SlotId::Distance,
            expr: Expr::literal(0.6875, Dimension::Length).expect("dyadic length literal"),
        },
        bump_root: fin,
    }
}
