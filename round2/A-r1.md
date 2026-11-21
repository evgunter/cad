## For Ev

**Recommendation (likely).** Make each **face pair** the unit an ask answers. A face pair is settled by its first finding, either a verified crossing or a refusal, and the walk then drops that pair's remaining cells. The walk goes on over the pairs that are still open. The gate lists one finding per face pair that does not hold. A budget run-out is one finding that names the pairs it left open. The budget itself stays the one per-ask backstop you ruled on PR 7101.

The defensible alternative is to **stop the whole ask at its first finding**. It is cheaper, but a body that truly crosses itself can then be told only "the kernel cannot tell". What is not defensible is today's behaviour, which pays for every finding and reports one.

Terms used below:
- **Ask:** the embedding question put to one solid.
- **Cell pair:** two patches of faces, which the walk splits until each is decided.
- **Frontier:** the cell pairs still waiting to be decided.
- **Finding:** one entry in the gate's refusal list.

**Premise check**
- The row asks "what does an ask owe after its first refusal?" That is half the problem. Today's code is incoherent in two ways (sure):
  - **It walks on, then keeps only one refusal.** It keeps walking to find more, but it keeps only the first refusal it met, in frontier order. The other refused cells reach the receipt only as a count. That is the "folding" your PR 6326 words ruled out ("can we just return a list of failures so we don't have to agglomerate them"). An ask that stopped at its first refusal would not fold anything, because it would never have found the rest.
  - **It treats crossings and refusals differently.** A crossing ends the ask at once; a refusal does not. So whether the person sees a second finding depends on which kind came first.
- **Stopping early saves less than the row suggests (likely).** The walk is breadth-first: it decides every cell pair at one depth before going a level deeper.
  - A refusal raised at the depth floor (40 splits) or by the pair budget arrives at the end of the walk, so stopping there saves almost nothing.
  - Only refusals raised immediately can save much. Examples: the departure arm refusing two curved faces across a shared spiric, a torus that is not a ring, an unsupported carrier.
  - The dearest torus asks may be of that immediate kind. Nobody has measured where their 27,000 to 65,500 splits go.
- **The face pair is already the certificate's unit.** `[topology.tiers]` says "the embedding certificate is one fact per pair of faces", and a crossing and most refusals already name two faces.
  - A budget refusal names no face today, so a person told "ran out" does not know where.
  - Refusals raised before the walk starts stop the ask at the first face. These are a face with no admitted window, or a torus that is not a ring. They name one face.

**Option A: settle per face pair (recommended).**
- What it makes true:
  - **Every finding is kept**, and no pair reports twice. "Kept" here means each face pair's first finding.
  - **Crossings and refusals behave alike.** Each settles its pair, and neither ends the ask.
  - **Nothing hides a crossing in another pair.** If pair P refuses and pair Q crosses, both are listed. The person learns that the model is at fault at Q and that the kernel's limit is at P. `[loud-failure]` requires exactly this: the recourse says whose fault each one is.
  - **A budget run-out names its pairs**, the ones still open.
- What it leaves possible: a crossing that sits behind a refusal **in the same face pair** is reported only as that pair's refusal. At most 38 of 341,880 asks end in a crossing after an earlier refusal, and we don't know how many of those are within one pair. The pair is still named, so the person is pointed at the right place.
- Cost:
  - A refusing ask costs about what the same body costs once the engine reads the refused pair. Its other pairs are walked exactly as a holding ask walks them.
  - The refused pair stops at its first refusal.
  - A crossing ask now walks its other pairs too. Crossings were 61 s in the first census.
  - How much of the 11,434 s this removes depends on a number that does not exist yet (below).
- Code:
  - The 13 `get_or_insert` sites become one place where a face pair is settled.
  - The walk skips the cells of settled pairs, and the receipt counts them as abandoned.
  - The verdict is derived: the solid is embedded exactly when no pair has a finding.
  - It applies to the lower bound, the "every pair is at least c apart" question that injectivity is, which splits over face pairs. An upper bound ("some pair is within c") is a single "there exists" claim, so its refusal stays one per ask.
- Reversible to B by stopping at the first entry.

**Option B: stop the ask at its first finding.**
- What it makes true:
  - **It is the simplest code:** every refusal site ends the walk, as a crossing already does.
  - **It is the cheapest:** the person waits only until the first finding.
  - It honours "every finding kept" as written: the list holds everything the ask found, and it found one thing.
- What it leaves possible:
  - **A body that crosses itself can be reported as a kernel limit**, with a recourse that blames the kernel for a fault in the model. This happens in the 38 asks where a crossing was found after a refusal, or more, because B also never finds crossings in pairs it does not reach.
  - **Which finding is reported depends on the order of the faces.**
  - **Engine work sees only the first blocker in each body.** Today's classes, such as the 1,076 valid bodies the engine refuses, are counted by their first refusal, and they would stay that way.
