//! **A loft section is named by a minted `SectionId`** (`names/README.md`,
//! N1): its walls, seams and interior rings spell the ids of the sections
//! they lie at, so an edit elsewhere in the section list renames none of
//! them, two sections of one profile stay apart, and both doors that
//! write a loft's ids hold them to what the mint drew.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture::{self, insert, len};
use editor_core::{
    Axis3, DocEdit, EditError, EvalOptions, Formula, LoftSection, Node, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, SectionId, SectionIdFault, SlotId, SlotValue, StableName,
    StationMove,
};
use geom_core::Tol;

/// A section of `profile` slid up the first section's `z` by `z`.
fn up(profile: RecipeNodeId, z: f64) -> LoftSection<Operand, Formula> {
    LoftSection {
        profile: profile.into(),
        station: vec![StationMove::Slide {
            along: Axis3::Z,
            by: len(z),
        }],
    }
}

/// Squares of half-sides `halves`, one profile each, and a degree-1 loft
/// of them, section `k` at `z = k`.
fn loft_of(halves: &[f64]) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("loft-section-ids", Tol::witness());
    let mut profiles = Vec::new();
    for &h in halves {
        let (d, p) = insert(
            doc,
            Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, h)])),
        );
        doc = d;
        profiles.push(p);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            first: profiles[0].into(),
            then: (1..profiles.len())
                .map(|k| up(profiles[k], k as f64))
                .collect(),
            v_degree: Formula::count(1),
            ids: Vec::new(),
        },
    );
    (doc, profiles, loft)
}

/// `SetLoftSections` on `loft`: `sections` as `(profile, z)`, the
/// first's `z` unused, each keeping the old section `kept` names by its
/// index or `None` for a new one.
fn set(
    doc: &ProfileDoc,
    loft: RecipeNodeId,
    sections: &[(RecipeNodeId, f64)],
    kept: &[Option<usize>],
) -> Result<ProfileDoc, EditError> {
    let old = fixture::section_ids(doc, loft);
    doc.apply(
        &DocEdit::SetLoftSections {
            node: loft,
            first: sections[0].0.into(),
            then: sections[1..].iter().map(|&(p, z)| up(p, z)).collect(),
            ids: kept.iter().map(|k| k.map(|k| old[k])).collect(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

/// The sections a loft name spells, `None` for one that spells none (a
/// cap, a rim, a cap vertex, the body).
fn spelled(name: &StableName) -> Option<Vec<SectionId>> {
    match name.path.first()? {
        RoleSeg::LoftWall(at) => Some(at.iter().map(|&(s, _)| s).collect()),
        RoleSeg::LoftSeam(at) => Some(at.iter().map(|&(s, _)| s).collect()),
        RoleSeg::LoftSectionEdge(s, _) | RoleSeg::LoftSectionVertex(s, _) => Some(vec![*s]),
        _ => None,
    }
}

/// Every name the evaluated `loft` publishes.
fn names(doc: &ProfileDoc, loft: RecipeNodeId) -> BTreeSet<StableName> {
    let ev = fixture::run(doc, &EvalOptions::default());
    let table = ev
        .table_at(loft)
        .unwrap_or_else(|| panic!("the loft evaluates: {:?}", ev.node_error(loft)));
    table.iter().map(|(n, _)| n.clone()).collect()
}

/// The names of `all` that spell sections, every one among `among`.
fn spelling_only(all: &BTreeSet<StableName>, among: &[SectionId]) -> BTreeSet<StableName> {
    all.iter()
        .filter(|n| spelled(n).is_some_and(|s| s.iter().all(|id| among.contains(id))))
        .cloned()
        .collect()
}

/// **Dropping or inserting an earlier section renames no later slab,
/// ring or vertex.** A four-section degree-1 loft of squares: the names
/// spelling only its last two sections — the last slab's 4 walls and 4
/// seams, and the third section's ring, 4 edges and 4 vertices — are
/// all published again after the second section is dropped, and after
/// a section is inserted between the first two. Broken by a slab or
/// ring keyed on its position (dropping the second section shifts both).
#[test]
fn an_earlier_section_dropped_or_inserted_renames_no_later_name() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45, 0.4, 0.35]);
    let ids = fixture::section_ids(&doc, loft);
    assert_eq!(ids.len(), 4, "one id per section, the first included");
    let before = names(&doc, loft);
    let later = spelling_only(&before, &ids[2..]);
    assert_eq!(
        later.len(),
        16,
        "the last slab's walls and seams and the third section's ring: {later:#?}"
    );

    let dropped = set(
        &doc,
        loft,
        &[(p[0], 0.0), (p[2], 2.0), (p[3], 3.0)],
        &[Some(0), Some(2), Some(3)],
    )
    .expect("the second section dropped");
    let after = names(&dropped, loft);
    assert!(
        later.is_subset(&after),
        "dropping an earlier section renamed {:#?}",
        later.difference(&after).collect::<Vec<_>>()
    );

    let (doc, extra) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.48)])),
    );
    let inserted = set(
        &doc,
        loft,
        &[
            (p[0], 0.0),
            (extra, 0.5),
            (p[1], 1.0),
            (p[2], 2.0),
            (p[3], 3.0),
        ],
        &[Some(0), None, Some(1), Some(2), Some(3)],
    )
    .expect("a section inserted");
    let after = names(&inserted, loft);
    let later = spelling_only(&before, &ids[1..]);
    assert_eq!(later.len(), 32, "two slabs and two rings: {later:#?}");
    assert!(
        later.is_subset(&after),
        "inserting an earlier section renamed {:#?}",
        later.difference(&after).collect::<Vec<_>>()
    );
    let minted = fixture::section_ids(&inserted, loft);
    assert!(
        !ids.contains(&minted[1]),
        "the inserted section's id is minted afresh: {minted:?} against {ids:?}"
    );
}

