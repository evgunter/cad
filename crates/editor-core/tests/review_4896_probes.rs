//! Review probes for PR 4896: the consuming carve's naming, through the
//! document door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture;
use editor_core::{
    EvalOptions, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId,
};
use fixture::{len, scl, table, tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

fn lateral_edge(doc: &ProfileDoc, rod: RecipeNodeId, vertex: u32) -> editor_core::StableName {
    fixture::ename(
        rod,
        editor_core::RoleSeg::LateralEdge(fixture::vpiece(doc, rod, 0, vertex as usize)),
    )
}

/// The D-rod document, with or without the witness bore, filleted at
/// `r`: its rod and fillet nodes.
fn doc(bored: bool, r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let c = rod_chord_at(ROD_FLAT);
    let d = LoopProgram::Chain(vec![
        ProgramStep::At([len(ROD_FLAT), len(c.half)]),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(ROD_FLAT), len(-c.half)]),
            b: scl(c.wall_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let mut loops = vec![d];
    if bored {
        loops.push(LoopProgram::Circle {
            centre: [len(0.297), len(0.395)],
            radius: len(0.002),
        });
    }
    let doc = ProfileDoc::empty_derived("d_rod", tol());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops,
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(ROD_L),
            side: ExtrudeSide::Along,
        },
    );
    let mut selection = vec![lateral_edge(&doc, rod, 0), lateral_edge(&doc, rod, 1)];
    selection.sort();
    let (doc, fillet) = fixture::insert(
        doc,
        Node::Fillet {
            radius: len(r),
            selection: editor_core::Operand::select(rod, selection),
        },
    );
    (doc, rod, fillet)
}

/// **The consumed bore leaves no trace in the fillet's names**: the
/// bored rod's fillet node at `ROD_FILLET` carries exactly the names
/// the unbored rod's does, and each resolves to an entity of the same
/// kind. A name the carve added, dropped or re-derived would differ.
#[test]
fn p01_the_consuming_carve_names_exactly_what_the_unbored_carve_does() {
    let (bd, _, bf) = doc(true, ROD_FILLET);
    let (pd, _, pf) = doc(false, ROD_FILLET);
    let be = fixture::run(&bd, &EvalOptions::default());
    let pe = fixture::run(&pd, &EvalOptions::default());
    // Node and step ids differ between the two documents (their
    // profiles differ), so names compare with every id erased.
    let erase = |s: String| -> String {
        let mut out = String::new();
        let mut rest = s.as_str();
        while let Some(i) = rest.find("MintId {") {
            out.push_str(&rest[..i]);
            let j = rest[i..].find('}').expect("closing brace");
            out.push('_');
            rest = &rest[i + j + 1..];
        }
        out.push_str(rest);
        out
    };
    let names = |t: &editor_core::NameTable| -> BTreeSet<String> {
        t.iter()
            .map(|(n, e)| {
                let shape = match e {
                    editor_core::Entry::Unique(_) => "unique".to_string(),
                    editor_core::Entry::Tied(v) => format!("tied {}", v.len()),
                };
                erase(format!("{n:?} -> {shape}"))
            })
            .collect()
    };
    let (b, p) = (names(table(&be, bf)), names(table(&pe, pf)));
    let extra: Vec<_> = b.difference(&p).collect();
    let missing: Vec<_> = p.difference(&b).collect();
    assert!(
        extra.is_empty() && missing.is_empty(),
        "bored-only names {extra:#?}\nunbored-only names {missing:#?}"
    );
    assert_eq!(
        table(&be, bf).iter().count(),
        table(&pe, pf).iter().count(),
        "the same number of names"
    );
}
