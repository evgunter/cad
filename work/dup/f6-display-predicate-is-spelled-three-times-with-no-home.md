---
id: f6-display-predicate-is-spelled-three-times-with-no-home
kind: issue
title: F6's Display predicate is now spelled three times in three crates with no shared home
status: closed
opened: 2026-09-06
refs: [2053]
closed: 2026-09-19
pr: 2880
branch: dup/f6-one-display-predicate
---


Found by the style review of PR 2053 (`view/refusal-all`), which added
the third spelling.

## The three copies

    crates/editor-core/tests/display_contract.rs:42
        !shown.contains('{') && !shown.contains("node:") && !shown.contains("name:")
    crates/viewer/tests/panel_edits.rs:538
        !rendered.contains('{') && !rendered.contains("node:") && !rendered.contains("name:")
    crates/editor-core/tests/m4_pr4_hit.rs:358
        !shown.contains('{') && !shown.contains("node:")

The first is `assert_f6`, the ratified `Display` contract's own helper.
The third is a partial copy inside editor-core itself. The second is
new, and its doc comment announces the duplication in prose — *"The
shape asserted below is F6's — editor-core's ratified `Display`
contract (`crates/editor-core/tests/display_contract.rs`)"* — which is
the comment that exists to reconcile two spellings of one rule.

## What drifts

`assert_f6` takes a `dumps` roster as well as the punctuation
predicate, and asserts each named variant identifier absent. The viewer
copy approximates that half with `!rendered.contains(arm)` — one
identifier per arm rather than the enum's roster — so a rendering that
leaks a SIBLING arm's identifier passes there and fails under
`assert_f6`. A fourth clause added to F6 reaches one of the three
copies.

`assert_f6` lives in a `tests/` file, so it is not importable across
crates as it stands; giving the predicate a home means a shared
test-support item (`crates/test-utils`, which viewer already depends
on) rather than a further copy. That is the decision, and it is not
this finding's to make.

## The home now exists (2026-09-15, S-TINT's TINT-1)

`crates/test-utils/src/f6.rs` — `test_utils::f6::assert_f6(err, wants,
dumps, fields)`, plus `variant_identifier`, which reads a value's
variant name off its own derived `Debug`. TINT-1 needed the predicate
for its own work and `crates/test-utils` was inside its fence, so it
made the decision this row says is not the finding's to make, in the
place this row names.

**Two of the three copies are gone.** `display_contract.rs`'s
`assert_f6` is now a three-line wrapper that supplies the binary's field
roster and delegates; `m4_pr4_hit.rs`'s partial copy is deleted and that
row calls the same wrapper. **The divergence this row predicted had
already happened**: `m4_pr4_hit.rs` banned `"node:"` while
`display_contract.rs` banned `"node:"` and `"name:"`, so one of the two
spellings of one rule was a clause short. It is now one spelling.

**What remains, and it is this row's:** `crates/viewer/tests/panel_edits.rs`
still carries the third spelling, and still approximates `dumps` as one
identifier per arm rather than the enum's roster — the half this row
already describes, and the half that actually differs in what it
catches. `viewer` already dev-depends on `test-utils`, so the
conversion is a call-site change. Two more copies of the same predicate
live outside this row's scope, inlined in
`crates/topo/tests/display_contract.rs` and `crates/mesh/tests/errors.rs`,
each with its own field roster; they are carried on
`work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too`.

**One thing the new home did NOT do**: derive `fields` from the
payload's `Debug`. It was tried and false-positives on a door whose
prose PREFIX is a field name (`MeshPickError::PositionOutOfRange`
renders "pick index: triangle 5 …" and has an `index` field), so the
roster is still the caller's. The reasoning is at the module doc rather
than only here.

## Closed 2026-09-19 — the fold landed and the strengthening is a null result

`crates/viewer/tests/panel_edits.rs`'s
`refusals_render_as_sentences` now calls
`test_utils::f6::assert_f6` with `REFUSAL.identifiers()` — a
`test_utils::f6_variants!` census over all 23 `Refusal` arms — and a
`REFUSAL_FIELDS` roster carrying `"node:"`, `"name:"` and the
quotation mark this row's extra clause bans. The three hand-spelled
asserts and the per-arm ban are gone.

**The strengthening found nothing.** All six sampled renderings pass
the whole-roster ban. `cargo test -p viewer --test all`: **626 passed,
0 failed, 1 ignored** at the merge base `077ea8bed` and the same after
the fold. No verdict moved.

**The delta is real anyway, and was proved by mutation, not asserted.**
A sibling identifier planted in `Refusal::NoSuchParam`'s `Display`
(`"no document parameter named {} — GestureInFlight, declare it
first"`) is **green under the old per-arm form and red under the new
roster form**, failing at `test_utils/src/f6.rs`'s dump clause. A
second mutation — a twenty-fourth `Refusal` arm — reds the
`f6_variants!` block with `E0004` at `panel_edits.rs`, so the ban list
cannot fall behind the enum without the file ceasing to compile.

