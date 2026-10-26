---
id: circle-side-reads-a-far-segment-ends-rim-uncharged
kind: issue
title: circle_side reads a segment end's rim against a circle uncharged, which a line end far from the origin rounds past the band
status: open
opened: 2026-10-21
priority: P3
cost: E
---


Found writing PR 5552 and disclosed in its body; filed by its FULL review
(MINOR 3).

**What.** `circle_side` (`crates/profile/src/seg.rs:989`) decides
`|q − c| − r` with no charge. The end `q` is stored, but the margin is
computed: the difference `q − c`, its norm and the subtraction of `r`
round at `|q|`, `|c|` and `r`. Its one reader is `carrier_side`
(`seg.rs:2264`), which `touches_from` and `end_in_stretch` read for a
segment's end against an arc's carrier.

**Where it binds.** Inside the arcs' reach (about 2.8 km at ε `1e-12`)
the review's oracle bounds the uncharged rim error by about `0.4·Kε`. A
line's end has no reach, though: an end `10⁵`–`10⁶` m out, read against a
circle inside the reach, rounds past the band. PR 5552's own sweep found
one at about `7·10⁵` m that read outside at `f64` and not in ℝ, which
is why its certificates read the ends through the charged
`end_clear_of_circle` (`circle_end_clear`) instead.

**Proposed.** Read the end's rim through `end_clear_of_circle`'s charge
wherever `carrier_side` meets an arc, or show that a wrong definite
reading there only moves a refusal. Measure first: no witness through
`validate` yet. A sibling of
`computed-points-read-uncharged-where-err-means-no-contact`, in the same
regime.

## Re-homed at PATHS's close (2026-10-21)

Filed on PATHS's slate on the day it closed (PR 5552's review) and moved to PROFILETAIL (`work/profiletail/`) with the merge that closes PATHS. PROFILEREST holds validation's other reads but was at budget. The item's id, content and history are unchanged.
