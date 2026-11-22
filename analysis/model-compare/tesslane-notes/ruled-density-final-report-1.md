(4c implementer) **REPORT, part 1 of 2. Final head: e760867739** (pushed).

**`mesh`-only digest, e760867739 against 8ad2d3fd0f.**
- `mesh` whole on both trees, every `tessellate` patch hashed, each tree in its own target dir (a `Compiling mesh` line in each).
- 58,154 patch keys in common: **every one byte-identical**. Nothing moved, no refusal was added, and no base key is missing.
- 102 keys are head-only, all from rows this span adds or extends:
  - the bored-ball row's 1.3 cells: 80;
  - `a_split_side_body_records_the_walks_decisions`: 12;
  - the D-prism row: 6;
  - the undecided-reach row: 4.
- That matches the commits: 4e82b645ea, 469c4b6c54, 826552a0e9, 48b6eed279 and e760867739 each said no mesh moves. The `FaceVerdict` refactor touches the rectangle lane and the CDT emit, and the digest confirms it changed no byte.
- Both trees passed `mesh` whole: base 543/543, head 546/546.

**The runs on this head:**
- gates rc=0; fmt clean; workspace clippy (all-features) clean; mesh rustdoc clean; work lint ok.
- `mesh` whole with the slow set: green at the default ε, 1e-6 and 1e-12 (after the inventory re-pin in e760867739).
- `test_binding_census` against a fresh wheel: 12/12.
- Viewer slow rows: 12/13. Sweep's meshing slow rows: 143/144. Both reds are main's: identical on 54e6a5c5b8 (`review_pick_r2`'s tally; `bisector_nodes`' Euler escalation in the boolean).
- Mutants: as sent; each red under its own rows. Cut never reds by `OverusedSide`; undecided-as-greater reds the band row.
- Apex band: no panic on any cell at three ε, nor in the 81-gap sweep.

**Not done or open:**
- `may_make` is sound only while Kε > a + r (≈ 6e-12 m at the box's edge). Every row's ε satisfies it. The admission floor does not guarantee it at ε_min at the box's edge, where the census refuses what slips. This is stated at the site; whether it wants a row is your call.
- The count of main's rule firing where `may_make` would exclude: not counted, as you said. The filed row says so.
- Worktrees `/home/user/wt-base` and `/home/user/wt-head` still carry the uncommitted digest hooks. Nothing of them is on the branch.

**Territory** (`work.py territory --base origin/main`, as of 8ad2d3fd0f; this span adds no new owner's file): 16 paths.
- Double claims: chords, lib, sizing, tessellate, trimmed, types, walk; tests all, errors, reach_probes, split_rulings.
- Owned elsewhere: `.config/nextest.toml` (green). The pncad-py files are now back to main's text, so their entries are moot.

Part 2 follows: the changed PR-body paragraphs. Your 18 November set stands for everything not listed.
