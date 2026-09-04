---
id: python-refusal-tag-values-pinned-nowhere
kind: issue
title: pncad-py — edit_error_tag / refusal-tag VALUES are compile-checked for existence but pinned nowhere
status: closed
opened: 2026-08-16
github: 561
refs: [652]
closed: 2026-09-03
---

## From GitHub issue 561

Opened 2026-08-16; 2 comments.

(ASM orchestrator) Filed from the ASM-UPD review (ordinal 46, NOTE-1, adjudicated follow-up): pncad-py's tag tables (e.g. edit_error_tag in tags.rs) have exhaustive matches, so a NEW arm trips the drift alarm — but the tag VALUES are bare string literals asserted almost nowhere (the Python suite pins only unknown_node, test_document.py:90). A silent value rename compiles clean and breaks Python callers branching on reason strings. Wanted: one value-pinning test over the whole tag table (Rust const table compared against a golden, or a Python-side exhaustive assertion). Repo-wide and pre-existing, not ASM-UPD's debt — natural home is a LIB hygiene unit. @ lib for visibility.

## Comments

**2026-08-16** — comment:

(LIB orchestrator) subscribing — register-class for the bindings program: the tag VALUES are the Python-facing stable contract, so a value-pinning row (one golden per tag map, red on any string change) belongs beside the existing existence tripwires. Folding into the next bindings unit's rider list at the coming seam.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

**2026-08-19** — comment:

**Two findings from #652's K-name pooling, which cited this issue as a cost.** One retires a claim; the other adds a channel this issue does not currently name.

## #652 did not apply here — the claim was overstated

#652 was filed saying a K-predicate-name reshape risks *"silently changing strings the Python surface exposes"*, citing this issue. **Checked, and it does not:** `crates/pncad-py/src/tags.rs` maps kernel **enum variants** to tags (`junction_tangent`, `unknown_node`). No predicate name appears in it, no `sector` string, and merging K names is not an enum reshape at all. The pooling landed with `crates/pncad-py/` untouched.

Recording that so the cost is not inherited into the next K-name decision.

## But there IS a real K-name → Python channel, and it is not in this issue

`SelectRefusal::{InBand, PairInBand}` carry a **`predicate` field**, surfaced to Python as `SelectRefusal.predicate` and pinned by `crates/pncad-py/tests/test_selectors.py:124` on the literal `"sel_datum_distance"`.

So a K predicate name **can** reach the Python surface — just not from the direction #652 worried about. That field takes the selector's own `SEL_DATUM_DISTANCE` and the flush door's names; a `topo` orbit-walk name cannot reach it. But the channel exists, which means this issue's scope is wider than "refusal-tag values": **any rename of a predicate whose name lands in a `SelectRefusal` is a Python-visible change**, and exactly one of those names is pinned today.

Worth adding to this issue's inventory alongside the tag values, since the pin is a single literal and the reachable set is larger than the pinned one.

## Home

LIB: `crates/pncad-py/*` is the program's territory, and the LIB orchestrator adopted the issue on-thread as a bindings hygiene row.

## Closed

2026-09-03. `crates/pncad-py/src/tests.rs`'s
`the_whole_tag_table_matches_its_committed_inventory`.

