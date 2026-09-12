# S393 — the path sweep's start frame has a door; the copies go

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-12).** Binds
the implementer of unit `S393`; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/S393.md`.

## 0. The finding, and what the tree says against it

The row: `sweep_body`/`sweep_geometry` carry a start placement along the
path, but the STARTING frame — the plane through the path's start point
whose normal is the start tangent, in-plane axes off whichever world
axis is least parallel to it — is every caller's own, written out in
`crates/sweep/tests/common/mod.rs` (`normal_start_place`) and
`demos/tour/src/skinned.rs` (`normal_start_place`), the tour's copy
"stated" because "no public door hands it out — a façade crate cannot
link a dev-gated test home". The row asks what a door would be called,
where it lives, and whether a caller wanting a different roll is owed a
second door.

**The door exists.** `geom_core::linalg::frame::path_start_frame(origin,
tangent, tol) -> Result<Affine3<T>, FrameError>` is public, is the
recipe (local +Z is the tangent, roll from the reference ladder world +Z
then world +X), decides the tangent's length and each rung's off-axis
margin under the band with typed refusals, and is already bound into
Python as `place.path_start_frame` (`crates/pncad-py/src/py/place.rs`).
The façade reaches it as `pncad::geom_core::linalg::frame::path_start_frame`.
No Rust production caller uses it; the two copies do not know it is
there. So the unit is not "mint a door" but "the copies go onto the
door", and the row's class corrects from **M** to **E** in
`work/scalar/plan.md`'s table (the lane says so in the PR and edits the
table's class cell and its "what it is" cell in the same PR).

**The one semantic difference, which is the whole review.** The copies
pick the helper axis by a hard cone: `+Z` if `|n.z| < 0.9`, else `+X`.
The door takes `+Z` whenever `|Z × n̂|` decides definitely positive under
the linear band, and falls to `+X` only when the tangent is coincident
with or in-band of `+Z`. Outside the cone (`|n.z| < 0.9`) the two agree
bit for bit: same helper, `u = Z × n̂` normalized by the same divide,
`v = n̂ × u` by the same cross. Inside the cone (`0.9 ≤ |n.z| < 1 − band`)
the copy rolls off `+X` and the door rolls off `+Z`: a different frame,
a different section placement, a different swept body. Which fixtures
and tour scenes start inside the cone is the unit's first measurement.

## 1. What this unit delivers

- `crates/sweep/tests/common/mod.rs`: `normal_start_place` becomes a
  thin call to `path_start_frame(path.eval(lo), path.deriv(lo),
  Tol::witness())` — or is deleted and its eight call sites (`common`,
  `turning_orientation.rs`, `m8_14_long_turn_sweep.rs`) call the door
  directly, whichever leaves the suites reading as a user would write
  them; the `Result` is unwrapped at the test boundary with the refusal
  visible, never `.ok()`'d.
- `demos/tour/src/skinned.rs`: the copy goes; the scene calls the door
  through the façade. **The narration changes with it** — today it
  tells the user this recipe is "the first thing a real caller has to
  write", which is false. Rewrite it to what is true: the kernel hands
  the start frame out, decided under the band, and a caller who wants
  a different roll composes a rotation about the tangent onto it
  (`Mat3::rotation_about`) — that is the row's "second door" question,
  answered: no second door; roll is a composition. Demos are evidence
  (`memories/demo-purpose.md`, implementer-discipline §3): if calling
  the door through the façade is awkward — the module path, the `Tol`
  witness, the `Result` — the awkwardness is a library finding and goes
  in the PR body and on LIB's or PORT's slate, not smoothed over in the
  scene.
- `crates/sweep/src/skin.rs`, the `sweep_geometry`/`sweep_places` docs:
  where they say the start placement is the caller's, add the one
  sentence that names `path_start_frame` as where a caller gets it.
  Present tense only, no history.
- The `pcurve`/Python side needs nothing; note in the PR that the
  binding already exists, so Rust and Python callers now meet at one
  door.

## 2. The measurement, and what moves

Before changing anything, measure: for every corpus fixture and tour
scene that calls the recipe, is `|n̂.z|` at the path start inside
`[0.9, 1)`? Put the table in the PR body (fixture, start tangent,
`|n̂.z|`, helper the copy chose, helper the door chooses). For every
row where they differ, the swept body changes; that is the kernel's
answer replacing a test-local convention, so **re-baseline and say what
moved** — never restore the old frame by composing a roll to match the
cone. If nothing is inside the cone, say so, and the unit is bit-
identical by the argument in §0, which the PR states once.

## 3. The pin

- A row in `crates/geom-core` (or the sweep suite, where the door is
  used) that a start tangent with `|n̂.z|` = 0.95 gets the `+Z` rung from
  the door — the case the cone would have sent to `+X` — and that the
  frame is right-handed, its local +Z the unit tangent, its origin the
  path start. If such a row already exists in `frame.rs`'s tests, cite
  it instead of duplicating it.
- The existing sweep suites and the tour's committed renders/pins are
  the differential: green unchanged, or moved with the reason.
- A tour row (`demos/tour/tests/`) that the sweep cells call the door
  and not a local recipe — `grep`-shaped rows are documentation
  (implementer-discipline §2); prefer a row that fails if the scene's
  start frame is not the door's (build both, compare bits).

## 4. Sweep

The class: a hand-built frame whose +Z is a curve tangent. Sweep
`crates/*/src`, `crates/*/tests`, `demos/*/src`, `pncad-py` for
`cross(` pairs feeding `Affine3::from_frame`/`from_parts`/`Mat3::from_cols`
where one input is a `deriv(`/tangent, and for the `0.9` cone literal.
Hit list with a disposition per hit in the PR body; say what the pattern
could not match (a recipe routed through a helper that names neither
`cross` nor `deriv` at the call site).

## 5. Fence

This program claims no paths. This unit reaches `crates/sweep/tests/*`
(S-TCOST's and S-TINT's), `crates/sweep/src/skin.rs` docs only (BLEND's),
`demos/tour/src/skinned.rs` (no program's paths; the tour is CIW-gated
and rendered — check `.github/workflows` for the render-lane step the
scene rides). Announced by the orchestrator in `work/scalar/log.md` and
on the PR. Merge `origin/main` immediately before opening the PR; run
`python3 scripts/work.py territory --base origin/main` and put the
output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p sweep` at default features, and the tour:
`(cd demos/tour && cargo clippy --all-targets -- -D warnings && cargo
nextest run)` — the tour is outside `--workspace` and CI is the only
other thing that compiles it. Hosted CI proves the rest, including the
render lane; poll the run to conclusion in the foreground and report
the run id. Report ≤150 lines: the cone table of §2, what moved and
why, the narration as rewritten (quote it), the sweep hit list and
blind spot, deviations, findings filed outside the fence with row
names, the class correction to the plan table.
