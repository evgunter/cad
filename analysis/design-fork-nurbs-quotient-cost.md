# Design fork: the rational quotient bound's per-call cost (NURBS)

- Blinding byte (/dev/urandom, drawn 2026-10-13 22:38Z): 35. Even means Opus is A; odd means Fable is A. Mapping: Fable=A, Opus=B.
- Protocol: `docs/DESIGN-FORK-PROTOCOL.md` at 5dedb0a41a.
- Dispatched concurrently with the same problem statement; reports are recorded below as delivered.
- Dispatched 2026-10-13 22:39Z:
  - A (Fable): session_01EABGnRgEgT4zcDBajFnzob;
  - B (Opus): session_01XTfwppDNHis1qRvXkKbiEZ.
  Both were given the same problem statement at 6f2656af.

## First reports (recorded as delivered, before any reconciliation)

- **B, 22:43Z.**
  - **Answer:** keep the ruling. Each span cell gets one memoised dyadic quotient tree: products are formed once at the root, and children come by exact de Casteljau halving. A window reads its cover of at most 3×3 nodes; there is a level cap. `CellNet::cut`, the per-piece restriction and the cut/whole meet all retire.
  - **Argument:** the cost is the order of operations (restrict, then multiply, per window), not the statement. The first and second orders already spell the statement two ways.
  - **Confidence:** likely; the design argument is sure, the speed is unmeasured.
  - **Framing:** rejected the brief's "the arithmetic is unavoidable" premise.
  - **Ratified text:** proposes changing none. NURBS's derivative module docs gain one paragraph.
  - **Rejects:** hull-first (two recipes; the recorded margin would depend on path), accepting the cost, and fewer pieces.
  - **Needs Ev:** no.
