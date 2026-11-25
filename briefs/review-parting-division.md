# Review: PR 7783 (`heldrecords/parting-division`), frozen head `e7429ceda2`, base `heldrecords/split-on` (PR 7481) — single FULL review

**Probes:** only to `heldrecords/review-parting-division-<6 hex>`.

**Read first:**
- `CLAUDE.md`;
- `docs/prompts/reviewer-style-lane.md` IN FULL;
- the row `work/heldrecords/the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover.md`, which is on origin/main, not on this base;
- the PR body;
- `[topology.conventions]` and tier 3 in `crates/topo/README.md`.

**Review** `git diff origin/heldrecords/split-on...e7429ceda2`. The clone may be shallow, so run `git fetch --unshallow` first.

**The gate is red only from the stack, per the lane.** Confirm each red by running it on the base `80f11deb86`:
- lint: `payload-rung-sweep` NodeErrorClass;
- topo: `a_record_the_census_reads_in_the_band_refuses` at 1e-6 and 1e-12;
- the `msolve2` timing.

**What it builds** (in `boolean/slit.rs`):
1. **`CurvedFace` narrowed** to a slit or a zero-area ring left in a curved face. It is read on the final state: the kept and minted edges must bound two faces.
2. **New `EdgeInFaceGap::PartsTouch`.** A curved division leaving two pieces that touch only along the parting refuses, because tier 3′ cannot yet record a curved seat between two solids.
3. **A closed conic inside a ringless face divides it** (`divide_closed`, `disc_loop`). Plane closed rims also take this arm.
4. **Tangent arms on a curved face refuse `Tangent`.**
5. **A conic on a curved surface carries its samples' margin** (geom_brep `Variety::read_piece_on`; on a torus, the worse of that and `torus_between`). `ArcOnFace` cites it.

**Claims to falsify:**
1. **No wrong body ships.**
   - Every newly built witness is right: re-derive the volume and solid count for the ball √2 ∖ revolved keel (one lune; across both meridians), the revolved ball, the plane closed rims, and PR 5793's pierce rows. Check t3 and t3′ at three ε.
   - Attack the final-state read: find a parting sequence where an intermediate slit closes into something that is *not* two ordinary faces, or a final state wrongly read as two faces.
2. **`disc_loop` picks the right side.**
   - On a plane: toward the centre.
   - On a curved surface: "the side of the conic's plane away from the outline".
   - Attack: a conic near a pole, a conic on a torus (inner side), a conic whose plane cuts the outline, a face with a hole elsewhere.
3. **`PartsTouch` is not hiding a sound body that should ship.** Is the census gap real (`work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids`)? Is the gap narrow, firing only on two-piece results that touch along the parting?
4. **The margin (item 5) is a bound.** Check `read_piece_on`'s margin on sphere, cylinder, cone and torus against sampled distance. Check that `ArcOnFace` consumers use it.
5. **The moved list is complete and every move is right.**
   - Diff verdicts head vs base: topo, sweep and editor-core `ci`, plus the slow rows the PR touches, at three ε.
   - Built → refused must be empty.
   - Every renamed test keeps its old intent.
6. **Euler-op correctness of `divide_closed`** (`mev_null` across the fan, then `kemr`, then `mfkrh`): orientation, loop nesting, and a face that already has rings (should it refuse?).

**How to run:**
- Own `CARGO_TARGET_DIR` outside the checkout; per crate.
- Foreground rule: never end a turn with background work.

**Report** (≤150 lines):
- verdict;
- claims held or broken;
- MAJOR, MINOR, NOTE;
- `## Style`, each marked sure, likely or unsure;
- probe rows.

Send it by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere, and no model names in commits.
