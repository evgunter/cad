---
id: the-angle-reads-cleared-two-cross-scalar-census-disagreements-untraced
kind: issue
title: PR 6482's angle reads cleared the at-rest census's f64/Interval disagreement on die_composed_tour and reshaped_rod at 1e-12, undisclosed; which read did it is untraced
status: open
opened: 2026-11-09
priority: P3
cost: E
---


`crates/editor-core/tests/m10_di_dual_corpus.rs` `at_rest_census_verdicts_match_f64_at_interval` asks that the at-rest census be blinded on the same corpus documents at f64 and at Interval. At ε 1e-12 it stopped on `die_composed_tour` before PR 6482 (`onecopy/angle-reads`, merge `a6f0b5e99c`), and its pin was written for `reshaped_rod`, the document it stopped on when the pin was taken. From `a6f0b5e99c` on it passes whole: both documents read the same at both scalars. PR 6482's description discloses neither. The row is unpinned by PR 6691.

Both disagreements were an Interval census refusing where f64 did not: per `work/census/an-interval-census-of-the-reshaped-rod-finds-what-f64-does-not-at-1e-12.md`, two `ContactLane(NotCertifiable)` findings from `curve_pair::meet` refusing `MeetFault::Unsettled`. 6482 replaced the angle reads (one anchored `Azimuth::near` over `Period::near`, `Conic::azimuth` for `angle_of`), so the likely mover is the meet's angle read settling at Interval where it did not. That is a guess, not a measurement.

What is owed: run the row at `a6f0b5e99c^1` and `a6f0b5e99c` with the census's findings printed per document, name the read that moved, and say whether the Interval refusal was a defect 6482 fixed (and the row now pins it) or a refusal the new read no longer reaches. Or close this with the reason it is not worth the trace: the row is green and both scalars agree on the f64 reading.
