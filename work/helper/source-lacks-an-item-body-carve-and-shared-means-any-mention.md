---
id: source-lacks-an-item-body-carve-and-shared-means-any-mention
kind: issue
title: test_utils::source has no item-body carve (five hand-rolled copies) and the census's Shared check accepts any mention of the crate
status: open
opened: 2026-09-05
priority: P4
cost: E
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

## Widened from D50's review (2026-09-06)

The shared home owns every operation over a blanked view EXCEPT
"find the next `fn` item": `topo/src/source_walk.rs`'s `fns` /
`ident_after` / `param_list_start` scan stays in `topo` (D50 widened it
from `pub fn` to every named `fn` rather than moving it), and the tree
carries at least three weaker `pub fn` scans —
`crates/editor-core/tests/gui1_pick_r2.rs:617`,
`crates/pncad/tests/all.rs:3747`, `crates/pncad-py/src/tests.rs:2422`.
Two smaller siblings, same home: the identifier-boundary predicate has
four spellings (`live.rs identish`, `source_walk.rs is_token`,
`source.rs token_start`, `pncad/tests/all.rs token_starts_at`), and
`source.rs:113` promises callers can report a line number and offers
no `line_of` — `live.rs`, `source_walk.rs`'s `gave_up` and
`gui1_pick_r2.rs` each re-derive it.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane C)

**VERDICT: PARTIAL** — finding 1's shared operation LANDED and one of
the five copies was converted; four copies are still hand-rolled.
Finding 2 is untouched and its corollary has changed shape. Two of the
three "widened from D50" siblings are live and one is stale-fixed.

### Finding 1 — the item-body carve: the home now has the operation

**Stale-fixed half.** `test_utils::source` publishes `ItemBody`
(`Body`/`Declaration`/`Unterminated`) and `pub fn item_body(blanked,
from)`, whose doc states the whole content of the operation: *"**The
terminator is the first `{` or `;` OUTSIDE every round and square
bracket**, which is the whole content of the operation and the part a
call-site copy gets wrong."* Ten call sites now use it — `topo`'s
`source_walk.rs`, `review_d18.rs` and `live.rs`, `pncad/tests/all.rs`,
`profile/tests/generic_replay.rs`, `sweep/tests/review_fillet_t_r1_probes.rs`
and `editor-core/tests/wire_operand_door.rs` (twice).

**Live half — the copy list, re-derived by name:**

| copy | today |
| --- | --- |
| `crates/topo/src/review_d18.rs` `code_body` | **converted** — calls `item_body`, matches `ItemBody::Body` |
| `crates/topo/tests/quad_lane_is_the_certified_lane.rs` (`fn quad_cut_face` scan) | **live** — `balanced_end` over the arg list, then `find(';')`/`find('{')` with no BRACKET depth, so an array-typed return still mis-reads |
| `crates/editor-core/tests/gui1_pick_r2.rs` (the `pub struct`/`pub enum`/`pub fn` surface walk) | **live** — `code[from..].find('{')` / `.find(';')`, non-depth-aware |
| `crates/geom-brep/tests/pcurve_conic.rs` `route_table_has_no_wildcard_arm` | **live** — `src[start..].find('{')`, non-depth-aware |
| `crates/pncad-py/src/prose_census.rs` (cited at `:977-980`, rotted) | **live** — the same `find('{')` + `balanced_end` pair at the `Display for` scan and at two further sites |

**New floor: four hand-rolled copies, not five.** Two of the four
(`gui1_pick_r2.rs` and `pcurve_conic.rs`) are the "non-depth-aware
`find('{')`" the body names; `quad_lane_is_the_certified_lane.rs` and
`prose_census.rs` are the same shape with a partial guard. All four are
now S117-kind conversions onto an operation that exists.

