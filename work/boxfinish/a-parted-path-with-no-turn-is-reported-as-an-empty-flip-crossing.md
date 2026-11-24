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

Three designers (the draw), independently, on one statement of the
problem, then one round in which each read the other two. Their
reports are on `boxfinish/briefs`, `probe/briefs/fork-parted-path/`.

**At first report they split three ways on what the certificate
compares**: the choices only; the build; the whole record with a
turned check a named flip and a parting bisected. They agreed that a
`FlipCrossing` names what turned by type, that a parted leaf is never
one, and that the kernel disagreeing with itself is a test
instrument's question.

**After one round all three recommend one final state: the certificate
compares the build.** A leaf with every predicate definite certifies
when it builds what the witness builds (per node: its standing, and
its bodies' cells and their incidence under their names), and is
refused `FlipCrossing` when it builds another, naming the first node
that builds otherwise, what differs there, and the decision that
turned where the log shows one. No third fate exists among decided
leaves, so nothing parted is bisected and no refusal names nothing.
The verdict vector rides every run as the explanation.

**Three facts decided it, each read in the tree in the round:**
- a check passes with either definite sign from geometry alone (an
  edge's `dihedral_sign` read at rest: a crease convex at the nominal
  and concave over a leaf is the same cells);
- the extra candidates a box scalar's pruning query hands on do log
  decisions (`topo::boolean::ops::walk_pairs`: a pair past the marked
  box test is decided by the section certificate), so a wide leaf
  lengthens its record with no change of build, and `VerdictVector`'s
  doc ("decide nothing about them") is false;
- a lane choice (`geom_brep::intersect::closed_form_shown`: the
  closed form or the general lane for one result) is logged as a
  choice, so a turned one refuses a leaf whose cells are the
  witness's under any gate on the log.

**The kernel disagreeing with itself** is asked by a test instrument,
never the drive: each corpus document replayed at a zero-width box on
the witness has the witness's whole log, checks included. A leaf that
holds the witness point and builds otherwise is a `debug_assert!` in
the classifier.

**Measure first** (all three): that the build's record exists and is
the same at `f64` and at the box scalar on a zero-width leaf across
the corpus; that leaves either side of known flips differ in it and
no pair with equal choices does; what a reader uses that it leaves
out; and how much of today's refused mass it re-sorts. **If the
record cannot be made complete, the fallback all three name** is the
gate on choices and standings: a turned choice is a flip, a parting is
a third way a leaf is unsettled and bisects, and nothing in the drive
panics.

**Rows the reports found off the question**, each filed on this
slate: `drive::contained` reads an empty `FlipCrossing` as containment
(`containment-reads-a-flip-crossing-that-names-nothing-as-proof`); a
`Budget` refusal drops the node it stopped on
(`a-budget-refusal-drops-the-node-it-stopped-on`); `classify_replay`
sends a node that fails definitely over the leaf to bisect until the
budget (`a-node-that-fails-definitely-over-a-leaf-bisects-to-the-budget`).
