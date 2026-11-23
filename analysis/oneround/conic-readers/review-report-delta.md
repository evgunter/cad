# Delta review: PR 7510 `oneround/conic-readers` at `32d065158` (reviewer r2)

This is the delta read of the fix pass (H1–H10) and the merge of main,
against my review of `bd07a867e7` (`review-report.md` beside this
file). It is on branch `oneround/conic-readers-review-r2`. The probes,
the mutant runner and the raw tables are in
`analysis/oneround/conic-readers/review-r2/delta/`.

I did not fetch, check out or read the other lane's branches,
scratchpads, CI artifacts or PR comments, and I glimpsed none of them.

## Verdict: APPROVE-WITH-FIXES

Both MAJORs of my first review are closed, and I could not break either
fix:

- the `Interval` extent along the conic's own normal (H1);
- the nearest-distance cancellation (H2).

The new piece reads no frame (H5). The census agrees with the tree to
within one read (H8). The merge dropped nothing of main's (9). Every
mutant I planted is red, including the five former under-covers (H3).

What remains:

- **The gate can still be passed.** There are five spellings that both
  clippy `-D warnings` and the gate accept for a new production read of
  the derived centre (MINOR D1).
- **H2's change to a centre-held reading** moves bits in both
  directions, not only "toward holding" (NOTE D2).

Neither is unsound in the kernel's lanes. D1 is a hole in the unit's
own guard.

## Executed and only read

EXECUTED, each tree on its own `CARGO_TARGET_DIR`:

- **Frozen head `32d065158`**, my probes added (`cfg(test)` only).
- **Main at the merge point `c9cf004f9`**, the same probes.
- **A spare head worktree** for the gate bypasses, the census and the
  mutants.

The runs:

- **H1 probes** (`delta_h1_normal_and_tilts`,
  `delta_h1_bracketed_realizations`):
  - 4 sizes (2 m to 2.2e7 m) × 3 ratios b/a × 9 windows × the normal,
    its negation, `v` and `u`, each tilted by 0 to 1e-3, 513 samples;
  - bracketed data (semi-axes and anchor widened by 1e-12, 1e-6 and
    1e-3 relative), four corner realizations each, judged by my own
    double-double oracle.
- **H2 probes**:
  - 180 fixed centre-held readings (`delta_h2_centre_held_far`);
  - 8,000 random ones (`delta_h2_random_far`: centre-held ellipses,
    sizes 1 m to 1e7 m, windows 1e-10 to 1 rad, a point and a line
    site);
  - both run on main and on the head and diffed by value.
- **19 mutants** against the 14 arc-held rows (theirs and mine); the
  survivors, plus one, against the whole `cargo nextest run -p geom`.
- **Gate bypasses**: `cargo clippy -p geom --lib -- -D warnings`, then
  `scripts/gates/conic-centre-reads.sh`.
- **The census, re-taken by clippy**: `--keep-going --workspace
  --all-targets --all-features`, the expectations turned to `warn`.
  On main at `c9cf004f9` I armed the five original doors the same way.
- **The differential**: `-p geom -p geom-brep -p topo`, filtered to
  `cone|cover|arc_ball|depth_gap|section|extent|aabb|held_arc`, main
  against head at ε 1e-9 (the default). The 1e-12 pair is in the table
  below.
- **The full `-p geom` on the head** at ε 1e-6, the default and 1e-12,
  then `scripts/run-gates.sh`, which is all OK (68).
- **Main's reds re-run on `c9cf004f9`**: the foot-seed row, and
  `one_loop_keeps_its_order_near_the_node` at 1e-6.
- **The merge**, checked line-level over the 43 files both sides
  changed.

ONLY READ:

- H6's row and `work/README.md`'s bands;
- H9's mints (`plane_cylinder_section`, `RigidMotion::squared`, the
  re-seams);
- the PR body's disposition of each item;
- hosted CI: not read.

## Findings

### MINOR D1 (H7). Five spellings pass both clippy `-D warnings` and the gate

**DEMONSTRATED, sure.** I added these to `crates/geom/src` at the
frozen head, as production code with no `#[expect]`, each reading
`Conic::frame(e).origin()` or `e.center()` of an `Ellipse`:

1. `#[allow(clippy :: style)]` (spaced path);
2. `#[allow(clippy::` then a newline, then `style)]` (the path split
   over two lines);
