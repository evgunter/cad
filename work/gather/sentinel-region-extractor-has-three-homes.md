---
id: sentinel-region-extractor-has-three-homes
kind: issue
title: Three hand-rolled sentinel-region extractors; test_utils::source::sentinel_region is now the home and topo's copy has not moved to it
status: closed
opened: 2026-09-12
priority: P1
cost: E
closed: 2026-09-24
pr: 3141
---



## Finding

Raised by the delta review of PR 2480, as a class with a sweep
obligation. Confidence `sure`, the reviewer's.

A guard whose subject is one REGION of a file brackets it with two
sentinel comments and reads between them. Three sites had written that
walk themselves:

- `crates/editor-core/src/eval/mod.rs`'s node-kind vocabulary census —
  a `split_once` pair.
- PR 2480's operand-door census — a private `fn region`.
- `crates/topo/tests/shell_tolerance_chain.rs`'s `fn region(&self)`,
  which is nearly line-for-line the second.

`test_utils::source` is the declared home for exactly this class, and
none of the three was in it.

## What PR 2480 did

Hoisted it: `test_utils::source::sentinel_region(text, what, begin,
end) -> Range<usize>` is the home now, and **two of the three callers
moved onto it** — the operand-door census (which is the new one) and
`eval/mod.rs`'s node-kind census, both WIRE's ground.

## What is left

`crates/topo/tests/shell_tolerance_chain.rs`'s `fn region(&self)` has
not moved. It is TOPO's, TCOST's and TINT's ground by
`scripts/work.py territory`, so PR 2480 announced it rather than
editing it. Its shape differs in one way worth carrying across rather
than dropping: it also answers the FILE LINE the region starts at, so a
hit is reported where a reader can open it. If that is wanted generally,
it belongs on the shared helper rather than in the one caller.

**The sweep this row does not close**: the reviewer would look at every
file in the tree carrying an `X BEGIN` / `X END` comment pair, not only
at the three sites named above. Nobody has taken that grep.

## A second instance of the same shape, in the same file

The floor-message idiom — *"the census found only N — the sentinels or
the scan have drifted from the thing they read"* — is written three
times in one file. Two of PR 2480's three census rows replaced their
count floors with set equalities at the delta review's direction, so
the idiom has fewer instances than it did; whether the survivors want a
helper is the same question one level down.

## Closed

PR 3141. The third caller now uses the shared walk.
`crates/topo/tests/shell_tolerance_chain.rs`'s `Stretch::region` calls
`test_utils::source::sentinel_region`, and gets its file line from
`test_utils::source::line(source, range.start)`. That is the same line
the old walk reported, because the range starts on the sentinel's own
line. Answering the line did not need to move onto the helper: `line` is
already the shared operation, and the caller composes it.

**Sweep: every `BEGIN` / `END` sentinel pair in the tree**
(`grep -rn BEGIN` over `*.rs`, `*.py`, `*.sh`, `*.yml`, `*.toml`):

- `OPERAND-DOOR` (`eval/wire.rs`): read by `wire_operand_door.rs` and
  `wire_entity_door.rs`, both through `sentinel_region`. Already on the
  helper.
- `OPERAND-VOCABULARY` and `NODE-KIND-VOCABULARY` (`eval/mod.rs`): read
  through `sentinel_region`. Already on the helper.
- `SHELL-TOLERANCE-CHAIN` (`topo/src/props.rs`,
  `geom-brep/src/offset_fit.rs`): read by `Stretch::region`. Moved in
  this PR.
- `R1-DOOR-ONLY-BEGIN` (`crates/mesh/tests/mesh7r1_probes.rs`): has no
  reader. It is already filed as
  `work/tint/sentinel-markers-with-no-reader-are-grep-only.md`, so it is
  not this unit's.
- The `BEGIN` hits in `scripts/gates/*.sh` are awk `BEGIN` blocks, not
  sentinels.

**What that grep could not match:** a region bracketed by markers spelled
some other way (`START`, `OPEN`, `>>>`). The second pass looked for the
walk rather than the marker: `find` / `split_once` / `split` on a `"//`
needle or on an upper-case marker literal, plus every `sentinel` mention
in `crates`, `demos`, `tools` and `benches`. It found no other
hand-rolled extractor. A region carved by a code needle (`find("fn …")`
plus `item_body`) is a different operation and was not counted.

**The floor-message idiom, one level down:** two survivors remain.
`eval/mod.rs`'s `tags.len() >= 20` and `wire_operand_door.rs`'s
`doors.len() >= 2` are count floors with different subjects. The empty
case of each is now refused at the scan (see
`a-source-census-scan-that-matches-nothing-should-refuse-at-the-scan`),
and the `doors` message was reworded to state its floor. Two floors that
share no subject do not need a helper.
