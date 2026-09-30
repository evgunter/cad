---
id: sweep-per-step-differencing-helper-unhomed
kind: issue
title: turning_orientation's spine_chords is private to one suite while orient.rs routes stack readers to the end-to-end spelling
status: open
opened: 2026-09-15
priority: P4
cost: E
---

## Finding

- **Where**: `crates/sweep/tests/turning_orientation.rs` (`spine_chords`),
  `crates/sweep/tests/common/orient.rs`
- **Importance**: low
- **Confidence**: likely — nothing has copied it yet, so this is a prediction
- **Raised by**: a sibling review lane, relayed to the `S52` lane, 2026-09-15

`spine_chords` is a per-step differencing helper private to one suite, while
a doc in `orient.rs` routes a reader who wants a stack to `ring_centroid` —
the end-to-end spelling, which is not what a reader who wants per-step
differences needs. The next suite that wants one copies it.

**Deliberately not taken into `S52`'s home, and the reason is the routing
rule itself**: an item lives at the narrowest home all of its consumers can
reach, which for a one-consumer helper is the suite it is in. Moving it to a
shared home *before* there is a second consumer would make the home a place
things are put rather than a place shared things live, and `S52`'s own charter
question — does the shared helper still make each suite's intent readable at
its call site — cuts against a fixture with one reader.

What is actually wrong here is therefore the **doc in `orient.rs`**, which
sends a reader to a helper that will not answer their question. That is a
one-sentence fix on `BLEND`'s or `S391`'s ground, and it is the thing to do
first; homing `spine_chords` is what to do the day a second suite wants it.

## The admission rule this row runs into, which is itself unsettled

`crates/sweep/src/test_support.rs`'s header says a fixture earns a place
there **once a consumer outside the crate needs it or a second suite
inside it does**. A `tests/` file is a separate crate, so *any single
integration suite* satisfies the first clause on a literal reading —
which is why that module already carries around a dozen `pub` items with
one call site each. The routing rule quoted beside it says the opposite:
an item lives at the **narrowest** home all of its consumers can reach,
which for one consumer is that consumer's own file.

Two rules, one question, neither stated at the other's site. This row's
conclusion does not depend on which wins — `spine_chords` has one
consumer under either — but whoever settles it should settle it in the
module header rather than per-fixture, because the module's current
population is evidence that the looser reading is the one in force.

Related, on this slate: `work/tint/sweep-boolean-suite-brick-and-prism-copies.md`.
