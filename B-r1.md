# Fork `clear-enclosure` — designer B, round 1

## For Ev

**Recommendation (likely).** Yes, it is a fault, but not the one the row names. Keep one interval arithmetic and keep an f64 value entering as exactly itself. Give the engine's bound over a cell its own type, which cannot be decided through the lane's verdict funnel.

**The fault, in terms of meaning.** A lane `Interval` and a cell's bound are both "a set containing every value the quantity takes". They differ in what the width is over, and so in who an undecided reading is addressed to.
- **Lane `Interval`:** the width is over the document's parameter box. An undecided reading is the subdivision driver's cue to narrow that box, and a row in the log that is compared against the f64 build.
- **Cell bound:** the width is over a cell of a face's chart (times the family, at `Interval`). An undecided reading means only "split the cell".

Because they are one type, a cell bound is decided through the lane's `Decide` and lands in the lane's log. The tree already pays for this by hand:
- `docs/predicate-dimension-audit.md` rows F28, F29 and F30 are three hand-kept `k_stats::unlogged` scopes. F30 states the reason: logged cell readings "cue the driver to bisect a parameter box over cell pairs that only a smaller CELL answers (measured on the dumbbell neck: drives that certified at once ran past ten minutes)", and they are rows the f64 build cannot have.
- F29 reads each margin twice, once unlogged and once logged, for the same reason.
- F30 hand-picks the one reading that is the family's (`EngineRefusal::BoxDependent`) and logs only that.
- The shell's `cavity_embedded` runs the same sweep with no such scope (read from code, not run). The convention has already been missed once, and the at-rest gate would be the fourth caller to keep it by hand.

**Premise corrections (sure).**
- **f64 entering as `[x, x]` is not new and not wrong.** `Interval::from_certified` crosses a stored value as "the reals this value stands for". About 180 sites already use it, and tier 3 at f64 runs on it today (the quadrature lane, SSI certificates, the region lane). An f64 stored value stands for itself (`[simple.conditioned]`: computed "as if exact"). Your last comment on PR 6308 says the same: the body is required to be internally consistent, not to bracket a "true geometry".
- **The band is not in the crossing, at any scalar.** K·ε is how a reading is decided, and `Interval`'s own `sign_within` reads the band too. Widening an f64 value by ε at entry would count the band twice, would turn a body into a family nobody wrote, and is dimensionally wrong: ε is metres, and a direction moved by ε moves a far point by ε times its lever arm.
- **A separate enclosure type existed once.** `RingInterval`'s module doc argued "two interval roles, deliberately distinct". You retired it on PR 2701 ("i lean towards a more aggressive unification"). What that ruling removed was a second arithmetic with different semantics. The role split survived as doors on one type (C9's `Certification` trait and its gate). The engine is a third role that C9 does not cover: it calls transcendentals, so it is not certification arithmetic.

**Answer A (recommended): a cell-bound type over the same arithmetic.**
- **The type.** `CellBound` wraps `Interval` and delegates its arithmetic. It is not a lane scalar: no `Decide`, and it cannot be a `Body<_>` scalar. One funnel door of its own reads it (named predicate, K sample, band). Its undecided arm is "split or sliver", and it never writes the driver's log. So "a cell reading logged as a lane escalation" fails to typecheck, and F28–F30's scopes over cell readings go.
- **Where it begins.** It begins where a cell is put in (`Surface::rect_enclosure` and the injectivity jets). The lifted carrier before that point stays `Surface<Interval>`: the family of surfaces the stored face stands for, which has one member at f64.
- **How each scalar supplies values.** One door, bounded on certification rights, crosses stored values. f64 and Probe cross as the point, `Interval` as its bracket, `Sym` as its value channel's bracket. No caller passes a lift: `cavity_embedded`'s `lift: fn(T) -> Interval` parameter goes.
- **Point readings stay the body's.** A finding's verification (two stations, one distance) is read at the body's own scalar `T` through the ordinary funnel, and logged. That is where f64 decides with the band like any verdict, where Probe records, where `Sym` can prove a coincidence, and where an `Interval` straddle is rightly the driver's cue. This is F30's one exception made the rule.
- **Left possible.** A cell bound can still be unwrapped by a named accessor (receipts need endpoints).
- **Reversible.** It is a wrapper.
- **Against PR 2701.** It does not undo ruling 1: there is still one arithmetic. It is that ruling's intermediate newtype shape, applied to a role whose difference is the log's addressee, not refusal semantics.

**Answer B (defensible): C9's shape.** Keep `Interval` everywhere. Add the cell-reading funnel door, and a gate that keeps `k_stats::decide` out of the engine's files. This matches how you settled the certification role. It is weaker in two ways: a token gate has known gaps, and inside the engine at `T = Interval` nothing tells a lane value (logged) from a cell bound (not logged).

**Answer C (reject): a per-scalar engine arithmetic (`T::Enclosure`, Sym-native).** See the next section.

**Answer D (reject): leave it and document it.** The gate then adds a fourth hand-kept scope.

