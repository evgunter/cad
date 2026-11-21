## For Ev

**Recommendation (likely): end the ask at its first finding, whether that is a verified crossing or a refusal, and report that one finding.** I no longer recommend collecting one finding per face pair. The measurement settles it, for two reasons:
- **Speed.** On the slowest refusing asks, stopping the ask saves 99.9% of the time. Settling each face pair separately saves 25.9%.
- **The list is not needed.** I found no reader that needs every face pair the engine cannot read. Every such finding has the same recourse: the kernel cannot read it yet.

Stopping the ask is also the right final state, not only the first step. One further change belongs to the engine; it is described below and is separate from the stopping rule.

**Terms**
- **Ask:** the embedding question put to one solid: is its boundary embedded?
- **Cell pair:** a patch of one face against a patch of another. The engine splits cell pairs until each is decided. Touching cell pairs, which share a point of the boundary, are decided by a local certificate. The rest are decided by distance.
- **Finding:** one entry in the gate's refusal list.
- **Refusal:** a cell pair the engine cannot decide.
- **Crossing:** a verified pair of points where the boundary meets itself.

**What the measurement showed** (the gate lane, on the slowest asks plus samples by refusal kind)
- **The 13 slowest asks.** These are torus bodies with 4 to 7 faces, 150 to 564 s each, 4,434 s in total.
  - Each refuses first at depth 0, within 3 splits: two curved faces across a shared edge that no certificate reads.
  - Then 4 to 21 other face pairs refuse in turn. The time goes into those pairs, 3,250 s of it spent before each one reaches its own first refusal.
  - Stopping the ask leaves 4.6 s of the 4,434.
- **The cheap refusing asks** (166 sampled, 3 to 41 s per group). Their first refusal comes at the depth floor, 40 splits (10 for the second-order arm). Most of their time comes before it, so stopping saves 6 to 37% and settling per face pair saves 0.5 to 37%.
- **Bodies that cross.** In 38 asks a crossing turned up after a refusal had already been kept. In 19 of them the crossing is in the same face pair that refused first.
- **Savings over the whole suite** (an estimate, from a sample rather than the whole suite). Stopping should cut the 11,434 s of refusing asks by roughly 5,000 to 5,600 s.

**Premise check**
- **What runs on after a refusal is the proof, not a search for a crossing** (sure). Crossings come from cheap probes. The time goes into certifying touching cell pairs, and once one pair has refused, that work can no longer make the solid certified.
- **After a refusal, only a crossing can change what the person is told.** A crossing changes whose fault it is: the model's rather than the kernel's (`[loud-failure]`). A second face pair the kernel cannot read changes nothing, since its recourse is the same as the first.

**The decision: stop the ask at its first finding**
- What it makes true:
  - **The answer stays one of three verdicts per ask:** embedded, crosses with a witness, or refused with the engine's refusal.
  - **Simpler code.** The engine stops keeping a "first refusal so far" at 13 places in the code, and every refusal ends the walk, as a crossing already does.
  - **Nothing is folded.** The gate reports everything the ask found, and the ask found one thing. That is "every finding kept, none folded" (your words, PR 6326). Today's behaviour is the one that breaks it: the engine finds many refusals and keeps one.
- What it leaves possible:
  - **A body that truly crosses can be reported as "the kernel cannot read faces a and b"** when the walk meets the unreadable pair first. In the suites that happened in the 38 asks above (22 tests), and the count shrinks as the engine learns to read more.
  - **Which refusal is reported depends on face order and depth.** Each reported refusal is a true obstruction.
- Reversible: a list could be built on top of it later.

**Worked example.** A torus body of six faces. Tori T1 and T2 share an edge that no certificate reads.
- **Today:** that pair refuses within 3 splits. The engine then splits other pairs for up to about 560 s while each of them refuses in turn, and reports only the T1–T2 refusal.
- **Stopping the ask:** the same T1–T2 refusal, after about a third of a second.
- **Per face pair:** a list of 5 to 22 unreadable pairs after about three-quarters of today's time. Every entry has the same recourse.

**Why not one finding per face pair** (my earlier recommendation)
- **The price.** On the slowest asks the cost is in discovering that each further pair is unreadable. Settling per face pair still pays that. It saves 25.9% where stopping saves 99.9%.
- **The face pair is the wrong unit for a refusal.** In 19 of the 38 crossings, the crossing lies in the same face pair as the first refusal. A refusal is about one place, the shared edge or vertex its certificate failed at. It does not settle the whole face pair. Settling per face pair would report 21 of the 38 crossings as refusals; stopping the ask reports all 38 as refusals. So per face pair is not even a reliable way to find crossings.
- **No reader needs the list.** A person cannot act on an unreadable pair. An agent sizing engine work can have the measurement harness read past the first refusal; that is measurement, and `[simple.checks]` keeps it out of the gate.

