---
id: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing
kind: issue
title: A leaf whose path parts from its witness with no sign turned is reported as a flip crossing that names no predicate
status: open
opened: 2026-11-24
priority: P1
cost: M
design: true
---


Found by the lane repairing
`a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies`
(2026-11-24), measuring why a curved leaf's refusal carried no
evidence.

**What happens.** `editor_core::drive`'s `diff_verdicts` keeps only
turns: `first_turn` answers `Parted` when the leaf and its witness
name different predicates at one index with no sign turned, and
`.flip()` drops it. `classify_replay` then wraps the empty `FlipSet`
as `FlipCrossing`. So a leaf whose path parted is reported as a flip
that names no predicate, and counted as refused mass.

**Three states end in empty evidence today** (the lane's reading of
the code; the second is measured, on the plate):
- the choices part with no turn (`FirstTurn::Parted`);
- the choices agree end to end and the check sequences part (the
  plate, before its repair: fifteen premise checks logged three times
  by the witness and twice by the leaf);
- the choices agree and one check has another definite sign.
`first_turn` reads choices only (`Path::steps` filters
`Role::Choice`), so the third, a decided difference by any reading,
cannot be named either.

**Why it is a design question.** `docs/ERROR-DESIGN.md` E6 enumerates
the drive's refusal reasons and decides this case: "a leaf definite on
a *different* verdict vector is refused mass (`FlipCrossing`, flipped
predicates named)", and only an indeterminate decision bisects. And
`FlipEvidence::is_empty`'s doc states today's behaviour as intended
("an empty evidence set is possible and is honest rather than a
bug"). Whether a parted path is refused mass, or a leaf not proven
that a narrower box may re-align, and what its evidence names, changes
what E6 decides. Designers weigh it before anything is built.

The repair of the plate's row (the rounding lane's lifted pass is
unlogged) removes the one measured cause and leaves this
classification as it is. (BOXFINISH orchestrator)

## Weighed (2026-11-24)

Three designers, seven rounds (the record is on
`analysis/design-fork/a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`;
every report is on `boxfinish/briefs` under
`probe/briefs/fork-parted-path/`). They end agreed.

**E6's comparison stands as ratified:** a leaf certifies when its
whole record of decisions, checks included, equals the `f64`
witness's. Nothing new is compared and no second run is added. What
changes is what a difference is called:

- **A turn is `FlipCrossing`, and names the decision.** One question
  answered with the other definite sign; the evidence is never empty
  by type. It includes a node that fails on a decided answer where the
  witness builds: 27,667 leaves of one drive family decide an
  extrusion's depth `Zero` inside the band, a failure proven definite,
  and are bisected to the budget today.
- **Any other difference is `Parted`:** not proven the same part, not
  proven another, never a flip. It names the node and step and is
  refused at once.
- **The two lanes recording alike at the nominal is a kernel rule**,
  held by a parity test over the driven witnesses. A lane difference
  is repaired at its site (as the plate's premise check was, PR 7716),
  not tolerated by the gate.
- **A certified leaf "takes the nominal build's decisions"**, where E6
  says it "shares the nominal build's topology".

**What the measurements did to the other answers** (M13, its addendum
and M13b, `probe/` on `boxfinish/probe-build-record`):

- *Certify on what is built (named cells and how they meet)*, the
  first agreed answer: every one of 660 named flips builds the
  witness's cells. Cells cannot gate.
- *Compare with the nominal replayed at the leaves' own scalar:* that
  replay equals the witness wherever anything certifies (342 of 347
  driven witnesses; the rest are a loft that fails and the two
  plates), so once the lanes agree it checks nothing, and a parity
  test does its work off the drive.
- *Compare choices only, leaving checks out:* it certifies a notch
  wherever only a finished-body check reads a crease. No such leaf was
  found; the whole record cannot make that mistake.
- *Certify on form (cells plus what the finished body's checks
  read):* 122 flips move the body across its sketch plane through its
  frame, with no cell, sense or check differing. A record of what was
  built has now missed a fact the decisions held three times.
- All of these gates give every one of the 2,307 measured decided
  leaves the same class.

**A parting is refused at once, not bisected.** No decided leaf parts
from a same-lane replay; every parting measured is between the two
lanes and is there at zero width, where no split helps and bisecting
spends the leaf budget to reach the same sentence. E6 bisects only an
indeterminate decision. If a parting that a narrower box removes is
ever measured, that kind gets its own arm.

**Not settled here, and filed:** 318 flips (`segment_gap` 300,
`carrier_line_circle` 18) turn a decision and move only coordinates.
Whether each decision shapes the part or only routes the build is
read at its site
(`a-lane-choice-is-logged-as-a-choice-so-a-turned-one-refuses-a-leaf-the-general-lane-serves`).

**The unit** (after the text is ratified): `FlipEvidence` holds at
least one named decision by type and `is_empty`, `Unproven::FlipUnfounded`
and the unnamed flip go; `classify_replay`'s failure arm reads the
failed node's log; `Parted` with its node and step; the parity test;
a row that plants a parting at zero width and pins what the drive
spends on it. `drive::contained` then reads only named flips
(`containment-reads-a-flip-crossing-that-names-nothing-as-proof`).
(BOXFINISH orchestrator)
