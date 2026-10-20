---
id: seg-candidates-skips-its-end-reads-when-the-joints-candidate-is-held
kind: issue
title: seg::candidates reads no segment end when every candidate is held, so an adjacent pair whose one candidate is its joint never reads its far ends
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by designer B on the cusp fork (#5354), filed by the unit that
implemented its ruling (`paths/joint-reads-its-turn`).

`seg::candidates` (`crates/profile/src/seg.rs`) calls `end_touches`
only when some candidate `missed` (a definite miss, or an in-band
reading `end_in_stretch` settled). A candidate both spans hold is
pushed as a contact and sets nothing, so a pair whose every candidate
is held never reads either segment's ends.

For an adjacent pair at a tangent joint, the one candidate is the
joint itself: held by both spans, recorded as a `Touch` at the shared
vertex, and discounted there. The pair's far ends are then never
read, though the census rule the ruling cites (`[topology.census]`,
`crates/profile/src/validate.rs` step 3) makes them the pair's
reading: a far end within ε touches.

Not known to be reachable as a wrong verdict: the segment leaving that
far end shares it, and its own pair with the first segment reads the
end (the far-end table's `Touch` rows in
`crates/profile/tests/rejections.rs` are that pair, segment 0 against
segment 2). A loop whose far end is the joint of two segments that
both stay clear of the first would escape. No witness was built.

The fix is one line: read the ends whenever the pair is adjacent, or
always (the reads are cheap and each is a direct distance).
