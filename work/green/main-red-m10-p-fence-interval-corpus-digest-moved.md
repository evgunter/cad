---
id: main-red-m10-p-fence-interval-corpus-digest-moved
kind: issue
title: main red: m10_p_fence's Interval corpus digest moved (the_corpus_evaluation_is_bit_identical_at_interval)
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by `onecopy/small-reads` running editor-core's full suite, slow set included. The run was on that branch, then reproduced on `origin/main` at `27205d771` with the same digests.

`editor-core::all m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval` panics at `crates/editor-core/tests/m10_p_fence.rs` (~352):

> the corpus's Interval evaluation moved
> left: (1502694720337009394, 16097816026763852582)
> right: (18329942033039532738, 2410839470392928758)

It is a slow-set row, so the PR gate does not run it. No open row names it: `green/main-reds-editor-core-slow-set-and-the-k-probe-sweep` is closed. Fix: find the PR that moved the Interval corpus evaluation. If its output is right, re-pin the digest with the reason (`memories/output-stability-as-justification.md`).

Reproduce: `cargo nextest run -p editor-core -E 'test(=m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval)'`.
