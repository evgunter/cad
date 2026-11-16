---
id: the-snaps-roll-jumps-by-a-half-turn-at-its-band-edge
kind: issue
title: The snap's roll jumps by a half turn about its target at the least turn's band edge, which a drag can cross at a small ε
status: open
opened: 2026-11-15
priority: P2
cost: M
---

Raised by PR 6988's delta review (the decided least-turn mint, fork
270), and measured there.

**What is there.** The viewer's snap turns a copy's mated direction onto
its target through `mate::turn_onto`
(`crates/editor-core/src/mate/turn.rs:42`, called at
`crates/viewer/src/session/snap.rs:319`). Outside the least turn's band
that is the decided mint (`RigidMotion::least_turn`), levered over the
session box (`Lever::session_box()`, since a gesture has no extent in
hand). Inside the band, where the mint refuses, it is the half turn
about `across(from)` followed by the least turn from `−from` onto `to`.
Both seat `from` on `to` within a few ulps, but they roll the copy about
`to` differently. Across the band's edge the turn jumps by a half turn
about `to`.

**Measured** (an oblique `to`, `from` its opposite tilted about an
oblique axis, the edge bisected to the last refusing tilt, the two turns
compared one ulp of tilt either side). At ε 1e-9 the edge is at a tilt
of 3.947e-4 rad from backwards. The two turns differ by 1.915 in their
largest entry and by 2.828 (2√2) in Frobenius norm, which is a roll of
π about `to`. At ε 1e-12 the edge is at 0.397 rad, with the same jump
(1.922; 2.828; π). The landing is within 2.3e-16 on both sides at both
ε.

**Why it matters.** At a small ε the edge sits 0.4 rad (about 23°) from
backwards. A drag that brings a copy's direction round toward its
target's opposite can cross it, and the dragged part then flips about
the mated axis between one frame and the next.

**The cause.** The band is sized by the session box's lever: the snap
decides the least turn as if the dragged part reached the whole box. By
`lever-sources`' rule a verdict levers over the extent it consumes, here
the dragged part's. That extent is not in hand at `turn_onto`'s caller:
the viewer holds the document and its evaluation, and the copy's reach
is read only inside the fold (`copy_reach`,
`crates/editor-core/src/eval/wire.rs:677`, private). A lever read from
it would be `Lever::unsourced`, a pinned site and a question for Ev.

**What would fix it.** Either a door that gives the snap its copy's
reach, so the band shrinks to the part's own (it would still jump at a
narrower edge), or a fallback whose roll joins the least turn's at the
edge, so no jump remains at any lever. The second is a design question:
which roll the snap writes for a copy standing nearly backwards.