3. `#[allow(clippy` then a newline, then `::disallowed_methods)]` (the
   lint's own name split);
4. `#[allow(renamed_and_removed_lints, clippy::disallowed_method)]`
   (the lint's old name, which rustc still maps to it);
5. `#[path = "../../../tools/review/outside.rs"] mod outside;`, a
   module whose source, carrying
   `#[allow(clippy::disallowed_methods)]`, sits outside `crates/*/src`,
   which is all the gate scans.

`cargo clippy -p geom --lib -- -D warnings` is green, and
`conic-centre-reads.sh` is OK (1037 files scanned). Files:
`delta/gate_bypass_probe.rs`, `delta/gate_bypass_outside.rs`.

A `#[macro_export]` macro defined in `conic.rs` and expanded elsewhere
is caught.

This is the class my first review named: a line-based scan of
attribute text cannot hold what clippy's own lint levels hold. The
header's list of what the gate cannot see names none of these five.

### NOTE D2 (H2). `Arc::far`'s new bulge factor moves centre-held bounds both ways

**DEMONSTRATED, sure (numbers) / likely (that it is harmless).**

- **The 180 fixed readings** (circles and ellipses, 22 m to 2.2e7 m,
  five windows, four sites) did not move a bit: there the circle bound
  wins the `min`.
- **The 8,000 random readings** with short windows moved 1061
  farthest bounds: 523 rose and 538 fell, by up to 3.6e-9 relative.
  Taken alone, the move is at `u·a`, the centre's own rounding. The old
  `1 − cos h` carried a rounding of `a`; the new `2·sin²(h/2)` does
  not.
- **11 bounds that held the stored curve on main now fall under it**,
  by at most 1.26e-10 m (`a` = 1.3e6, `u·a` = 1.5e-10). Every one is
  under `u·a`, so no reading misses past the f64 lane's convention. No
  reading misses past 8 u of its scale on either tree.

So the change is neutral at the centre's rounding, but not "toward
holding the arc" as the PR body states.

- **No targeted row pins it.** Zeroing or halving `Arc::far`'s bulge
  (D17, D19) passes all 14 arc-held rows. It is red only in older
  cover rows (6 to 7 of them).
- **Verdicts**: no verdict moved in the filtered differential (table
  below).

### NOTE D3 (H1). The fix holds; its width ceiling is loose

**DEMONSTRATED, sure.**

- **No `Interval` support misses a stored point** in any configuration
  (worst `truth − hi` = −1.8e-15). On bracketed data, where `k`
  straddles zero, none misses a realization (worst −1.6e-12).
- **The width along the normal** is 2.3e-14 m at a = 2 and 4.3e-8 m at
  2.2e7. The worst over every tilt is 1.1e-7 of the arc's own size (it
  was 1e292 m at `bd07a867e7`).
- **The `A + |k|` clamp** never cuts a true value. Mutants that cut it
  are red: the clamp at `A`, the clamp at `max(A, |k|)`, the ratio arm
  alone, and the ratio halved.
- **The ceiling is weak.** `the_interval_support_along_the_arcs_own_normal_is_of_the_arcs_size`
  holds a flat 1e-6 m at every size. That is about eight orders over
  the true width at a = 2 and five at 2.2e4. A mutant widening every
  crest by a constant 4e-7 m (D18) passes that row; two other rows
  catch it.
- **A stale doc**: the row's doc still opens "**RED at the frozen
  head**" (`held_arc_rows.rs`, above the row), and it is green.

### NOTE D4 (H3). All mutants are red

**DEMONSTRATED, sure.** Mutants D01–D16 and D18 are each red on at
least one of the 14 arc-held rows. They include:

- my eight first-round survivors re-planted: bulge halved, a piece
  dropped, near without the bulge, far without the bulged corners,
  ball by the smaller semi-axis, `v_ref` flipped, the chord ball
  halved, support at the ends only;
- `bulge_factor` at `quarter/8`, and without its 2;
- six at `over_anchor`: the clamp dropped, the clamp at `A`, the clamp
  at `max(A, |k|)`, the ratio arm alone, the ratio halved, and the
  trough read as a crest;
- `bulge_factor` itself, through D17 and D19 on the centre-held path,
  which are red only on the whole `geom` suite (see D2).

### NOTE D5 (H9). The seam's squareness rests on one projection, which a near-parallel reference defeats

**BY-INSPECTION, unsure.**

- **`plane_cylinder_section`'s seam is `v × n`**
  (`tilted_section_major`): square to the axis to a few `u`.
- **The re-seams turn by quarter turns** with a crossed `v`: square to
  `u`.
- **But `Frame::new` → `RigidMotion::squared_about` is one
  Gram–Schmidt pass**: `u = normalize(u − a(a·u))`. It leaves
  `|u·a| ≈ u_round / sin∠(u_raw, a)`.

So any door handed a reference nearly along the axis stores a seam
that far off square. STEP's `ELLIPSE` `ref_direction` is such a door:
the file's own direction, which STEP forbids parallel but not nearly
parallel. To reach ε = 1e-9 at a = 1e3 m, the reference must be within
about 1e-4 rad of the axis: a pathological file, not a constructed
body. I did not measure it.

### NOTE D6 (9). The merge dropped nothing of main's

**DEMONSTRATED, sure.** This covers the 43 files both sides changed
since the merge base `c15b837eb`:

- **Every line main added is in the merge**, the unit's
  `frame().X()` → `X()` normalized.
- **Of the unit's added lines, 8 are absent.** All 8 are in code main
  itself rewrote: `torus_region.rs`'s converted function is gone on
  main, and two `ring_path.rs` sites changed with main's rework.
- **One direction read returns**: main's new code reads
  `p.circle.frame().axis()` (`ring_path.rs:1616`). It sits inside a
  listed function, so the gate holds it, but it is a direction read
  through a door that the unit's conversion had removed.

### NOTE D7 (H6). The looser cover on long windows

**DEMONSTRATED (no verdict moved) / BY-INSPECTION (band), unsure.**

- **No verdict moved** in the filtered differential (table below).
- **The band**: P2 is defensible as `work/README.md` words it ("work
  that matters soon but neither blocks authoring nor entrenches").
  `Arc` and `HeldArc` are two covers of one locus, and that is the P1
  shape ("two implementations of one underlying logic"). Ev's call.

## The claims, in brief

| # | claim | finding |
|---|---|---|
| 1 | H1 | holds (D3) |
| 2 | H2 | holds for the arc-held path; the centre-held move measured (D2) |
| 3 | H3 | holds (D4); the ceiling is loose (D3) |
| 4 | H5 | holds, see below |
| 5 | H6 | see D7 |
| 6 | H7 | falsified (D1) |
| 7 | H8 | holds to one read, see below |
| 8 | H9 | see D5 |
| 9 | the merge | holds (D6) |
| 10 | the main reds | confirmed main's, see below |

**H5**: `Piece` is private to `cover.rs`, and every match on it names
`HeldArc` (support, range, about, ball; `circle_range` through
`_ => whole()`, which reads support and range). The only `Arc`
construction outside `Cover::arc` is the tube's spine, not a conic of
the curve. `arc_of` reads no frame and carries no `#[expect]`.

**H8**:

- **The head** reads 207 times through the listed doors in 58 files:
  - 199 through the five original doors;
  - 8 through the new ones (`unit_in_plane` 3, `circle_at` 1,
    `Cover::arc` 4);
  - 144 `#[expect]` functions, equal to the pins (58 files,
    144 sites).
- **Main at the merge point** reads 335 times through the five
  original doors, in 62 files.
- **So 136 were removed, against the claimed 137.**
- **No direction read is left through the five doors**: all 72 + 50 +
  12 of base's kind are gone.

**10, the main reds**:

- **`a_foots_seed_is_read_only_where_it_seeds_a_search_or_a_bound` is
  red on a clean `c9cf004f9`**, with the same 11-entry listing that
  names `viewer/src/session/range.rs` and `pncad-py/src/py/range.rs`.
  It is main's. It is the one red of the head's full `-p geom` (663
  run) at ε 1e-6, the default and 1e-12.
- **`pair_section::one_loop_keeps_its_order_near_the_node` is red on
  `c9cf004f9` at ε 1e-6.**
- **`the_square_wall_arm…` is red on both trees** at the default ε
  (random seed, germ's row).

## Differential

`-p geom -p geom-brep -p topo -E 'test(/cone|cover|arc_ball|depth_gap|section|extent|aabb|held_arc/)'`:

| ε | main `c9cf004f9`: run / failed | head `32d065158`: run / failed |
|---|---|---|
| 1e-9 | 1086 / 2 (my old red probe, which main still fails; square_wall) | 1096 / 1 (square_wall) |
| 1e-12 | 1086 / 2 (my old red probe; square_wall) | 1096 / 1 (square_wall) |

No verdict moved, at either ε. The head runs 10 more tests: its own `held_arc` rows and my delta probes.

## Style (delta)

Questions exercised: Q1, Q2, Q3, Q4, Q6.

- **(Q3) sure**: the H1 ceiling is flat across six orders of size (D3).
  The H2 bulge factor on the centre-held path has no targeted row (D2).
- **(Q2) sure**: the stale "RED at the frozen head" doc (D3).
- **(Q1) likely**: `Arc::far` and `HeldArc::range` still each cut four
  pieces and fold corners. They now share `bulge_factor`, but the
  piece cut is written twice (`HeldArc::pieces`, and inline in
  `Arc::far`).
- **(Q6) sure**: H7's disclosed "no planted red runs clippy" is the
  gap D1 walks through. It is disclosed, but no row schedules it.
- **(Q4) likely**: a fix that closes a hand-written guard's hole adds
  more hand-written patterns, so the gate's `GROUP` regex is a new
  instance of a text scan standing for a type-resolved fact (D1).

Ratings:

- **idiom/structure 4**: `HeldArc` is its own piece, and `over_anchor`
  is one formula.
- **test quality 4**: every mutant red, but loose bars (D3) and no
  centre-held row for H2 (D2).
- **doc/comment honesty 4**: the PR body's dispositions match the tree
  except "toward holding the arc" (D2); one stale row doc (D3).
