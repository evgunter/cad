You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you and reviews and merges your work.

**PR:** evgunter/cad#4128, branch `reach/pierce-tangent-off-face` (head `4ec0d4b25c`). A root set whose door cannot settle is read against the FACE by `topo::boolean::carrier_touch` (`clusters`, `ball_off_face`, `edge_clear_of_ball`) and becomes `SpanVerdict::OffFace`. Read:
- the PR body (GitHub MCP `pull_request_read` method `get`);
- the unit item `work/reach/edge-tangent-to-a-curved-carrier-off-the-face-refuses-at-the-pierce.md`;
- the full review on `analysis/reach-review/4128` (`review.md` and `probes/`).

This is the PR's LAST fix pass. You have explicit permission to commit and push to `reach/pierce-tangent-off-face`, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1:** merge `origin/main` first, and resolve any conflict keeping both sides' behaviour. Main recently moved face-boundary walks to `face_boundary_linked` (`f2c43f5fbb`, `3d3cb33e84`). If `carrier_touch` walks a face's loops, port it to that API, and satisfy every gate.

**MAJOR (blocking): `carrier_touch` assumes an ellipse stores `major ≥ minor`** (`carrier_touch.rs:122`, `:363`).
- Nothing guarantees that order. The ellipse doc says readers take the semi-axes as magnitudes in either order (`crates/geom/src/curves.rs:156-161`), and STEP import stores them as written (`crates/step-import/src/entities.rs:939`).
- With them swapped:
  - `speed_bound` under-states arc length by `minor/major`, so both the Lipschitz bound and the second-order bound clear pieces that hold roots;
  - `edge_clear_of_ball`'s annulus `max(ρ−major, minor−ρ, 0)` reports a positive gap inside the true annulus.
- The reviewer's probe `a_swapped_ellipse_keeps_its_crossings` gets no cluster in 60/60 transversal double crossings. The ordered control keeps them in 60/60.
- **Fix:** route the speed and curvature bounds through the shared home, `geom_brep::implicit::Conic::speed_hi` / `speed_lo` (`implicit.rs:705`; also see `pcurve_cache::param_rate`, `pcurve_cache.rs:3618`). Do not write a fourth copy. Compute the annulus from `min/max(|major|, |minor|)`.
- **Rows:**
  - promote the reviewer's swapped-ellipse probe to a committed row. It must be red on the head;
  - add an elliptic-boundary-edge row for `edge_clear_of_ball` with swapped axes;
  - if you can build an end-to-end pose that reaches the swapped path through the public API (an ellipse stored minor-first, e.g. through STEP import or the curve constructor, whose conic door answers `Uncertain`), add it. If you cannot, say in the PR body why, and file an item naming the reachability.
- **Sweep:** grep the tree for every other reader of an ellipse's `major` and `minor` that assumes their order (bounds, speeds, annuli, boxes). Fix each in this PR if it is a soundness hole of the same shape; otherwise file it. List what you checked in the PR body.

**MINOR fixes:**
1. **The necessity paragraph** (`crates/geom-core/src/real.rs:1251-1266`, `scripts/gates/bounds-allowlist.sh:461-467`) argues from the wrong failure.
   - The reviewer propagated `Decide + CertifiedBounds` through 16 signatures and they all compile. The first real failure is `Dual<f64>: CertifiedEnclosure` (`crates/topo/tests/inside_out_operand.rs:103/141`): the public boolean is instantiated at `Dual`.
   - Restate the necessity from that failure.
   - Correct "every verdict it returns is a `Decide` call": `edge_clear_of_ball` returns `Ok(true)` on box non-overlap (`carrier_touch.rs:326-328`). That is a terminal `Bounds` grant in the disjointness direction (#571). Say so plainly.
2. **The rows' module doc** (`crates/sweep/tests/pierce_tangent_off_face.rs:11-16`) cites the removed `reduce::CarrierTouch`, and presents `√(2r·(zero+escalate))` as the ball radius. The kernel's radius is `ℓ + |d(m)| + escalate` (`carrier_touch.rs:89`); the two agree within 1.02–1.04 for these poses. Fix the doc so it states both.

**Coverage (Q3):** no row reaches an ellipse span, an elliptic boundary edge, the torus inner side, or a span midpoint on the torus axis. Add rows for each:
- promote the reviewer's e2e F and G (`probes/e2e_pierce_tangent.rs`) and `a_midpoint_on_the_torus_axis`;
- an "ellipse speed = minor" mutant must turn a row red.

**Style:**
- `distance` (`carrier_touch.rs:129`) duplicates `carrier_eq::distance_to` (`carrier_eq.rs:1075`). Use the existing one if it gives the same signed distance; otherwise cite it and say why there are two.
- `PIECE_BUDGET`, the floor `√(κ·escalate)/8` and `CLUSTER_BUDGET = 8`: give each a one-line derivation at its site, or tie it to the sibling budgets (`circle_roots::SUBDIVISION_BUDGET`, `spiric_arc::MAX_PIECES`).
- Q4: in the ON-endpoint arms (`reduce.rs:2290`, `:2347`), check whether `vertex_on_curved_face` can answer In or On for an end that `OffFace` cleared. If it can, make the disagreement refuse typed. If it cannot, say why in a comment.

**File, don't fix** (`python3 scripts/work.py new …`, each with file:line citations):
- (a) NOTE-1: circle × torus and ellipse × torus touches pass the pierce and then refuse `FallbackExtentUnsupported`, so "fixed" is not observable for them. Cite the reviewer's two poses.
- (b) NOTE-2: the four new decisions have uncalibrated margins and 0 K samples. Name the rows here as probe candidates.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages.

**Before pushing:**
- with `CARGO_INCREMENTAL=0` (check `df -h` first): `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p geom-core -p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).
- The known reds on main at 1e-6 are `pocket_ring_steep_ellipse` and `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates`. Do not fix either.

Show each new row red without its fix (name the mutant).

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. If CI is red for a reason other than the known reds, fix it and push again;
- update the PR body with a "Last fix pass" section listing each finding → change → pinning row, plus the mutant results;
- end your turn by stating the head sha, the CI result and the test counts.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
