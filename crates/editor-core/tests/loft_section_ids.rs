//! **A loft section is named by a minted `SectionId`** (`names/README.md`,
//! N1): its walls, seams and interior rings spell the ids of the sections
//! they lie at, so an edit elsewhere in the section list renames none of
//! them, two sections of one profile stay apart, and both doors that
//! write a loft's ids hold them to what the mint drew.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture::{self, insert, len, section_ids};
use editor_core::{
    Axis3, DocEdit, EditError, EvalOptions, Formula, LoftSection, Node, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, SectionId, SectionIdBreak, SectionIdFault, SlotId, SlotValue,
    SpokenName, StableName, StationMove,
};
use geom_core::Tol;

/// A section of `profile` slid up the first section's `z` by `z`.
fn up(profile: RecipeNodeId, z: f64) -> LoftSection<Operand, Formula> {
    LoftSection {
        id: None,
        profile: profile.into(),
        station: vec![StationMove::Slide {
            along: Axis3::Z,
            by: len(z),
        }],
    }
}

/// `up`, keeping the old section `id`.
fn kept(profile: RecipeNodeId, z: f64, id: Option<SectionId>) -> LoftSection<Operand, Formula> {
    LoftSection {
        id,
        ..up(profile, z)
    }
}

/// Squares of half-sides `halves`, one profile each, and a loft of them
/// at degree `degree`, section `k` at `z = k`.
fn loft_at(halves: &[f64], degree: i64) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
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
            first_id: None,
            then: (1..profiles.len())
                .map(|k| up(profiles[k], k as f64))
                .collect(),
            v_degree: Formula::count(degree),
        },
    );
    (doc, profiles, loft)
}

/// [`loft_at`] at degree 1.
fn loft_of(halves: &[f64]) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    loft_at(halves, 1)
}

