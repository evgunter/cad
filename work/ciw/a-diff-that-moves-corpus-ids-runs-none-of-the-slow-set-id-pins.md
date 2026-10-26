---
id: a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins
kind: issue
title: A diff that moves corpus ids runs none of the slow-set rows that pin them; three INTENT/ATREST units in two days left them red for the nightly
status: open
opened: 2026-10-24
priority: P2
cost: M
---


## What

Several slow-set rows pin corpus ids or names, among them:
- `lib_g16_corpus_name_digests::every_corpus_documents_name_tables_are_golden`;
- `edit_placement_corpus_bits::every_corpus_placement_places_its_body_by_the_pinned_bits`;
- `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_f64` and `…_at_interval`;
- `perf2_name_keying_differential::…`;
- `name_words_corpus::…` (`SAID_DIGEST`);
- the `m10_*` and `revalue_corpus` goldens.

A diff that changes what a node's preimage hashes moves all of them. That happens through a wire field, a content-key `VERSION` or a mint. The `ci` profile runs none of these rows unless the diff names them (`.config/nextest.toml`, `.github/workflows/ci.yml` change filter). So a green PR can leave them red for the nightly.

## Instances (2026-10-23/24)

- **INTENT D′2 (PR 5613):** merged green and left seven slow-set goldens red on main, ids only. They were re-blessed in PR 5616.
- **ATREST stage 4 I (PR 5637):** review 1 found six of the rows above red at head and green at base. The cause is ids after a mate, once the mate wire lost `class`. The lane had validated with `--profile ci`.
- **INTENT D′3 (PR 5636):** its handoff had to run the slow set by hand to find its own moves.

## Owed

Make the gate select the corpus id-pin rows for any diff that can move ids. Two possible ways:
- a seed on the content-key/wire modules (`eval/content_key`, `persist/`, `tag::format`);
- one cheap id-only digest row in the fast set, which the slow rows then refine.

Until then, implementer briefs must say to run the slow set of every touched crate before reporting green. — (INTENT orchestrator)

## A wider instance: an inventory of call sites (2026-10-26, SPLICE)

The class is wider than corpus ids. It covers any slow-set row that
inventories the tree, which a diff can break without naming the row.
PR 5715 (`splice/touch-doors`) added one test-side `boxes::edge_box(`
call in `boolean/carrier_touch_rows.rs`. That turns
`topo boolean::boxes::tests::every_door_that_reads_a_box_is_inventoried`
red, because its PINNED list has no row for that file. The PR's gate
was green and did not select the row. A reviewer's full local run of
geom, geom-brep and topo found it. (SPLICE orchestrator)

## Evidence (2026-10-26, ATREST orchestrator)

Another instance, this time for verdicts rather than ids. PR 5745 changed what the topo census reports for aligned face pairs (`crates/topo/src/census.rs`). Its PR gate was green and selected no slow census golden. `editor-core` `perf12_census_goldens` had moved at all three ε rows: heat_sink went from 105 findings to 5 at every size. The lane found the move only because the orchestrator asked it to run the slow census goldens by hand before merging. So a diff to `census.rs` should select the `perf12_census_*` rows, by the same mechanism this row asks for ids.
