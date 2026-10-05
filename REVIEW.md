# Review of PR #4061: one position order and one run rule round a vertex

Frozen head `2c3f7072`, run as `b3be5e30` (head merged locally with main `ba4db916`, which carries
PR 4059's `pinch_runs_battery`; main has no topo change since the merge base). Release builds.

**Verdict: APPROVE-WITH-FIXES**. MAJOR 0 · MINOR 1 · NOTE 4.

## Claims

1. **No behaviour change: holds** (executed). Byte-identical bar `finished in`, main vs head:
   `pierce_runs_battery` 4 542 lines, `pinch_runs_battery` 3 030, both reflex batteries 1 158 each,
   13 `rc_wide` shards (0, 7, …, 77, 83 of 84) 486 each, and my `rv4061_corner_pairs_battery` 16 386
   (committed here): five corner pairs (notch343², w343·w330, w345·notch, w300·w270, w350·w200) × the
   84 grid directions + 7 axis/diagonal ones × six turns incl. 0, π/2, π (exact face ties), every op
   both orders. It reaches A-plans with n = 2/4/6/8 (5 238 / 15 228 / 7 920 / 648, instrumented).
2. **The fan branch's extra comparison cannot err: holds.**
   - By inspection: a fan has `lo.0 != hi.0`, because `run_fan` returns empty for `from == to`
     (`insert.rs:1856`; empty fan ⇒ strut, `:990`). So whenever one side of the between-test compares within one entry, the other
     compares across entries and answers `Some(true)`. The old four-way ladder maps case by case onto the
     new match, and `(None, Some(false))` / `(Some(false), None)` cannot arise.
   - Executed: an instrumented tree recomputes the old fan ladder beside the new one and asserts that
     they agree, that `lo.0 != hi.0`, and that neither impossible pair occurs (`review/pr4061/*.patch`).
     It ran green over every battery above, the full `sweep --test all` suite (2 096 passed) and topo's
     lib (1 405 passed).
   - Coverage (only `pinch_runs_battery` reaches `held_cut`): fan cuts 401 in the lo entry, 325 in
     the hi entry, 227 between, 3 393 outside; 5 319 strut readings; **no tie at all**, so the
     `tied_held` arms are equivalent by inspection only (NOTE 1).
3. **A's run value is unchanged: holds.** `positions` is taken after `rotate_left`, so A's pairs
   always sit at (2k, 2k+1); `run_order(n, 2k, 2k+1)` is `Some(None)` only at n = 2, else
   `Some(Some(false))`; B plans never touch `a_pos`. Executed: `a_run == (n > 2).then_some(false)`
   asserted over every plan of every battery, n = 2 to 8, nested B plans included.
4. **The kept spellings are justified: holds, with residue** (inspection).
   - `reconcile_pass`'s `run_degenerates` asks whether a direction can mint, not which one; that is fair.
   - `walk_faces_first`'s `strut_order` within an entry is fair too, but its agreement with `walks_after`
     is asserted only in prose (S4).
   - The sweep's blind spot: it could not match a between-test spelled on `precedes` (`holds_whole`,
     S3), the integer interval reading in `b_runs` (S1), or the backward first-entry walk inlined in
     `precedes` (S5). None of these is a behaviour defect.
5. **The rows can see a regression: partly falsified** (executed; `review/pr4061/mutants.py`).
   - Five mutants go red on the new unit rows: `walks_before` with its tie flipped, or ignoring its
     origin; `walk_run` with its interval flipped, all-forward, or with two survivors forced forward.
   - `precedes` reading from `p.0` goes red on 1 topo row and 4 sweep rows.
   - A's run with two survivors forced forward goes red on 2 join2 rows.
   - **`held_cut`'s fan origin set to 0 survives** (MINOR 1).

## Findings

- **MINOR 1: no committed row sees a regression in `held_cut`'s fan reading** (executed).
  - The mutant is `walks_before(secs, lo.0, …)` → `walks_before(secs, 0, …)` at `insert.rs:998`.
  - Every non-ignored row stays green: topo `boolean` (406) and sweep `join` (97).
  - `pinch_runs_battery` changes on 51 lines, all SOUND→`ClassificationInvariant "a vertex at a shared
     point is the In end of one null edge…"`. For example `i=0 j=3 k=2 ba U/I/S` and `i=1 j=5 k=2 ba *`.
  - The cause: committed rows reach `held_cut`'s fan branch only 26 times (2 held, 24 outside), never
    with a cut in an end entry. That is the branch this PR rewrote. The one battery that reaches it is
    `#[ignore]`d.
  - The new unit row `walks_before_reads_one_order_from_each_origin` tests the comparator, not the
    origin `held_cut` passes it.
