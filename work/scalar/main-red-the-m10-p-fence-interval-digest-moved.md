---
id: main-red-the-m10-p-fence-interval-digest-moved
kind: issue
title: main red: editor-core m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval reads a digest off its pin on main 27205d77
status: closed
opened: 2026-11-03
priority: P1
cost: E
closed: 2026-11-19
---


Filed by the SPLICE orchestrator (2026-11-03) from SPLICE's `nurbs-revert-roster` lane, which met it on PR 6476 and reproduced it on main `27205d771f`'s own tree with the identical digest.

## What

`crates/editor-core/tests/m10_p_fence.rs`, `the_corpus_evaluation_is_bit_identical_at_interval`: the Interval lane's corpus digest no longer matches its pin on main. The f64 and id-masked rows were not reported moving. A change on main moved the Interval lane's bits without re-pinning (the same shape as the closed `green/main-reds-editor-core-slow-set-and-the-k-probe-sweep`'s m10_p_fence entry).

Reproduce: `cargo nextest run -p editor-core -E 'test(=m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval)'` on main.

## What closing it takes

Bisect the main commit that moved it (candidates: today's REFS corpus re-pins `07442331`, the face-loop-home refactor), decide that the new bits are right, and re-pin with the move named. P1, because a red editor-core row on main masks every program's regressions there.

## Closed (2026-11-19, SCALAR orchestrator)

The red is gone from main.
`m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`
passes in the 2026-11-17 nightly (run 44023697569, main `cdee746161`)
at the default ε, 1e-6 and 1e-12. The Interval fence was re-taken with
its cause by the lanes that moved it (`c938343f40`, ONECOPY's angle
reads; `10f862ba3c`, the slow set's head-only reds), and nothing has
touched `m10_p_fence.rs` since that nightly's commit.
