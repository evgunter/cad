# MSOLVE exit walk — criteria vs evidence

**STATUS: DRAFT (one unit in flight — MSOLVE-9, dispatched
2026-09-20 on `msolve/9-from-face` after Ev's word on its A11 sentence
and spec (#2895); this walk becomes PROPOSED when it merges, and on
Ev's sign-off it is MSOLVE's done-state of record and the program
closes).**

MSOLVE = the mate solve's correctness residue (`work/msolve/plan.md`,
`work/msolve/log.md`; opened 2026-09-04 on Ev's steer as S-MATE's
successor for the residue that is assembly SEMANTICS rather than
document custody; A/B band 2600–2699, claimed for bookkeeping only —
the program ran no A/B rows). The criteria are the program's charter
sentence (`program.md`) and the plan's slate commitments, quoted
verbatim, one per row; dispositions per the M5–M8/ASM/S-MATE
convention: MET / MET-WITH-RECORDED-HONESTY / CARRIED (named owner) /
IN FLIGHT.

Eight units merged: PRs **#1929, #2039, #2081, #1960, #2090, #2116,
#2885, #2896**; two `[ev]` rulings taken in-program (**#2086** the
lever from the part's evaluated body, **#2118** the edit door takes
the reach with replay pure over the log) and one on a predecessor's
question that this program drafted the design for (**#2256**, the
`FromFace` frame — its A11 sentence is on **#2895**). Every kernel
unit had one style review and one correctness arm on a frozen head,
then a fix pass; every unit merged on its own green hosted head.

## The walk

| # | Criterion (verbatim) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | charter: "one live silent wrong answer in the solve" (`mate-solve-is-transform-blind`) | MET | **#1929** (MSOLVE-1): a mate reads at its operand; the solve composes every pose-bearing node between the operand and the minting instance (Ev's ruling, 2026-09-05); `fix_xblind_probe.rs` deleted. **#2039** (MSOLVE-2): the member chain — nested patterns, sibling distinctness at every level, the loop-closure rows. |
| 2 | charter: "one refusal that reports a false cause" (`mate-dangling-head-is-a-catch-all-that-reports-a-false-cause`) | MET | **#2081** (MSOLVE-3): `DanglingHead` no longer a catch-all; `MateFault::PlacerRefused` carries the evaluation layer's own typed refusal, and the placement axis is decided. |
| 3 | charter: "one ruled vocabulary extension gated on the first" (`nested-pattern-mate-heads-refuse`, the PR 1731 ruling) | MET | **#2039** (MSOLVE-2) lands the rest of the ruling; A11 rule 5's last sentence in `ASSEMBLY.md` is the vocabulary's ratified home (answered without a PR, `plan.md` §Where the extended vocabulary is written down). |
| 4 | charter: "two records that state something untrue" (`aq8-skip-half-is-cited-as-ratified-and-is-not`, `mate1-sweep-inferred-a-remap-from-a-refuted-reachability`) | MET | **#1914**: the SKIP half was ratified on PR 592's addendum and sits in the AQ8 clause; the remap record corrected as a record (2026-09-06). |
| 5 | slate: "`MSOLVE-4` — a mate's memo key carries the solve's answer" | MET | **#1960**: a blamed mate reads `Failed` in the evaluation that blames it; CHROME's viewer-side guard retired with it (the closed row's citation corrected 2026-09-19). |
| 6 | slate: "`MSOLVE-5` — the at-rest gate asks the operand's own table before it says a name vanished" | MET | **#2090**: `RefusedRef::ReadBelowARoot { at }`; `NodeGone` deleted; the kind question later left runtime with EDIT's typed head (ruled: an ordering over one question is empty). |
| 7 | slate: "the program closes when the lever … [is] in" (`mate-lever-needs-the-parts-extent`, `reconcile-solves-with-no-resolver`) | MET | **#2116** (MSOLVE-6, Ev's B on #2086 and (a) on #2118): `L = (R_a + ‖a‖) + (R_b + ‖b‖) + Σ|lengths|`, every term an upper bound, no floor; `MateReach` keyed by the part; `apply` takes the reach, the log records the maintenance, replay never solves; recorded rows held to the `SetPlacement` predicate at load. |
| 8 | slate: "the member residue (with the wire hole)" | MET | **#2885** (MSOLVE-7): one nominal environment per solve handed in by the evaluator; the axis operand's refusals seated where the evaluation seats them through one classifier; the flat index closed by citation (ruled: the walk keeps the flat `Part`); `deny_unknown_fields` on `MatePrimitive`. |
| 9 | slate: "the margins' arm (with the witness and the `MateFault` note)" | MET | **#2896** (MSOLVE-8): `Lever { Roll, Residual }` and the closed `Clash`; `Subgroup` directions as `UnitVec3` with `parallel` deciding once and returning its witness; `MateFrame::frame` over geom-core's `point_at_frame` (the fence widened by that one door, announced on SCALAR's tracker); the `MateFault` consumer sentence, no `subject()`. Honesty: the lane argued past the spec's stop clause and was reversed by ruling; the boundary pairs escalate under the one predicate, as measured. |
| 10 | slate: "the face-resolved frame" (`mate-frames-resolve-from-a-face-at-evaluation`, Ev's (F) on #2256) | IN FLIGHT | `docs/MSOLVE-9-SPEC.md` and A11 rule 5's inputs sentence ratified on **#2895** (`[ev]`, Ev's word 2026-09-20, merged at `5530c0633`); the lane dispatched from main after MSOLVE-10, since both rewrite `mate/solve.rs`. |
| 11 | slate: "the static clocking refusal" (`mate-clocking-has-no-gui-path`, half 1) | MET | **#2913** (MSOLVE-10): the insert door asks the solve's own per-mate admission (`admit_mate` — walks, one `check_references` shared with the solve's first loop, `admit_class`, `mate_coset`) and refuses `EditError::MateRefused` carrying the solve's fault unaltered; the table's static gaps have one home (`mate::table_gap`, read by the table and the viewer's tool); replay re-decides the datum and declines the rider (`Maintain::reach` states the two replay rules). Honesty: measured, the rider on a coincidence is DECIDED over the mate's lever, not static, so the door asks the reach it holds; the reviews retired the spec's "never enters the document" — the doors decide edits and the solve decides states (A11 rule 1, an elaboration), a state a mate comes to hold after insert and a loaded snapshot stay the solve's, and a mate on a pair the fold never reads is refused on the datum alone (pinned). The correctness arm's probes were run by the orchestrator after the arm hung: door equals solve to the bit. Half (2) recorded, not built (a coaxial mate with a rider is the spelling; the convention is MSOLVE-9's; the affordance CHROME's). |
| 12 | process: "one style review per unit, plus a correctness arm where a unit moves a kernel answer" (`plan.md` §Review posture) | MET | Every kernel unit: two reviews on a frozen head, adjudicated against `docs/prompts/implementer-discipline.md` and `reviewer-style-lane.md`, a numbered fix pass, then merge; the adjudications are `log.md` entries. |
| 13 | process: every unit merged on its own green hosted head | MET | Nine unit merges (and one `[ev]` merge on Ev's word), each with the run's conclusion read before the merge call carried the full head SHA; runs cancelled by a follow-up push were verified cancelled, never read as red. |

## Walk evidence beyond the criteria

- **What the reviews caught that the lanes did not**: MSOLVE-6's
  recorded maintenance rows were trusted bytes at load (a reflection
  loaded; fixed to the `SetPlacement` predicate); MSOLVE-7's
  evaluator built a second nominal environment and called the solve
  (the class the unit closed, open at its own boundary) and a
  reachable two-seat shape its own row could not see; MSOLVE-8's
  second decision on the planar-pair line, two ulps from the first,
  refusing under a name of its own. Each was the correctness arm's or
  the style lane's, none the implementer's — the two-review posture
  earned its cost on every kernel unit.
- **Rulings this program leaves behind**: the mate solve's inputs are
  the document plus its parts' evaluations, read through one lazily
  asked door (A11 rule 5, pending #2895); a decided margin is spelled
  once and its witness minted by the decision that decided it; a
  deviation past a stop clause is reversed, not argued; a walk keeps
  naming a copy by the document's own `Part(k)`.
- **Handoffs ledger at exit** (each with a home, re-homed at the
  sweep, never left in the closed directory): `gauge-of-recomputes-
  the-clusters-per-placement-lookup` (measured quadratic; a perf
  owner), `lever-refusal-respells-reach-refusal` (mate.rs — the
  successor of this territory), `mate-primitive-unit-variants-load-
  from-a-null-payload` (WIRE's question), `mate-solve-carries-the-
  cluster-maintenance-half` (a document-layer owner — the maintenance
  is D-3's), `near-parallel-planes-refuse-under-a-false-predicate`
  (the solve's translation system — the successor), `msolve6-part-
  reach-double-evaluation` (`work/issues/`), plus what MSOLVE-9 and
  -10 file. LIB's `no-door-mints-mate-frame-from-face` follows
  MSOLVE-9 by announcement.
- **Open with Ev at exit**: this walk's sign-off; #2895's sentence.
