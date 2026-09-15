---
id: sweep-per-step-differencing-helper-unhomed
kind: issue
title: turning_orientation's spine_chords is private to one suite while orient.rs routes stack readers to the end-to-end spelling
status: open
opened: 2026-09-15
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
