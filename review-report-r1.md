# GUI-0 (PR #1094) — review R1

Frozen head reviewed: `f511a00` on `gui/gui-0-scaffold`. Local runs are unique
signal only; the pinned suites ride the PR gate, and **this verdict is
conditional on hosted green**.

**Verdict: APPROVE-WITH-FIXES.**

The unit does what its spec asked and does it well. The camera is a genuinely
typed, renderer-free layer-3 vocabulary; the scene is a real body through the
public doors; the tests pin real contracts and I confirmed by mutation that
they go red. The fixes below are one gate hole, two framing defects, and a
tested-path/shipped-path mismatch — none of them structural.

---

## 1. Findings

### MAJOR-1 — the new `pncad::tolerance::witness()` door is a general bypass of the `witness-not-ambient` gate, and nothing re-establishes the gate

`crates/pncad/src/tolerance.rs:78-92`, `scripts/gates/witness-not-ambient.sh:82`.

The gate matches a **string**: `grep -F 'Tol::witness'`. Its completeness rested
on that being the only spelling reachable from `crates/*/src`. This PR adds a
second, public, documented spelling in the façade every downstream crate already
depends on. Planted differential, run at the frozen head:

| planted in `crates/viewer/src/zzz_probe.rs` | gate |
|---|---|
| `pncad::tolerance::witness().eps()` | **OK: no kernel library code mints a tolerance witness (326 files)** |
| `Tol::witness().eps()` | ERROR, names the file and line |

