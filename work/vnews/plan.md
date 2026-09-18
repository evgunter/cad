# VNEWS — the viewer's news vocabulary (plan)

**STATUS: OPEN (2026-09-17).** Opened in VIEW's re-scope, on fourteen
rows that arrived by `git mv` with their bodies unchanged. Live state
is `work/vnews/log.md`'s tail and the item files beside this plan,
never this file.

Branch prefix (the #396 convention): **`vnews/`** — unit branches
`vnews/<unit>-<slug>`, orchestrator branch `vnews/orchestrator`.
Away-channel tag `(VNEWS orchestrator)`. A/B ordinal band
**VNEWS = 5200–5299**.

## Charter

**Every row here is a defect in the vocabulary a fact travels in on
its way to a reader — never in the fact.** The viewer works out
something true: this control cannot be used, and why; this frame
refused, and what else it had to say; this seat list has three
entries; this evaluation is outstanding. Then it spells that fact as a
bare `bool` with the reason kept somewhere else
(`environmental-facts-answer-usable-as-a-bool-…`,
`is-instance-collapses-absent-and-wrong-kind`), as a literal beside a
shared constant that exists (`seat-line-spells-the-list-mark-as-a-
literal`, `converged-recourse-has-no-home`,
`the-new-document-button-states-its-refusal-twice`), as a raw variant
identifier where a word was owed
(`viewer-preview-names-a-verb-by-its-variant-identifier`), as one of
two enums that are the same enum
(`outstanding-and-progress-are-two-three-state-enums-one-hop-apart`,
`ranked-and-unranked-verdicts-are-one-type`), as a value in one file
and a comment in two others
(`tone-is-a-value-in-frame-and-a-comment-in-two-panes`), or as a rank
that silently drops its siblings
(`rank-one-discards-the-frames-other-news`,
`one-line-one-subject-loses-a-mixed-frames-expiry`).

**The test that separates this program from its three siblings.** A
VNEWS fix changes the TYPE or the DOOR a fact arrives through; it does
not change the fact, and nothing it touches survives the frame that
produced it. That is false of VGEOM, whose rows are about the VALUE
being wrong rather than the word carrying it; false of VSEAM, whose
rows are about state that outlives its frame and the boundary that
owns it; and false of VDOC, whose rows change what the tree says about
itself and change no viewer behaviour at all.

Applying it the other way, as this program's own rule about splits
demands: **a row belongs here only if a reader would see the
difference.** A rename nobody reads is not news.

## Order

E-first. Nothing here is blocked, nothing here waits on Ev, and the
three sibling successors are file-disjoint from this one except at the
shared files `program.md`'s `keep_out` names — so this program is
dispatchable in parallel with VGEOM and VSEAM from its opening day.

1. **The literals that have a home already** —
   `seat-line-spells-the-list-mark-as-a-literal` (`frame::LIST_SEPARATOR`
   exists and `seat_line` does not use it),
   `the-new-document-button-states-its-refusal-twice` (a literal beside
   a comment naming the `Refusal` variant that should back it, the two
   sentences differing), `converged-recourse-has-no-home` (two literals
   in two crates held in step by a test). Three rows, one class, one
   shape of fix; the third crosses a crate and announces.
2. **The collapses** — `is-instance-collapses-absent-and-wrong-kind`
   and `environmental-facts-answer-usable-as-a-bool-with-the-reason-
   elsewhere` are the same defect at two doors: a `bool` answered
   because one caller wanted one, with the reason kept elsewhere or
   nowhere. Take them together or the second re-mints the first.
3. **The two-enums rows** —
   `outstanding-and-progress-are-two-three-state-enums-one-hop-apart`
   and `ranked-and-unranked-verdicts-are-one-type`. Both carry an
   undecided fork (collapse, or name the difference), and the second's
   fork decides what the first collapses INTO, so they are one
   conversation. `outstanding-…` reaches `session.rs` and `pickcache.rs`,
   which are VSEAM's: announce.
4. **The discards** — `rank-one-discards-the-frames-other-news` and
   `one-line-one-subject-loses-a-mixed-frames-expiry`. Both are about
   news the ranking throws away rather than spells; the first names a
   discarded free-move placement as unrecoverable, which is the row's
   own argument for taking it before the other.
5. **The rest, unordered** —
   `a-disabled-control-says-why-in-four-shapes`,
   `a-fold-row-composes-a-producer-with-a-dead-door`,
   `document-news-has-no-home`,
   `tone-is-a-value-in-frame-and-a-comment-in-two-panes`,
   `viewer-preview-names-a-verb-by-its-variant-identifier`.

**Not scheduled, and why.** `document-news-has-no-home` and
`a-disabled-control-says-why-in-four-shapes` are both censuses of a
class before they are fixes, and the VIEW register's rule about a
population applies to each: re-derive the population by subject before
naming a door, because the count in the row is evidence only as of its
filing.

## Inbound

`joined-notices-nest-their-own-separator` is this program's row and is
still on VIEW's slate: its lane is in flight at PR #2665 and a rename
mid-review is a merge conflict for nothing. It arrives here when that
PR merges, or with VIEW's exit walk, whichever is first.

## The register

**`work/view/plan.md`'s rule register binds every lane dispatched from
this program, inherited BY REFERENCE and not copied.** Read it in full
before writing a dispatch.

The reason it is not copied is the register's own: a claim fixed in one
place and stale in another contradicts itself, and four copies of a
register that is re-derived every wave guarantee four divergent copies
within a week. The register is also evidence — every rule in it is a
named failure at a named PR — and a copy detached from the program that
paid for it reads as a rule without its receipt.

**What that costs, said plainly:** `work/view/plan.md` goes when VIEW's
directory goes at its exit walk, and this reference dangles that day.
The register's permanent home is
`work/view/the-lane-register-has-no-home-after-views-directory-goes`,
open on VIEW's slate, and it is a precondition of VIEW's exit walk
rather than a follow-up to it. This section re-points when it lands.

## Review posture

**Inherited from VIEW unchanged (Ev, in-chat, 2026-09-04, reaffirmed
2026-09-04 evening; `docs/MODEL-AB-LOG.md`'s roster line).** No A/B
duals, no row in `docs/MODEL-AB-LOG.md`; the band stays claimed and
empty. The default is a style review against
`docs/prompts/reviewer-style-lane.md`, with a correctness arm added
only where a unit's failure mode is a confident wrong answer rather
than a refusal, and the dispatch says which it chose and why.

## Exit shape

Every row above landed or ruled out, and the vocabulary the fixes
converge on stated once in `crates/viewer/README.md` with the sweep
rule that produces its population beside it — that last clause because
this program's output is exactly the kind of universal the VIEW
register says is where the next defect hides. The README clause is
VDOC's territory and lands as an announced crossing. The walk
convention applies; residue re-homes per `work/README.md`.
