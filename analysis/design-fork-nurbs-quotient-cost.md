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
- **A, 22:50Z.**
  - **Answer:** keep the ruling. A cell's raised quotient is formed once, and every reading restricts it. `CellNet::cut` becomes `raised.restrict(window)`, an exact-window de Casteljau at the window ends; the re-differentiation and `CellAxis::Bezier` go. One restriction primitive, de Casteljau halving, forms the dyadic pieces and retires `piece_weights` / `Cut` and the exactness ceiling. A window is read once for every quantity.
  - **Argument:** restriction commutes with ring operations, so per-window work is a restriction. Today one quantity is formed by two recipes.
  - **Confidence:** sure on the premise; likely on the cost factors.
  - **Framing:** rejected "the levers are exhausted" as one level off.
  - **Ratified text:** proposes changing none in DESIGN/README. The 03:00Z ruling's "exact restriction weights" clause is reworded to "exact-parameter subdivision".
  - **Rejects:** hull-first, fewer pieces, accepting the cost, and the pair term.
  - **Needs Ev:** no.

## Comparison after both first reports

- **Converged:**
  - the diagnosis (form once, then restrict);
  - de Casteljau halving replacing the piece tables;
  - first and second order on one recipe;
  - retiring the cut-then-form path;
  - the same rejections (hull-first, fewer pieces, accepting the cost);
  - no Ev.
- **Differ on the window reading:**
  - B reads a memoised dyadic-tree cover of at most 3×3 nodes: multiplication-free, but it reads up to 3w;
  - A restricts to the exact window by de Casteljau at its ends: as tight as today's cut, with interval-by-f64 work per restriction and no memo across windows.
- **Reconciliation round 1:** each designer is shown the other's report.

## Reconciliation round 1

- **B revised, 22:52Z.** B moves to A's shape whole: exact restriction by de Casteljau at the window's ends, with dyadic pieces as `restrict` at t = ½. B withdraws the tree cover.
  - **Why:** the cover's 3w read costs tightness at the tube margin and on the flat wall; exact restriction needs no state, so it satisfies D9 by construction; the cover's memo would only add 20–50%.
  - **Subsumption:** the tree is this design plus a memo and snapping, which nothing calls for.
  - **Adds:** the whole-cell meet becomes a debug containment check rather than a mechanism, unless rounding is shown to matter.
  - **Confidence:** likely; subsumption sure.
