# SYM-3 — what stands at a bulge that is not 1 (spec)

**Program:** SYM (`work/sym/plan.md`, the ceiling lane). **Item:**
`work/sym/rule-d-reaches-the-unit-bulge-only.md`, its second and third
asks — the RENDERED residual at `bulge = 2` before any rule is
proposed, and what `abs(b)` is for a parameter `b`. **Track:** a
MEASUREMENT unit; it moves nothing the tier decides, so it is outside
the A/B experiment: no ordinal, one review that reproduces the render,
no row. Any rule the measurement argues for is a later unit under the
full dual.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item; `crates/geom-core/src/sym/trig.rs`'s header (rule D — what folds
and what never does); `sym.rs`'s header sections "The arc family" and
"The form-level algebra (M10-10)"; the instrument
`m10_10_what_stands_rendered` in
`crates/editor-core/tests/m10_10_evidence_interval.rs` and the harness
`crates/editor-core/tests/m10_8_harness.rs` (`over_band_set`, `bound`,
`render_over_band`); the two fixtures — R1's circular-segment boss
(`segment_boss` in `m10_10_r1_probes_interval.rs`, a literal bulge 2)
and R2's D-tab (`d_tab` in `m10_10_r2_probes_interval.rs`, a bulge that
is a literal or a document parameter by flag).

## The question

M10-10's headline is a property of the UNIT bulge: every measured
document authors its arcs through `LoopProgram::Circle`/`CircleSplit`,
kernel bulge `1`, so `θ = 4·atan 1`, `abs(1)` folds under A0, and the
two spellings of the arc meet at every sample. Two documents measure
the limit and neither has been RENDERED:

- **literal `bulge = 2`** (the boss): `carrier_matches_mapped_source`
  6 of 54 still numeric, `carrier_on_surface_2` 27 of 90, ceiling
  unmoved by the algebra. Rule D folds the trig — `sin`/`cos` of
  `q·atan 2` are closed forms in `sqrt 5` — so what stands is
  something else, and the item's guess ("`atan|b|` against `atan b`
  and the sagitta forms in `b`") is a guess: A0 folds `abs(2)`, so the
  two `atan` atoms should already be one. Which atoms the residual
  still carries is the whole of what this unit finds out.
- **a PARAMETER bulge** (the D-tab with the flag on): ceiling
  `3.52e2·ε` with the algebra on and off alike — entirely outside the
  mechanism, which rule D's pins state as its honest limit:
  `atan|b|` against `atan b`, related by the turn SIGN, which no
  value-free rule reads.

## Phase 1 — the render

Make both fixtures reachable from ONE instrument. The evidence file's
`documents(tol)` list (or the harness, if that is where fixtures shared
by several files belong — say which and why) gains the boss and the
D-tab (both bulge flags), so `m10_10_what_stands_rendered` and the
over-band instruments take them by `CAD_M10_10_DOC`. Moving a fixture
is a move: the R1/R2 rows keep their names and their numbers.

Then, for each of the three documents (boss; D-tab literal; D-tab
parameter), at the nominal and at ceiling + δ (the harness's `bound`,
never a multiple), shipped rules and `without_the_algebra`:

1. the over-band SET and the per-predicate theorem/gated/registered/
   numeric split;
2. for every over-band or still-numeric identity-shaped predicate,
   the first blocked decision's EARLY form rendered with its atoms
   (`atoms_of`) and the DAG below it explained to enough levels to
   name the node the early walk froze or the atom it kept opaque —
   `CAD_M10_10_EXPLAIN`, raise it until the answer is visible;
3. the same at the three ε rows if the atoms differ by row (they
   should not; say so if they do not).

## Phase 2 — the diagnosis

Read the rendered forms and answer, in the item body, per document:

