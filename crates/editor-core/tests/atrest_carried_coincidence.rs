//! **An instance carries its part's unproven coincidences** (D10
//! `[assertions]`, `[placement.world]`; ASSEMBLY A5): a part's unproven
//! row is the part's finding, judged by the part's own door, and the
//! report over a document instancing it carries it once per part row,
//! by its route.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::insert;

use editor_core::{CheckFinding, CheckId, ChecksConfig, Node, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// **A plate resting on a block, unioned**: the union's `Rest` of the
/// block's top and the plate's bottom, two constructions, is unproven.
fn resting_part() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("carried-coincidence-rest", Tol::witness());
    let (doc, base) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, plate) = block_on(doc, base, (0.5, 1.5), (0.5, 1.5), 1.0, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![base.into(), plate.into()]),
        },
    );
    crate::fixture::place(doc, union).0
}

/// The `unproven-coincidence` findings of the checks over `doc`.
fn unproven(doc: &ProfileDoc, opts: &editor_core::EvalOptions) -> Vec<CheckFinding> {
    let run = crate::fixture::run(doc, opts);
    editor_core::run_checks(doc, &run, &ChecksConfig::default(), Tol::witness())
        .unwrap_or_else(|e| panic!("the checks run: {e}"))
        .findings
        .into_iter()
        .filter(|f| f.check == CheckId::UnprovenCoincidence)
        .collect()
}

/// **A box split in two and reunited**: every row of the union lies on
/// the split's one tool plane, and the door proves each.
fn reunited_part() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("carried-coincidence-reunite", Tol::witness());
    let (doc, the_box) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let tool = crate::fixture::plane_on(&doc, the_box, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: the_box.into(),
            tool,
        },
    );
    let half = |half: editor_core::SplitHalf| {
        doc.output(split, half.port())
            .expect("a split defines a port per half")
    };
    let members = vec![
        half(editor_core::SplitHalf::Above).into(),
        half(editor_core::SplitHalf::Below).into(),
    ];
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(members),
        },
    );
    crate::fixture::place(doc, union).0
}

/// **A part's unproven `Rest` is reported through its instance.** The
/// part reports its union's rows unproven; the host instancing it
/// reports each once, the part's, by the route through the instance.
///
/// *Breaks if* the instance drops the part's rows (the host's lint is
/// silent, and `Certified` reads that as proven).
#[test]
fn a_parts_unproven_rest_is_reported_through_its_instance() {
    let part = crate::fixture::assemble_world_copies(resting_part());
    let own = unproven(&part, &editor_core::EvalOptions::default());
    assert_eq!(
        own.len(),
        5,
        "the premise: the rest and its four corners: {own:?}"
    );
    let (host, ids, opts): (ProfileDoc, Vec<RecipeNodeId>, _) =
        crate::intent_part_census_seam::instantiating(
            "carried-coincidence-host",
            &resting_part(),
            1,
        );
    let carried = unproven(&host, &opts);
    assert_eq!(
        carried.len(),
        own.len(),
        "each of the part's rows, once, through {ids:?}: {carried:?}"
    );
}

/// **A part's proven rows report nothing through its instance.**
///
/// *Breaks if* the host re-decides a row the part's door proved.
#[test]
fn a_parts_proven_rows_report_nothing_through_its_instance() {
    let part = crate::fixture::assemble_world_copies(reunited_part());
    let own = unproven(&part, &editor_core::EvalOptions::default());
    assert!(
        own.is_empty(),
        "the premise: the part's door proves every row: {own:?}"
    );
    let (host, _, opts) = crate::intent_part_census_seam::instantiating(
        "carried-coincidence-proven",
        &reunited_part(),
        1,
    );
    let carried = unproven(&host, &opts);
    assert!(carried.is_empty(), "nothing of the part's: {carried:?}");
}
