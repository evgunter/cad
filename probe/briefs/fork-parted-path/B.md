## For Ev

**Recommendation (likely).** A leaf's two fates once every predicate is definite should be decided on **the build**, not on the decision log. A leaf certifies when it builds what the witness builds: per node, the same standing and the same names. It is refused as `FlipCrossing` when it builds something else, and that refusal always names the first node that builds otherwise, with the decision that turned there as the cause where one did. The decision log stops being the gate and becomes only the explanation. Nothing is left between the two fates, so "a flip that names nothing" can't happen: the evidence is non-empty by type and `FlipEvidence::is_empty` is deleted. A replay where the kernel disagrees with itself (the plate's fifteen premise checks) is caught by a test instrument, not by the drive.

**Terms.** *Witness*: the nominal document evaluated at `f64`, at one point. *Leaf*: a sub-box of the parameter box, replayed at a box scalar (`Sym<Interval>`). *Verdict log*: per node, every definite decision through the funnel: predicate, sign, and *role*. A *choice* is a decision where another sign **can** change what the node builds. A *check* is one where another sign can only make the node refuse. *Verdict vector*: per node, its standing (built, failed, poisoned or absent) plus its whole log, checks included. It is what the gate compares today. *Turn*: the same predicate, reached along an identical stretch of choices, decided with the other sign. *Parting*: any other first difference: another predicate, one log ending first, or a pruning query handing on other candidates (a *mark*). *Name table*: the names each node output carries, minted from lineage and structure, never from floats. *Masses*: the share of the box's probability that is certified, refused per reason, or outside the box.

**Premise check: the question as asked is one level too low (likely).** It asks what a parted log means. The real fault is that the gate compares a *record of how the leaf decided* when the claim it certifies is *what the leaf built*. E6 itself argues that the vector matching implies the leaf "shares the nominal build's topology". So the log was only ever a stand-in, and it is a bad one in both directions:
- **It can prove "same" but never "different" (sure, by the role contract).** Identical choices give an identical build, since every topology-determining branch is a logged choice. But a choice that turns only *can* change the build. DESIGN `[verdicts.consumers]` names choices that never do: a *lane choice* (a class or pose reading that picks the closed form or the general lane) "picks a form for one certified statement the body has either way". Today a lane choice that is Zero at the nominal and definite off it refuses as "proven different" a leaf whose cells are the witness's (likely reachable; not measured).
- **The three no-turn states the item lists are not findings:**
  1. *Choices part with no turn.* By `resolve/vdiff.rs`'s own account this means something unlogged steered the runs: an unmarked filter, a heuristic, or an `unlogged` reading. Or a pruning query handed on other candidates, which happens either because a box's enclosure is wider than a point's or because geometry moved the overlaps. Pruning adds work, not cells. Nothing here shows the build differs. (likely)
  2. *Check sequences differ, choices agree.* At a witness-containing leaf this is only the kernel's two lanes logging different passes, as on the plate. Elsewhere it can also be a check-scope query handing on more candidates. Either way the build is the same. (likely)
  3. *A check has another definite sign, choices agree, node built.* By the role contract the build is the same. The sign is geometry, and refining never removes it: `props_verdict_shapes.rs` row 3 is that log shape. (likely)

  Today all three are counted as decided-different mass. Without the structural gate, any rule that bisects them instead would refine states 1 (geometric) and 3 down to the budget floor.
- **The consumers already compensate**, which shows the layer below produces the wrong thing. The range search reads an empty flip as `Unproven::FlipUnfounded`, "a defect of the leaf protocol to name", and it asserts in debug builds that an interval holding the current value is never decided different. Two other consumers take any `FlipCrossing` as proof, so today an empty one can mislead them: E2's chamber containment (`drive::contained`) and the viewer's "boundary at the current value".

**Ratified text.** E6's no-flips rule is Ev's own proposal ("adopted round 3"): refuse the far side of a flip, enumerate no branches. This keeps it whole and changes only what "the far side" is: another build, not another log. The wording "definite on a *different* verdict vector … flipped predicates named" is the mechanism, not the decision. The shallow checkout did not show who wrote it, so I couldn't confirm. The `is_empty` doc ("honest rather than a bug") came in with PR 7166 and is agent-written. Proposed E6 leaf-protocol text, replacing its first two bullets' conditions:
> Every predicate definite → the leaf **certifies** if it builds the witness's build (per node: standing and names; assertions excluded), and is **refused mass** if it builds another (`FlipCrossing`, naming the first node that does and the decision that turned there, where one did). The log explains; it does not gate.

No DESIGN.md text is needed.

**The final state (A, recommended).**
- *Certify*: fully definite, and per node the standing and name tables equal the witness's. It is sound under the same assumption today's gate makes: every decision is definite over the leaf, so every point takes the leaf's path and builds its structure.
- *FlipCrossing*: fully definite, structures differ. The evidence is `{ node, difference: standing | names gone/new, cause: Turn | Parting | structure flip }`, with a required first node. Every consumer reads a proven different build. `DifferenceKind` is read off this evidence instead of being re-derived from standings.
- The certified leaf carries the key of the structure it shares, so leaves can still be presented coalesced.
- *Masses*: certified means proven to be the witness's build. `flip_crossing` means proven to be another build, the finding E6 means. Every other class means unsettled. There is no hidden fourth meaning.
- *Kernel self-disagreement*: (i) a test instrument replays each corpus document at a point-width leaf on the witness and asserts that its log equals the witness's, checks included. It is never in a shipped build and is no receipt column, the shape E12 ratified for kernel questions. (ii) A leaf that holds the witness point and builds otherwise is a kernel bug, `failure.taxonomy` row 5: a `debug_assert` in the drive. The range search's own copy then goes.
- *Consumers*:
  - `range.rs`: `flip()`'s unfounded arm, `Unproven::FlipUnfounded`, its viewer sentence and its Python tag `flip_unfounded` are deleted.
  - The viewer's `Floor(FlipCrossing)` sentence is deleted, since the arm stays unreachable.
  - `render_reason` loses " unnamed".
  - The tour (Budget-only assertions) is unaffected.
  - E2 containment becomes true as stated.
- *Reversible*: yes. The gate is one function, and the log still rides every evaluation.

**The alternative (B): keep the log as the gate, make it honest.** The gate compares choices and standings only, with checks bearing on certification through definiteness alone. This settles states 2 and 3 and certifies their leaves. A turn is `FlipCrossing`, non-empty by type. A parting becomes a third way a node leaves a leaf unsettled (`Unsettled::Parted { step, parting }`): bisect on that node's reads; at the floor, `ReadsNoVariable` or `Budget` naming it. Pros: no structural record to trust, and a smaller change. Cons: a turned lane choice stays a false "different". A geometric parting refines to the floor at full cost and ends as Budget mass. "Different" still means "logged differently". I lean A, and would fall back to B only if name tables turn out not to be a complete record of the build (next).

**Load-bearing unknown (unsure):** whether per-node standing plus name tables (plus the recorded coincidences) states everything "the same build" must mean: cells, adjacency, which cells were glued. E6 already leans on "lineage-scoped key identity" for exactly this. The first step of the unit should be to measure it against the corpus (witness against point-width leaf, and against leaves either side of known flips) before anything is built.

## For the orchestrator

- **Assumed, not checked:**
  - That name tables are scalar-independent at `f64` and `Sym<Interval>`. I read `NodeValue::outputs` only.
  - That a turned lane choice is reachable in the corpus. Nothing I read measures it.
  - That parted leaves away from the witness occur on booleans of moving parts.
  - Provenance: `git log -S` here returned graft and STATUS commits for E6's phrase, so "Ev's proposal" rests on E6's own parenthesis.
- **The witness-premises ruling** (unlogged rounding pass, F48) stands on its own terms, but under A the drive no longer depends on log parity. Parity becomes the instrument's to enforce. Don't let the gate become a parity check again.
- **Off-question defect (P1, falls with this unit):** `drive::contained` reports E2 chamber containment when boundary leaves are empty `FlipCrossing`s, so it can claim containment on nothing. File it on BOXFINISH, or note it on the row.
- **`classify_replay`'s `_` arm** bisects a node that fails definitely in the leaf. Under A it is still not read as a different build. That stays conservative, and it stays out of scope.
- Both answers delete `FlipEvidence::is_empty` and `Unproven::FlipUnfounded`. Any unit for this item should list `range.rs`, `viewer/src/session/range.rs`, `pncad-py/src/tags.rs` and `py/range.rs` as touched.