The related literal-needle balanced carve ("locate over `code_only`,
read the same offsets out of `code_and_literals`") is still not a shared
op; `source::blanked(view, searched, text)` is the nearest thing and is
a view selector, not a carve.

### Finding 2 — `Shared` still means any mention

**REPRODUCES verbatim.** `every_shared_entry_actually_reaches_the_shared_lexer`
is unchanged: one `code_only(&text).contains("test_utils::source")`.
The stricter row the reviewer wrote —
`every_shared_entry_reads_through_a_view_not_only_a_traversal` — is NOT
in `crates/test-utils/tests/reader_census.rs`; `grep -rn
"reads_through_a_view"` over the tree returns nothing. `LEDGER` now
holds 63 entries, **57 of them `Shared`**.

**The corollary has moved, and not in the direction the row expected.**
`crates/topo/src/boolean/boxes.rs` is still `Unconverted`, but its
reason is now written out in the ledger rather than silent:
`Unconverted("Track Q — reaches the shared lexer only through
`source_walk::CodeOnly`, topo's handle on it; the direct call is Track
Q's to make")`. Meanwhile **`crates/topo/src/live.rs` is dispositioned
`Shared` and is in the same position** — its blanked text comes from
`CodeOnly::of(&text)` and it calls no view directly (only `balanced_end`
and `item_body`). So the tree now disposes two files that reach the
lexer the same way in two different ways, and the reviewer's proposed
row would go RED on `live.rs` if written today. That asymmetry has to be
ruled on before the stricter row can land, and it is new since filing.

### The D50 widening — re-derived

- **`line_of` is stale-fixed at the home and live at the copies.**
  `source.rs` now publishes `pub fn line(text, at) -> usize` (*"Shared
  because two censuses wrote it byte-identically before this existed"*),
  so the "promises callers can report a line number and offers no
  `line_of`" half is closed. Three sites call it
  (`topo/tests/shell_tolerance_chain.rs`, `geom-core/tests/flagged_census.rs`,
  and `source.rs` itself). **Six hand-rolled re-derivations remain** —
  the row named three and the floor is now double: `topo/src/live.rs`
  (`fn line_of`), `topo/src/source_walk.rs` (`fn gave_up`),
  `editor-core/tests/gui1_pick_r2.rs` (a `line_of` closure over
  `partition_point`), plus three the row never named —
  `pncad/tests/all.rs` (`fn line_of`),
  `sweep/tests/review_blend5_r5_probes.rs` (`fn line_of`) and
  `tools/tess-lint/tests/cut_line_pin.rs` (`fn line_of`).
- **The identifier-boundary predicate: four spellings, still four, but
  not the four listed.** Today they are `source.rs`'s private
  `token_start` and its public `boundary_before`, `source_walk.rs`'s
  `is_token`, `live.rs`'s inline `identish` closure inside `fn mentions`,
  and `pncad/tests/all.rs`'s `token_starts_at` — **five sites**, of which
  two are in the home. The row's `live.rs identish` is a closure, not a
  `fn`, so `grep "fn identish"` finds nothing; cite it as
  `live.rs`'s `mentions`.
- **The weaker `pub fn` scans: one, not three.** `grep -rn '"pub fn"'`
  over `crates/` and `tools/` returns exactly one hit today —
  `crates/editor-core/tests/gui1_pick_r2.rs`. The two others the row
  named (`crates/pncad/tests/all.rs`, `crates/pncad-py/src/tests.rs`) no
  longer carry a `"pub fn"` needle. `topo/src/source_walk.rs`'s `fns`
  scan is still topo's and still unmoved.
  **Blind spot of that sweep**: it matches only the literal `"pub fn"`,
  so a scan written as `starts_with("pub ")` then `contains("fn ")`, or
  one over a byte slice, is invisible to it.

### Recommendation (orchestrator's call)

Keep open, split-shaped: finding 1 is now four mechanical conversions
onto `item_body`; finding 2 is blocked on a `live.rs` vs `boxes.rs`
disposition ruling that did not exist when the row was filed.
