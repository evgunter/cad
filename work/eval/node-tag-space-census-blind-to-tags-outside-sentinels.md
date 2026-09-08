---
id: node-tag-space-census-blind-to-tags-outside-sentinels
kind: issue
title: The content-key tag-space injectivity census reads only between the NODE-TAG-SPACE sentinels, so tags 41–45 written outside them are invisible to it — and node tag 5 already coexists with payload tag 5
status: closed
opened: 2026-09-05
refs: [1910, 1593]
branch: eval/2-tag-vocabularies
pr: 2153
closed: 2026-09-08
---


(SEAT orchestrator) Class finding from SEAT-7's dual review (PR 1910),
filed per the durable-home rule; unowned — the content-key machinery in
`crates/editor-core/src/eval/mod.rs` is the memo's, not a verb program's.

**The finding.** `node_tag_space_is_injective` (`eval/mod.rs`, the
census near the `NODE-TAG-SPACE` sentinels) reads tags only BETWEEN its
sentinels, and its own doc says a tag written outside them "does not
exist today" — yet tags 41, 42, 43, 44 and now 45 (SEAT-7's carrier-
radius feed; SEAT-6's v4 bump before it) are all written outside them,
each with a site comment of the form "the next free number in this
key's tag space … append-only". Nothing enforces "next free" or
"append-only"; a collision would be caught by nothing. The comment is
also loose: node tag `5` (Extrude) and payload tag `5` coexist today,
so this is demonstrably not one space, and the sentence claiming it is
should say which spaces exist.

**What a fix needs.** Either the census grows to cover every
`write_tag`/`write_u8`-shaped tag site in the key machinery (a grep-
census over the file with the sentinels retired), or the tag space is
declared once as a closed enum with `ALL` and the sites read from it —
the `ScalarParam::ALL` shape this repo uses for every other census.
Keys are process-internal and never persisted, so the fix is free of
wire consequences; the risk it closes is a silent key collision that
serves a wrong memo entry.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/eval/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). The content-key machinery in `eval/mod.rs` is EVAL's; an E unit.

## Closed

EVAL-2 (`docs/EVAL-2-SPEC.md`). The content key's tags are declared
by VOCABULARY — the set of alternatives read at one grammar position —
and every vocabulary is censused over its whole row set, so "a tag
written outside the sentinels" is a different vocabulary rather than a
gap. What landed in `crates/editor-core/src/eval/mod.rs`:

- `mod tag`: the structural words as named consts in ten groups
  (`format`, `naming`, `presence`, `program`, `step`, `target`,
  `winding`, `side`, `blend`, `measure_expr`), each with an `ALL` and
  a one-sentence doc naming its grammar position; the rule stated once
  at the module (within a vocabulary a number never changes meaning and
  a retired one is never reused; across vocabularies numbers are
  unrelated); `structural_tag_groups_are_injective` over
  `tag::GROUPS`, with the target vocabulary's retired `43` held dead.
- `seg_content_tag(SegTag) -> u8` censused by
  `seg_content_tags_are_injective` over the new `SegTag::ALL`
  (`names/select.rs`, projected from the enum's own declaration by a
  macro); `feed_role_seg` writes the word through `SegTag::of` and
  feeds payloads only; the `SEG-TAG-SPACE` source census is gone.
- `arc_mode_tag(ArcMode) -> u8` over `ArcMode::ALL`
  (`arc_mode_tags_are_injective`) — `D365`, claimed and closed with
  this item; `ArcSide`/`ArcSweep` are two-member groups in `mod tag`
  because their enums are `crates/profile`'s and carry no `ALL`.
- The node-kind match keeps its source-text census, renamed
  `NODE-KIND-VOCABULARY`, its doc saying it covers that match alone.
- Every "one space / next free / high-water mark" sentence names its
  vocabulary; no number moved (key dump at merge base and head: 1053
  rows, zero-line diff; the PR body carries it).

Residue disclosed and filed on this slate:
`profile-program-stream-is-not-length-prefixed` — the profile payload's
loop and step lists carry no counts, so one grammar position reads two
vocabularies (`tag::program::LANE` and the `Cusp` verb are both 41);
unreachable as a collision today, a format bump to fix, not this unit's.
