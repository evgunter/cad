---
id: doc-line-citations-rot-silently
kind: issue
title: docs' file.rs:NNN line citations rot silently — LIB-LOG cites a path that no longer exists and shell-door lines that moved; sweep the class
status: open
opened: 2026-08-31
github: 1410
refs: [1399]
---

## From GitHub issue 1410

opened 2026-08-31, 0 comments.

(SEAT orchestrator) Class finding from SEAT-1's dual review (PR #1399), filed per the findings-need-a-durable-home rule.

Program logs and the verb register carry `file.rs:NNN` citations as evidence pointers, and they rot with no signal:

- `docs/LIB-LOG.md` (the #918/G16 entry) cites `crates/sweep/src/fillet/build.rs:281` — a **path that no longer exists at all** (the blend unification moved it to `blend/build.rs`), so the citation was dead before SEAT-1.
- `docs/LIB-LOG.md:~1200` cites the shell doors at `crates/topo/src/shell.rs:463`/`:485`; already stale at SEAT-1's merge base, and SEAT-1 moves them again (to `:484`/`:505` — that one pair is corrected in the unit's own fix pass under SEAT-PLAN's courtesy clause; the rest of the class is not).
- `docs/KERNEL-VERBS.md` carries the same style of citation in many rows and has not been swept against the current tree.

The class: any doc outside `docs/DESIGN.md`'s ratified prose that cites code by line number. A sweep should re-resolve each citation and either fix it or replace it with a symbol-anchored form (`file.rs`, function name) where the line number adds nothing. Logs are append-mostly history, so the sweep should touch only citations that a reader would follow as live pointers (register rows, plan constraints), not narrative entries — the narrative's staleness is ordinary history.

## Home

`work/code-quality/` — a tree-wide prose-debt class (stale `file:line` citations across docs), which is the register's charter for structural findings rather than any one program's territory.
