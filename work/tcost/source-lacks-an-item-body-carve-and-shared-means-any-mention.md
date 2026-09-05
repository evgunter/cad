---
id: source-lacks-an-item-body-carve-and-shared-means-any-mention
kind: issue
title: test_utils::source has no item-body carve (five hand-rolled copies) and the census's Shared check accepts any mention of the crate
status: open
opened: 2026-09-05
---


## What

Two findings from the style review of TOPO's `D261` (PR 1919, the
collapse of `topo`'s four readers onto `test_utils::source`), both on
this program's ground, filed by the TOPO orchestrator.

**1. The carve "find the item's head → `balanced_end` over the params →
`find('{')` → `balanced_end` over the body" is hand-rolled five times
and the shared home has no operation for it.** Copies:
`crates/topo/src/review_d18.rs:388-398` (`code_body`, which D261's fix
pass moves onto a new shared op beside `balanced_end` — announced seam),
`crates/topo/tests/quad_lane_is_the_certified_lane.rs:89-109`,
`crates/editor-core/tests/gui1_pick_r2.rs:623-635`,
`crates/geom-brep/tests/pcurve_conic.rs:411-412`,
`crates/pncad-py/src/prose_census.rs:977-980`. Two of them use a
non-depth-aware `find('{')`. Once the shared op lands, the other four
are conversions of the S117 kind. Related, same home: a balanced carve
for a LITERAL needle — locate over `code_only`, read the same offsets
out of `code_and_literals` — which `work/topo/probe-message-carve` names
from the consumer side and which has no row here.

**2. `every_shared_entry_actually_reaches_the_shared_lexer`
(`crates/test-utils/tests/reader_census.rs:571-590`) accepts any code
mention of `test_utils::source`.** `crates/topo/src/source_walk.rs`
satisfies it through `crate_dir(`/`rust_sources(` alone, so a
`CodeOnly::of` reverted to a hand-rolled lexer would keep its `Shared`
line and the census would stay green — the silent direction the row's
own doc says it closes. The reviewer wrote a stricter row, green on
PR 1919's head, requiring a VIEW call:

```rust
/// **A `Shared` line means the file reads through a VIEW of the shared
/// lexer, not merely that it walks with the shared traversal.**
#[test]
fn every_shared_entry_reads_through_a_view_not_only_a_traversal() {
    const VIEWS: [&str; 5] = [
        "code_only(",
        "code_and_literals(",
        "comments_only(",
        "keeping(",
        "aggregation_violations(",
    ];
    let root = repo_root();
    let liars: Vec<&str> = LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, Shared))
        .filter(|e| {
            let text = std::fs::read_to_string(root.join(e.path))
                .unwrap_or_else(|err| panic!("reading {}: {err}", e.path));
            let code = test_utils::source::code_only(&text);
            !VIEWS.iter().any(|v| code.contains(v))
        })
        .map(|e| e.path)
        .collect();
    assert!(liars.is_empty(), "`Shared` entries with no view call: {liars:#?}");
}
```

A corollary the same review raised: `crates/topo/src/boolean/boxes.rs`
reads the shared lexer through `source_walk::CodeOnly` (a one-line
adapter after D261) yet stays `Unconverted`, because `Shared` would
fail the text check — a disposition chosen for what the check can see.
