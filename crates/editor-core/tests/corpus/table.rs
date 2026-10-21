//! Corpus document **corner_table** — the corner-aligned four-leg
//! table (`#71`; the kernel pins live in `topo/tests/corner_table.rs`).
//! Here it is a RECIPE: each leg is unioned onto the accumulated
//! body, and every flush contact — its two corner wall planes against
//! the top's sides, by leg 2 the already-MERGED side faces — is glued
//! by the union's own margins.
//!
//! Vocabulary: Profile, Extrude, Boolean (Union),
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

use editor_core::{DocEdit, Node, SlotId};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// The four leg footprints, corner order `(0,0) → (4,0) → (4,3) →
/// (0,3)`.
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
    // The top, the root: its own coordinates are the table's, one up
    // from the floor its legs stand on.
    let top_profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0)]],
    );
    let top = r.insert(Node::Extrude {
        profile: top_profile.into(),
        distance: len(0.25),
    });

    let mut acc = top;
    let mut first_leg = top; // overwritten by the first leg's extrude
    for (i, poly) in legs().into_iter().enumerate() {
        let prof = r.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![poly.to_vec()],
        );
        let ext = r.insert(Node::Extrude {
            profile: prof.into(),
            distance: len(1.125),
        });
        if i == 0 {
            first_leg = ext;
        }
        let leg = r.against(top, ext, &editor_core::Frame::translation([0.0, 0.0, -1.0]));
        let uni = r.insert(Node::Union {
            members: editor_core::Bodies::Spelled(vec![acc.into(), leg.into()]),
        });
        acc = uni;
    }
    r.in_world(acc, &editor_core::Frame::translation([0.0, 0.0, 1.0]));

    CorpusDoc {
        name: "corner_table",
        about: "corner-aligned four-leg table (flush contacts)",
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
            value: len(1.0625).into(),
            fresh: Vec::new(),
        },
        bump_root: first_leg,
    }
}
