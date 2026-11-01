# VIEW leaves the tracker — 2026-09-23

VIEW — viewer architecture — opened 2026-09-03 in the tracker-wide cut of
that day (`docs/WORK-TRACKS-2026-09.md` §VIEW) and closed 2026-09-23 **on
Ev's in-chat ruling that it needs no exit walk**, which `work/README.md`
allows in as many words: *"once a program closes — its exit walk ratified,
**or Ev's ruling that it needs none**"*. Asked whether a walk was owed, Ev:
*"i don't think `view` ever had specific acceptance criteria to begin
with, tbh"* — then *"no walk, then!"*. The closed-program rule
(`five-closed-programs-leave-the-tracker`): `work/view/` whole, with no row
re-homed at the sweep, because the slate was emptied first.

**There were no criteria to walk, and that is why.** VIEW's plan stated its
exit shape in two lines — *"The README states the module map and every item
above has landed or been ruled out; the walk convention applies"* — not as
a criteria table. Both halves hold. The Order's six units are each
disposed on the record: the session god-module split **done** across four
PRs; `pick-priority-filter-vocabulary` **deferred** with Ev's ratification;
`camera-fold-clears-status-line` **done** (#1849); `focus-marking-is-per-node-not-per-segment`
**blocked and handed off**; `layer3-recipenodeid-aliases-across-rewinds`
**ruled** at DI1's build; `pick-index-built-on-ui-thread` **done** (#1888).

**120 rows closed.** The program did not end by finishing its slate in
place: on 2026-09-17 it re-scoped, opening four successors on its own
ground and moving 86 of its 94 live rows to them or to seven other live
programs — **VNEWS 5200–5299**, **VGEOM 5300–5399**, **VSEAM 5400–5499**,
**VDOC 5500–5599**, all claimed in the commit that opened them. What
remained was worked out over 2026-09-21/22: the refusal-floor doors
(#3000), the two sketch guards (#2967), the field and pick P0s (#3007) and
the seam's dead residue (#3029) closed as units; five further rows were
re-homed on the receiving programs' own charter tests (#3026) and one that
arrived after (#3048). Band **1900–1999** was claimed at the opening and
never drew an ordinal — *"VIEW runs no duals — its reviews are style
reviews with a second correctness reviewer where argued, so the band stays
claimed and empty"* (Ev, in-chat, 2026-09-04) — and is claimed and empty in
`docs/MODEL-AB-LOG.md`, which this sweep does not edit.

**The lane register went before the program did.** `work/view/plan.md`
carried ~1,330 lines and 87 rules that four live programs inherited by
reference. Ev's test for keeping any — does it report an actual problem,
AND would an advance warning have prevented it rather than merely named it
afterwards — retired the whole file (#3024): of seven candidates, three
were already written in `docs/prompts/implementer-discipline.md` and
`memories/agent-lane-operations.md`, and the rest were retrospective
categorisation. One amendment came out of it (#3019). The four successors'
`plan.md` and `program.md` now point at `docs/prompts/` instead, which is
where a lane's standing obligations were meant to be; that re-pointing was
a precondition of this sweep rather than a follow-up to it.

**Thirty-one `refs:` entries in six other programs' rows named VIEW ids and
were stripped** — `ciw` ×2, `lib` ×1, `vdoc` ×6, `vgeom` ×2, `vnews` ×13,
`vseam` ×7 — a mechanical consequence of the deletion, on the
`fix-leaves-the-tracker` precedent. `work.py lint` validates `refs` against
live items and has no exemption for a swept one, so a sweep that leaves
them makes the repo-wide lint fail. Prose citations of `work/view/…` are
untouched and resolve at the SHA below.

Sweep SHA `e7a037898662b5a8b1f8958cfdc9941dfba16236`.

    git show e7a037898662b5a8b1f8958cfdc9941dfba16236:work/view/<FILE>

Done-state of record: this note, the program at the SHA above, and the code
and pages it left — the architecture ratified in
`crates/viewer/GUI-DESIGN.md` (G1–G5, GQ1–GQ7) with
`crates/viewer/README.md` beside it as the implementation record the
program maintains itself (`docs/DESIGN.md`'s companion table), the
renderer-free `editor-core`/`viewer` split those pages state, and the four
successor programs still carrying the viewer's open work.