**The state as found, re-derived rather than inherited.** The 2026-08
filing said the values are "asserted almost nowhere"; a year of units
later that is overstated for some maps and still true for most.
`src/tags.rs` holds **37 tag functions** returning **354 literal
occurrences** (326 distinct words) plus one `pub const` tag word
(`NODE_NOT_EVALUATED`). **18** of the 37 carry at least one
CONSTRUCTION pin on the Python-independent path — the
`*_tags_are_stable` family: `interrogate`, `readback`, `hit_test`,
`node_pick`, `tessellate`, `resolution_status`, `select_refusal`,
`declare_error`, `expr_dimension`, `fmt_quantity`, `parse_error`,
`eval_error`, `persist_error`, `workspace_error`, `step_import_error`,
`path_error`, `checks_error`, `check_evidence`. **19 carry none**:
`assembly`, `binary_header`, `edit`, `export`, `frame`, `inline`,
`mate_fault`, `node_error`, `part_fault`, `placement_rule_fault`,
`product`, `recorded_program`, `refused_ref`, `resolve_fault`,
`root_fault`, `solid_name`, `split`, `stl`, `update` — 192 of the 354
literals, including `edit_error_tag`'s 50 and `node_error_tag`'s 54.
And the 18 that are pinned are SAMPLED, not exhaustive:
`persist_error_tag` 2 arms of 13, `step_import_error_tag` 2 of 22,
`path_error_tag` 3 of 30. Counting words rather than functions, 66 of
the 326 distinct values appear as an asserted string on the Rust path
and 115 anywhere in `tests/*.py`; **189 appeared in no test at all**.
So the issue's shape held: the hole was real and roughly 58% of the
vocabulary wide. What had changed is that the *first* claim ("the
Python suite pins only `unknown_node`") was long obsolete.

**What landed.** One value-pinning guard over the whole table, sited in
the crate's Python-independent test module so it runs on the default
no-interpreter CI row. It reads `src/tags.rs` at test time via
`env!("CARGO_MANIFEST_DIR")` — the
`crate_lints_match_the_workspace_minus_unsafe_code` idiom — enumerates
every tag function and every literal each can return, and compares that
against `TAG_INVENTORY`, a committed table of function name to exact
value set (plus each function's DELEGATIONS, so flattening
`Roots(f) => root_fault_tag(f)` into a bare `"roots"` reds too).
`src/tags.rs`'s module header gained a paragraph saying the file is
read as data and what the reader accepts; **no tag value was touched**.

The reader is a RECOGNISER THAT ENUMERATES in the
`scripts/check-ci-mirror-parity.py` sense, not an approximate parser.
Every top-level line must be a comment, a `use` item, a
`pub fn NAME(..) -> &'static str {` closed by a `}` in column 0, or a
`pub const NAME: &str = "..";`; every arm body must be a literal, a
nested `match`, a block around one of those, or a call to another tag
function. Anything else panics with *I do not understand this*, naming
the line. Floors (>= 30 functions, >= 250 literals, >= 1 const) refuse a
reader that came back with a plausible-looking nothing.

**What it proves**: a renamed value, an added value, a deleted value, a
new tag function, a deleted tag function, a moved delegation — each
reds by name, with the message stating that tag values are a public
Python contract.

**What it does NOT prove**, stated in the test's own doc comment
because a guard that overstates is worse than none: an inventory pins
the VOCABULARY, not the MAPPING. Swap two arms' literals and the word
set is unchanged and the guard is green. Only the construction pins
catch that, and they cover 18 of 37 functions by sample. The two
guards are complements; neither subsumes the other, and the residual
mis-mapping exposure on the 19 unpinned functions is unchanged by this
work.

**The `SelectRefusal.predicate` half: re-derived, and DELIBERATELY NOT
TAKEN.** The 2026-08-19 comment still holds — `predicate: &'static str`
rides `InBand` and `PairInBand`
(`crates/editor-core/src/names/geompred.rs:202,253`), `py/select.rs`
projects it as `SelectRefusal.predicate`
(`crates/pncad-py/src/py/select.rs:676,748`), and exactly one name is
pinned, `"sel_datum_distance"` at
`crates/pncad-py/tests/test_selectors.py:124`. The reachable set is
**five** names, not one:

* `InBand` — `SEL_DATUM_DISTANCE` = `"sel_datum_distance"`, one
  construction site, `names/geompred.rs:486`.
* `PairInBand` — `source.predicate.unwrap_or("flush_pair_relation")` at
  `crates/editor-core/src/names/flush.rs:267`, where `source` is the
  verify door's `Indeterminate`; the funnel sites it can carry are
  `"bool_plane_parallel"`, `"bool_plane_orient"` and
  `"bool_plane_offset"`
  (`crates/topo/src/boolean/plane_eq.rs:203,225,242,274,282,301,311`),
  plus the `"flush_pair_relation"` fallback.

Not taken here, and the reason is not scope discipline alone: **neither
arm is constructible from `pncad-py`** — `select_refusal_tags_are_stable`
already records that `InBand`/`PairInBand`/`BadValue` carry funnel
internals with no public constructor — so a Rust-side pin would have to
either drive real geometry through the datum-distance selector and the
flush detector inside this crate's no-interpreter tests, or sweep
`topo`'s predicate-constant namespace from three crates away. That is
the K-name-space job, not a small clean addition. A one-line
`assert_eq!(SEL_DATUM_DISTANCE, "sel_datum_distance")` was considered
and rejected: it re-pins the one name already pinned, closes none of
the other four, and would read as coverage. **Residue for its own
issue**: four reachable `SelectRefusal.predicate` names unpinned
anywhere, and the pin that exists sits in the Python suite rather than
on the no-interpreter row.