/// `SetLoftSections` on `loft`: `sections` as `(profile, z)`, the
/// first's `z` unused, each keeping the old section `kept` names by its
/// index or `None` for a new one.
fn set(
    doc: &ProfileDoc,
    loft: RecipeNodeId,
    sections: &[(RecipeNodeId, f64)],
    keep: &[Option<usize>],
) -> Result<ProfileDoc, EditError> {
    let old = section_ids(doc, loft);
    let id = |k: usize| keep[k].map(|k| old[k]);
    doc.apply(
        &DocEdit::SetLoftSections {
            node: loft,
            first: sections[0].0.into(),
            first_id: id(0),
            then: (1..sections.len())
                .map(|k| kept(sections[k].0, sections[k].1, id(k)))
                .collect(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .map(|applied| applied.doc)
}

/// The sections a loft name spells, read off every segment of its path,
/// `None` for one that spells none (a cap, a rim, a cap vertex, the
/// body).
fn spelled(name: &StableName) -> Option<Vec<SectionId>> {
    let ids: Vec<SectionId> = name
        .path
        .iter()
        .flat_map(|seg| match seg {
            RoleSeg::LoftWall(at) => at.iter().map(|&(s, _)| s).collect(),
            RoleSeg::LoftSeam(at) => at.iter().map(|&(s, _)| s).collect(),
            RoleSeg::LoftSectionEdge(s, _) | RoleSeg::LoftSectionVertex(s, _) => vec![*s],
            _ => Vec::new(),
        })
        .collect();
    (!ids.is_empty()).then_some(ids)
}

/// Every name the evaluated `loft` publishes.
fn names(doc: &ProfileDoc, loft: RecipeNodeId) -> BTreeSet<StableName> {
    names_of(&fixture::run(doc, &EvalOptions::default()), loft)
}

/// Every name `ev` publishes for `loft`.
fn names_of(ev: &editor_core::Evaluation<f64>, loft: RecipeNodeId) -> BTreeSet<StableName> {
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

/// The first loft wall of `all` over exactly the sections `over`.
fn wall_over(all: &BTreeSet<StableName>, over: &[SectionId]) -> StableName {
    all.iter()
        .find(|n| match n.path.as_slice() {
            [RoleSeg::LoftWall(at)] => at.iter().map(|&(s, _)| s).eq(over.iter().copied()),
            _ => false,
        })
        .unwrap_or_else(|| panic!("a wall over {over:?}"))
        .clone()
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
    let ids = section_ids(&doc, loft);
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
    let minted = section_ids(&inserted, loft);
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
            first_id: None,
            then: vec![up(square, 1.0), up(square, 2.0)],
            v_degree: Formula::count(1),
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
        section_ids(&doc, loft),
        section_ids(&retyped, loft),
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

/// **A section list minted afresh is a new evaluation key.** Every
/// section re-minted over the same profiles and stations builds the same
/// body, so only the ids tell the two lofts apart; evaluated over the
/// first's evaluation, the loft's table spells the new ids and none of
/// the old. Broken by a loft key that leaves its section ids out (the
/// memo would hand back the old table, whose names no section of the
/// document holds).
#[test]
fn a_section_list_minted_afresh_is_a_new_evaluation_key() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let first = fixture::run(&doc, &EvalOptions::default());
    let old = section_ids(&doc, loft);
    let fresh = set(&doc, loft, &[(p[0], 0.0), (p[1], 1.0)], &[None, None])
        .expect("every section re-minted");
    let new = section_ids(&fresh, loft);
    assert!(
        new.iter().all(|id| !old.contains(id)),
        "every id minted afresh: {new:?} against {old:?}"
    );
    let again = fixture::run_with_prior(&fresh, &first);
    assert!(
        again.reused > 0,
        "the memo is live: the profiles are reused"
    );
    let spelled: BTreeSet<SectionId> = names_of(&again, loft)
        .iter()
        .filter_map(spelled)
        .flatten()
        .collect();
    assert_eq!(
        spelled,
        new.iter().copied().collect(),
        "the loft's table spells the new sections, not the old"
    );
}

/// **A slab's wall is said between its two sections, numbered as the
/// document holds them now, and a whole loft's wall by no section.** In
/// a four-section degree-1 loft the wall over the second and third
/// sections reads `between sections 2 and 3`, and by tag `between
/// sections <tag> and <tag>` in that order; with the second section
/// dropped, the wall over the third and fourth reads `between sections
/// 2 and 3` — the numerals are positions read off the document, not
/// spelled in the name. A degree-2 loft's wall spans every section and
/// is said with none, by the document and by tag. Broken by numerals
/// counted from the name's own list, swapped, or by tag left out, or by
/// a whole loft's wall listing its sections.
#[test]
fn a_slab_wall_is_said_between_its_sections_and_a_whole_loft_wall_by_none() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45, 0.4, 0.35]);
    let ids = section_ids(&doc, loft);
    let all = names(&doc, loft);
    let middle = wall_over(&all, &ids[1..3]);
    let said = doc.spoken_name(&middle).to_string();
    assert!(said.contains("between sections 2 and 3 "), "{said}");
    let by_tag = SpokenName::absent(middle).to_string();
    let tagged = format!("between sections {} and {} ", ids[1], ids[2]);
    assert!(by_tag.contains(&tagged), "{by_tag} says {tagged}");

    let last = wall_over(&all, &ids[2..]);
    let dropped = set(
        &doc,
        loft,
        &[(p[0], 0.0), (p[2], 2.0), (p[3], 3.0)],
        &[Some(0), Some(2), Some(3)],
    )
    .expect("the second section dropped");
    assert!(
        names(&dropped, loft).contains(&last),
        "the last slab's wall is kept"
    );
    let now = dropped.spoken_name(&last).to_string();
    assert!(now.contains("between sections 2 and 3 "), "{now}");
    let then = doc.spoken_name(&last).to_string();
    assert!(then.contains("between sections 3 and 4 "), "{then}");

    let (whole, _, cubic) = loft_at(&[0.5, 0.45, 0.4], 2);
    let ids = section_ids(&whole, cubic);
    let wall = wall_over(&names(&whole, cubic), &ids);
    for said in [
        whole.spoken_name(&wall).to_string(),
        SpokenName::absent(wall.clone()).to_string(),
    ] {
        assert!(
            said.starts_with("the loft wall over ") && !said.contains("section"),
            "a whole loft's wall is said by no section: {said}"
        );
    }
}

/// **The edit door refuses ids it was not given to keep**: a section of
/// another loft and one id kept twice, each typed with its fault; and an
/// inserted loft that carries ids of its own. Broken by a door that keeps
/// a foreign id (two lofts would share a section) or a doubled one (two
/// sections, one name).
#[test]
fn the_edit_door_refuses_a_foreign_and_a_doubled_id() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let (doc, other) = insert(
        doc,
        Node::Loft {
            first: p[1].into(),
            first_id: None,
            then: vec![up(p[0], 1.0)],
            v_degree: Formula::count(1),
        },
    );
    let ours = section_ids(&doc, loft);
    let foreign = section_ids(&doc, other)[0];
    let write = |first: SectionId, then: SectionId| {
        doc.apply(
            &DocEdit::SetLoftSections {
                node: loft,
                first: p[0].into(),
                first_id: Some(first),
                then: vec![kept(p[1], 1.0, Some(then))],
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
    let err = write(ours[0], foreign);
    assert!(
        err.to_string().contains("is not a section of the loft"),
        "the sentence: {err}"
    );
    assert_eq!(
        fault(err),
        SectionIdFault::NotThisLofts { section: foreign }
    );
    assert_eq!(
        fault(write(ours[0], ours[0])),
        SectionIdFault::Repeated { section: ours[0] }
    );
    let preminted = fixture::insert_refused(
        &doc,
        Node::Loft {
            first: p[0].into(),
            first_id: None,
            then: vec![kept(p[1], 1.0, Some(ours[1]))],
            v_degree: Formula::count(1),
        },
    );
    assert!(
        matches!(
            preminted,
            EditError::SectionIdsRefused {
                fault: SectionIdFault::Preminted,
                ..
            }
        ),
        "an inserted loft carrying ids: {preminted:?}"
    );
    // Every id kept, and the edit is admitted.
    set(&doc, loft, &[(p[0], 0.0), (p[1], 1.0)], &[Some(0), Some(1)])
        .expect("every id kept in place");
}

/// **The load door refuses a section id no door would write**: one
/// standing for two sections of a loft, one shared by two lofts, one the
/// mint log does not hold as a section's (a node's id), and a section
/// carrying none — a file written before sections were minted, refused
/// as a format break with the regenerate recourse. Broken by a load door
/// that reads a loft's ids unchecked, or checks uniqueness loft by loft
/// (two lofts would then share a section's names).
#[test]
fn the_load_door_refuses_a_doubled_shared_unminted_or_missing_section_id() {
    let (doc, p, loft) = loft_of(&[0.5, 0.45]);
    let (doc, other) = insert(
        doc,
        Node::Loft {
            first: p[1].into(),
            first_id: None,
            then: vec![up(p[0], 1.0)],
            v_degree: Formula::count(1),
        },
    );
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("the fixture saves");
    editor_core::load(&text, Tol::witness()).expect("the fixture loads");
    let ids = section_ids(&doc, loft);
    let theirs = section_ids(&doc, other);
    let spell = |id: SectionId| serde_json::Value::String(id.0.to_string());
    let doctor = |node: RecipeNodeId, edit: &dyn Fn(&mut serde_json::Value)| {
        crate::wire::doctored(&text, |wire| {
            edit(&mut wire["snapshot"]["nodes"][&node.0.to_string()]["Loft"]);
        })
    };
    let rows = [
        (
            doctor(loft, &|l| l["then"][0]["id"] = spell(ids[0])),
            SectionIdBreak::Repeated { section: ids[0] },
            "a doubled id",
        ),
        (
            doctor(other, &|l| l["first_id"] = spell(ids[0])),
            SectionIdBreak::Repeated { section: ids[0] },
            "an id two lofts share",
        ),
        (
            doctor(loft, &|l| l["first_id"] = loft.0.to_string().into()),
            SectionIdBreak::NotMinted {
                section: SectionId(loft.0),
            },
            "a node's id",
        ),
        (
            doctor(loft, &|l| {
                l["then"][0]
                    .as_object_mut()
                    .expect("a section")
                    .remove("id");
            }),
            SectionIdBreak::Unminted { section: 2 },
            "a section with no id",
        ),
    ];
    assert_ne!(theirs[0], ids[0], "the two lofts' own ids are apart");
    for (corrupt, want, what) in rows {
        match editor_core::load(&corrupt, Tol::witness()) {
            Err(editor_core::PersistError::Snapshot(
                error @ editor_core::SnapshotError::SectionIds { fault, .. },
            )) => {
                assert_eq!(fault, want, "{what}");
                if let SectionIdBreak::Unminted { .. } = want {
                    let said = error.to_string();
                    assert!(
                        said.contains(editor_core::REGENERATE_RECOURSE),
                        "a format break says to regenerate: {said}"
                    );
                }
            }
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
    assert_eq!(section_ids(&repointed, loft), section_ids(&doc, loft));
}
