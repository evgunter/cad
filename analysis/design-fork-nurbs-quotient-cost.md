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
- **A revised, 22:55Z.** A moves to B's dyadic tree, with refinements:
  - per-axis levels `(lu, iu, lv, iv)`, because windows are thin strips;
  - no depth cap, since the lazy keyed memo bounds itself;
  - the cover level chosen by an excess bound (`k = 2` ≤ 2w);
  - exact restriction kept out unless rung inflation is measured.
  - **Reasons:** the tree continues the existing cell fold as one mechanism, and it meets the piece clause literally (dyadic pieces, no rounded window-end ratios).
  - A asks for this to be read as convergence.
- **CROSSOVER.** Each designer moved to the other's previous position (B to exact restriction, A to the tree). Per protocol (Ev, 2026-09-30), round 2 follows: each designer gets the other's REVISED report and is asked what moved it and whether its own earlier argument is answered.

## Reconciliation round 2

- **B, 22:56Z.**
  - **The question beneath both** (sure, from the code): the memo's lifetime. `limb_three` calls `n.boxes()` once per rung, and `ChartedNurbs::boxes` builds a fresh `NurbsBoxes` each time. So the whole-cell memo (5001), and any tree hung off it, dies every rung, and every rung re-forms the products.
  - **Window geometry** (sure): one window per pcurve span; windows nest across rungs and never repeat exactly. A window-keyed cache would never hit; tree reuse needs a memo that outlives the rung.
  - **Final:** move the per-cell raised-quotient cache to the operand (`ChartedNurbs`), lifetime of the surface; read windows by exact restriction (leaning, likely); build no tree unless a measurement after the cache move calls for one.
  - **On A's points:** per-axis levels and no cap answer the strip and cliff objections (sure); the k=2 excess bound is right (likely); the piece clause does not decide (likely).
  - **Order:** cache move, then products-once with restriction, then measure; the tree only if still above 2×.
- **A, 22:57Z.**
  - **Final:** adopts B's revised final state, exact restriction, entirely. "Literal dyadic pieces" and "one fold" do not decide; tightness does (sure). A cell-anchored tree matches no reader's window frame: chart windows nest per span, boundary reads strip-dyadic pieces, the walk reads Krawczyk-dyadic boxes.
  - **Adds the same lifetime finding:** `NurbsBoxes` is rebuilt per rung in `certify` and per `partials` / `corner_class` call in `boundary`. Ownership fix: `ChartedNurbs` builds it at mint, and boundary's walker holds one.

## Outcome: CONVERGED, 2026-10-13 22:58Z. No Ev, no fork-log row (no `[ev]` PR).

**The final state:**
- An operand-lifetime `NurbsBoxes` and per-cell raised-quotient cache.
- Products formed once per cell, for both orders.
- Dyadic pieces by de Casteljau halving, retiring `piece_weights` / `Cut` / `combine` and the exactness ceiling.
- A window read by exact `restrict(a, b)` then 2×2 halving; one reading per window.
- `CellNet::cut`, `bezier_on`, `cut_axis` and `CellAxis::Bezier` retire; the cut/whole meet becomes a debug containment check.
- No tree; a window-keyed cache only if a measurement asks for one.

**Order:**
1. The lifetime fix.
2. First order on products-once with halving.
3. The cut becomes restriction.
4. Measure: time, rung counts and worst margins against main.

## Postscript (2026-10-14): implementation findings

