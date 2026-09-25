---
id: gates-readme-cites-a-deleted-memory
kind: issue
title: scripts/gates/README.md cites memories/interval-square-poison.md, deleted 2026-08-18
status: open
opened: 2026-09-24
priority: P3
cost: E
---

Found by VNEWS's census,
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`,
while tracing `crates/viewer/src/camera.rs`'s *"ratified
interval-square rule"*.

`scripts/gates/README.md` §Which greps are the right tool (:111)
argues for `interval-square-allowlist.sh` as a grep and cites
`memories/interval-square-poison.md` for its bug class. That file was
deleted on 2026-08-18 by `4ffcde545` (*"delete interval-square-poison
memory; the CI step is now the rule's home"*). The citation now points
at nothing, in a page the companion table marks Ratified.

The repair is to drop the citation or repoint it at the gate's own
header. That is a re-wording of a pointer that moved, which
`CLAUDE.md` says lands with the change and is not a second decision.