**Sym: compatible, but it decides nothing over a cell (likely).**
- **It can run.** A cell coordinate is one more indeterminate (`Sym::opaque` of the cell's range), so the engine could run at `Sym<Interval>` today, and soundly.
- **It buys no verdict.** The symbolic tier proves that a margin is identically zero. A cell bound's job is to show a margin stays away from zero over a set, and no normal form decides that. The equalities the question does need (where the boundary meets itself by construction) come from topology: shared edges and recorded contacts.
- **Cost.** One DAG node per operation per cell. Fork row 202 found the same in practice; it was decided by an orchestrator and you were not asked.
- **Where `Sym` does earn.** At the point reading. Two stations that coincide for every parameter value give a margin that is identically zero. `Sym` decides that at any box width, where `Interval` gives `[0, c·w]` and refuses. Today that is lost, because verifiers evaluate in the engine's type.
- **What would help families over cells.** A range evaluator that cancels the shared parameter dependence of two faces (both walls moving together). That is a new capability, not `Sym`. It is difficult, and justified only if the unit's measurement of `Sym<Interval>` leaves shows family width causing refusals.

**Worked example.** A tube with walls at radii 10 and 9.
- At f64, a cell pair's bound might be `[0.98, 1.3]`: clear of the band, discharged, nothing logged.
- Now a family whose wall thickness ranges over `[−0.1, 0.1]`. Every cell bound straddles zero at every depth. Today the engine splits to its depth floor and refuses on budget (unsure, not run). Under A, the engine stops asking cells and reads two stations at `T`. That reading straddles, is logged, and the driver narrows the box, which is the only thing that can answer it.

**Ratified text I would change.** `[topology.tiers]`, finished-body paragraph: "At `Interval` and `Sym` the stored brackets carry the family instead."
- **Provenance.** Commit `53c9a148d31` on PR 6308, agent-written. "Instead" was true when the point-scalar margin was a 2ε rounding allowance. After you asked for the ordinary K·ε band it became false: the band decides at every scalar.
- **Proposed.** "The question decides with the band at every scalar. A stored value enters as the reals it stands for: itself at a point scalar, its bracket at `Interval` and `Sym`, where the verdict then holds for the whole family."

C9 is unchanged.

**What changes for the at-rest gate unit.**
- **Item 1.** The yes/no verdict needs no per-scalar `Option`. It is a function bounded on certification rights (PR 2701 ruling 3's shape). Only the measure keeps its f64 `None`.
- **Logging.** Cell readings are unlogged by type, and each finding is one logged reading at `T`. No new audit row.
- **Item 7.** The shell's cavity check loses its `lift` parameter and becomes a reading of the same gate.
- **Local readings.** The spec must say, for each of them in `topo::injective`, that it passes by the band. That module's "few ulps in the chart" concessions and its `exactly` test were written when only `Interval` ran it.

**Confidence.**
- Fault as stated: sure (F28–F30 say it in the tree's own words).
- A over B: likely.
- Sym answer: likely.
- The `[topology.tiers]` rewording: sure.

## For the orchestrator

- **Nothing was built or run.** Every claim is read from code at `b125b84b307`. My target directory was never created. The clone is at `/root/.local/share/cad-work/fork-enclosure-b/cad`, unmodified.
- **Errors in the brief.**
  - "The unit that first brings f64 into the engine" is true of this engine only. f64 through `from_certified` into interval enclosures is kernel-wide (`topo/src/props/quad_lane.rs` 34 sites, `geom-brep/src/ssi/enclose.rs` 29).
  - The row's "reads as treating the float as the value" has it backwards. Treating the float as the value is the ratified f64 meaning.
  - The row and brief omit the history that bears most on it: `RingInterval` (PR 2701 ruling 1, H5 at `git show e8d0883eda:work/scalar/H5.md`), C9's "one type in two roles", and F28–F30.
- **Defect off the question (likely, read not run).** `topo::shell::cavity_embedded` runs `lane.sweep` with no `k_stats::unlogged` scope, while `editor-core/src/eval/wire.rs` `wire_clearance` wraps the same sweep in F30. A shell node evaluated at `Interval` or `Sym<Interval>` would then log the sweep's cell rows and in-band escalations: F30's own pathology (leaf rows the f64 witness cannot have, and driver cues no box answers). It belongs in `work/clear/`. I did not file it, since I may not push. Worth a pin before the gate unit copies the shell's call.
- **Could not check.**
  - How the verdict-diff engine pairs rows when f64 and the leaf verify different candidate pairs for one finding. Under A, each logs one row per finding, but possibly for different pairs.
  - Whether the K experiment wants the engine's cell readings sampled at Probe. F28 says in-band cell samples would fire k-lint rule 1.
  - The compile cost of A if `CellBound` has to implement `Real`. I assumed it does not: ring operations, `sqrt` and the few functions the engine calls are enough, because the lifted carrier stays `Surface<Interval>`.
- **Assumed.** `Candidate`'s API can hand a verifier f64 stations to evaluate at `T`. `wire_clearance` already verifies on the f64 witness bodies, so the two verifiers differ today, and the unit should make them one.
- **Row 202's provenance.** Orchestrator-decided, no `[ev]` PR. Its conclusion (the arithmetic stays interval; Sym-native pays for nothing) stands under A, on semantic ground as well as cost.