- **The cut/whole meet stays; it does not become a containment `debug_assert`.** Build C measured step 3's restricted cut on geom-brep's SSI rows: 578,728 meets. In 1,072 of them (0.19%) the cut box falls outside the whole cell's box, by a median 0.14%, p90 77% and max 108% of the whole box's width. This is not rounding: a window's own 2×2 pieces are not inside the union of the whole cell's pieces. Both boxes are sound enclosures of the same window, so their meet is sound, and a containment assertion would be false. The cut is usually much tighter (median 4.6% of the whole box's width).
- **Restriction's tighter slope exposed a latent soundness bug on main.** `NurbsBoxes::rect_box` evaluates the midpoint over a point window in plain f64 on the f64 lane, so the result is not an enclosure. `edge_runs`' mean-value form decided a wrong sign on a wall touching its edge (`ssi::one_arc` touch row). Main's slope was wide enough to hide it. The fix lands first as its own PR, with step 3 on top.
- **The cut's restriction primitive (2026-10-14):** restricting the raised grids by the blossom pyramid (O(n³), n = 2p+1) made step 3 slower than step 2 (shelling 208 s vs 120–139 s). The textbook O(n²) two-split restriction stays within the converged design (one primitive) and wins on every row.
  - **Time:** shelling 126 s vs 139 s, r1 64 s vs 76 s.
  - **Parity:** rung counts and worst margins match step 2 exactly.
  - **Meets:** the restricted cut lies outside the whole-cell box in 0–0.37% of meets, against 69–88% for step 2's restricted-net-plus-products cut. That one is often far wider than the whole box and is mostly discarded by the meet.
- **Step 3 timing reopened (2026-10-14 13:02Z).** Against its own base (main with step 2's cut and #5059's compensated midpoint): r1 is 49 → 65 s (+33%) and shelling is 149 → 123 s (−17%), with rung counts identical. Restricting the raised (2p, 2q) grids costs about 1.6× step 2's restrict-then-form per cut on r1's walls. The earlier "(A) beats step 2" compared against a base without #5059. A postscript round was sent to both designers (A and B) with options (B), (A′), (D′) and open.

### Postscript round: B's answer (independent, received 13:03Z)
- **Recommends A″:** keep "the cut restricts the formed quotients", but restrict only what each reader reads.
  - Axis-lazy restriction: one-axis readers (`deriv_box`, `speed_sup`, r1's hot path) need N_d plus W², that is 4 of 7 channels.
  - A one-piece cut with no per-cut 2×2 halving; the meet with the whole cell's 2×2 reading stays.
  - Batched channels per line.
- **Rejects (B):** step 2's cut lies outside the whole box in 69–88% of meets, so it is cheap and mostly discarded. It also re-forms products per window.
- **Rejects (D′):** two recipes chosen by a cost threshold.
- **Measure before deciding,** in order: (1) axis-lazy, bit-identical; (2) plus the one-piece cut, checking rungs, margins and the flat-wall row at 1e-9; (3) plus batching. Skip "restrict un-raised, then raise", since it is about 1/(2p+1). Diagnostic: time the per-step `between` meet once with it off.
- **Falsifiers:**
  - With 1 and 2 applied, still slower than step 2 on r1 and no faster on shelling: then take (B) with `split_restrict`.
  - Variant 2 moves a rung or margin: then keep the 2×2 halving.
  - The cut is no longer dominant and the rows are still more than 2× pre-4954: then the remaining excess is elsewhere.

### Postscript round: A's answer (received 13:05Z) — NOT blind
- **Blindness breach, orchestrator's error.** I appended B's answer to this record (13:03Z) before A had answered, and A's brief told it to read this record. A disclosed reading it. The two answers are therefore not independent. Recorded as a protocol finding: append a postscript answer only after both are in.
- **Recommends the same direction as B:** keep step 3 ("the cut restricts the formed quotients"); reject (B) and (D′). Its reasons for (B) and (D′) match B's, plus one addition: (B)'s looseness is structural (differentiating after restriction amplifies by degree/width).
- **Its distinct lever:** the restriction *step's weight*. `Interval::between` is about 15 interval operations per step, built for the blossom door's general ratios. On a window ratio, a thin convex step (about 3 operations, clamped) should give about 5× on the restriction, preserving readings within rounding.
- **Order to measure:** (1) thin-ratio step, with meet statistics; (2) axis-lazy (same as B); (3) one restriction per window across doors, i.e. a window-reading object; (4) batching; (5) one-piece cut, last and only if needed, checked on rungs, margins and the flat wall.
- **Fallback if 1–4 still lose on r1:** restrict the cell's formed low-degree grids (A, W, A_u, W_u, A_v, W_v), never re-differentiate, and form the window's products from them. That is an orchestrator call, not Ev's.
- Both rows stay above 2× pre-4954 even with the cut free; the remainder is a separate row.

### Outcome of the postscript round: CONVERGED on direction (13:07Z)
- Keep step 3's semantics. (B) and (D′) are rejected by both.
- Measurement order, merging both answers:
  1. thin-ratio restriction step (A), with meet statistics;
  2. axis-lazy restriction (A, B);
  3. a window-reading object, restricting once across doors (A);
  4. batching (A, B);
  5. one-piece cut, only if the rows are still over target, checked on rungs, margins and the flat wall at 1e-9 (A, B).
- Falsifier fallbacks, in order: A's "restrict the formed low-degree grids", then (B) with `split_restrict`.
- No ratified text changes. No Ev, no fork-log row.