## The row's census was stale in both directions

- **`crates/topo/tests/display_contract.rs` and
  `crates/mesh/tests/errors.rs` are already folded**, by S-TINT's
  `sibling-display-contract-suites-hand-mirror-their-enums-too`
  (closed 2026-09-15, PR #2694). Both now call
  `test_utils::f6::assert_f6_every_variant` with an `f6_variants!`
  census. The paragraph above naming them as live copies is wrong as
  of that PR.
- **Two members this row never named are live**, and both are filed:
  - `crates/viewer/tests/error_display.rs`'s `debug_shaped`/`prose` —
    `" { "` plus **one** identifier per arm, 29 call sites, no field
    ban and no dump-equality clause. It is the same approximation this
    row is about, in the same crate, one file over, and no grep for
    `contains('{')`, `assert_f6`, `dumps` or `guts` reaches it.
    → `work/tint/viewer-error-display-prose-bans-one-identifier-per-arm`
  - `crates/quantity/src/tests.rs`'s
    `fmt_quantity_error_display_names_its_content_not_its_struct` — a
    **verbatim** copy of `assert_f6`'s body, panic wording included,
    inline mid-file in a `src/` unit-test module.
    → `work/fix/quantity-fmt-error-display-row-is-a-verbatim-copy-of-assert-f6`

### Instruments, and what each could not see

Run over **every tracked file with no path argument**, at merge base
`077ea8bed`.

1. **The punctuation atom, every spelling** —
   `(contains|find|matches|starts_with|ends_with|split|any)\(['"]\s*\{`
   plus `["{"`, `"{"`, `" { "`. This is the denominator: `{` is the one
   clause `assert_f6` bans unconditionally, so every full copy of F6
   contains it. ~140 hits, classified backwards by receiver; the
   executable Display-vs-Debug checks among them are the list below.
   **Blind spot:** a copy that bans only identifiers and never the
   brace. Not closable by construction — an identifier ban is any
   CamelCase string literal — so it is stated rather than closed.
2. **Name-shaped** — `assert_f6|variant_identifier|f6_variants|VariantCensus`.
   Found every site already on the home, and is precisely why it found
   no copy: it matches the fold, never the copy. Structurally blind to
   a new name, and the reason `topo`/`mesh` read as live in the text
   above.
3. **Prose** — `debug dump|struct dump|reads as a sentence|reads_as_prose|Debug guts`
   over `*.rs`. This is the instrument that found `quantity`'s copy
   (its doc comment announces *"never as the `Debug` struct dump"*) and
   `error_display.rs`'s module doc. **Blind spot:** a copy whose prose
   says nothing about `Debug`.
4. **The dump-equality clause** — `assert_ne!\(\w+, format!\("\{`.
   Four hits, three of them already on the home or the row's own text.
   **Blind spot:** a copy that omits the clause — which is most of
   them, so it is a confirmatory instrument, not a census one.

### Dispositioned, not converted — and why

| site | what it spells | disposition |
| --- | --- | --- |
| `viewer/tests/panel_edits.rs` | all four clauses + quote | **this unit** |
| `viewer/tests/error_display.rs` | `" { "` + one identifier, 29 sites | filed, S-TINT |
| `quantity/src/tests.rs` | all four, verbatim | filed, S-FIX |
| `editor-core/tests/lib_doors_node_result.rs` (3), `asm_r2b_assembly.rs` (2) | `for guts in ["{", …]` | already `work/tint/dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead` |
| `editor-core/src/names/emit.rs` (4), `pncad-py/src/tests.rs`, `sweep/src/blend/mod.rs`, `topo/src/validate.rs` (2) | `" { "` only, no identifier | already `work/census/the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates` — a neighbouring class (`reads_as_prose`'s needle), not this one |
| `sweep/tests/m5_pr6_pcurves.rs`, `topo/tests/m3_pr3_split.rs` | brace only, one line each | not-this-unit: a one-clause spot check, not a copy of the predicate |
| `viewer/tests/frame_policy.rs` | one identifier, no brace | not-this-unit: the blind spot instrument 1 declares, and at N=1 |
| `crates/pncad-py/tests/*.py` (6 sites) | `assertNotIn("{" / " { ")`, one with an identifier roster | not-this-unit: python-side, and `test_utils` is a Rust crate. Unfiled — no Rust-shaped home exists for it and `reads_as_prose` is the python side's own answer |

**The scope sentence:** this census is over the *F6 predicate* — a
check that a rendering carries neither `Debug` punctuation nor a
variant identifier. It is **not** over `reads_as_prose`'s needle, which
is a different predicate with its own row, and it is not over
one-clause spot checks.
