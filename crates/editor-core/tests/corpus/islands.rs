//! Corpus documents **nested_islands_105**,
//! **nested_islands_106_depth1** and **nested_islands_106_depth2** —
//! the doubly-nested island chain from the issue #93 adversarial
//! review, as recipes.
//!
//! The chain is in GENERAL POSITION (no value-equal plane pair
//! anywhere), so nothing here is declared: these documents exercise
//! the boolean-of-boolean island machinery, not the declaration rung.
//!
//! - `nested_islands_105`: plate ∪ tube ∪ pillar. On pre-#93 main the
//!   second union SILENTLY returned 22.5 while the exact answer is
//!   22.4375 (issue #105 — a fail-loud violation); the corpus pins the
//!   exact value at the RECIPE level, so a regression shows up in the
//!   model layer, not only in `topo`.
//! - `nested_islands_106_depth1`: the same chain WITHOUT the pillar,
//!   intersected with a slab across the tube's midriff.
//! - `nested_islands_106_depth2`: WITH the pillar — island(tube outer)
//!   ⊃ ring(tube inner) ⊃ island(pillar) on the slab faces. This one
//!   was carried as a typed-REFUSAL pin through M4 PR 8a's first pass
//!   (the anchor-exhaustion arm defeated the consecutive-triple
//!   centroid generator); **#113 closed the gap** with the
//!   vertex-pair-chord anchor tier, so the pin has been RETIRED per its
//!   own baked instructions and the document promoted to a green,
//!   exactness-pinned member of the corpus. `topo`'s
//!   `issue93_nested_islands.rs` carries the kernel-level twin (and a
//!   depth-3 case).
//!
//! Vocabulary: Profile, Extrude, Boolean (Subtract/Union/Intersect),
//! `InsertNode`, `SetParam`.
//!
//! Exact oracles, derived here (all dimensions dyadic):
//!
//! - tube = outer `2·2·2.5 = 10` − hole `1·1·2.5 = 2.5` = `7.5`
//! - plate ∪ tube = `16 + 7.5 − 1.5` (the tube's `z ∈ [0.5,1]` slab
//!   overlap, footprint `4−1 = 3`) = **22.0**
//! - ∪ pillar = `22 + 0.5 − 0.0625` (the pillar's `z ∈ [0.75,1]`
//!   overlap with the plate, footprint `0.25`) = **22.4375**
//! - depth-1 ∩ slab = annulus `3` × slab height `1` = **3.0**
//! - depth-2 ∩ slab = (annulus `3` + pillar footprint `0.25`) × slab
//!   height `1` = `13/4` = **3.25** (the slab `z ∈ [1.375,2.375]` sits
//!   strictly inside both the tube's `[0.5,3]` and the pillar's
//!   `[0.75,2.75]`, so the cross section is constant through it)

use super::PlacePart;
use editor_core::{DocEdit, Node, RecipeNodeId, SlotId};

use crate::fixture::len;

use super::{CorpusDoc, MassPin, Recorder};

/// A rectangle on its own xy plane, extruded `h`.
fn block(r: &mut Recorder, x: (f64, f64), y: (f64, f64), h: f64) -> RecipeNodeId {
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(h),
    })
}

/// `body` placed against `target`, `z` up its own z: the operand a
/// combine with `target` reads.
fn on(r: &mut Recorder, target: RecipeNodeId, body: RecipeNodeId, z: f64) -> RecipeNodeId {
    r.against(
        target,
        body,
        &editor_core::Frame::translation([0.0, 0.0, z]),
    )
}

/// Plate ∪ (outer − hole): the depth-1 chain, exact 22.0, in the
/// plate's space. Returns the union node and the plate, the root every
/// later member is placed against.
fn plate_with_tube(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let plate = block(r, (0.0, 4.0), (0.0, 4.0), 1.0);
    let outer = block(r, (1.0, 3.0), (1.0, 3.0), 2.5);
    let hole = block(r, (1.5, 2.5), (1.5, 2.5), 3.0);
    // The hole's foot 0.25 up the plate, so 0.25 below the outer's.
    let hole = on(r, outer, hole, -0.25);
    let tube = r.insert(Node::Subtract {
        from: outer.into(),
        tool: hole.into(),
    });
    let tube = on(r, plate, tube, 0.5);
    let union = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![plate.into(), tube.into()]),
    });
    (union, plate)
}

