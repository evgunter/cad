---
id: shell-tolerance-chain-hand-rolls-sentinel-region-and-diverges
kind: issue
title: shell_tolerance_chain's hand-rolled sentinel region includes the opening sentinel where source::sentinel_region excludes it
status: open
opened: 2026-09-13
priority: P4
cost: E
---


## Finding

Found by the delta review of PR 2517, which swept
`test_utils::source::line`'s duplicates and labelled this file as
carrying "a second instance of a different shared function". **That
label undersold it**, and the correction is the finding.

`crates/topo/tests/shell_tolerance_chain.rs` hand-rolls what
`test_utils::source::sentinel_region` does — `find` the opener, `find`
the closer, assert not inverted — and the two **do not agree**: the
hand-rolled one returns a region that INCLUDES the opening sentinel,
where `source::sentinel_region` returns `b + begin.len()..e` and
excludes it.

That divergence is the exact drift a shared function exists to stop: a
guard written against one convention, read by someone who knows the
other, over a region whose first line is present in one and absent in
the other. The file already depends on `test-utils`.

## What a taker owes

Fold it into `source::sentinel_region` and adjust whatever the
included opener was carrying (a one-line offset in the reported first
line, if anything). `crates/topo/tests/` is TCOST's and TINT's; PR
2517 read this file and converted only its line-number computation,
which was byte-identical and safe, deliberately leaving the
behaviour-changing half to its owners.
