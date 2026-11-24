//! **How many finishes the corpus runs**: every registered corpus
//! document evaluated at `f64` and run through the default checks,
//! counting each certifying finish where it runs
//! (`topo::finishes_on_this_thread`). The count is deterministic, so it
//! is pinned. A finished body moved by an isometry is not finished
//! again (`topo::FinishedBody::moved`), and a move of the count either
//! way is a change in where the kernel derives verdicts, to be read
//! and stated, not silenced.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use editor_core::ChecksConfig;
use geom_core::Tol;

/// The corpus's finishes, per document in the registry's order.
#[cfg(debug_assertions)]
fn finishes() -> Vec<(&'static str, u64)> {
    corpus::documents()
        .into_iter()
        .map(|d| {
            let before = topo::finishes_on_this_thread();
            let run = corpus::eval::<f64>(&d.doc);
            editor_core::run_checks(
                &d.doc,
                &run,
                &ChecksConfig::default(),
                &editor_core::Stop::default(),
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{}: the checks run: {e}", d.name));
            (d.name, topo::finishes_on_this_thread() - before)
        })
        .collect()
}

#[cfg(debug_assertions)]
#[test]
fn the_corpus_runs_its_pinned_count_of_finishes() {
    let per_document = finishes();
    let total: u64 = per_document.iter().map(|(_, n)| n).sum();
    assert_eq!(
        total, PINNED,
        "the corpus ran {total} finishes, pinned {PINNED}: {per_document:?}"
    );
}

/// The count at the default ε, measured with this row on one corpus:
/// main before a finished body's move carried its verdict ran 505;
/// with the move and no f64 leaf re-finish, 389; with the box-scalar
/// switch's price, a sweep leaf moved at `f64` finished again on its
/// image (`topo::LeafBody::moved`), 662, re-measured at 662 on the
/// corpus as merged with main `2f03dde9`. The switch's deletion
/// (`work/boxfinish/delete-the-box-scalar-sweep-leaf-switch.md`) takes
/// it back to about 389. ONEHOME fork 259 measured a leaf finished in its
/// own frame and then moved at 443 on that merged corpus, and reverted it
/// at the box scalars: a box-scalar leaf's tier 3 alone is not decided
/// over a box (its orientation, its seams), which is why the switch keeps
/// it tier 2 there. Then the readings between two operands (measure, the
/// quieting rule, the replay's carrier verdict, clearance and its
/// witness) read the other through its seat and move no body (PR 7088),
/// so no moved f64 sweep leaf is finished again on its image for them:
/// 662 -> 466, the 196 exactly those re-finishes (routing the seat back
/// through the whole-body move puts back 662). Then a gather of placed
/// copies grafts each through its seat (`topo::boolean::graft_placed`,
/// PR 7099): the image is the aggregate's construction state, and the
/// aggregate finishes once, so no mapped f64 leaf is finished on its
/// image at the gather: 466 -> 445 (heat_sink 44 -> 39, declared_tangency
/// 6 -> 4, kitchen_sink 21 -> 15, part_select 8 -> 6, pin_in_bore
/// 12 -> 6). Then the n-ary fold and the at-rest intersection take placed
/// operands (`topo::nary_placed`, `topo::boolean_placed`): a member, or a
/// copy, is finished in its own frame and the door moves the finished
/// body, carrying its verdict, so no mapped f64 leaf is finished again on
/// its image there: 445 -> 390, the fold 50 (die_composed_tour 94 -> 74,
/// die_tool 27 -> 22, heat_sink 39 -> 34, corner_table 23 -> 19,
/// heat_sink_fins 10 -> 6, nested_islands_106_depth2 12 -> 10, kiss_carry
/// 13 -> 11, and one each in nested_islands_105, nested_islands_106_depth1,
/// kitchen_sink, boss_union, face_sketch, plate_param, axis_projection and
/// blocks_on_one_floor) and the localization 5 (heat_sink 34 -> 29).
/// Moving each member's leaf before its take again puts back 440, and the
/// copies' as well, 445. Then an interference finding offers the assertion
/// that quiets it only where the quieting rule takes that assertion at
/// every member of the family it is written over
/// (`checks::at_rest::written_pair`), and reading each member's pair
/// finishes: 390 -> 398, all in heat_sink (29 -> 37). Offering the pair
/// without the members' reading puts back 390. Then a gather of finished
/// members carries their verdicts (`topo::boolean::gather_finished`):
/// 398 -> 375, one finish fewer in each document whose product's census
/// gathers finished copies, and two in cut_cylinder.
#[cfg(debug_assertions)]
const PINNED: u64 = 375;
