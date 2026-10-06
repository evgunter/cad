You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust. The REACH orchestrator dispatched you. This brief is your complete instruction and it is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

**PR:** evgunter/cad#4123, branch `reach/split-gate-sphere-azimuth` (head `08cf6427e0`). You have explicit permission to commit and push to that branch, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Problem.** The head no longer compiles once merged with `origin/main`. CI's merge build fails:

```
error[E0433]: cannot find type `LoopBoundary` in this scope
  --> crates/topo/src/boolean/boxes.rs:1596:33   (and :1675)
```

Main changed how face boundaries are walked after this branch last merged it:
- `f2c43f5fbb` topo: review fixes on the face-boundary iterator;
- `3d3cb33e84` face boundary walks go through `face_boundary_linked`;
- `e22e81efe3` witnesses for the torn hops; named premise panics in `sphere_zone_reach`.

Read those commits and their PR and README text first.

**Do:**
1. Merge `origin/main` into the branch.
2. Port the branch's `sphere_window` (and anything else in `boxes.rs` and `census.rs` that walks a loop by `LoopBoundary`) to main's face-boundary API:
   - use `face_boundary_linked` or whatever main now requires, not a bare import of `LoopBoundary`;
   - honour main's torn-hop witnesses and named premise panics. Main's version of the code `sphere_window` replaced (`sphere_zone_reach`) carries named premise panics; keep their meaning in `sphere_window`, or their typed equivalent.
   - Keep the behaviour of the last fix pass unchanged: the strut-cap fallback, the pole level, the 16-ulp widening and the `SphereWindow` enum.
3. Check whether any gate in `scripts/gates/` now polices raw loop walks, and satisfy it.
4. Check the territory: main's commits came from another program, so the face-boundary work may own rows in `work/`. Update the citations that point at `boxes.rs` lines you move.

**Before pushing:**
- with `CARGO_INCREMENTAL=0` (check `df -h` first): `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).
- The known reds on main at 1e-6 are `pocket_ring_steep_ellipse` and `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`. Do not fix either.
- Re-run the last fix pass's mutants M1 and M2 on `a_caps_strut_does_not_bound_its_latitude_window`; both must still go red.

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. If CI is red for a reason other than the known reds, fix it and push again;
- add one line to the PR body's "Last fix pass" section naming the merge and the port;
- end your turn by stating the head sha, the CI result and the test counts.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write. Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.
