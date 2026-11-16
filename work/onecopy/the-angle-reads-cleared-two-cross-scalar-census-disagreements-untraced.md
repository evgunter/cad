---
id: the-angle-reads-cleared-two-cross-scalar-census-disagreements-untraced
kind: issue
title: PR 6482's angle reads cleared the at-rest census's f64/Interval disagreement on die_composed_tour and reshaped_rod at 1e-12, undisclosed; which read did it is untraced
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-10
---


`crates/editor-core/tests/m10_di_dual_corpus.rs` `at_rest_census_verdicts_match_f64_at_interval` asks that the at-rest census be blinded on the same corpus documents at f64 and at Interval. At ε 1e-12 it stopped on `die_composed_tour` before PR 6482 (`onecopy/angle-reads`, merge `a6f0b5e99c`), and its pin was written for `reshaped_rod`, the document it stopped on when the pin was taken. From `a6f0b5e99c` on it passes whole: both documents read the same at both scalars. PR 6482's description discloses neither. The row is unpinned by PR 6691.

Both disagreements were an Interval census refusing where f64 did not: per `work/census/an-interval-census-of-the-reshaped-rod-finds-what-f64-does-not-at-1e-12.md`, two `ContactLane(NotCertifiable)` findings from `curve_pair::meet` refusing `MeetFault::Unsettled`. 6482 replaced the angle reads (one anchored `Azimuth::near` over `Period::near`, `Conic::azimuth` for `angle_of`), so the likely mover is the meet's angle read settling at Interval where it did not. That is a guess, not a measurement.

What is owed: run the row at `a6f0b5e99c^1` and `a6f0b5e99c` with the census's findings printed per document, name the read that moved, and say whether the Interval refusal was a defect 6482 fixed (and the row now pins it) or a refusal the new read no longer reaches. Or close this with the reason it is not worth the trace: the row is green and both scalars agree on the f64 reading.

## Closed (2026-11-10, traced)

**Measured.** The setup: a probe printed every corpus document's could-not-look findings at f64 and at Interval with `CAD_TOLERANCE_EPS=1e-12`, and a print sat at `curve_pair::pose`'s `Polish::closest`. Neither was committed. It ran at `e9eaae651d` (`a6f0b5e99c^1`) and at `a6f0b5e99c`. Exactly two documents differ between them, and only at Interval:

- `die_composed_tour`: 8 `ContactLane(NotCertifiable)` findings, from 4 circle × circle meets.
- `reshaped_rod`: 2 findings, from 1 circle × line meet.

All five meets refuse `Unsettled(Meter)` at one meter, the polish's `decide(row::POLISH, Margin::of(moved))`. The basin and travel meters pass. `moved`, the polish's last step, is enclosed as `[0, w]`. That meter reads Zero when `w` lies under the band's edge, and the corpus puts that edge between `w = 9.38e-13` (Zero) and `1.0055e-12` (escalated). Before, the five meets read `w` = 1.005e-12, 1.006e-12, 1.039e-12 ×2 and 1.317e-12. At `a6f0b5e99c` the same meets read 0.887e-12 at most, and the rod's reads 0.731e-12. Each still finds its one meeting, so the census is clear at both scalars.

**The read that moved** is mostly the polish's start seed on the second carrier: `Curve3::foot`'s circle arm (`geom/src/curves/on_carrier.rs`). Before, it was `angle_near(x, y, near)`, which rotates the in-plane point by the anchor and takes the `atan2`. Now it is `Conic::azimuth(c, q).near(near)`, the point's own angle renamed by whole turns.
- That seed's enclosure narrowed from 1.94e-12 to 0.77e-12 on the worst die pair, and from 1.88e-12 to 0.47e-12 on the rod.
- The start's crossing angle on the first carrier barely moved (2.51e-12 to 2.43e-12).

Putting the old seed back into `a6f0b5e99c` alone takes the worst die pair to `w = 0.971e-12`. That is still just under the edge, so the seed is not the whole move. The rest is upstream: the corpus carriers' enclosures differ between the two commits (circle a's axis, for one), so the polish's inputs moved too. Which other 6482 read built them differently was not traced; it is not needed to answer the row.

**Fixed or masked: neither, as a defect.** The Interval refusal was the polish meter reading at the band's edge. At ε = 1e-12, `w` scales with the input enclosures, which are themselves ~1e-12 wide. The Interval read did not claim what its enclosures could not show, and f64 was right. 6482 narrowed a read, so the same refusal no longer fires. Nothing was masked: the new seed reads the same angle with a narrower enclosure.

**What the row pins.** `at_rest_census_verdicts_match_f64_at_interval` at 1e-12 holds this with about 10% headroom: the worst `w` is 0.887e-12 against an edge near 1e-12. A future red of that row at 1e-12 on these two documents is most likely this meter again. Read `w` at `Polish::closest` first.
