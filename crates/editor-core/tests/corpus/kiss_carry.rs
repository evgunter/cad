//! Corpus document **kiss_carry** — the tier-3′ carry as a recipe: a
//! corner-kiss assembly union whose surviving v-v record the next
//! boolean carries from its operand, so a boolean value with NON-EMPTY
//! surviving contacts exists in the corpus.
//!
//! That non-emptiness is the document's reason to exist: every other
//! corpus boolean's `ContactRecords` come out empty (glued rests are
//! consumed into seam structure; subtracts drop the B side), so no
//! other corpus row can tell a lowering that carries the contacts from
//! one that drops them. The two boolean values here carry one v-v
//! record each — the first DISCOVERED by the op at the kiss, the second
//! CARRIED from the first union, its operand.
//!
//! Vocabulary: Profile, Extrude, Boolean (Union), `InsertNode`,
//! `SetParam`.
//!
//! Geometry (dyadic): block a `[0,1]³`; block b `[1,2]² × [1,2]`,
//! kissing a at the single point `(1,1,1)`; mover c
//! `[1.5,2.5]² × [1.5,2.5]`, crossing b transversally (no coincident
//! planes anywhere — the only contact in the document is the kiss).
//!
//! Exact oracles, derived here:
//!
//! - volume = `1 + 1 + 1 − 0.5³` (the b∩c corner cube) = **2.875**
//! - area: a contributes `6` (a point contact removes no area); b∪c =
//!   `6 + 6 − 6·(0.5·0.5)` (each cube loses three `0.5×0.5` face
//!   patches interior to the other) `= 10.5` ⇒ **16.5**
//!
//! D2 bump: the mover's `Distance` (mid-DAG — its cone is that
//! extrude plus the second union; the kiss chain is reused).

use super::PlacePart;
use editor_core::{DocEdit, Node, SlotId};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// The kiss-carry corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    // Block a: [0,1]³. Profile (0,0)(1,0)(1,1)(0,1) → vertex 2 is
    // (1,1), the kiss corner on the TOP cap.
    let a_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let a = r.insert(Node::Extrude {
        profile: a_p.into(),
        distance: len(1.0),
    });
    // Block b: [1,2]² × [1,2], placed against a 1 up. Profile (1,1)(2,1)(2,2)(1,2) → vertex 0
    // is (1,1), the kiss corner on the BOTTOM cap.
    let b_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]],
    );
    let b = r.insert(Node::Extrude {
        profile: b_p.into(),
        distance: len(1.0),
    });
    let b_on_a = r.against(a, b, &editor_core::Frame::translation([0.0, 0.0, 1.0]));
    // The kiss union: the v-v kiss at (1,1,1) is DISCOVERED by the op
    // and recorded in the result's contacts.
    let u1 = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![a.into(), b_on_a.into()]),
    });

    // The mover: [1.5,2.5]² × [1.5,2.5], placed against a 1.5 up, a
    // transversal crossing of b.
    let c_p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.5, 1.5), (2.5, 1.5), (2.5, 2.5), (1.5, 2.5)]],
    );
    let c = r.insert(Node::Extrude {
        profile: c_p.into(),
        distance: len(1.0),
    });

    let c_on_a = r.against(a, c, &editor_core::Frame::translation([0.0, 0.0, 1.5]));
    let u2 = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![u1.into(), c_on_a.into()]),
    });

    r.place_part(u2);

    CorpusDoc {
        name: "kiss_carry",
        about: "corner-kiss assembly; the surviving v-v record carried into the next union",
        edits: r.edits,
        doc: r.doc,
        result: Some(u2),
        pin: Some(MassPin {
            volume: 2.875,
            area: Some(16.5),
        }),
        bump: DocEdit::SetParam {
            node: c,
            slot: SlotId::Distance,
            value: len(1.25).into(),
            fresh: Vec::new(),
        },
        bump_root: c,
    }
}
