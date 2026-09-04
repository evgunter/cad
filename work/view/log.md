# VIEW log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/view/plan.md`. A/B band 1900–1999
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose VIEW section is the
charter this plan restates. Opens after CHROME's slate. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `viewer-session-god-module-split` from `work/issues/`
- `pick-priority-filter-vocabulary` from `work/issues/`
- `camera-fold-clears-status-line` from `work/issues/`
- `focus-marking-is-per-node-not-per-segment` from `work/issues/`
- `pick-index-built-on-ui-thread` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Hand-off from DOCM (2026-09-04)

`layer3-recipenodeid-aliases-across-rewinds` re-homed here by
header-preserving `git mv`: the rule is ratified
(`docs/DOCM-IDENTITY-DESIGN.md` DI1 — a held id is valid on the history
branch that minted it; tools clear on history replacement), the build
is the viewer's. Signed (DOCM orchestrator).

## Opened for work (2026-09-04)

**The opening condition is met.** The plan says this program opens after
CHROME's slate; CHROME's 2026-09-04 entry parks the nine items whose
ground is `session.rs` or `app.rs` on `viewer-session-god-module-split`
and names it their trigger. So CHROME's remainder is no longer
competing for these files — it is queued behind unit 1 — and the
dispatchable half of CHROME's slate names no path this program's
`paths` cover. Nothing is left to wait for.

Orchestrator seat taken this session. The orchestrator work lands from
a session branch rather than `view/orchestrator`; the `view/` prefix
still governs unit branches.

**This program runs no A/B duals (Ev, in-chat, 2026-09-04).** Reviews
here are style reviews against `docs/prompts/reviewer-style-lane.md`,
with a second reviewer on correctness for the units named below. VIEW
therefore draws no ordinal from its band, which stays claimed and
empty; the band table in `docs/MODEL-AB-LOG.md` is amended in this
commit to say so, so an analyst reading 1900–1999 as a gap does not
have to guess why. This is a dispatch-posture instruction, not a
protocol amendment: v6 is untouched everywhere else.

### Review posture, per item, and why

The default is a style review and nothing more. Three items are argued
above that default, and one is argued down.

**`pick-index-built-on-ui-thread` is re-cut into three.** It was the
program's one D→H unit and it stacked three separable things: revising
a ratified seam, moving work across a thread boundary, and inventing a
staleness rule for frame state that now arrives asynchronously. Stacked,
its failure mode is a picture and a pick index that disagree —
intermittent, silent, and invisible to a style lane, which is the
argument for an adversarial review. Unstacked, it is not:

- **6a, the seam ruling** — an `[ev]` PR extending GUI-3 §5's
  frame-state inventory and stating the staleness rule. No code, so
  nothing to review adversarially.
- **6b, the move** — tessellation and `PickIndex::build` onto the
  `EvalService` worker over the shipped `CancelToken` and
  cancel-and-restart, `PickCache`'s retry policy travelling with it.
  Style review plus a second reviewer on correctness.
- **6c, staleness as frame data** — the rule lands in `frame` as values
  with headless rows. `frame` was extracted precisely so these rules
  are values a row can execute rather than app-gated code nobody can
  reach; `camera-fold-clears-status-line` makes that argument in its
  own text. A rule a row executes is testable, so a style review is
  the right instrument.

**If 6a rules the staleness rule is NOT expressible as frame data**,
6b and 6c collapse back into one unit that does want an adversarial
review, and that is a question for Ev before dispatch rather than a
call to make at dispatch time.

**Two units get a second reviewer on correctness**, on the same
argument in both cases — the failure mode is a confident wrong answer
with no refusal anywhere, which is the shape a style lane is not
looking for:

- `layer3-recipenodeid-aliases-across-rewinds`: the filed reproduction
  authors a real `Node::Revolve` of nodes nobody picked, and every kind
  gate passes. Narrow enough (one rule, three holders) that a second
  correctness read is proportionate to it.
- the authored-step to canonical-segment door under
  `focus-marking-is-per-node-not-per-segment`: the item names the
  hazard itself — a wrong map lights a confidently wrong edge, silently,
  with the user believing the picture.

**`pick-priority-filter-vocabulary` is argued DOWN, to parked.** Its own
text sets the trigger at a third asymmetric tool (a vertex pick), and
no such tool exists or is scheduled. `crates/viewer/README.md` GQ7
ratifies the deferral in as many words — the filter vocabulary waits on
sketcher and tree design — so building it now is not executing this
program's plan, it is overturning ratified design for two tools that
are already served. Parked; the blocker is recorded in the item's
header once unit 1 has said where a tool states what it wants, since
`ToolKind::pick_kinds` is one of the hand-maintained lists over the
tool set that unit 1's charter collapses.
