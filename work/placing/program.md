---
id: placing
kind: program
title: PLACING — how the viewer writes a placement: the mate tool, and the gestures that re-point, move or delete one
status: active
opened: 2026-11-01
group: gui/author
prefix: placing/
tag: (PLACING orchestrator)
paths: [crates/viewer/src/matetool.rs, crates/viewer/src/session.rs, crates/viewer/src/session/*, crates/viewer/src/pane/create.rs, crates/viewer/src/pane/properties.rs]
keep_out: [opened by CHROME's 2026-11-01 cut (Ev in chat 2026-11-01 - three tracks each with a full P0 and P1 slate) and its rows moved from work/chrome by git mv with ids and bodies kept, the kernel half of composing mates is MSOLVE's (work/msolve/a-consistent-second-mate-after-a-rest-is-refused.md and the mate kinds rows) - a mate-tool row that needs the solve changed is a hand-off there, SPACES decides where a new body lands and so what a gesture finds placed - the two are coupled and a live lane in either says so on the other's log, CHROME keeps the banner a mate commit floods (committing-a-mate-floods-the-banner), shared ground in crates/viewer is legitimate per work/README.md - run scripts/work.py territory --base origin/main on every branch]
---

**What a gesture writes into a placement's bundle.** The mate tool
offers two of the kernel's five mate kinds and picks faces only, and
commits one mate where Ev wants a draft that holds a placement's whole
bundle and commits it as one undo step. Beside it, three gestures
write placements wrongly under D10: a feature gesture re-points every
copy of its target, deleting a placed feature cascades its target's
placement away, and Transform moves a placed body through a node where
it should edit the placement.

`mate-tool-commits-one-mate` carries an open design question; it goes
to designers (`docs/prompts/designer.md`) before a lane builds it, and
the mate-kinds P0 should be cut so it does not pre-empt that answer.

Opened 2026-11-01 by CHROME's cut, sized by its P0 and P1 load (Ev, in
chat: oversize is fine where the extra is P2 and below). Narrative in
`work/placing/log.md`.