Identical semantics, opposite verdicts. The exposure inside the scan is bounded
today (`viewer` and `pncad-py`'s non-FFI modules are the only `crates/*/src`
trees downstream of `pncad`), but `viewer` is precisely the crate that will grow
document evaluation at GUI-3, and the door is what the gate exists to prevent
from becoming ambient. The PR body's argument for the door — that the gate's
header sanctions `crates/pncad/src` as the entry-point home — is an **honest
reading of the script** (the header says the door's "whole job is to be the
place a program starts"); what is not stated is that the door's mechanism at the
*call site* in `crates/viewer/src/bin/viewer.rs` is not an exemption but a
spelling the grep cannot see. Had the bin written `Tol::witness()`, the gate
would have fired on that path.

Fix is a line: add `| grep -vE 'tolerance::witness'`-shaped coverage for the
second spelling outside `crates/pncad/src`, or take the alternative the PR body
itself names (widen the exclusion list to `crates/*/src/bin/`) and drop the
door. Either way it is the orchestrator's call — the PR says so — but the
choice should be taken knowing the gate is currently blind.

### MINOR-1 — `Camera::framing` / `fitted` silently returns a camera that does not contain the scene

`crates/viewer/src/camera.rs:283-296` (`fitted`), `:520-525` (`clamp_distance`).

`fitted` computes `distance = radius * 1.15 / sin(half)` and hands it to
`Camera::new`, which **clamps** it into `[0.05·r, 100·r]`. On a tall viewport the
horizontal half-angle binds and the required distance exceeds the band; the
clamp wins silently and `Ok` comes back. Measured on the spike's own plate:

| aspect | distance | worst \|ndc\| over the drawn vertices |
|---|---|---|
| 1.78 | 0.10901 | 0.5706 |
| 0.50 | 0.20571 | 0.8613 |
| 0.10 | 1.00803 | 0.8652 |
| **0.02** | **3.62767 (= max_distance)** | **1.1986 — outside the frustum** |

`Frame`'s own doc says it "back[s] off far enough that the bounding sphere fits";
`Camera` refuses rather than inventing state everywhere else. This is the one
place it neither fits nor refuses. Pinned as
`review_gui0_r1::framing_at_an_extreme_aspect_should_contain_or_refuse`
(`#[ignore]`d, RED at the frozen head — un-ignore it when this is fixed or the
contract is narrowed in prose).

### MINOR-2 — the app never feeds `CameraOp::Frame` a real aspect

`crates/viewer/src/app.rs:127` (`Camera::framing(&mesh.bounds(), 1.0)`) and
`:218-221` (the `Fit` button hardcodes `let aspect = 1.0;`), while
`viewport_ui` computes the true aspect ten lines away. For aspect ≥ 1 the
framing is unchanged (`half = min(half_v, half_h) = half_v`), so this is
invisible on a landscape window — which is exactly why it will not be found by
running it. On a viewport pane taller than it is wide, `Fit` under-fits and the
model is clipped horizontally. `CameraOp::Frame::aspect` is thus a knob the
shipped application never varies.

### MINOR-3 — the input-mapping test exercises a fold the application does not use

`crates/viewer/src/input.rs:196-215` (`map_stream`) against
`crates/viewer/src/app.rs:300-322` (the inline loop in `viewport_ui`).

There are three folds of one shape in this crate: `camera::fold`,
`input::map_stream`, and the hand-rolled loop in `viewport_ui`. The spec's
required "synthetic event stream folds to the expected operation sequence" test
drives `map_stream`; the application drives the third copy. They have already
diverged in behaviour — `ViewerApp::navigate` clears `status` on success and the
viewport's copy does not — so the tested path and the shipped path are different
code with different semantics. The spec's testability claim is satisfied by a
function `--features app` never calls.

### MINOR-4 — `DisplayTolerance::new`'s doc claims a condition it does not check

`crates/viewer/src/scene.rs:52-62`: "the same condition `mesh::tessellate`
refuses, checked at the door instead of four call sites later." Measured:
`DisplayTolerance::new(f64::MIN_POSITIVE)` is **accepted**, and `scene_of` then
returns `NotTessellated(ResolutionOverflow { count: inf })`. The door's
condition is strictly weaker than the one it names. Nothing breaks — the refusal
is still typed — but the sentence is false.

### NOTE-1 — the framing rows have a wide insensitivity band on the horizontal axis

Deleting `/ aspect` from `projection_matrix` leaves **both** shipped framing
rows green (`framing_puts_the_whole_scene_inside_the_frustum` and
`a_camera_framed_on_the_scene_contains_every_vertex`); only
`a_pan_keeps_the_point_under_the_cursor` catches it, incidentally. The plate is
small enough in x that a 1.78× horizontal stretch still lands inside. This is a
property of the fixture, so it is a **class**: any later "the scene fits" row
copying this shape inherits it. Covered by
`review_gui0_r1::the_projection_carries_the_field_of_view_and_the_aspect`.

### NOTE-2 — hand-written counts in the PR body, in the PR that removes one

The one-line `crates/bvh/tests/aggregator_headers.rs` edit is right and is the
file's own subject. The PR body then restates three counts the compiler knows,
and two are wrong: "Scene (7)" (there are 6 scene rows; the 26th test is
`every_suite_file_is_aggregated`), and "`viewer` makes it thirteen" — there are
**14** aggregating `crates/*/tests/all.rs` files, so the retired "twelve" was
already stale by one before this PR and the `>= 12` floor never noticed.
Separately the dependency arithmetic does not reconcile: 63 → 255 is +192, and
the census names +140.

### NOTE-3 — the run-level CI comparison is not step-for-step

Run 33092409948's `k-lint (gate)` job ran "compile and list every probe-gated
test target" + "K-telemetry probe sweep"; run 33094649160's ran "mesh budget
meter" instead. The two jobs the +2-minute claim rests on are unaffected, but
"whole run 833 s → 709 s" spans more than archive-step variance, and the PR
attributes the difference to that cause without checking it
(`memories/review-and-dependency-policy.md`: never enshrine a causal story you
have not checked).

### NOTE-4 — the left mouse button is bound to nothing

`InputMap::default()` binds orbit to Middle and pan to Secondary; Primary maps
to `None` with or without shift (measured from an outside consumer). A
maintainer's first interaction with the spike will be a left-drag that does
nothing.

---

## 2. Claims to falsify — disposition

1. **26 tests pin real contracts and can go red** — **SURVIVED, with one
   qualification.** 26 confirmed locally (13 camera + 6 input + 6 scene + the
   aggregation guard). Mutations: reversing winding in `fetch` → *the volume row
   RED at scene_build.rs:115*; dropping the fov factor in `world_per_px` → *pan
   row RED, "a 137 px drag moved the world point 330.7 px"*; halving the fov in
   `projection_matrix` → *both framing rows RED*. **Refuted for one case:**
   deleting `/ aspect` from `projection_matrix` leaves both framing rows green
   (NOTE-1).
2. **The scene is genuinely through the public API** — **SURVIVED.** `Doc` →
   `evaluate` → `product` → `tessellate` → `SceneMesh`; the only geometry
   literals are the document's own dimensions. No `Mesh`/`FacePatch`
   construction anywhere. Independently re-derived: bounds are 60×40×8 mm, the
   drawn surface is closed (edge pairing balanced at three δ), and no vertex
   sits inside the declared ⌀24 hole by more than the chordal sag.
3. **Typed, renderer-free layer-3 operations, no layer-2 leakage** —
   **SURVIVED.** `rg 'egui|wgpu|eframe|winit|bytemuck|arena'` over
   `camera.rs`/`input.rs`/`scene.rs` returns prose only. Type ascriptions for
   `apply`, `Camera::framing` and `InputMap::map` compile in an out-of-workspace
   consumer. **`fold` genuinely stops at the first refusal** — proven the way the
   shipped row cannot: `fold(-dolly, NaN-orbit)` → `NonPositiveDolly`,
   `fold(NaN-orbit, -dolly)` → `NotFinite { what: "yaw" }`. A 2.3M-operation
   randomised walk found no state violating the camera's documented invariants.
4. **The `app`-not-default argument** — **SURVIVED, verified at the step level.**
   `scripts/ci-filter.py --files crates/geom-core/src/lib.rs` → `TIER=closure`,
   `CARGO_SCOPE=… -p viewer`; same for `topo` and `mesh`. So `viewer` is in
   scope for kernel PRs and a default-on toolkit would compile in each scoped
   row. The new step **ran**: job 98596373276, step 7
   `clippy (viewer app feature - eframe + wgpu)`, `conclusion: success`,
   16:45:31→16:46:46 = **75 s**. (Nit: the manifest and workflow comments say
   "four `--workspace` jobs"; in tier `closure` those rows are `-p …`, not
   `--workspace`. The substance holds.)
5. **CI cost: +2 billed minutes, 0 on the critical path** — **SURVIVED.**
   Measured from the jobs API: run duration 833 000 ms → 709 000 ms;
   `build + archive (default)` 638 s → 580 s (critical path *shorter*);
   `clippy` 143 s → 218 s (+75, crossing 3→4 billed minutes);
   `rustfmt + rustdoc (gate)` → 316 s as claimed (4:10 → 5:16 crosses 5→6). Both
   `nextest archive` jobs and both test matrices untouched. See NOTE-3 for the
   one attribution I could not confirm.
6. **`witness()` adds no behaviour; the gate-header reading is honest** —
   **SURVIVED on both halves** (`pub fn witness() -> Tol { Tol::witness() }`;
   the header does say `crates/pncad/src` "is the place a program starts"). What
   the claim does not cover is MAJOR-1.
7. **Dependency discipline** — **SURVIVED.** crates.io, checked today:
   `egui_tiles` 0.17.0 released 2026-08-05 (22 d, ≥2 weeks ✓), 0.17.1 released
   2026-08-18 (9 d — correctly avoided), `Cargo.lock` pins 0.17.0 exactly;
   `egui_dock` newest is 0.21.1, licence **MIT** ✓; egui 0.36.1 2026-08-07
   (20 d). Census over the reachable non-dev linux graph: `self_cell` =
   `Apache-2.0 OR GPL-2.0-only` ✓, `epaint_default_fonts` =
   `(MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0` ✓, every package has a
   licence field, nothing copyleft-only. One `wgpu` major (30) in the lock.
8. **The wasm guard** — **SURVIVED, and the necessity confirmed directly.** The
   step is green on the PR run (job 98596373705, step 11). `viewer` does depend
   on `pncad` (cargo tree). And `cargo check -p viewer --target
   wasm32-unknown-unknown` here fails on exactly `getrandom`'s wasm_js gate — the
   same failure that put `pncad` on the exclusion list. Phrasing nit: "a leg that
   excludes the façade *cannot* include a crate sitting on top of it" is not a
   cargo rule (an excluded package is still built as a dependency); the real
   reason is the getrandom row, which is the same reason, better stated.
9. **No frame-to-frame widget state; no interior mutability** — **SURVIVED.**
   `rg 'RefCell|Cell<|Mutex|RwLock|UnsafeCell|OnceLock|AtomicU|static mut'` over
   `crates/viewer/src` returns exactly one line: `render_state.renderer.write()`
   in `ViewerApp::new`, which is egui's own lock reached through eframe's API,
   not state of ours. `ViewerApp`'s fields are `Doc`, `Tol`, `DisplayTolerance`,
   `Arc<SceneMesh>`, `u64`, `Camera`, `InputMap`, `Tree<Pane>`,
   `Option<String>` — no mirror of anything the document owns. The PR's own
   qualifier (the reading is worth little until GUI-3 edits) is honest and I
   agree with it.
10. **`cull_mode: None` documented at the site; winding still discriminates** —
    **SURVIVED.** `gpu.rs`'s module header carries the whole argument and the
    field carries `// See the module docs: both sides are drawn.` The volume
    assertion is mutation-proven to catch reversed winding (claim 1).

---

## 3. Style

Per `docs/prompts/reviewer-style-lane.md`. Questions exercised: **Q1, Q2, Q3,
Q4, Q5, Q6, Q7, Q8**. Q8 was taken on `camera.rs` (628 lines, the largest file)
and `app.rs` (450), both read end to end.

**S1 (Q1, `likely`)** — `crates/viewer/tests/camera_ops.rs:16-29` and
`crates/viewer/tests/input_mapping.rs:15-28`: `plate_bounds()` is byte-identical
in both files and `framed()` differs only in its aspect literal; the plate's
dimensions (`0.060`, `0.040`, `0.008`, `0.012`) are hand-copied from
`scene::plate_with_hole` into three test files with no sentence tying them to
their source. Change the spike's plate and two suites keep testing a box the
scene no longer has. This is a **class** — I would look at every future GUI-unit
suite that needs the spike's geometry, and `tests/all.rs`'s own header already
discusses where a shared `mod` helper lives.

**S2 (Q1, `sure`)** — `crates/viewer/src/app.rs:158-166` vs `:317-321`. Two
spellings of "apply one camera operation, record a refusal": the method
`ViewerApp::navigate`, and an inline `match camera::apply(...)` inside
`viewport_ui`. They have already drifted — the method clears `self.status` on
success, the inline copy does not, so a stale refusal message survives every
subsequent successful drag. Counting `input::map_stream` there are three folds of
this shape in one crate.

**S3 (Q7, `sure`)** — `crates/viewer/src/app.rs:419-428` and
`crates/viewer/src/gpu.rs:283-305`. Both write into fixed-size arrays through
`get_mut(..)` with indices that are statically in range, silently doing nothing
if a write misses. `clippy::indexing_slicing` is deliberately **off** in this
workspace (`Cargo.toml`'s lint note argues at length that an index panic is a
kernel bug, not an input-reachable failure), so nothing forced this shape. In
`gpu.rs` the silent-miss outcome is a partly-zeroed uniform block — a black or
unlit viewport with no error.

**S4 (Q2/Q5, `sure`)** — `crates/viewer/src/scene.rs:52-62`. See MINOR-4: the
doc names a condition (`mesh::tessellate`'s) that the door does not implement.
Comments that instruct other code to rely on them are the dangerous ones.

**S5 (Q7, `likely`)** — `crates/viewer/src/scene.rs:86-100` and `:150-156`:
`SceneError::EmptyMesh` is returned both for "no triangles" and for "a patch
index outside the shared position table", and the comment at the second site
says in as many words that it is "a broken mesh, not a display choice" — the
code knows the arm is misnamed. The same shape in `camera.rs`:
`CameraError::UnusableBounds` is what `fitted` and `projection_matrix` return
for a **non-positive or NaN aspect**, which is not a bounds problem; my own
suite had to assert `UnusableBounds` for "aspect 0" to pass, which is how I
noticed.

**S6 (Q2, `likely`)** — `crates/viewer/src/camera.rs:398-401`:
`far() = distance + scene_radius * 2.0`. The back of the bounding sphere is at
`distance + radius`; the `2.0` is unexplained slack in a file where every other
constant is a named `const` with a paragraph (`POLE_MARGIN`,
`NEAR_FLOOR_FACTOR`, `FRAMING_MARGIN`, …). The asymmetry is the tell.

**S7 (Q6 / `memories/demo-purpose.md`, `likely`)** —
`crates/viewer/src/scene.rs:226-231`: within one profile, `LoopProgram::polygon`
takes bare `(f64, f64)` tuples while `LoopProgram::Circle` takes
`Expr::literal(x, Dimension::Length)`. That is two ways to give a length, ten
lines apart, in the code the spike offers as "written the way a user would write
it". `demo-purpose` says awkwardness met while authoring is a **library finding**
to gap-comment at the site and record; this one is neither, though the author
plainly knows the discipline (the `minted` postcondition *is* carried as a value
with a comment saying why).

**S8 (Q7, `sure`)** — `crates/viewer/src/input.rs:110-118`. The Primary button
is bound to nothing, with or without shift. Middle-drag-to-orbit is defensible
CAD convention; leaving the most-clicked button inert in a spike whose
acceptance step is "a maintainer runs it" is a first-impression cost for no
gain.

**S9 (Q7, `unsure`)** — `crates/viewer/src/app.rs:288-300`. The loop pushes one
`Drag` per button `dragged_by` reports, each carrying the same
`response.drag_delta()`. Two buttons held together would apply that delta twice.
I had no display and could not exercise it.

**S10 (Q2/Q8, `sure`)** — the PR body restates counts the compiler knows and two
are wrong, in the same PR whose out-of-crate edit is *removing* such a count.
See NOTE-2. Treat as the class: the habit, not the three instances.

**S11 (Q6, `likely`)** — three disclosed items are recorded rather than
scheduled: "Flipping `default = [\"app\"]` later is a one-line change"; the
Ubuntu Font Licence row "Flagged, not decided here"; and "Turning culling on is a
one-line change for whoever first runs this on hardware." Protocol v5 wants an
issue number or a named unit for each. GUI-2 and GUI-5 are plausible homes for
two of them; none is named.

**S12 (Q3, `sure`)** — `crates/viewer/tests/camera_ops.rs:379-402`:
`a_fold_stops_at_the_first_refusal` asserts only that a `NonPositiveDolly` comes
back, which a fold that kept going and returned the last error would also
satisfy. The title claims more than the row can see.

**S13 (Q5, `likely`)** — `crates/viewer/src/scene.rs:198-201`: `bounds()` is
documented as "the scene's bounding box … what a camera frames against" but is
computed from `mesh.positions` — the tessellator's whole shared table — rather
than the corners actually emitted into `positions()`. Measured: they coincide on
this scene. The direction is the safe one (superset), but the doc claims
identity.

**S14 (Q7, `unsure`)** — `crates/viewer/src/gpu.rs:150`:
`write_mask: ColorWrites::COLOR` leaves the pane's alpha at whatever egui's clear
left. Invisible on an opaque surface; not obviously right on a transparent one.
No GPU here to settle it.

**S15 (Q4, `sure`)** — the gate-premise invalidation is MAJOR-1; recording it
here too because it is the archetype of this question: the premise lived in a
`grep -F` pattern rather than in a sentence, and nothing in the PR's process
reads a gate's *matching rule* when a new spelling is added.

---

## 4. What the end-to-end exercise revealed

There is no GPU and no display in this container, so **the eframe app itself was
never run**: `ViewerApp::new`, `eframe::App::ui`, every `pane_ui`,
`initial_layout` and all of `gpu.rs` are compiled and clippy-clean here and
never executed, exactly as the PR body says. For that surface the toolchain and
CI commands are the e2e equivalent, and I verified them at the step level rather
than by job-name green. A maintainer-side run remains owed.

What I *could* drive is the renderer-free half, and I drove it from **outside the
workspace** — a separate cargo project depending on `viewer` by path, which is
the truest statement of "is this API usable by a consumer". It is. Notes on
scope and ergonomics:

- The public surface is small and complete for what it claims: `Camera` +
  `CameraOp` + `apply`/`fold`, `InputMap` + `ViewportEvent` + `map`/`map_stream`,
  `plate_with_hole`/`product_body`/`scene_of` + `SceneMesh`. I wrote a
  nine-section consumer program against it without once reaching for a private
  item or a workaround.
- Refusals are pleasant to consume: every arm names the offending field and
  carries the value. `Unframeable(NotFinite { what: "aspect", value: NaN })`
  told me exactly what I had done wrong.
- The two rough edges a consumer meets first are **S8** (left-drag does nothing)
  and **MINOR-2/MINOR-1** (framing ignores the real aspect in the app, and
  under-fits silently at an extreme one). Both are about the *aspect* concept
  being present in the type but absent from the wiring.
- `pncad::tolerance::witness()` works from a crate outside the workspace, which
  is what made MAJOR-1 obvious: the door is ordinary public API, not a
  privileged one.

## 5. Code quality report

**Counts.** MAJOR 1 · MINOR 4 · NOTE 4 · Style 15.
Spec deviations: **5 reported** (`app` non-default; wasm-guard exclusion; the
`pncad::tolerance::witness` door; `cull_mode: None`; the `aggregator_headers`
prose edit) · **0 silent**. Everything the spec asks for is delivered or
disclosed; three of the five reported deviations are unscheduled pickups (S11).

**Idiom and structure — 4/5.** Module boundaries are the architecture: three
toolkit-free modules and two behind `app`, with the split enforced by the
feature rather than by convention, and `rg` over the renderer-free half finds no
toolkit type. Marked down for the three parallel event-folds (S2, MINOR-3) and
the defensive `get_mut` idiom (S3).

**Test quality — 4/5.** The tests pin the real contract, not the
implementation: the pan row asserts a *property* (a 137 px drag moves the world
point 137 px) and I confirmed it reds on a dropped fov factor with a diagnostic
message; the winding row states `mesh::FacePatch`'s contract as an enclosed
volume and reds on a reversed `fetch`. Marked down for the two rows that cannot
see what their titles claim (S12, NOTE-1) and for the tested-path/shipped-path
mismatch (MINOR-3).

**Doc and comment honesty — 4/5.** Unusually good: the PR body volunteers that
its own seam reading "does not reach the case §5 was worried about" and names
GUI-3 as where the measurement is actually taken, and the "What was NOT
exercised, exactly" section is exact. Marked down for three sentences that are
not true as written — `DisplayTolerance::new`'s "same condition" (MINOR-4),
`bounds()`'s claimed identity (S13), and the PR body's restated counts (NOTE-2)
— and for the gate-header reading that is honest about what it quotes while
leaving out what the door does to the gate (MAJOR-1).

## 6. Artifacts on this branch

- `crates/viewer/tests/review_gui0_r1.rs` — the independent consumer suite
  (8 rows + 1 `#[ignore]`d reporting row), wired into `tests/all.rs`. Two rows
  are counterexample searches with a **varying** seed logged unconditionally
  (`GUI0_R1_SEED` replays, `GUI0_R1_EFFORT` scales counts). Green at the frozen
  head; 34 tests in 0.06 s at effort 1, 5.6 s at effort 3000
  (≈2.3M camera operations, no counterexample). Verified to go red on the
  aspect and winding mutations.
- `review-probes/gui0-r1-consumer.rs` — the out-of-workspace consumer program
  (built against `viewer` by path from a separate cargo project; not a workspace
  member and not compiled by CI), with its recorded run in
  `review-probes/gui0-r1-consumer-output.txt`.

## 7. Lane isolation

No material from the concurrent review lane, `gui/gui-1-ray`, or
`docs/MODEL-AB-LOG.md` was fetched, opened, or seen. `git fetch` of the single
branch printed a list of `nightly/*` tags, which carry no lane content.

**Wall clock ≈ 105 min. Tokens ≈ 265k.**
