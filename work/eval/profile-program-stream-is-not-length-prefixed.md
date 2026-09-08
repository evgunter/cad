---
id: profile-program-stream-is-not-length-prefixed
kind: issue
title: The profile node's program payload is not length-prefixed, so a resolved loop boundary reads the verb vocabulary beside the profile-payload words — where the lane opener and the Cusp verb are both 41
status: closed
opened: 2026-09-08
branch: eval/9-nominal-in-the-key
pr: 2190
closed: 2026-09-08
---

(EVAL-2 implementer, found while declaring the content key's tag
vocabularies; in EVAL's fence, so filed here per
`docs/prompts/implementer-discipline.md` §6.)

**The finding.** In `content_key`'s `Node::Profile` arm
(`crates/editor-core/src/eval/mod.rs`, the `tag::program` group), the
resolved program feeds as `LOOP_START` then the loop's steps, per
loop, with neither the loop count nor the step count written. So at
the word after any step the reader admits, at ONE grammar position:
the next verb tag (`verb_tag`, 10–42), `LOOP_START` (1), `LANE` (41),
`CARRIER_RADIUS` (45), and the first word after the payload (the
upstream-key count, which for a profile is its plane — `1`). Two of
those coincide: `tag::program::LANE` and `verb_tag(Verb::Cusp)` are
both `41`, and `LOOP_START` and a profile's upstream count are both
`1`.

**Why no key moves today.** The stream is a hash of `u64` words, not
a parse, so a collision needs two whole streams to agree, and at this
position they cannot even begin to: a `Cusp` cannot END a loop — its
tip is `DirectedIncoming` (`crates/profile/src/path/program.rs`, the
`Cusp` row), no closing verb accepts that tip, and an open end refuses
at replay (`Transition { verb: None }`) — so the word after a `Cusp`
is always the next step's verb tag (≥ 10) and never `LOOP_START` (1),
`LANE` (41) or `CARRIER_RADIUS` (45). At the end of the stream the
word after the last step is the upstream-key count — `1` for a
profile, whose one input is its plane or frame — which no verb tag is
(the verb vocabulary starts at 10), so the end of the stream cannot
read as a `LoopStart` either; and the lane stream that `LANE` opens
continues with `LOOP_START` then `LANE_SCALAR` (42) and bit patterns,
where a resolved loop carries verb and target tags. So the two
vocabularies read at one position never produce the same next word.
The conclusion stands; it rests on the transition table's closing
rule and on the profile's upstream count, not on an alignment
argument.

**What a fix needs.** Length-prefix the loop list and each loop's step
list (the shape every other list in the key already has: selection,
members, crossings, upstream keys), which makes the boundary position
a single vocabulary. That changes every profile-bearing key, so it is
a key format bump (`tag::format::VERSION`) — free on disk (keys are
process-internal) and a whole-memo invalidation once. EVAL-2 moved no
number and no byte by its own acceptance, so the fix is not in it.

## Closed

EVAL-9 (PR 2190), riding the format bump the nominal feed needed
anyway. Every list in the profile payload is length-prefixed — the
loop count then, per loop, `LOOP_START`, the step count and the steps,
in the resolved stream and the lane stream alike — and the resolved
stream's presence is `tag::presence`'s word, since a loop count of `0`
and the old `tag::program::NONE` would both be a single `0` at one
position. The `Cusp` tip-state argument is kept at `tag::program`'s
doc as the second, independent reason the boundary is
single-vocabulary, beside the sentence that the payload's END is still
read against the upstream-key count. Measured: the count words alone
move 1245 corpus rows (249 per lane pass) — every profile node and its
downstream cone.
