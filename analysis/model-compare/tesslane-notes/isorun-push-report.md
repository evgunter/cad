(iso-run-density implementer) PUSHED. Head: b448200999.

**Commits since a79bf304d5** (none carries a trailer or a session line; the author is the box's git config, as before):
1. cfacd1584f: the two-hole row builds every cell at every ε. Holes are 0.008·r; the column is read off the unit pipe; `try_less` and the skip are gone.
2. 638e764c03: one home for "no rows". The lane reads `lattice_rows` on every arm. Also the `trimmed_metric` citation and the rods issue, first pass.
3. d6c35cefd7: the `trimmed_metric` doc gives the 900-cell table split by δ.
4. a242c622b5: the crossing-rods doc (identity vs "no rows" vs the cure's cost); the pinch the identity does not cover; the cone issue's "1.2 to 223".
5. b448200999: the second reviewer's four items. Doc comments and `work/` only: `chart_table.rs` doc, `rowless_cylinders.rs` doc, the unit file's two-hole section, the rods issue.

**The second reviewer's four:**
1. Rods issue: the refusing face 1v1 is a plane, so the refusal is the planar lane's (`planar.rs`). Each reviewer's cells are listed separately, and the two runs disagree at θ 0.8 below δ 5e-2. The title now reads "0.5 to 0.9 … on a plane face". "Found by review, not re-measured here" stays.
2. `chorded_to_lattice` is not the same fact, so it stays by kind. It decides whether the chord pass bounds the boundary's chart steps by the arm's steps. That concerns boundary points, not the interior. It is read before any window is known, and main already had cylinder=false while cylinders still laid rows. Its doc now says this and that it is not `lattice_rows`' "no row".
3. The unit file and the row doc now say: "main refuses cells of the class at r = 10: 2 of the first probe, and 10 of 108 of a second that PR 7526's review ran (gap 1.5 to 1.6, two and three holes)".
4. Body lines are below.

**What ran on which tree:**
- On b448200999 (pushed, clean), all rc 0:
  - `work.py lint`; `run-gates.sh`
  - `cargo fmt --all --check && (cd benches && cargo fmt --all --check)`
  - `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`
  - `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`
  - `cargo clippy -p mesh --all-targets --features budget -- -D warnings`
  - `cargo test -p mesh --features budget`: 252 + 317 passed
  - `cargo check -p mesh --tests`
  - viewer app clippy
  - viewer slow rows: 13/13
  - sweep's meshing modules: 595/596. The one red is `bisector_nodes::the_near_node_poses_at_the_witness_band`, main's tracked red.
- On a242c622b5 (one below; code-identical, b448 is doc only): mesh 562/562 at 1e-9 and at 1e-6, plus the same short list, all green.
- Mesh at 1e-12 has not completed on either tree. On a242 I stopped it when your four items came in. The run of all three ε on b448 is in progress now, and I'll report each line as it lands.

**Body draft lines** (corrected against what the reviews read):
- Line 3 "a fact of the chart (`lattice_rows`)": add "— true since 638e764c03; on the head the reviews read (a79bf304d5) `tessellate_trimmed` also hard-coded the cylinder's empty rows, so a mutant in `lattice_rows` alone reddened only `chart_table`'s unit row. Now one mutant there reds the three rowless rows."
- Line 11 "`pole_side_columns` is not called there": replace with "every analytic arm runs `pole_side_columns` and `lattice_rows`; a cylinder has no singular point, so its column count comes back unchanged, and it lays no row."
- Line 10 metric: the sentence I sent at 13:4x (900 cells: 180 at δ 1e-3 refuse with the budget spent; 540 at δ/r 1e-2 mesh with 47 to 177 points more; 180 at δ/r 1e-1 with 0 to 33 more; 16 mesh and 16 sweep rows red).
- Line 39 main's two: "Main refuses cells of the class at r = 10 (2 of the first probe, 10 of 108 of a second the review ran, gap 1.5 to 1.6, two and three holes), all of which this head meshes."
- Two-hole row: "all 10 cells build and assert at ε 1e-9, 1e-6 and 1e-12 (holes 0.008·r wide, the column read off the unit pipe at δ/r); there is no skip." Drop any "five cells skipped" or "skips loudly".
- Line 54: "`crossing_rods_…` holds the identity, which is true of any triangulation whatever inserted the points (rows laid again pass it). 'No rows' is held by the rowless rows' p = 0 and the two-hole row's p = 4. The shared-side cure's cost here is measured (n 28/60/130 → 86/182/408 triangles, up to 2.2× the rowed face), pinned by nothing, and retires with PR 7573's unit."
- Line 80 rods: "rods of radius 1 and 0.999 crossing at θ 0.5 to 0.9, A ∖ B, refuse `Triangulation` on a PLANE face (the planar lane), on main and on this head alike; each reviewer's cells are in the issue; found by review, not re-measured here."
- The pre-push line: as "What ran on which tree" above. The clippy claims (all-features and budget) are true of b448200999.
- The cone issue range: "1.2 to 223 times".
