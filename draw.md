# Design fork `embed-price` — draw

Dispatched 2026-11-18 by the EMBED orchestrator, on `work/embed/the-at-rest-gate-certifies-embedding.md` (P0, H). Raised by the gate lane's census: the gate is most of the suites' test time. Protocol `d327ffb520`.

- Arm byte **204** (≥ 86): two designers.
- Label byte **232** (even): **Opus is A, Fable is B**.

The problem statement every designer was handed is `problem.md`.

## First reports (2026-11-18)

Each designer saw only the problem statement (`problem.md`).

- **A:** split the certificate into one fact per pair of faces (of the two faces and the cells they share). `AtRestBody` keeps those facts; a door built from finished operands hands them to `finish`, which compares faces exactly (carrier, sense, loops, edge geometry, which cells are the same entity), reads the fact of every pair carried unchanged and asks only the rest. Each carried fact records its spend, charged against the one gate budget, so the same body gets the same verdict by any route. The intermediate finishes are not the waste (a Boolean's readers need an embedded operand); the waste is that each ask re-proves the whole body. The door declares no face map. Leaves pay in full. Confidence: likely (soundness unproven until the decomposition is pinned; the saving unsure for `sweep`). Rejected the framing (the lane's "every intermediate finish asks" names the wrong lever). Proposes changing `[topology.tiers]` and the places row (one added sentence).
- **B:** keep the promise as ruled and change what the gate asks: only the pairs the finishing door made, carrying the rest from the door's finished operands as contact records are carried. `finish` takes the body with its finished operands; the gate reads "kept" by comparing each face with the operand's (never declared by the door). A second step carries two pieces of operand faces that shared nothing (their distance can only have grown). Regrouping doors and moves ask nothing; a leaf asks its whole body. The pair budget attaches to a face pair, so a pair's verdict is a function of its two faces and nothing caps a body's size (the per-body budget is also a ceiling on size). Confidence: likely (that it alone meets the CI bar: unsure). Rejected the framing in part (nearly every finish is a door's own result, which is PR 6326 as ruled). Proposes changing `[topology.tiers]` (one sentence) and the row's "one fixed pair budget".

They agree on the final state's core: the certificate is a conjunction of face-pair facts; a finish carries the facts of pairs kept unchanged from a finished operand, established by exact comparison and never by the door's word, and asks the rest; leaves pay in full; narrowing where the promise holds is out. They differ on the budget (A: one budget per body, with carried spend charged to it, for history independence; B: a budget per face pair, removing the size ceiling) and on carrying pairs of pieces that shared nothing (B's second step; A does not have it). Both name the same missing measurements. A reconciliation round is owed.

Both corrected the brief: "D5 (latency)" is not DESIGN's D5 (`[identity]`).

Harness token counts (first report):
- A (Opus): 164,575
- B (Fable): 221,823

## Round 2 (2026-11-18)

Each designer was shown the other's round-1 report. They hold the core together and both now leave pieces out of the carry. On the budget they crossed.
- **A** moved to a budget per face pair (B's round-1 position): it gives the same verdict by every route with no spend to record and charge, and it removes the size ceiling A had not seen. A notes its own PR 6014 objection (work should not depend on how a surface is cut) cuts both ways. It keeps the per-body budget with charged spend as a stated alternative, and puts to Ev: is the budget's unit the body or the face pair, and is a valid body's certification time allowed to grow with its near pairs uncapped. A did not take pieces: "this face is a piece of that one" is the door's lineage or a new certified question, and far pairs are pruned for nothing.
- **B** withdrew the budget per face pair, on reading fork-log row 220 and `work/clear/log.md` (2026-11-01): per-face bounds were rejected there as depending on how a surface is cut. It took A's charging (route independence stated as a requirement) and dropped pieces (containment cannot be read by bit comparison; a piece pair has no recorded spend). It now offers Ev a per-body constant as today, or a per-body allowance that grows with the body's first reading (PR 6102's form for callers), leaning to the second once the largest bodies' receipts are measured: the constant is also a ceiling on size (a planar solid of roughly 700 to 1,300 faces would refuse). B corrects both first reports' "linear in the chain": a face trimmed at every step loses its pair facts each step.

Agreed: the certificate is one fact per pair of faces, of the two faces and the cells they share; `finish` takes the finished operands, compares exactly, carries the facts of matched pairs and asks the rest; regrouping doors and moves ask nothing; leaves pay in full; the same body gets the same verdict by any route; the decomposition must be pinned before anything reads a carried fact. Not agreed: the budget's form (A: per face pair; B: per body, constant or growing). B names the question beneath: what the pair budget bounds, the latency of one ask or how hard a body the kernel accepts. Round 3 puts that question to both.

Totals after round 2: A 176,371; B 252,164.

## Round 3 (2026-11-18)

Each designer was shown the other's round-2 report and asked the question beneath: what does the pair budget bound, the latency of one ask or how hard a body the kernel accepts? Both answer the same way: once carried facts must give the same verdict by every route, the budget cannot bound an ask's latency, so it bounds acceptance. On its unit they crossed a second time.
- **A** withdrew the budget per face pair, on reading fork-log row 220 and `work/clear/log.md` (2026-11-01): a pooled budget charged at recorded spend depends on the cut only through the geometry's own reading, while a per-pair budget multiplies the allowance by the cut. It offers Ev a per-body constant (today's, a ceiling on size) or a per-body allowance that grows with the body's first reading, leaning to the second, settled by receipt pairs against face count on the largest holding bodies. It accepts that a chain is not linear. It names a second question, not taken now: whether a fact is about a face pair or about how two faces leave one shared edge.
- **B** returned to the budget per face pair. The cut objection does not pick a side (a seam changes a per-body spend too, and `[topology.tiers]` defines the question relative to the body's own cells). Row 220's reason does not transfer: a measurement is one question whose faces share an error budget, so only a pooled bound means anything; the certificate is a conjunction with nothing to pool. A per-body budget's price (a stored spend on every fact; a pair's verdict depending on unrelated faces; a valid body refused for being large) buys only a bounded wait, and a refusal is the wrong tool for bounding a wait. It states the per-body charged form as the alternative, and puts the acceptance to Ev: a valid body's certification time grows with its near pairs, with no cap. It drops the growing allowance it proposed in round 2. It leaves out of the agreed text whether a refusal stops at the first pair (the spec says every finding is kept).

Agreed: the core as after round 2; that the budget bounds acceptance and not latency; that Ev rules on the budget's unit; the measurements. Not agreed: the lean (A: per body, growing; B: per face pair), and it turns on one point each has argued without seeing the other's argument: whether row 220's rejection of per-face bounds transfers from a measurement to the certificate. Round 4 puts each round-3 report to the other on that point.

Totals after round 3: A 190,927; B 271,925.

## Round 4 (2026-11-18)

Each designer was shown the other's round-3 report and asked whether row 220's rejection of per-face bounds transfers from a measurement to the certificate. Each was persuaded by the report it read, so they crossed a third time.
- **A** now holds the budget per face pair and takes B's round-3 `## For Ev` as its own. Row 220's first reason (a tile must meet a pooled error bound alone, so the answer depends on the cut) does not transfer: the certificate is a conjunction over the body as a cell complex, the cut is part of its subject, and there is nothing to pool. Its second reason (only a per-call bound is bounded) is a latency reason, and a charged budget bounds no finish's latency. The 2026-11-01 entry predates carrying. The per-body form's costs (a stored spend on every fact; a far edit flipping a holding body; a refusal whose recourse is outside the decision's levers) decide it.
- **B** withdrew the budget per face pair again, this time on the tree: `work/clear/injectivity-price-on-tour-lofts-exceeds-the-pair-budget.md` measures a wall against itself as a frontier that is an area (the helix: 558 splits at one turn, 1,249 at two, unchanged from 33 to 65 stations; the Klein loop 48,162 splits on a few faces), so a self-pair's spend follows the surface and a per-pair wall gives a long wall one allowance and the same wall in two faces two. Spend is additive over the surface, which is what a pooled budget tracks. B adds a fact against the growing allowance as either wrote it: the engine's first reading is one root per face (`CellReceipt::candidates` is "the number of subdivision roots"), so "a multiple of one reading" would be a multiple of a face count; PR 6102's form is cut-independent only because PR 6092 laid round-zero cells per knot span. So form 2 waits on an engine reading of surfaces on cells of their own, with the constant meanwhile, its ceiling named (by face count, and by extent: a spring of roughly a hundred turns, extrapolated). It adds what a pooled growing allowance leaves possible: a finish that asks nothing can refuse. It names one measurement: the same wall asked as one face and as two across a seam.

The orchestrator checked B's two tree facts: `CellReceipt::candidates` is documented as candidate face pairs and "the number of subdivision roots", and the helix rows read as quoted.

Agreed: the core; that the budget bounds acceptance; that its unit is Ev's ruling. Not agreed: the lean (A: per face pair; B: per body, the constant now and a growing allowance once the engine reads surfaces on their own cells). Round 5 puts both round-4 reports before both designers at once.

Totals after round 4: A 202,576; B 306,772.