- **NOTE 1: tie paths unwitnessed** (executed census). No battery and no row reaches
  `(None, _)` / `(_, None)` in `held_cut` (`insert.rs:1009-1010`). `tied_held`'s doc already says so for
  a fan (`insert.rs:1049`). Claim 2's equivalence there rests on reading.
- **NOTE 2: pre-existing, not this PR: the sweep oracle's `convex_volume` misreads some poses**
  (executed).
  - 154 `OK BAD` lines in my battery (t3p=true, the volume off from `want`) are identical on main.
    Example: `w343-w330 xy psi=0.7000 ab I` gives v=4.3178 against want=5.0916.
  - A pure-Python Monte Carlo over the same pieces (`review/pr4061/mc_common_volume.py`) gives
    common ≈ 4.329, so the kernel is right and the oracle (`join_pierce_runs_sweep.rs:138`) is wrong.
  - A `want` that can be wrong could also certify a wrong body. Worth an issue row.
- **NOTE 3: pre-existing: 287 bodies build with `validate_pseudomanifold` failing** (t3p=false, the
  volume right). They are identical on main, all at exact-tie turns ψ ∈ {0, π/2, π}. Example:
  `w343-w330 i=0 j=1 psi=0.0000 ab U`. Not investigated; worth an issue row.
- **NOTE 4: dispatch premise.** The PR's acceptance base (`f37a945f`) predates PR 4059, so its pinch
  run used a copied battery. I re-ran on the true main and got the same result.

## Style (all of Q1–Q8 exercised; for Q8, insert.rs 1–1900 read in parts)

- **S1 (Q6, likely)**: `insert.rs:459-505`, `1480-1500`.
  - The row owes "one position type … against **one fixed origin**", and names `precedes`' moving
    origin as the defect ("not a total order").
  - The PR turns the origin into a parameter: three origins remain, and `precedes`' origin still moves
    with `p` and is never checked against `q`'s sector.
  - Only "no single struct" is disclosed. The fixed-origin ask and "carry the nesting too: an interval
    of the walk" (`b_runs`' `spans`/`interval`/`holds` remain a separate integer reading) are neither
    disclosed nor scheduled.
- **S2 (Q7, likely)**: `insert.rs:991-999`.
  - `before` keeps a strut/fan split (`precedes` vs `walks_before(lo.0)`) that is behaviour-equivalent
    under the `he` gate at `:1004`.
  - Instrumented: over all 5 319 strut readings, the `lo.0` reading agreed with `precedes` every time.
  - So it is one origin more than needed, and the comment beside it ("either way, round the orbit from
    before `lo`") says as much.
- **S3 (Q1, unsure)**: `holds_whole` (`insert.rs:757-761`) is a second between-test on `precedes`, with
  its own tie semantics (`!= Some(false)`, both-None excluded), next to `held_cut`'s new one. One
  "between" concept, two spellings.
- **S4 (Q2, likely)**: three comments assert that `strut_order` and `walks_after` agree within an entry
  (`insert.rs:1479`, `1621-1626`, `1668-1669`). Nothing checks it. This is the "two spellings
  reconciled in prose" shape, and the PR adds one of the three (`:1479`).
- **S5 (Q1, sure)**: other places spell the same order.
  - "The physical sector's first entry" is walked backward inline in `precedes` (`:1490-1498`) and
    forward in `next_edge_bound` (`:1379`).
  - `zip.rs:342` `within` is `walks_before`'s cross-entry formula verbatim. It is deferred to PR 4057,
    which is disclosed.
  - Also look in `vtxfac.rs:586,654,769`.
- **S6 (Q1, sure)**: strut-ness is spelled `run_fan(..)?.is_empty()` at `insert.rs:395`, `726`, `884`,
  `964`, `990`. This predates the PR.
- **S7 (Q3, likely)**: `walk_run_is_adjacency_then_the_interval`'s "turned start" loop
  (`insert.rs:2170-2180`) asserts position pairs (2k+1, 2k+2) that `plan_null_pairs` never produces,
  since positions are taken after the turn. It restates arithmetic. Claim 3 itself lives only in the
  comment at `:344-345`, and no row asserts it.
- **S8 (Q5, likely)**: `walks_before`'s doc (`insert.rs:1073-1075`) says `held_cut` reads "from the
  run's first entry". For a strut it reads through `precedes`, which starts at the physical sector's
  first entry.
- **Q4 (sure)**: nothing cites the retired spellings (`then_some(false)`, `held_cut`'s `rel`) outside
  dated log rows.
- **Q8 (likely)**: `insert.rs` is 2 662 lines; ~12 order/hold predicates for one vertex remain.

## Probe artefacts (this branch)

`rv4061_corner_pairs_battery` (ignored) in `crates/sweep/tests/join_pierce_runs_sweep.rs`; under
`review/pr4061/`: the instrumentation patch, `mutants.py` and `mc_common_volume.py`.

REVIEW COMPLETE