- Reversing it to A later means building A.

**Rejected: keep today's walk and let the engine work shrink the refusals.** It costs the most and reports the least, and it is incoherent for the reasons above, whatever it costs. Ranking refusal kinds, the way `geom_brep::ladder` ranks a search's refusals, is also rejected. It would pick a finding on the person's behalf, which `[simple.uniform]` ("nothing is ranked") and your PR 6326 ruling both rule out.

**Ratified text**
- `[topology.tiers]` "every finding kept, none folded": your own words, PR 6326. Under A it stays and gets one clarifying sentence: "the embedding certificate's findings are one per face pair it does not certify: its crossing or its first refusal; a budget run-out is one finding naming the pairs left open." Under B the sentence would instead say that a solid's embedding is one finding.
- "The constant is a backstop": your own words, PR 7101. Unchanged under both. A run-out remains a per-ask event, and only what it names changes.
- The ONECOPY disposition ("each bound keeps its own first refusal in frontier order ... no row, by ruling"): agent text. A keeps first-in-frontier order within one pair and nothing more; B keeps it per ask.

**Confidence**
- Today's state is the one to leave: sure.
- A over B on what each means: likely.
- How many seconds each saves: unsure, until the measurements below exist.

## For the orchestrator

**Checked against the tree** (`clearance.rs` on main; `git diff origin/main...origin/embed/gate`):
- `lower_refusal`/`upper_refusal` are kept with `get_or_insert` at 13 sites.
- `Sweep::run` picks the verdict as violation, then refusal, then holds.
- A verified violation ends the walk (`ControlFlow::Break`, with what is left counted abandoned); a refusal never does.
- `embedded` returns at the first face whose window fails (`lane.windows`).
- The gate's `solid_finding` returns `Option<ValidationError>`, one per solid.
- `Outcome::Refuse` arms in `injective.rs` refuse at once with no split: the walled-spiric departure where neither face is a plane, `CollapsedChart`, and the second departure `stuck` arm. All other `Uncertified` refusals come at `DEFAULT_MAX_CELL_DEPTH` (40) through a failed split, or at `CUSP_DEPTH` (10) for the second-order arm.
- I fetched Ev's words on PRs 6326, 7101 and 7068 with `gh api`.

**Measurements that would decide how much each option saves.** Instrument `embed/gate-measure` without changing any verdict. For each refusing ask, record:
- **M1:** the depth of the first refusal, and the splits and seconds after it, split into (a) cells of face pairs already settled and (b) cells of pairs still open. (a) is what A saves; (a)+(b) is what B saves. Do this for the twelve dearest torus asks first; if their first refusal is the walled-spiric arm, B's saving concentrates there.
- **M2:** the number of distinct face pairs that would refuse, with their kinds.
- **M3:** of the 38 asks that crossed after a refusal, how many crossings lie in the same face pair as an earlier refusal (lost under A) and how many in another pair (lost under B).

**Design details for the spec under A:**
- `EngineRefusal::Budget` carries no faces. The run-out finding needs the open pairs as data. In `walk`, `overflow` refuses the whole level, so the open pairs are the distinct candidate indices `k` on it.
- Window and torus refusals become one finding per face, and the walk runs over the remaining faces with any pair touching a refused face left out.
- A face pair can share several edges or vertices, so it can have several distinct departure refusals. The unit is still the pair, per the ratified "one fact per pair of faces". Say so in the spec so nobody keys findings on (pair, shared cell).
- The editor's clearance assertion (`editor-core/src/clearance.rs`) and `eval/wire.rs` run the same sweep. Its lower-bound verdict stays derived, so behaviour is unchanged except that a violation no longer ends the walk early. Check whether the assertion's report wants the list; nothing on main reads it.
- The PR 7101 carry needs no output ledger: a refused body is never finished, so nothing carries from it. I first assumed it did and retracted that. Do not use the carry to argue for A.

**Off the question:**
- On `embed/gate`, `crates/topo/src/validate.rs` places the new `embedding_refusal_words` between `linear_band`'s doc comment ("`Band::linear(tol)`, its refusal as the verdict vector...") and `linear_band`. The doc comment now attaches to the wrong function. Fix on that branch.
- The gate's `Crosses` arm turns a witness that does not hold at the body's scalar into `EmbeddingUncertified { WitnessUnverified, Cell }`. Under A that becomes one more per-pair refusal, which is fine. Note it in the spec.

**Assumed:** the "38 asks, 5 s" means 5 s of total ask time, not seconds spent after the refusal. I did not read `draw.md`, and I did not read the `analysis/design-fork/embed-refusal` branch.
