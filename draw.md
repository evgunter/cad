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
