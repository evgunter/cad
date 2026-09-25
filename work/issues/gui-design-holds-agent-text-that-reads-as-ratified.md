---
id: gui-design-holds-agent-text-that-reads-as-ratified
kind: issue
title: crates/viewer/GUI-DESIGN.md holds agent-written README paragraphs that read as ratified
status: open
opened: 2026-09-24
priority: P2
cost: D
---

Found by VNEWS's census,
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`.
No program claims `crates/viewer/GUI-DESIGN.md`
(`python3 scripts/work.py territory` assigns it to none), so the
finding is filed here.

**What is wrong.** `docs/DESIGN.md`'s companion row calls the page
*"Ratified; GUI v1 shipped"*, and its own header says changing a
clause *"waits for Ev"*. Much of its text never had Ev's agreement.
PR #2462 (`e9824abf3`, 2026-09-12, *"the ratified GUI clauses move to
GUI-DESIGN.md; the README is the record"*, not an `[ev]` PR) moved
paragraphs from `crates/viewer/README.md`, the implementation record
the program maintains itself, into the page. Traced examples:

- the off-thread pick index and fit seam, and the index/progress
  paragraph (*"one progress state … `frame::progress`"*). Written by
  `2622d14fa` (agent, 2026-09-05) into the README.
- *"GUI-3's §5 ratification"*, which is an orchestrator's GO reading
  at GUI-3's merge, recorded in the GUI log.
- GQ7's pick-priority text, from `f3411bbf1` (agent, GAUTH-2), which
  itself says *"Nothing here widens GQ7"*.

What IS Ev's, with the commit that records it: G1, G2 and the three
micro-decisions (`5267a9193`); GQ2/GQ3 (`57d762ef1`); G3 (`d972a1b36`);
G5 (`df8cc2787`); GQ6's toolkit row (`dc5f15444`).

**Why it matters.** `CLAUDE.md` says text is not ratified by sitting
in a Ratified file, but the page's own header tells a reader the
opposite. Every census and every "does this need Ev?" check that stops
at *"it is in GUI-DESIGN.md"* inherits the error. That is the error the
VNEWS census exists to correct.

**What deciding it takes.** Separating the page, or marking which
paragraphs are Ev's, changes what the page claims is decided, so it is
an `[ev]` PR. The census has the per-clause provenance to start from.
