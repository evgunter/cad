# SYM-14 — the chain: four links joined with an angular error at each joint, its dispersion drawn like the plate's, and the certified lane measured on it (spec)

**Program:** SYM (`work/sym/plan.md`). **Requested by Ev, in chat,
2026-09-22 — P1 specifically requested:** *"a demo of error
propagation like with the two holed plate but it's a chain of 4ish
elements joined with some error in the angle at each join and so it
will be visibly be more and more dispersed going down the chain"*; and:
*"if it is possible, or will be possible after some of your planned
work lands, take it on as a unit"*. It is possible today on the
advisory lane — the lane that draws the plate's picture — through
public doors, so it is taken on here. What is NOT reachable today is the
CERTIFIED version of the same picture (an enclosure per link); that is
this unit's Phase 2 measurement, and its walls are filed at P1 with
Ev's request carried on them
(`work/sym/a-widened-rotation-angle-is-unmeasured-on-the-certified-lane`).
**Track:** protocol v7 **OUT** — a demo cell and a measurement, no
kernel change: OPUS implementer, one OPUS style review with a
correctness arm, no draw, no ordinal, no row. **Cost H.**

**Read first, in full:** `docs/prompts/implementer-discipline.md`;
`demos/tour/src/plate.rs` (the plate's DOCUMENT, authored once and read
by two cells — the shape this unit copies), `demos/tour/src/mcplate.rs`
(the density sheet: 512 `f64` replays, the built bodies read back, the
SVG written by the tour, the replay-vs-`monte_carlo` bit-equality
self-check), `demos/tour/src/tolerance.rs` (the certified narration,
behind the `interval` feature; its header on why there is no "analyse
this document" door and what bounds the plate's certifiable box),
`demos/tour/src/main.rs` (the MC block beside `walk_tour` and the
`interval`-gated tolerance block), `demos/render-mc.sh` and
`demos/hosted-render-guard.sh` (the publish lane; the header defers a
composer to "a second MC cell" — this is it); `crates/editor-core/src/node.rs`
(`Node::Transform` — rotation about an axis THROUGH THE WORLD ORIGIN,
then translation, `SlotId::RotationAngle` at `Dimension::Angle`;
`Datum::FaceFrame` and its `spin`, which is a twist about the face
normal and not a tilt; `Datum::Frame { u, v }` with `Expr::cos`/`sin`),
`crates/editor-core/src/eval/wire.rs` (the transform's semantics),
`crates/editor-core/src/analysis.rs` (`analyzed_box`: a parameter's
`Distribution` becomes the box at analysis time — the document carries
no interval), `crates/pncad/src/analysis.rs` (why the Monte-Carlo lane
is ungated), `crates/editor-core/tests/m10_derived_frame_interval.rs`
(the only widened `Transform` in the tree — a widened TRANSLATION with
`rotation_angle: ang(0.0)`; a widened rotation angle has never been
measured anywhere), `m10_derived_frame_tilted_interval.rs` (`stacked`:
n cubes each on a `FaceFrame` of the previous cap — the derived-frame
chain, measured at n = 2, and the walls named at its rows),
`docs/ERROR-DESIGN.md` (the plate worked example; E11's deferred
densities); the SYM rows
`derived-frame-placement-freezes-on-the-symbolic-lane`,
`a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`;
`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`;
`memories/review-and-dependency-policy.md`.

## The claim

A chain of four links, each joined to the previous by a joint whose
angle carries an error, disperses more and more down the chain: the
angular errors add and each link's lever arm turns the sum into
position. On the advisory lane (`f64` replays over the parameters'
distributions, `monte_carlo`, the sheet the tour draws itself) that
picture is reachable today through the same doors the plate uses.
On the certified lane (`Interval`, `Sym<Interval>`, the drive) the same
document puts interval `sin`/`cos` of a widened angle into a rotation,
which nothing in the tree has measured; the derived-frame family's
walls are live; and a drive over four stacked bodies is expensive. The
unit builds the document once, draws the advisory picture, and measures
the certified lane on the same document — what certifies, where the
first refusal is, what it costs — filing every wall rather than fixing
it here.

**Ratified and not re-litigated:** E11/E12 (`docs/ERROR-DESIGN.md`);
the plate cell's picture (its SVG stays byte-identical); the tier's
rules (a change to the tier is a SYM/DECIDE unit under v7, never a
side effect of a demo).

## Phase 1 — the document and the picture

1. **The document** (`demos/tour/src/chain.rs`, authored once, read by
   both cells as `plate.rs` is): four links — rectangular bars of one
   length `L` and one section, extruded from a sketch — joined end to
   end; joint `k` (k = 1..4, joint 1 at the base) carries a parameter
   `joint_k` of `Dimension::Angle`, nominal 0 (a straight chain) or a
   small common bend if the straight chain reads worse on the sheet
   (choose, and say why), with a `Distribution` (Normal at a stated σ,
   or Uniform ±; ONE choice, the same at every joint, with the reason).
   The placement door: `Node::Transform` composed so that joint `k`'s
   error moves links `k..4` — the transform rotates about an axis
   through the world origin and then translates, so either nest the
   transforms over the downstream sub-chain or place each link by a
   `Datum::Frame` whose axes carry `cos`/`sin` of the partial sum
   `joint_1 + … + joint_k` through `Expr`; say which and why (the one
   that expresses the kinematics AND keeps the certified lane's
   argument shortest). A `FaceFrame` chain (`stacked`'s shape) is the
   tier's harder family; measure it in Phase 2 only if the document
   builds through it cheaply. The measured quantity at the tip: the
   tip end-face centre's position against the base datum (its lateral
   deviation, or its distance) through `Node::measure` + an
   `Assertion` with a bound sized so the advisory numbers say
   something (a tolerance band the tip must stay in).
2. **The sheet** (`demos/tour/src/mcchain.rs`, or `mcplate.rs`
   generalised — one drawing module if the generalisation is small,
   two if it is not, with the reason): 512 `f64` replays as `mcplate`
   does (`sample_offsets` + `SetDocParamValue`), each replay's every
   link outline READ BACK FROM THE BUILT BODY (the planar faces'
   corners in the sketch plane — never recomputed from the parameter
   values, exactly as `mcplate` reads its cylinders back) drawn as a
   faint polygon, the tip's centre as the cloud, the nominal dashed;
   two panels (the whole chain; the tip zoomed) with the spread
   dimensioned per joint. The replay-vs-`monte_carlo` bit-equality
   self-check kept. Written by the tour beside the plate's sheet
   (`out/mc/chain-density.svg`); `demos/render-mc.sh` publishes both
   (the composer its header defers, or a sibling script — CIW's file:
   announce the seam). The plate's `plate-density.svg` byte-identical
   before and after.
3. **The numbers in the PR body**: per joint the tip cloud's spread
   (σ or the band's width) at links 1, 2, 3, 4 — the dispersion growing
   — and the advisory yield against the assertion's bound.

## Phase 2 — the certified lane, measured on the same document

Behind the `interval` feature (`tolerance.rs`'s shape, a sibling
narration or a section of it): `analyzed_box` → ONE leaf at `Interval`
and at `Sym<Interval>` at the nominal box, for chains of 1, 2, 3 and 4
links: certifies / refuses, the first refusing predicate BY NAME, the
leaf's cost and its `SymCounts`; the drive at a leaf budget ≤ 64 where
a leaf certifies; the widest box that certifies whole (the plate's
`CERTIFIABLE_FRACTION` analogue, MEASURED and named in `chain.rs` as
the plate's is). One table in the PR body and in the cell's header. If
the certified lane reaches the tip's assertion for the four-link chain
at any box, draw that box on the sheet per link beside the cloud — the
picture Ev asked for in full. Every wall — a refusal the tier does not
reach, a freeze, a cost above the plate's line — is FILED as a row on
SYM (`work.py new`, P1, "specifically requested by Ev, 2026-09-22" in
the body, refs this unit), and the unit does not fix it: the row
`a-widened-rotation-angle-is-unmeasured-on-the-certified-lane` is
answered by the table either way (closed if the lane reaches the tip;
otherwise it carries the walls' names and stays open at P1).

## Scope

- Files: `demos/tour/src/chain.rs` (new), `demos/tour/src/mcchain.rs`
  (new) or `mcplate.rs` generalised, `demos/tour/src/main.rs` (the
  `mod` lines and the two cell calls), `demos/tour/src/tolerance.rs`
  or a sibling for the certified narration, `demos/render-mc.sh`,
  `demos/renders-mc/chain-density.svg` (published), the `.github`
  render lane only if it enumerates sheets by name. Tests: the tour
  suite row that CI runs (`ci.yml`'s `demos tour suite`).
- No kernel change; no new dial; no new feature flag (the MC half is
  ungated as the plate's is; the certified half rides `interval`); the
  plate cell untouched.
- Territory: `demos/tour` is unowned; `demos/render-mc.sh` is CIW's
  (announce); the analysis lane (`analysis.rs`, `mc.rs`, `stackup.rs`)
  is PROPS' and is READ, not changed.

## Acceptance

1. `demos/renders-mc/chain-density.svg` published and visibly
   dispersing (the tip cloud wider than joint 1's; the per-joint
   spreads in the PR body, growing); the plate's sheet byte-identical.
2. The certified table (link count × scalar × certifies/refuses/first
   predicate/cost) in the PR body and the cell's header; every wall
   filed with Ev's request carried.
3. Local checks: `cargo fmt --all -- --check` (the tour is outside
   the workspace — run it in `demos/tour` too); clippy `-D warnings`
   on `demos/tour` at default and `--features interval`; the tour run
   (`cd demos/tour && cargo run --release -- ../out`, with and without
   `--features interval`); `demos/render-mc.sh` under the hosted guard's
   preview override (read `hosted-render-guard.sh`); `scripts/doc-gate.sh`;
   `python3 scripts/work.py lint`. The hosted matrix and render lane
   are the verification of record.

## Review

Protocol v7 OUT: OPUS implementer; one OPUS style review
(`docs/prompts/reviewer-style-lane.md`) with a correctness arm — the
sheet reads the built bodies, the self-check holds, the certified table
reproduces on the reviewer's box for the one-link and two-link rows;
no draw, no ordinal, no row.

## Landing

PR against `main`; the spec deleted at merge with its
`docs/DOC-LEDGER.md` entry; the unit closed with the result; the SYM
log carries it; the filed walls stay open at P1. Branch
`sym/14-chain-demo`.
