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

### What re-deriving the order against the tree changed (2026-09-19)

The order above was written at the re-scope and every row in it was
written earlier still. Re-deriving each row's fix SITE against the tree
at this program's first dispatch moved four of them. Recorded here
rather than fixed silently, because the order is the thing a later
session reads first.

- **`converged-recourse-has-no-home` left group 1 and then left this
  program — it is EDIT's now** (moved 2026-09-19; Ev granted the move
  and the standing authority to re-home a unit between tracks without a
  ruling). The group called it *"the third crosses a crate and
  announces"*; announcing is not what it needed. Both shapes the row
  states — a `pub const` beside `EditError`, or a `recourse()` method on
  it — **add API surface to `crates/editor-core`**, and the
  authorisation this program inherits is scoped to `EditError`'s
  `Display` WORDING (Ev, in-chat, 2026-09-04). `EditError` is declared
  in `crates/editor-core/src/edit.rs`, which is EDIT's territory. Only
  the viewer arm stays here: a forward at one site in
  `session/refuse.rs` once `editor-core` exposes the recourse, landing
  with the EDIT unit that exposes it.
- **`viewer-preview-names-a-verb-by-its-variant-identifier` is CLOSED
  — it was already discharged when this program inherited it.**
  `profile::path::Verb` has had a `Display` since
  `work/fix/verb-and-dimension-render-through-debug` (FIX, PR 2347,
  2026-09-11), and `PreviewError`'s arm in `crates/viewer/src/sketch.rs`
  forwards to it — `{verb}`, not `{verb:?}`. **This entry first said the
  row "cannot land from here alone" and routed it to PATHS**, which was
  wrong: the orchestrator re-derived the row's citation and not its
  premise. The correction is kept visible rather than overwritten
  because the rule it breaks — *a row's premise ages against the tree
  exactly like a citation does* — is the one handed to every lane this
  program dispatches, and the register's own instances of it are mostly
  the orchestrator's.
- **`tone-is-a-value-in-frame-and-a-comment-in-two-panes` keeps its
  place and loses a citation.** `pane/features.rs`'s hand-picked
  `ui.weak` / `ui.colored_label` pair is there as described, with the
  rule in a comment; `tree::RowStatus::badge()` is there and takes no
  tone. The row's THIRD copy at `pane/create.rs:582-586` is not: those
  lines are the `ShapeKind::Path` notation block today. The subject is
  re-derived by the lane, not repointed by arithmetic — this register's
  own rule.
- **`the-new-document-button-states-its-refusal-twice` waits on the
  census, one group later than the order puts it.** Its two answers are
  *"read the refusal"* and *"keep the literal and delete the claim"*,
  and the row says which is right is what
  `a-disabled-control-says-why-in-four-shapes` asks generally. The
  button is a genuine member of that general question — it IS gated on
  the condition `NewDocument` refuses — so deciding it alone decides
  the class from its easiest instance. The census goes first and this
  row applies its rule.

**And the frame.rs cluster is serialized, which the order does not
say.** Groups 3, 4, part of 5 and `document-news-has-no-home` all edit
`crates/viewer/src/frame.rs`. Under merge-only rules two lanes in that
file at once is a conflict bought for nothing, so **at most one
`frame.rs` lane runs at a time**, whatever the group order allows in
parallel elsewhere.

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

**A third tier, from protocol v7 (Ev, in-chat, 2026-09-19;
`docs/MODEL-AB-LOG.md`, the v7 entry, item 2).** v7 triages units into
and out of the A/B protocol, and its out-of-protocol half states a tier
below the style review that this program's posture predates and does
not mention: **a mechanical change, where neither correctness nor style
is meaningfully at risk, merges on green CI and the orchestrator's own
read — no review lane, no row.** v7's out-of-protocol clause governs
every unit that is not triaged in, and under the posture above no unit
here is ever triaged in, so the tier is available on this slate.

Nothing else about the posture moves. v7's triage question — is the
logic especially tricky, or is this an architectural decision whose
impact is broad or hard to reverse — is the question this program
answers with a **correctness arm** rather than with a dual, because
Ev's posture ruling is the more specific instruction and is not
withdrawn by v7. So the three tiers here are: orchestrator's read;
style review; style review plus a correctness arm. **The dispatch says
which tier and why, and the reason is recorded in `log.md`** (v7 item
5), so an un-reviewed unit is auditable rather than invisible.

## Exit shape

Every row above landed or ruled out, and the vocabulary the fixes
converge on stated once in `crates/viewer/README.md` with the sweep
rule that produces its population beside it — that last clause because
this program's output is exactly the kind of universal the VIEW
register says is where the next defect hides. The README clause is
VDOC's territory and lands as an announced crossing. The walk
convention applies; residue re-homes per `work/README.md`.
