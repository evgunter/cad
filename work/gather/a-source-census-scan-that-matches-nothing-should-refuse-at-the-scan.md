---
id: a-source-census-scan-that-matches-nothing-should-refuse-at-the-scan
kind: issue
title: A source census whose scan matches nothing in a file that must contain its markers should refuse at the scan, not return an honest zero for an assertion to maybe catch
status: closed
opened: 2026-09-14
refs: [2480, 2501, 2517, 2555]
priority: P3
cost: E
closed: 2026-09-24
pr: 3141
---

## Finding

From Ev's reading of `[ev]` PR 2555 (declined), which corrected the
framing that PR was built on and produced a better result than the
proposal did.

The orchestrator's rule said *"a scan that died reads as a pass"*. Ev:

> *this sounds like it could be a bug? if it "died" in the sense of
> encountering an error condition, it shouldn't be returning an empty
> set*

Correct, and the sentence hid two different cases. A scan that hits an
**error** and returns empty is a bug in the scan; a non-emptiness
assertion downstream treats a symptom. What WIRE's censuses actually met
is the other case — the scan ran correctly and **honestly found
nothing**, because its needle had stopped matching: a sentinel renamed, a
file moved, a type path re-spelled.

**But the question points past the rule it was aimed at.** A scan over a
file that *must* contain its markers, finding none, is not an honest zero
either: its premise is broken. The repair is to **refuse at the scan**,
which is this project's own fail-loud rule applied one level earlier than
the rule was applying it — and it is a code change rather than a
paragraph, which is the half that makes it checkable.

## Where it lands

The source censuses WIRE built or touched, each of which currently
returns an empty or short set where a broken premise should refuse:

- `crates/editor-core/tests/wire_operand_door.rs` — the operand
  vocabulary census (door region, macro arms, phrase consts).
- `crates/editor-core/tests/wire_entity_door.rs` — the entity-kind
  carrier census.
- `crates/editor-core/tests/switch_program_vocabulary.rs` — the
  document-vocabulary censuses.

Each carries a non-emptiness assertion today, which is the downstream
guard this row proposes to replace at the source. **Where a scan can
legitimately match nothing, that is a different thing and must stay** —
the row is about scans whose file is required to contain their markers.

## Why this is better than the rule it replaces

A non-emptiness assertion says *"this set should not be empty"* at the
place that consumes the set, which is one hop from where the knowledge
lives. A refusing scan says *"this file does not contain the thing I was
built to read"* **at the read**, names the file and the needle, and
cannot be forgotten by the next census that reuses the helper — which is
the property the assertions do not have, since each new equality has to
remember to carry one.

Read beside `work/tcost/reader-ledger-is-keyed-on-the-file-not-the-census.md`:
the shared helpers in `crates/test-utils/src/source.rs` are where a
refusing scan would live, and that is also where a second census inside
an already-listed file currently arrives invisibly.

## What a taker owes

A decision about `sentinel_region` and its siblings in
`test_utils::source`: refuse on a missing marker, or return a typed
absence the caller must handle. Either closes it; a `bool` or an empty
slice does not. And a sweep of the three censuses above to say, per
scan, whether its file is *required* to contain the markers — because
the ones that may legitimately match nothing are exactly the ones this
change must not break.

## Closed

PR 3141. **The decision: refuse.** `sentinel_region` already panicked on
a missing or inverted sentinel. The sibling it lacked was a plain needle
scan. `test_utils::source::required_matches(text, what, needle)` is
`match_indices` for a text that is required to carry its needle. When
the needle matches nothing it refuses, and the refusal names the text and
the needle. Its test is
`source::tests::a_required_needle_that_matches_nothing_refuses_at_the_scan`.

Per-census sweep (each scan: is its file required to contain the markers?):

- `wire_operand_door.rs`:
  - The `family::` / `phrase::` vocabulary scan: **required**. It now
    uses `required_matches` per prefix, which is stricter than the old
    union non-emptiness assert. `eval/wire.rs` carries both prefixes.
  - `doors()` over the `OPERAND-DOOR` region: **required**. It refuses
    inside the scan, and the call site keeps its `>= 2` floor, with the
    message reworded.
  - The `family_word!` arm scan: **required**. It refuses right after
    the read and before the equality it feeds. The trailing
    `!heads.is_empty()` assert is gone.
  - The behavioural `phrases`/`families` floor: not a source scan, so it
    stays.
- `wire_entity_door.rs`:
  - `variants()`: **required**. It refuses when there is no variant or
    no `found:` field, and the two call-site asserts are gone.
  - `!built.is_empty()`: deleted. Once `declared` is non-empty, the
    `declared == built` equality already reds on an empty walk.
- `switch_program_vocabulary.rs`: **the premise does not hold.** It has
  no source scan (no `include_str!`). Its non-emptiness asserts guard a
  corpus built in code, and they stay.

The legitimate-zero case is still served by `str::match_indices` itself,
and the helper's docs say so.
