You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust, finishing an existing PR. The REACH orchestrator dispatched you and reviews and merges your work.

**PR:** evgunter/cad#4044, branch `reach/trimmed-sphere-escape` (head `4bd8f5a8b9`). It cuts a trimmed sphere face along its chart's meridian when a section circle lies wholly inside it and a plane face (`boolean::ops`, `apply_cut_ins`, `SphereCutIn`). Read the PR body (GitHub MCP `pull_request_read` method `get`), the unit item `work/reach/trimmed-sphere-group-escaping-through-a-plane-face-refuses.md`, and the two dual-review reports on `analysis/reach-dual/4044-r1` and `analysis/reach-dual/4044-r2` (review.md and probes/ on each).

This is the PR's LAST fix pass: the adjudicated union of both reviews. Neither raised a MAJOR. You have explicit permission to commit and push to `reach/trimmed-sphere-escape`, with merge commits only (never rebase or force-push). Do not merge the PR, and post no GitHub comments.

**Step 1: merge `origin/main` first.** It now contains PR 4046 (`topo::boolean::sphere_region`, which classifies trimmed sphere faces) and PR 4042 (one conic × quadric door). Resolve any conflict keeping both sides' behaviour.

**MINOR fixes:**
1. **Pole-to-pole span sign** (ops.rs ~3922-3924). When both ends are poles, or the `u_arc·to_hi` term is decided negative with the cross term decided zero, `atan2` returns ±π depending on a rounded zero. On −π, `mef` refuses with an untyped Euler `IntervalNotForward`.
   - Normalise `span` into (0, 2π) for a forward interval, or decide the cross-term sign.
   - No Euler refusal may escape a `CutIn`.
   - Pin it with the reviewer's pose: revolve wedge θ=4 at lat 0 / az −114.6°, centred. Every op must build to the closed form.
2. **Stale `cut.face` across several cut-ins on one face** (ops.rs ~3639/3655). After each `mef`, re-resolve which piece holds the next cut's circle (re-place it against both pieces, or a point-in-face test on its crossing).
   - Every order must build the same body, or every order must refuse the same typed way.
   - Pin it with two cut-ins on one face, in both orders: lens ∖ bricks at az 130°/50° and 50°/130°, and banded ∩ cube at 45° and 135°.
3. **The nearest-hit choice is unguarded** (ops.rs ~3820-3843). Add a built row whose half-meridian meets the boundary more than once on one side, for example a sphere∖box with an arc hit plus a pole beyond it. Keeping the farther hit (flip the comparisons at ~3827/3836) must turn it red.
4. **The "only R-loop verdicts cut" gate** (ops.rs ~3210).
   - The pole-strut row should now build with 4046 merged. Flip `a_slab_cutting_a_cap_off_a_pole_strut_carve_refuses_the_unplaced_witness` to a build against the measured closed forms: ∪ 36.30261762663585, ∩ 8.79155069153e-5, strut ∖ slab 0.30261762663585, slab ∖ strut 35.99991208449308, both directions. Rename it to say what it now holds.
   - Add a row that pins the R-loop gate without leaning on that row's refusal text. The "any verdict cuts" mutant must go red.

**Cheap doc fixes:**
- refusal_routes.rs ~965-972: move the extent-scan lever doc back above `EXTENT_LEVER`, and reword "stands clearly clear".
- ops.rs ~2963: the `SphereCutIn` doc should state the `u_ref` fallback (a point of the circle when the centre direction is polar).
- ops.rs ~3102-3106: the closure comment should say it returns the holding face and its refusal.
- ops.rs ~775-776: say that `apply_cut_ins` relies on `carve` preserving kept keys (splitting/finish.rs ~984-987).
- `CUT_LEVER` against the `section_meridian` refusal (ops.rs ~3680-3687): either broaden the lever to cover turning the plane, or give that refusal its own reason.
- PR body: ∪/∖ results carry the cut's meridian edges, so "the topology the z-tilted pose built" applies to the operand, not the result.

**File, don't fix** (`python3 scripts/work.py new …`, each with file:line citations):
- (a) One shared circle × plane `FirstHarmonic` door that these all route through, with an audit of its noise charge: `apply_cut_ins` (ops.rs ~3725-3760), `sphere_region::ray_roots` (from 4046), `geom_brep::circle_sphere_harmonic` and `conic_cylinder_harmonics`.
- (b) Refactor `apply_cut_ins`:
  - one loop-walk helper;
  - a type for the two-ends-on-one-arc order;
  - distinct names for the overloaded `bool_sphere_cut_{span,half,meridian}` rows.
- (c) Decide whether "circle holds a pole" (ops.rs ~3677-3680) is reachable. Add probes for the hole-inside-circle, two-loops and uncertified-roots refusals, or say why each is unreachable.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Before pushing:**
- with `CARGO_INCREMENTAL=0` (check `df -h` first): `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`). The known red on main is `pocket_ring_steep_ellipse` at 1e-6.

Show each new row red without its fix (name the mutant).

**Then:**
- update the PR body with a "Last fix pass" section listing each finding → change → pinning row, plus the mutant results;
- end your turn by stating the head sha and the test counts.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