/// **One profile lofted at three stations names each slab's walls
/// apart.** The same steps draw every section, so only the sections'
/// ids tell the two slabs' walls apart. Broken by a wall keyed on its
/// pieces alone (the two slabs' walls would collide and the table
/// refuse the second).
#[test]
fn one_profile_at_several_stations_names_its_walls_apart() {
    let mut doc = ProfileDoc::empty_derived("one-profile", Tol::witness());
    let (d, square) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)])),
    );
    doc = d;
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            first: square.into(),
            then: vec![up(square, 1.0), up(square, 2.0)],
            v_degree: Formula::count(1),
            ids: Vec::new(),
        },
    );
    let walls: BTreeSet<StableName> = names(&doc, loft)
        .into_iter()
        .filter(|n| matches!(n.path.first(), Some(RoleSeg::LoftWall(_))))
        .collect();
    assert_eq!(walls.len(), 8, "two slabs of four walls: {walls:#?}");
    let pieces: BTreeSet<_> = walls
        .iter()
        .map(|n| match n.path.first() {
            Some(RoleSeg::LoftWall(at)) => at.iter().map(|&(_, e)| e).collect::<Vec<_>>(),
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        pieces.len(),
        4,
        "each slab's walls pair the same pieces, so only the ids part them"
    );
}

/// **A retyped station renames nothing.** A value typed at a station's
/// slide mints a fresh variable (VR6), which a section keyed on its
/// station's reads would follow; a section keyed on its id does not.
#[test]
fn a_retyped_station_renames_nothing() {
    let (doc, _, loft) = loft_of(&[0.5, 0.45, 0.4]);
    let before = names(&doc, loft);
    let retyped = doc
        .apply(
            &DocEdit::SetParam {
                node: loft,
                slot: SlotId::StationSlide {
                    section: 1,
                    step: 0,
                },
                value: SlotValue::Formula(len(1.5)),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("the station retyped")
        .doc;
    let Some(Node::Loft { then, .. }) = retyped.node(loft) else {
        panic!("a loft");
    };
    let Some(Node::Loft { then: was, .. }) = doc.node(loft) else {
        panic!("a loft");
    };
    assert_ne!(then[0], was[0], "the retype minted a fresh variable");
    assert_eq!(
        fixture::section_ids(&doc, loft),
        fixture::section_ids(&retyped, loft),
        "a retype keeps every section's id"
    );
    assert_eq!(names(&retyped, loft), before, "a retype renames nothing");
}

/// **Appending a section at degree 1 keeps the existing walls' names.**
/// Through two sections a wall spans both, and after the append the
/// first slab's wall spans the same two. Broken by a wall spelled one
/// way through two sections and another through more.
#[test]
fn appending_a_section_keeps_the_existing_walls() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let (doc, third) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.4)])),
    );
    let before = names(&doc, loft);
    let walls: BTreeSet<StableName> = before
        .iter()
        .filter(|n| {
            matches!(
                n.path.first(),
                Some(RoleSeg::LoftWall(_) | RoleSeg::LoftSeam(_))
            )
        })
        .cloned()
        .collect();
    assert_eq!(walls.len(), 8, "four walls and four seams");
    let appended = set(
        &doc,
        loft,
        &[(p[0], 0.0), (p[1], 1.0), (third, 2.0)],
        &[Some(0), Some(1), None],
    )
    .expect("a section appended");
    let after = names(&appended, loft);
    assert!(
        walls.is_subset(&after),
        "the append renamed {:#?}",
        walls.difference(&after).collect::<Vec<_>>()
    );
}

