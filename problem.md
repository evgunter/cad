# Problem: a crossing pair at a vertex other pairs share, with no clear way round

You are a designer lane on evgunter/cad, a B-rep CAD kernel written in Rust. Read `docs/prompts/designer.md` IN FULL first: it binds you. Also read `CLAUDE.md`.

**Setup.**
- Clone with `git clone /home/user/cad <your dir>/cad`. In the clone, run `git fetch https://github.com/evgunter/cad.git gauge/shared-vertex-crossings && git checkout --detach FETCH_HEAD`. That branch is recent main plus the row's measured section.
- Your directory and your `CARGO_TARGET_DIR` are given in your dispatch line. Build only into that target directory.
- Wrap heavy cargo in `local-scripts/with-build-slot.sh -- <cmd>`, with `CARGO_INCREMENTAL=0`. Release builds can run out of memory, so use `CARGO_BUILD_JOBS=2` for them.
- Never background builds or arm waiters, and never end your turn with background work running.
- The container can restart without warning. Keep notes in your report draft as you go.
- Do not push, do not comment on GitHub, and do not edit the main checkout.
- Delete your target directory when you are done.

## The ground

- **The row.** `work/gauge/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md` (P1). Read it in full, above all its `## Read (GAUGE, lane gauge-svc, 2026-11-11)` section: the measured count, the classes, the reading and the experiment. That section ends with numbered options. An implementer wrote them before anyone weighed the question. They are hypotheses, not candidates you must choose between: the options are yours to find, and that includes questioning the row's framing.
- **The witness.** `work/cleavetail/three-corners-alternating-round-a-corner-refuse-at-the-join.md` and its test `three_corners_alternating_round_the_cube_refuse_three_ops`.
- **The battery.** `join_pierce_runs_sweep::four_pairs_battery` in `crates/sweep/tests/`. Replay: `cargo test -p sweep --release --test all four_pairs_battery -- --ignored --nocapture`.
- **The lane's probes**, uncommitted, if you want to re-run them:
  - `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/gauge-svc-probe-and-retry.patch`;
  - the classifier, `gauge-svc-classify.py`, in the same directory;
  - logs `gauge-svc-battery.log` and `gauge-svc-retry.log`.
- **The code.** `crates/topo/src/boolean/insert.rs`: its module docs state the insertion contract. Read `reconcile_shared`, `reconcile_pass`, `hang_at_shared`, `tied_held`, `holds_whole`, `pairing_start_turns` and `mint_directed`. Also read `finish::setopfinish`'s seam correspondence, which raises the `JoinDesync` the experiment hit.
- **Who governs the area.** The topo README clauses on the boolean's insertion and the finish, and `docs/DESIGN.md`'s boolean and coincidence sections. Check the provenance of any clause you lean on, as `designer.md` §2 says.

## The case

When several crossing pairs of the two operands meet at one vertex of a shared corner, each pair must take runs round that vertex. Today the pairing start is chosen from A's kept side, for the finish. With that start, 982 of the battery's poses, and the three cleave ops, have a pair whose runs have no way round clear of the other pairs' cuts, and the boolean refuses `SharedVertexCrossings`. The row's reading says the refusal is true of the plan taken, not that the pose has no body: the oracle's volume is defined.

## What needs deciding

- What should these ops produce, and what is the fault behind the refusal, if there is one?
- How should a pair be paired, and how should its runs be placed, at a vertex that other crossing pairs share? What, if anything, should change in the insertion contract `insert.rs` states, and in the finish that consumes it?
- Is the typed refusal an acceptable final state for any of the two classes (laminar and crossing), and why?
- What does your answer do to the poses that build today, and to the `JoinDesync` the experiment hit?
