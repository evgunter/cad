# PROPS ONB-measure — the facts that decide the orthonormal-basis sign hull (evidence only)

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Linalg
interval honesty; the item is
`work/props/interval-orthonormal-basis-sign-hull.md`, read it in full
including §Sized; difficulty logged at spec: **S**). An evidence-only
unit: it changes NO production code, adds `#[ignore]`d instruments and
a corpus census whose numbers the orchestrator carries to Ev as an
`[ev]` ruling. Single style review, outside the A/B experiment. Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/onb-measure`, cut from `main`.

## The question the numbers decide

`Vec3::orthonormal_basis` (`crates/geom-core/src/linalg/vec.rs:405`)
takes `s = 1.copysign(n.z)`; at `Interval`, `copysign`'s zero-containing
arm (`crates/geom-core/src/interval.rs:361`) returns the two-sided hull,
so a wall whose normal has `n.z = 0` exactly stores `u_ref.z ∈ [−|n.x|, |n.x|]`.
The doc's stated reason for the hull is containment of an f64 replay
that sees `−0.0`. Two `Interval`-only fixes are on the table and one
fact chooses between them:

- **(c)** at a POINT enclosure of zero (`lo == hi == 0`), transfer the
  sign BIT of that zero — sound iff the backend preserves signed zeros
  at width zero through construction and through the arithmetic that
  produces the normal, so that the point replays the f64 program's own
  zero bit-for-bit. Zero f64 change.
- **(c′)** canonicalise at f64 — `copysign(1, n.z + 0)` (IEEE:
  `−0.0 + 0.0 = +0.0`) — and let the point-zero arm answer `+`. Moves
  f64 bits exactly on walls whose Newell normal has `z = −0.0` today.

## What to measure (each an `#[ignore]`d instrument with its corpus as literals, output quoted in the PR body)

1. **Signed zero through the backend.** In `crates/geom-core/tests/`
   (feature `interval`): does `Interval::from_f64(-0.0)` keep the bit
   (`lo().is_sign_negative()`, `hi()` likewise)? Does it survive
   `Interval` arithmetic that produces zero — `(−a)·[0,0]`,
   `[−0,−0] + [−0,−0]`, `[0,0] − [0,0]`, and the exact spelling of the
   Newell cross-sum in `crates/geom-brep/src/newell.rs:149-161` on a
   vertical wall's ring (replay that loop at `Interval` on a literal
   square wall) — and does `normalize()` keep it? Report a table: op,
   f64 result bit, `Interval` lo/hi bits. Say what inari (or whichever
   backend `interval.rs` wraps) documents about signed zeros, with the
   line cited.
2. **Corpus census of wall normals at f64.** For every planar face
   minted through `newell_plane` across the tour (`demos/tour`), the
   wild corpus (`demos/wild`), the STEP export fixtures'
   source bodies (`crates/step-export/tests`), and `editor-core`'s
   test corpus (`crates/editor-core/tests/corpus`), classify the stored
   normal's `z`: exactly `+0.0`, exactly `−0.0`, `|z| < 1e-12`
   nonzero, other. Count per corpus. The instrument may live where the
   corpus is reachable (an `#[ignore]`d test in `demos/tour` or
   `crates/editor-core/tests`); the count is the deliverable.
3. **Under (c′), which committed bytes move.** For each `−0.0` wall in
   (2) that is exported by a byte-golden STEP fixture
   (`crates/step-export/tests/fixtures/*.step`, `DIRECTION` records
   written from `u_ref`), name the fixture and the record. For
   `Datum::FaceFrame` (`crates/editor-core/src/node.rs:704-716`): does
   any committed document or test place a `FaceFrame` on a wall whose
   normal has `z = −0.0`? Cite or state none.
4. **What (c) buys at `Interval`.** Replay M10-5's 12-gon prism fixture
   (`crates/editor-core/tests/m10_5_r1_probes_interval.rs`, the pin at
   `:1066`) with a LOCAL, test-only copy of `orthonormal_basis` under
   rule (c) (a private function in the instrument, not an edit to
   `vec.rs`): for each wall, the stored `u_ref` width before and after,
   and whether `Surface::eval` over a halved `(u, v)` window then
   refines (the `refines` predicate at `clearance.rs:2207-2226`, called
   or copied). This is the payoff column.

## Posture

- No production code changes. Instruments are `#[ignore]`d, assert
  nothing, print; corpora as literals (the `cert3_evidence.rs` shape).
- No `CI-Config:` trailer; no `Co-Authored-By`; hosted CI green
  (instruments must compile in both lanes).
- Landing: the PR body is the deliverable — four tables and one
  paragraph per option stating what the numbers say; the item gets
  `pr:` on this branch; the spec is deleted at merge.

## Acceptance

Four instruments committed; four tables in the PR body with the corpus
sizes stated; the backend's signed-zero behaviour cited from its docs
AND measured; the (c′) byte-movement list complete or stated empty; the
(c) payoff measured on the prism.
