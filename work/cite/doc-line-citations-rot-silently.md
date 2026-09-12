---
id: doc-line-citations-rot-silently
kind: issue
title: docs' file.rs:NNN line citations rot silently — LIB-LOG cites a path that no longer exists and shell-door lines that moved; sweep the class
status: closed
opened: 2026-08-31
closed: 2026-09-11
github: 1410
refs: [1399]
---

## From GitHub issue 1410

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Class finding from SEAT-1's dual review (PR #1399), filed per the findings-need-a-durable-home rule.

Program logs and the verb register carry `file.rs:NNN` citations as evidence pointers, and they rot with no signal:

- `work/lib/log.md` (the #918/G16 entry) cites `crates/sweep/src/fillet/build.rs:281` — a **path that no longer exists at all** (the blend unification moved it to `blend/build.rs`), so the citation was dead before SEAT-1.
- `work/lib/log.md:~1200` cites the shell doors at `crates/topo/src/shell.rs:463`/`:485`; already stale at SEAT-1's merge base, and SEAT-1 moves them again (to `:484`/`:505` — that one pair is corrected in the unit's own fix pass under SEAT-PLAN's courtesy clause; the rest of the class is not).
- `docs/KERNEL-VERBS.md` carries the same style of citation in many rows and has not been swept against the current tree.

The class: any doc outside `docs/DESIGN.md`'s ratified prose that cites code by line number. A sweep should re-resolve each citation and either fix it or replace it with a symbol-anchored form (`file.rs`, function name) where the line number adds nothing. Logs are append-mostly history, so the sweep should touch only citations that a reader would follow as live pointers (register rows, plan constraints), not narrative entries — the narrative's staleness is ordinary history.

## Home

`work/code-quality/` — a tree-wide prose-debt class (stale `file:line` citations across docs), which is the register's charter for structural findings rather than any one program's territory.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **H** — tree-wide sweep of every `file.rs:NNN`
in docs, plus live-pointer vs narrative judgement per hit. The class is
a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.


## Closed 2026-09-11 — the convention, not the sweep

The class is real and the two instances this row names are real. **The
tree-wide sweep it proposes is declined**, and closing rather than
deferring is deliberate: nothing is waiting for a trigger and nothing is
scheduled, so `deferred` would misdescribe it — what happened is that
the remedy was adopted in a cheaper form.

The disposition is `work/cite/plan.md`'s *The convention: cite by name,
and the number is optional*, which adopts exactly this row's proposal —
*"replace it with a symbol-anchored form where the line number adds
nothing"* — as the standing rule, and this row's disposition rule
(*touch only citations a reader would follow as live pointers, not
narrative entries*) as one of the three never-repointed shapes.

Why the sweep does not follow from the rule, on this row's own numbers
once `tracker-file-line-citations-measured` supplied them:

- **96% of the citations are already anchored.** The sweep would mostly
  delete redundant numbers beside names that already work.
- **A gate is worse than nothing here.** 2.1% of citations fail a
  line-range check; VIEW's hand-sweeps found ~75% of what they touched
  pointing at the wrong subject, nearly all inside the passing 2.1%'s
  complement. A checker would be silent on almost every real defect.
- **Recovery is a lookup** (Ev, 2026-09-09): `git blame` the citing file
  and the cited line is whatever it was at that hash.

**Repairs happen opportunistically**, when a lane already has the file
open, under the convention. The three this program did by hand —
`work/door/`, `work/topo/`, `work/tint/` — are the worked examples of
what one looks like, including what it costs: two rounds and a full
review, for three files.

The specific instances this row names are NOT swept and are left
standing as the record: `work/lib/log.md`'s `fillet/build.rs:281` (a
path that no longer exists), its `shell.rs` door pair, and
`docs/KERNEL-VERBS.md`'s rows. Any lane touching those files repairs
what it touches.
