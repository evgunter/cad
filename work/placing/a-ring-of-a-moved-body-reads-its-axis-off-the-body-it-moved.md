---
id: a-ring-of-a-moved-body-reads-its-axis-off-the-body-it-moved
kind: issue
title: the circular-pattern gesture reads a moved body's axis off the body it was moved from, so the ring's mate is refused NotOnTheCopy
status: review
opened: 2026-11-02
priority: P1
cost: M
pr: 6394
branch: placing/values-direction
---



Found by `placing/pattern-direction` (the sweep for its row
`a-linear-patterns-direction-components-land-in-its-slides`), run, not
only traced, at `c3626bf54a`, and run again after PR 6356 retired the
transform gesture into the placement editor (merge of `origin/main` at
`d21c469a2a`): it still refuses.

**What.** `SessionOp::AddPattern` with a `PatternRuleSpec::Circular`
rule on a body the Transform tool's Create door moved (`combine_ops`'s
`two_boxes` second block, `common::moved_to`: a `Place` of the raw
block into the raw block's own space) is refused at the door:
`Refusal::Edit(MateRefused { fault: NotOnTheCopy { .. } })`.

**Why.** `DocSession::add_pattern` (`crates/viewer/src/session.rs`)
reads the ring's axis with `DocSession::read_off`, which re-routes a
body placed into a construction's space through that construction
(`crate::world::construction_of_space`): the moved block's axis is read
off the raw block. The same read is then both sides of the ring's one
`Axis` mate (`pncad::pattern::circular_pattern`'s `(on, to, along)`),
and the copy's source is the moved block, so `on` reads a body the
placement does not copy and `mate::solve::admit_mate` refuses it
(`on_reads_the_copy`). A ring of a body never moved stands; only the
moved case is refused. No row covers it: every circular row in
`combine_ops`, `story_authoring` and `panel_display` patterns an
unmoved body.

**Reproduce.** In `crates/viewer/tests/combine_ops.rs`:
`let (mut session, _, proto) = two_boxes(Tol::witness());` then
`session.perform(SessionOp::AddPattern { input: proto.into(), count: 4,
rule: PatternRuleSpec::Circular { through: len3([0.0; 3]), direction:
scl3([0.0, 0.0, 1.0]), step: ang(FRAC_PI_2) } })` answers the refusal
above.

**Owed.** The copy side of the mate read off the copy's own source
(the moved block), the target side where the space's coordinates put
it; or one read that is both, off the moved block. Note
`work/held/the-viewers-numeric-gestures-read-a-frame-back-off-geometry.md`:
`read_off` itself is an open defect, so a repair may land with that
one's.
