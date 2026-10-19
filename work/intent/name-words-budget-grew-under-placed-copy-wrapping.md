---
id: name-words-budget-grew-under-placed-copy-wrapping
kind: issue
title: The corpus's name words grew under D's placed-copy wrapping: the full max 196 → 366 and the CrossingUnverified admission 80 → 92
status: open
opened: 2026-10-19
priority: P2
cost: M
parent: a-placement-pins-by-written-values-without-a-mate
---

INTENT stage 3 D (`transform-retires-into-a-placement`) re-blessed `name_words_corpus::every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget` at grown numbers (the orchestrator's ruling: re-bless at the measured values, do not hide the growth).

What grew (`crates/editor-core/tests/name_words_corpus.rs`, `NAME_WORDS`, `OVER_BUDGET`, `SAID_DIGEST`):
- full names: p99 122 → 128, max 196 → 366, total 411230 → 444393 (p50 19 held);
- scoped faces: total 55615 → 59553 (p50 16 and max 57 held; p99 fell 43 → 41);
- `OVER_BUDGET`'s `NodeErrorKind::CrossingUnverified` at the 90th-percentile name: 80 → 92 words.

Why. Under D every construction is the root of its own space, so each operand after a combine's first is placed against the first operand's root (`mate::stand` / `place_against`). Its names come in wrapped `RoleSeg::Placed { of }` (and `InPart` under an instance), said "joined at Union … from Place …". The longest full name is in `nested_islands_106_depth2`: an edge of the final `Place`'s copy, a piece of a lateral edge between two crossings, whose two ends say the edge again, so every extra placement read on the way in is said three times over. The `CrossingUnverified` row (the die's pip band face, cut into the die) says one more "from Place …" in its name and one more "through read … at node …" in its claimed name.

Expected resolution. Retiring `RoleSeg::Placed` and `InPart` in D′ (PR 5315's picks) should bring the budget back; D′ re-tightens `NAME_WORDS` and `OVER_BUDGET` (both are ratchets: a number that shrinks lowers the pin, so the re-tightening is forced by the row itself).