/// **The edit door refuses ids it was not given to keep**: a section of
/// another loft, one id kept twice, and a count that is not one per
/// section, each typed with its fault; and an inserted loft that carries
/// ids of its own. Broken by a door that keeps a foreign id (two lofts
/// would share a section) or a doubled one (two sections, one name).
#[test]
fn the_edit_door_refuses_a_foreign_a_doubled_and_a_miscounted_id() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let (doc, other) = insert(
        doc,
        Node::Loft {
            first: p[1].into(),
            then: vec![up(p[0], 1.0)],
            v_degree: Formula::count(1),
            ids: Vec::new(),
        },
    );
    let ours = fixture::section_ids(&doc, loft);
    let foreign = fixture::section_ids(&doc, other)[0];
    let write = |ids: Vec<Option<SectionId>>| {
        doc.apply(
            &DocEdit::SetLoftSections {
                node: loft,
                first: p[0].into(),
                then: vec![up(p[1], 1.0)],
                ids,
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("refused")
    };
    let fault = |err: EditError| match err {
        EditError::SectionIdsRefused { node, fault } => {
            assert_eq!(node.id(), loft, "the refusal names the loft");
            fault
        }
        other => panic!("SectionIdsRefused, got {other:?}"),
    };
    let err = write(vec![Some(ours[0]), Some(foreign)]);
    assert!(
        err.to_string().contains("is not a section of the loft"),
        "the sentence: {err}"
    );
    assert_eq!(
        fault(err),
        SectionIdFault::NotThisLofts { section: foreign }
    );
    assert_eq!(
        fault(write(vec![Some(ours[0]), Some(ours[0])])),
        SectionIdFault::Repeated { section: ours[0] }
    );
    assert_eq!(
        fault(write(vec![Some(ours[0])])),
        SectionIdFault::Count {
            sections: 2,
            given: 1
        }
    );
    let preminted = fixture::insert_refused(
        &doc,
        Node::Loft {
            first: p[0].into(),
            then: vec![up(p[1], 1.0)],
            v_degree: Formula::count(1),
            ids: ours.clone(),
        },
    );
    assert_eq!(fault(preminted), SectionIdFault::Preminted);
    // Every id kept, and the edit is admitted.
    set(&doc, loft, &[(p[0], 0.0), (p[1], 1.0)], &[Some(0), Some(1)])
        .expect("every id kept in place");
}

/// **The load door refuses a section id no door would write**: one
/// standing for two sections, one the mint log does not hold as a
/// section's (a node's id), and a count that is not one per section.
/// Broken by a load door that reads a loft's ids unchecked, which a
/// replay would then mint again for another section.
#[test]
fn the_load_door_refuses_a_doubled_unminted_or_miscounted_section_id() {
    let (doc, _, loft) = loft_of(&[0.5, 0.45]);
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("the fixture saves");
    editor_core::load(&text, Tol::witness()).expect("the fixture loads");
    let key = loft.0.to_string();
    let ids = fixture::section_ids(&doc, loft);
    let with = |ids: serde_json::Value| {
        crate::wire::doctored(&text, |wire| {
            wire["snapshot"]["nodes"][&key]["Loft"]["ids"] = ids;
        })
    };
    let spell = |id: SectionId| serde_json::Value::String(id.0.to_string());
    let rows = [
        (
            with(serde_json::json!([spell(ids[0]), spell(ids[0])])),
            SectionIdFault::Repeated { section: ids[0] },
            "a doubled id",
        ),
        (
            with(serde_json::json!([spell(ids[0]), loft.0.to_string()])),
            SectionIdFault::NotMinted {
                section: SectionId(loft.0),
            },
            "a node's id",
        ),
        (
            with(serde_json::json!([spell(ids[0])])),
            SectionIdFault::Count {
                sections: 2,
                given: 1,
            },
            "one id for two sections",
        ),
    ];
    for (corrupt, want, what) in rows {
        match editor_core::load(&corrupt, Tol::witness()) {
            Err(editor_core::PersistError::Snapshot(editor_core::SnapshotError::SectionIds {
                fault,
                ..
            })) => assert_eq!(fault, want, "{what}"),
            other => panic!("{what}: refused at load, got {other:?}"),
        }
    }
}

/// **A `SetMembers` re-pointing a loft's profiles keeps its section
/// ids**: it writes reads, never the sections' identities.
#[test]
fn set_members_on_a_loft_keeps_its_section_ids() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let (doc, other) = insert(
        doc,
        Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.3)])),
    );
    let repointed = doc
        .apply(
            &DocEdit::SetMembers {
                node: loft,
                members: editor_core::Bodies::Spelled(vec![p[0].into(), other.into()]),
            },
            Tol::witness(),
        )
        .expect("a section re-pointed")
        .doc;
    assert_eq!(repointed.upstream(loft), vec![p[0], other]);
    assert_eq!(
        fixture::section_ids(&repointed, loft),
        fixture::section_ids(&doc, loft)
    );
}
