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
the program maintains itself, into the page.

**Ev approved that move.** #2462's body opens *"Ev approved this shape
in chat (2026-09-12)"*, after he read #2456 and said the banner
*"reads kind of oddly as a README rather than a working design
document"*. What he approved was the shape of the cut: one file split
into a design page and an implementation record. The PR rewrote no
paragraph, and nothing records Ev reading the paragraphs that crossed
the cut as clauses. So approving where to cut does not ratify each
paragraph on the design side of it. Traced examples:

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

**A second defect in the same page: a clause a later ratification
narrowed.** `crates/viewer/GUI-DESIGN.md:87-89` (G3, *What v1 is*) still
says *"Hiding and free-move are display state, never persisted into the
recipe"*. DI5 (`crates/editor-core/IDENTITY.md` §DI5, ratified in chat
2026-09-04, `087779036`) narrowed that: *"The viewer may record a
free-moved placement persistently"*, its release emits one
`DocEdit::SetPlacement`, and *"`hidden` stays display state"*. So only
hiding still holds. The design page contradicts a later ratification,
and its readers (`crates/viewer/src/display.rs`'s module docs among
them) repeat the superseded half. Re-wording G3 to say what DI5
decided changes the page's claim, so it belongs in the same `[ev]` PR.

**Why it matters.** `CLAUDE.md` says text is not ratified by sitting
in a Ratified file, but the page's own header tells a reader the
opposite. Every census and every "does this need Ev?" check that stops
at *"it is in GUI-DESIGN.md"* inherits the error. That is the error the
VNEWS census exists to correct.

**What deciding it takes.** Separating the page, or marking which
paragraphs are Ev's, changes what the page claims is decided, so it is
an `[ev]` PR. The census has the per-clause provenance to start from.