- **Which atoms stand.** For the boss: is it `abs(2)` unfolded
  somewhere A0 does not reach (an `abs` inside an `atan` argument
  before the constant fold runs?), the `sqrt 5` the closed forms
  mint against a different `sqrt` the sagitta spells (`sqrt(1 + b²)`
  vs the chord/apothem's own), a form past `EARLY_AB_TERMS` or the
  budget (a freeze — read `SymCounts::frozen` and the shape report's
  sizes), or the `MAX_MULTIPLE`/`MAX_HALVINGS` bounds in `trig.rs`
  (a `q` the fold refuses)? Each is a different next unit, and the
  render says which.
- **For the parameter bulge:** what the residual is as a form in `b`
  — write it out for one sample — and whether it is identically zero
  for `b > 0` given `abs(b) = b` (then the question is rule C's kind:
  a box that does not straddle zero decides the sign, `sign_gated`,
  the machinery already built and dial-off) or whether the sign enters
  elsewhere too (the turn, `arc_apex`'s `nhat`, the sagitta's `1/b`).
  Count how many decisions on the D-tab each of the item's two routes
  would touch — rule C's clause 3 over `abs(b)` alone, against a
  structural fact from the authoring door (the profile program knows
  the turn's sign: what would the door have to hand the tier, and is
  it a registration, `Sym::register_equal`, or a new atom kind?).
  Report both with their counts; **do not choose** — that is the
  orchestrator's decision, with Ev in view if the structural route
  changes what a constructor states.
- **Is the boss's residue an instance or a class?** Sweep the tour and
  the wild corpus for arcs authored with a bulge other than 1
  (`ArcTo(Bulge(..))`, `Fillet`, `CircleSplit` with a non-unit kernel
  bulge, revolve profiles) and say how many documents in the
  repository author a non-unit bulge, so the next unit knows whether
  it buys one document or a family.

## Phase 3 — the record

- The item body gains `## What stands (SYM-3)`: the rendered forms
  (trimmed to what argues; the full renders as a committed text
  fixture beside the row if they are long), the atom diagnosis, the
  two-route count for the parameter bulge, the corpus sweep with its
  pattern and its blind spot (implementer-discipline §5).
- The rows: the render rows stay `#[ignore]` evidence rows; ONE
  pinning row per document that reds if the per-predicate split at
  the nominal MOVES (the M10-10 pin shape), so the next unit's claim
  "this rule takes the boss's 6 and 27" has a before-number a test
  holds. Name the values it pins in the row's doc comment.
- `trig.rs`'s header: the "What folds, and what never does" section
  gains one sentence stating, in the present tense, what stands at a
  non-unit bulge with the instrument named — only if the render says
  something that section does not already say.

## Scope

- **No rule, no dial, no budget moves.** Rule C stays dial-off. The
  trig bounds stay. If a one-line fix is obvious after the render
  (e.g. A0 not reaching an `abs` under an `atan`), write it as the
  proposal, with the number it would move, and stop.
- **No new gating row beyond the pins**, which run at the nominal only
  (state their cost in the PR body: the boss's nominal replay is
  seconds). New editor-core rows go under `crates/editor-core/tests/m10_*`,
  subject-named (`m10_bulge_interval.rs`), registered in `tests/all.rs`.
- **Territory:** `sym/*` and `crates/editor-core/tests/m10_*` are this
  program's; the test glob is also S-TCOST's and S-TINT's (announced
  at dispatch); `profile`/`sweep` sources are not touched. Run
  `python3 scripts/work.py territory --base origin/main` before the PR.

## Acceptance

- The three documents rendered at the nominal and at ceiling + δ under
  both rule sets, with the atoms named per blocked predicate.
- A per-document diagnosis naming which of the four causes stands on
  the boss, with the render that shows it.
- The parameter-bulge residual written out for one sample; both routes
  counted; no choice made.
- The corpus sweep with its pattern and blind spot.
- One nominal pin per document; hosted CI green on the full matrix;
  the editor-core interval shard's wall within noise of `main`'s.

## Review

One reviewer, outside the experiment. Claims to falsify: (1) the
diagnosis — the reviewer re-runs the render on the boss and reads the
same atoms; (2) the two-route count on the D-tab; (3) the pins red on
a planted change (the reviewer flips one count and shows the row red);
plus `docs/prompts/reviewer-style-lane.md` in full. Fix pass on the
implementer's lane; the unit's log entry rides the PR last.

## Landing

Status `review` on `work/sym/SYM-3.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge. The item
stays open on its first ask's residue until a rule unit takes it or
the diagnosis says none can.
