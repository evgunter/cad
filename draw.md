# Design fork `embed-records` — draw

Dispatched 2026-11-17 by the EMBED orchestrator, on `work/embed/the-at-rest-gate-certifies-embedding.md` (P0, H; lane C, item 3). Raised by the gate lane (`embed/gate`): the finish holds no contact records. Protocol `d327ffb52`.

- Arm byte **254** (≥ 86): two designers.
- Label byte **127** (odd): **Fable is A, Opus is B**.

The problem statement every designer was handed is `problem.md`.

## First reports (2026-11-17)

Each designer saw only the problem statement (`problem.md`).

- **A:** the embedding promise is each solid's, never the whole body's. `finish` asks the certificate once per solid, that solid's faces against each other, and never pairs faces of two solids; what two solids do to each other is the census's, at the `⊔` that set them side by side. `AtRestBody` holds its records (body, records, verdict), so the gate derives the verdict against them. Only a record between two cells of one solid reaches the certificate; the gate confirms it, and the certificate treats the recorded cells as a place the faces meet (not a face-pair exclusion). Confidence: likely (the pinch reading: unsure). Rejected the framing (the benches' contacts are a question the gate should not ask). Proposes changing `[topology.tiers]`' finished-body paragraph (each solid's boundary; the validity class rides the finished type).
- **B:** the gate is handed what the door decided and asks only that. `finish` takes the body with the door's records and its arrival partition; the engine is offered self-pairs within each solid and pairs of solids within one arrival, never a recorded pair (the face-pair exclusion, scoped by arrival) and never a cross-arrival pair, which is the `⊔`'s census's. `AtRestBody` holds the door's records. Its limit B′: an aggregate composes its members' kept certificates rather than asking again. Confidence: likely. Rejected the framing ("where do we get the records at `finish`" would be a patch). Proposes changing `[topology.tiers]`' finished-body paragraph (each solid's boundary and the solids of one arrival).

They agree that the whole body is the wrong subject (an assembly's meetings are its census's, and an overlapping assembly must still finish), that certifying after the gate at tier 3′ is wrong, and that the finished body holds its records. They disagree on whether two solids of one door's result are asked against each other (A: no, that is the census's cross-solid gap; B: yes, within one arrival), and on what a record inside one solid does to the question (A: a confirmed meeting place; B: an excluded face pair). A reconciliation round is owed.

Both corrected the brief: the records an op writes are in hand at the Boolean's `gate` (`Held`), and `boolean/nary.rs` calls `record_at_rest` only in a test (the production callers are `editor_core::product::gather_sources` and `step_import`'s `gate3`).

Harness token counts (first report):
- A (Fable): 286,489
- B (Opus): 162,184

## Round 2 (2026-11-17)

Each designer was shown the other's round-1 report. Both moved, toward each other:
- **A** took B's lead principle (the gate asks the pairs whose meeting the building door decided) and added what it had left as "possible": two solids of one deciding door's result that the door decided are certified apart or recorded at that door's finish, and refused otherwise. Its reader is the census, with the engine's two-body question as the census's curved lane. A did not take "every deciding door is one arrival": two solids carried from one operand were decided beneath the door, so the partition is by lineage, and the shell of an overlapping assembly must still finish. A names the question beneath: does `finish` confirm the records it reads? A recommends it does.
- **B** dropped the arrival as the certificate's scope (it is the census's `group`, and asking those pairs with the certificate would be a second reader of the census's fact) and moved to per solid; took A's reading of an in-solid record as a confirmed meeting place, not a face-pair exclusion; kept that the solids of one deciding door's result must not cross undetected, through the census grouped by who decided the meeting, run inside the gate. B names the question beneath: what a finished body promises about the relation between its solids, and which reader decides it at the gate.

The positions nearly meet: per solid for the certificate; the finished body holds its records; the gate confirms a record before the certificate steps around its cells; between solids the census is the one reader, refusing at the door that decided the meeting and reporting at a `⊔`. What is left is whether "decided by this door" is read off the door (every solid of a deciding door's result) or off lineage (only pairs descending from different operands, or cut from one solid). Round 3 shows each the other's round-2 report.

Totals after round 2: A 308,738; B 172,659.

## Round 3 (2026-11-17)

Each designer was shown the other's round-2 report. Both said the positions meet on four parts: the certificate asks each solid against itself; the finished body holds its records; a point or edge record inside one solid is a confirmed meeting place the certificate still reads around (an area record inside one shell keeps today's typed refusal); between solids the census is the one reader, refusing at a deciding door only on meetings that door made and reporting at a `⊔`, with the engine's two-body question as the census's curved lane. Both state the question beneath as one: is tier 3′, in both directions, part of `finish`? Both say yes.
- **A** moved on area records and took B's sentence "refuses only where the door's own decision was at stake". It tightened its own lineage rule: a meeting the door only carried, cells keeping their geometry, keeps the verdict it arrived with; a cell the door moved or minted is the door's. It withdrew its round-2 example (the shell of an overlapping assembly: the cavity wall is minted) for a drill through two overlapping plates. It left the grouping as "one choice for Ev", against B's round-2 bullets.
- **B** gave up grouping by door (it refuses a meeting a `⊔` only reported), and stated the rule as: exempt exactly what a decision beneath covers, a record or a reported overlap; its case is a fillet whose new material reaches a neighbouring solid of the same operand. It notes the general rule needs a `⊔`'s reported overlaps to ride the finished body beside its records.

Each report answered the other's round 2, so the last difference (by door, or by who made the meeting) was already closed from both sides when the reports crossed. Round 4 shows each the other's round 3 and asks for the final text.

Totals after round 3: A 324,532; B 184,621.

## Round 4 (2026-11-17), closing

Each designer was shown the other's round-3 report and asked whether the two rules are one. **A** took B's rule in full ("exempt exactly what a decision beneath covers"): its own cell test ("a moved or minted cell is the door's") turns a `⊔`'s report into a refusal one op later where a door mints a cell inside a reported overlap, which is the argument A had used against grouping by door. **B** merged the two wordings into one rule. Agreed. Each issued its final `## For Ev`; A states two interim choices, B keeps "by who made it" against "by door" as a choice for Ev with both leaning the first.

The `[ev]` PR is 7068 (`embed/ev-finished-body`); `docs/DESIGN-FORK-LOG.md` row 284 carries the recommendation half.

Totals: A 340,716; B 195,736.

## After the PR: Ev's questions on decision 3 (2026-11-18)

Ev answered decisions 1 and 2 yes on PR 7068 (02:23Z), and questioned decision 3 three times (`ev-thread-2026-11-18.md`: Ev's comments verbatim, the orchestrator's replies summarised). The last, "how could − ever create a new overlap", shows the agreed text asks for more than a removing door needs: A's final report states that a door which only removes material cannot make a new meeting between two solids its operand held, and neither report weighed "store nothing on the body; only material-adding doors read between pieces" as an option of its own. Both designers (the same two sessions; A is the Fable designer, B the Opus designer, as drawn) were resumed at 02:50Z with the thread and the problem restated from Ev's side, each without the other's answer. They were also told that no round of this fork read `crates/topo/README.md` tier 3, which already ratifies decision 2 (PR 5967).

**The two reports on Ev's questions (each without the other's).** Both drop the stored overlap: nothing rides the finished body but its records, and `[open.lint-output]` is untouched. Both say no removing door (`−`, `∩`, split, shell, which thickens inward) can make an overlap between pieces, that `topo::shell` has no outward form, and that a `∪` fusing two overlapping pieces gives one solid crossing itself, which decision 1 refuses.
- **A: by descent.** A door's gate does not read between result solids that descend from different pieces of one operand; the meeting they arrived with stands and the finding stays the assembly's. A one-input door that adds material to a piece (a fillet in a concave corner) reads what it added against the other pieces and refuses a meeting there. It names its round-3 error (the rule put at the cell, which refused a shell's cavity wall, repaired by storing the overlap) and corrects it one level up, at the piece. Leaves possible: a cut through the curve where two pieces' skins cross leaves them touching with no record, inside the reported overlap, unread. It adds a gap off the decision: a cut whose tool's pieces overlap builds crossing cavity walls.
- **B: per piece.** Every one-input door over an assembly acts on each piece, as `−` and `∩` do through `distribute::assembled`, and returns a `⊔` whose census runs again over the new pieces; so the touch a cut leaves is recorded or reported, and a fillet that drives one part into another is reported, not refused. A gate checks between two result solids only where the faces come from different operands of that door or one is a cell it moved or minted. It notes the drilled-plates case never reaches one kernel finish today.

They differ on whether the meeting between pieces is read again after the op (B: by the result `⊔`'s census; A: no) and on a fillet reaching the neighbouring piece (A: refuses; B: reported). A reconciliation round is dispatched, each shown the other's report.

**Reconciliation, first round (each shown the other's report).** Both moved, toward each other's earlier text, and did not quite meet.
- **B** took A's rule by descent in full and withdrew acting per piece: `[operations.operators]` gives reporting an overlap to `⊔` alone and `[topology.records]` makes a meeting inside one op's result that op's defect, so a fillet reaching the neighbouring piece refuses; re-running a census over what a `⊔` decided is "decided again". It accepts the assembly-as-tool gap. It names facts to check: whether the kernel's `∪` and cut classify correctly against an operand whose solids overlap.
- **A** withdrew "read nothing between pieces": what an op made (minted or moved) between pieces is read once, at that op. A touch there is the op's record (two crossed, overlapping bars with the centre cut out leave four stubs touching along edges the cut made; `[topology.records]`: touching is always recorded). An overlap at a face bounding material the op added is a new interference finding, reported (its lean; refusing is the literal reading of one ratified sentence, so it puts the fillet to Ev as a choice). An overlap at a face cut inside a piece's old material is the old overlap and says nothing; old or new is read off the operand, which the op holds. It rejects a fresh census over all the new pieces as the measured double decision of `work/tally/a-cut-distributed-over-a-seat-re-decides-the-seat-and-reports-its-split-tool.md` (P1: one finding raised to six), and notes the evaluator's per-piece distribution is being retired by `work/tally/the-editors-distribution-over-a-seat-retires-for-the-kernels-whole-body-boolean.md`.

Agreed: nothing rides the body but its records; `[open.lint-output]` untouched; removing doors make no overlap; cells carried unchanged between pieces are not read again; a `∪` bridging two overlapping pieces refuses under decision 1. Not agreed: whether a touch a removing op leaves between pieces is read and recorded (A) or left inside the reported overlap (B); and the fillet reaching a neighbour (A: reported, Ev's choice; B: refused). A second round is dispatched.

**Reconciliation, second round (each shown the other's latest, asked not to re-decide on the newer argument).** Agreed.
- **B** accepted that the touch a removing op leaves must be read and recorded (it checked the crossed bars: the stubs meet along edges of the cut cube, on cells the cut minted, with no overlap left for the touch to lie inside; `[topology.records]`, `[coincidence.door]`), and moved to "report" for the fillet: the clause it had refused on is about a touch no record backs, not an overlap. It withdrew acting per piece with a fresh census as the measured P1 defect.
- **A** held its last text, narrowed: a removing op reads the cells it made on one piece against the other piece, records a touch, says nothing of an old overlap; an adding op reads what it added. It states the fillet neutrally as Ev's choice with its lean (report) in one marked paragraph, and answers B's three facts from the tree: `solid_contain` is a signed count, not parity; `fillet_edges` and `chamfer_edges` are generic over the finished body with no one-solid refusal found; split merges its operand's solids before cutting, so descent there is recoverable only from cell lineage. Off the question it notes that PR 6301's stated reason quoted the point-in-solid door as parity, which the module doc contradicts.

Agreed recommendation: after an op over an assembly, cells carried unchanged between two pieces keep the meeting they arrived with and are not read again; what the op made is read once against the other pieces (a touch is the op's record; an overlap at a wall cut inside the old overlap is the old one; old or new is read off the operand); a solid that crosses itself never passes; nothing rides the body but its records. One choice put to Ev, both leaning the same way: a fillet whose added material reaches the neighbouring piece is reported (lean) or refused (the ratified text read literally).

Totals (final): A 426,494; B 249,674.