/// The #105 exactness document: the doubly-nested union chain.
pub fn document_105() -> CorpusDoc {
    let mut r = Recorder::new();
    let (u1, plate) = plate_with_tube(&mut r);
    let pillar = block(&mut r, (1.75, 2.25), (1.75, 2.25), 2.0);
    let placed = on(&mut r, plate, pillar, 0.75);
    let u2 = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![u1.into(), placed.into()]),
    });
    r.place_part(u2);

    CorpusDoc {
        name: "nested_islands_105",
        about: "doubly-nested island union chain (#105 exactness pin)",
        edits: r.edits,
        doc: r.doc,
        result: Some(u2),
        pin: Some(MassPin {
            volume: 22.4375,
            area: None, // the island rings make the area non-obvious; volume is the oracle
        }),
        bump: DocEdit::SetParam {
            node: pillar,
            slot: SlotId::Distance,
            value: len(1.75).into(),
            fresh: Vec::new(),
        },
        bump_root: pillar,
    }
}

/// The #106 depth-1 control document: the same chain intersected with
/// a slab across the tube's midriff.
pub fn document_106_depth1() -> CorpusDoc {
    let mut r = Recorder::new();
    let (u1, plate) = plate_with_tube(&mut r);
    let slab = block(&mut r, (-1.0, 5.0), (-1.0, 5.0), 1.0);
    let placed = on(&mut r, plate, slab, 1.375);
    let cut = r.insert(Node::Intersect {
        members: editor_core::Bodies::Spelled(vec![u1.into(), placed.into()]),
    });
    r.place_part(cut);

    CorpusDoc {
        name: "nested_islands_106_depth1",
        about: "depth-1 nested-island intersect (the #93 coverage control)",
        edits: r.edits,
        doc: r.doc,
        result: Some(cut),
        pin: Some(MassPin {
            volume: 3.0,
            area: None,
        }),
        bump: DocEdit::SetParam {
            node: slab,
            slot: SlotId::Distance,
            value: len(0.75).into(),
            fresh: Vec::new(),
        },
        bump_root: slab,
    }
}

/// The **#106 depth-2 document**: (plate ∪ tube ∪ pillar) ∩ slab —
/// island ⊃ ring ⊃ island on the slab faces. Promoted from a typed
/// refusal pin to a green exactness pin by #113 (module docs).
pub fn document_106_depth2() -> CorpusDoc {
    let mut r = Recorder::new();
    let (u1, plate) = plate_with_tube(&mut r);
    let pillar = block(&mut r, (1.75, 2.25), (1.75, 2.25), 2.0);
    let placed = on(&mut r, plate, pillar, 0.75);
    let u2 = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![u1.into(), placed.into()]),
    });
    let slab = block(&mut r, (-1.0, 5.0), (-1.0, 5.0), 1.0);
    let placed = on(&mut r, plate, slab, 1.375);
    let cut = r.insert(Node::Intersect {
        members: editor_core::Bodies::Spelled(vec![u2.into(), placed.into()]),
    });
    r.place_part(cut);

    CorpusDoc {
        name: "nested_islands_106_depth2",
        about: "depth-2 nested-island intersect (#106 closed by #113; exact 13/4)",
        edits: r.edits,
        doc: r.doc,
        result: Some(cut),
        pin: Some(MassPin {
            volume: 3.25,
            area: None,
        }),
        // Mid-DAG: the pillar feeds the union, which feeds the
        // intersect. Shortening it to 7/4 keeps z ∈ [0.75, 2.5] ⊃ the
        // slab, so the depth-2 nesting (and the answer) is unchanged.
        bump: DocEdit::SetParam {
            node: pillar,
            slot: SlotId::Distance,
            value: len(1.75).into(),
            fresh: Vec::new(),
        },
        bump_root: pillar,
    }
}