**The engine change (likely): learning that a pair is unreadable should be cheap.** On the slowest asks, pair after pair splits thousands of times and then refuses. When the departure certificate (how two faces leave their shared edge or vertex) fails on a cell pair, it answers "split, a smaller pair may certify". Here that claim is made without grounds: no smaller pair ever certified. The engine already refuses at once in one case it knows cannot be read (two curved faces across a shared spiric edge where neither face is a plane). Each further configuration like that, recognised up front, makes the refusal cheap.
- This matters under any stopping rule. After stopping, most of the remaining refusal time is spent before the first refusal: in budget-exhausted asks, and in asks whose first refusal comes only at the depth floor.
- This is engine work, one row per certificate. It is not a design text change.

**Considered and not recommended yet: after a refusal, keep looking only for a crossing.** Probes would run without the proof. That would fix the 38. But we don't know whether probes alone reach those crossings, or what they would cost (named for measurement). Build it only if the numbers say it is both cheap and effective.

**Ratified text**
- `[topology.tiers]`: add one sentence: "The embedding question ends at its first crossing or refusal; that is its finding." "Every finding kept" (your PR 6326 words) stands as it is. The 7101 backstop is unchanged.
- ERROR-DESIGN E7's three verdicts per question are unchanged. This is the smaller text change of the two options.

**Confidence**
- Stopping the ask over per face pair: likely.
- Today's behaviour being wrong: sure.
- The suite-wide saving of about 5,000 to 5,600 s: unsure; it is extrapolated from the sample.
- The engine claiming "a smaller pair may certify" without grounds: likely. It fits the measurement, but nobody has traced it pair by pair.

## For the orchestrator

**What moved since the first report**
- My first-round claim that stopping early "saves less than the row suggests" holds for the cheap asks and fails for the slowest ones.
- The other report's condition ("if the spend in other pairs dominates, stopping is the answer to the price and the pair a later step") is met. On "the pair a later step" I go further: given M3 and the needs test (`designer.md` §3, "ask whether a thing is needed"), I see no later step at the face pair.
- Both first reports agreed on the face pair; I now depart from that.
- Ev should hear the face-pair option as "considered", with the 25.9% figure and M3.

**Measurements still named**
- **M5, on the 38 crossing-after-refusal asks:** the depth of each crossing; whether it came from a probe at a fresh root or at the depth floor; and what a continuation that probes and splits only non-touching cell pairs after the first refusal would cost and find. This decides whether the crossing-only continuation is worth building.
- **M6, on the slowest asks:** for each later-refusing pair, which certificate answered "split" at each level and at which shared cell. This gives the engine rows their configurations.
- **Whole-suite figures:** the savings from stopping by refusal kind over the whole suite. The 5,000 to 5,600 s is my extrapolation: the sample's 4,434 s from the 13 slowest asks, plus about 10 to 17% of the remaining ~7,000 s.

**For the unit, if Ev agrees**
- In `Bound` (`crates/topo/src/clearance.rs`), `lower_refusal: Option<EngineRefusal>` goes. Each lower-bound refusal site breaks the walk with the refusal, and the receipt counts the rest as `abandoned`. The `Sweep::run` match becomes violation, then refusal, then holds, read off the break.
- `upper_refusal` stays. The upper bound is a "there exists" question: a refusal of one pair does not answer it.
- **One rule for every lower bound, the editor's clearance assertion included** (`editor-core/src/clearance.rs`, `eval/wire.rs`). A lower refusal already makes that verdict `Refused` unless a violation turns up later, so the only change is that a violation found after a refusal is lost. Pin or re-baseline the assertion rows that flip.
- A budget refusal comes at admission on a specific cell pair (M1, column d = 0), so it can carry that pair's faces. Today `EngineRefusal::Budget` names none. Add the faces with the unit.
- Doc comments to re-word: `Sweep::run` ("stops at the first VERIFIED violation"), `BoundVerdict`, and the `clearance.rs` module doc.
- PR 7101's carry is unaffected: a refused body is never finished, so nothing carries from it.

**Engine row to file:** "the departure certificate answers `Split` where no smaller pair certifies". Evidence: M1's c-pre column (3,250 s over 13 asks); the existing immediate arm (`walled` in `injective.rs`'s departure certificate, which refuses at once when neither face is a plane) as the precedent; `Outcome::Split`'s doc ("a smaller pair may") as the claim at issue. Place it near `nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance`.

**Off the question, carried from round 1**
- On `origin/embed/gate`, `validate.rs` puts `embedding_refusal_words` between `linear_band`'s doc comment and `linear_band`, so that doc comment now attaches to the wrong function.
- `GATE_MAX_CELL_PAIRS`'s doc ("does not depend on who asked") is stale since PR 7101.

**Not checked:** the new `designer.md` paragraphs are on `origin/main`. The checkout's copy predates them, and I read them there. I did not read `draw.md`.
