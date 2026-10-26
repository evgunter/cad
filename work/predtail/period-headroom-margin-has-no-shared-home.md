---
id: period-headroom-margin-has-no-shared-home
kind: issue
title: The period-headroom margin Margin::levered(T::tau() - span, lever) is spelled at ~18 sites across 6 crates under ~12 predicate names with no shared home
status: closed
opened: 2026-09-06
priority: P1
cost: H
branch: pred/period-turn-sites
pr: 5710
closed: 2026-10-25
---


The decide "this span does not definitely exceed one period" —
`Margin::levered(T::tau() - span, lever)` classified at the linear
band, `Zero | Positive` admit, `Negative` refuses typed, the ambiguity
band escalates — is certification's `interval_span_winding`
(`crates/geom-brep/src/certify.rs:1496`), and it is re-spelled by
hand wherever a span is read that certification did not bound: a
reconstructed span, a chart-inverted pair, a hand-built loop, a
boolean trim window. Every re-spelling carries its own predicate name,
its own `Margin::levered(...)` expression and its own `match` on the
three dispositions, and nothing ties them together: a change to the
shape at one site (the lever, the sign convention, which of `Zero` /
`Positive` admits) is invisible to the others. MESH-12 (PR 1617)
added two more of them for the sphere parse and, in its fix pass, a
third for the forward half `0 < Δt` — which is the second half of
the same certification premise, `interval_span_forward`
(`certify.rs:1488`). Of the sites below, `certify.rs`,
`props/curved.rs:1639` and `fold_chain` (`props/curved.rs:2288`)
decide both halves; whether each of the others does was not read for
this filing, and at any that does not, a reversed span reads as
headroom `τ + |Δt|`, definitely `Positive`, and is admitted — the
defect MESH-12's fix pass closed at the sphere parse.

The sites, each line read at this head (PR 1617's landing merge `267e2510f`, merge base `522e8840a`,
PR 1617's fix-pass tree), with its predicate name:

| site | name | lever |
|---|---|---|
| `crates/geom-brep/src/certify.rs:1496` | `interval_span_winding` (circle) | carrier radius |
| `crates/geom-brep/src/certify.rs:1515` | `interval_span_winding` (ellipse) | minor semi-axis |
| `crates/geom-brep/src/pcurve_cache.rs:2658` | `pcurve_azimuth_period` | `arm` |
| `crates/geom-brep/src/pcurve_cache.rs:3363` | `pcurve_azimuth_period` | `u_arm` |
| `crates/geom-brep/src/props/curved.rs:1650` | `props_meridian_span_winding` | sphere radius |
| `crates/geom-brep/src/props/curved.rs:2299` | `props_meridian_pieces_winding` | torus minor radius |
| `crates/topo/src/boolean/reduce.rs:2120` | `bool_split_span_period` | radius |
| `crates/topo/src/boolean/solid_contain.rs:892` | `bool_cone_trim_period` | `max(|v0|, |v1|)` |
| `crates/topo/src/boolean/solid_contain.rs:1118` | closure over `name` (two callers) | `lever` |
| `crates/topo/src/boolean/solid_contain.rs:1567` | `bool_wall_trim_period` | `lever` |
| `crates/topo/src/boolean/solid_contain.rs:1873` | `bool_sphere_trim_meridian_span` | radius |
| `crates/topo/src/boolean/solid_contain.rs:1966` | `bool_sphere_trim_period` | radius |
| `crates/topo/src/boolean/contain.rs:498` | `bool_contact_arc_span` | radius |
| `crates/topo/src/boolean/contain.rs:639` | `bool_curved_contain_period` | radius |
| `crates/sweep/src/revolve/mod.rs:707` | `revolve_angle_headroom` | `r_max` |
| `crates/sweep/src/revolve/tube.rs:434` | `tube_window_headroom` | `arm` |
| `crates/topo/src/chart_region.rs:1621` | `chart_region_cyl_wrap` (spelled as the excess `span − τ`, sign reversed) | radius |
| `crates/topo/src/chart_region.rs:2741` | `chart_region_seam_span` (the excess form again) | radius |

Eighteen sites in ten files across three crates (`geom-brep`, `topo`,
`sweep`; the two reviews that found the class counted six, by module).
Beside them, not decides but the same quantity read raw:
`crates/sweep/src/blend/surgery.rs:2346` compares
`(T::tau() - (st1 - st0)).lo() <= 0.0` on an interval directly, and
`crates/topo/src/chord_join.rs:2849` halves `TAU - width` in `f64`.
The sweep that produced this list matched the spelling
`tau() -` / `- T::tau()` / `TAU -` in non-test sources; a headroom
formed through a named helper or as `period - span` with a local
`period` binding would be missed.

What a shared home would be: one function on `Decide` — the span, the
lever, the band, a predicate name — returning the three-way
disposition (or the typed `Result` with the caller's error), with the
forward half decided in the same call so a site cannot take one half
without the other; `predicate-dimension-audit.md` then documents the
helper's key once and each site's row cites it. The names stay per
site (they are what the funnel records), the arithmetic does not. The
MESH-12 fix pass is the immediate evidence for the cost of the status
quo: the forward half was missed at the sphere parse because the
winding half was copied without it, and the same omission stands at
the twelve `bool_*` / `chart_region_*` / `*_headroom` sites above
until each is read.

## Re-homed to PRED (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PRED collects the rows where one numeric fact is decided in several
places, each with its own margin. This row is one of them.

Its class at the cut was **H** — eighteen sites, ten files, three
crates; new `Decide` API plus latent defects. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.

## PCERT B2a (PR 5244, 2026-10-17)

The two `pcurve_cache.rs` rows of the table are gone from the
closed-form lanes: the harmonic lane's with B2a's stored windings, and
the cone-section and spiric-wall lanes' in its fix pass (each restated
the edge certificate's `interval_span_winding` on a carrier it already
bounds). One `pcurve_azimuth_period` spelling is left, in
`pcurve_cache/projected.rs`'s check 2, beside its `TubePeriod` twin.

## Moved by CHART (2026-10-24)

Branch `chart/ringed-curved-doors` deletes three of the sites above with
the cylinder and cone chart trims: `bool_cone_trim_period`,
`bool_wall_trim_period` (and the closure over `name` beside it) and
`bool_curved_contain_period`. Cylinder and cone faces are read from
their boundary edges (`topo::boolean::ruled_region`) and read no period.

## Re-census (PRED orchestrator, 2026-10-25, main `2d5677e1`)

**The latent defect is closed at every live site.** None admits a
reversed span as headroom. Each site does one of these:
- decides the forward half in the same arm;
- reads a max − min or a hull width;
- reads a certified edge's span;
- admits only `Zero` on an excess.

What remains is consolidation. **Two partial homes now exist**:
- `geom_brep::props`'s `require_span_within_period`: both halves in one
  call, for the meridian, rim, cone-ellipse and sphere-arc name pairs.
- `topo::splitting::containment`'s `ConicArc::of`: the two-sided
  Δ = τ − w rule with its min and speed levers. It serves six names
  (`point_in_arc_loop_conic_span`, `pm_census_ef_cross_arc_span`,
  `bool_{torus,sphere,ruled}_region_arc_span`, `bool_contact_arc_span`).

About eight one-off spellings sit beside them:
- `certify`'s `interval_span_winding`;
- `pcurve_cache/projected.rs`'s check 2;
- `props/curved.rs`'s `fold_chain`, `props_sphere_pole_arc_full`,
  `props_rim_only_closed` and `props_cone_apex_loop_sweep`;
- `bool_split_span_period`;
- `bool_sphere_trim_*`;
- `chord_join`'s `split_ring_chart_window`;
- `chart_region`'s two excess forms;
- `revolve_angle_headroom`, `tube`'s `WindowHeadroom`, and
  `profile`'s `path_arc_sweep_full`;
- the raw interval read in `blend/surgery.rs`.

The sites added since filing are `split_ring_chart_window` and the
`ConicArc` users.

Re-priced **M**, `design: true`. The question is where the shared home
lives: lifted onto `Decide` across crates, or one home per crate plus
agreement tests. No in-flight PR touches these sites, and INTENT's
remaining stages are unlikely to move them, since these are guards on
stored spans, not coincidence decisions.

## Design (PRED orchestrator, 2026-10-25): weighed by a designer pair, converged

Both designers, one Opus and one Fable, were given the same problem and
neither saw the other's report. They reached the same final state
independently. It is not an Ev fork: no ratified text changes. This is
the reconciled design the unit builds.

**The premise correction both made.** It is one QUANTITY, the headroom
`h = τ − extent`, read for three different questions. The sites differ
in the question they ask, not only in how they spell it:
- **"not over"** admits `Zero | Positive`. Sites: certification,
  `require_span_within_period`, `fold_chain`, `props_cone_apex_loop_sweep`,
  `split_ring_chart_window`, `chart_region_seam_span`, projected check 2,
  and `ConicArc::of`'s lower half.
- **"strictly partial"** admits `Positive` only. Sites:
  `revolve_angle_headroom`, `tube_window_headroom`, `path_arc_sweep_full`,
  `bool_sphere_trim_meridian_span`, `bool_sphere_trim_period`, and
  surgery's raw read.
- **"whole turn"** asks whether `Zero` is the answer. Sites:
  `chart_region_cyl_wrap`, `props_rim_only_closed`, and
  `props_sphere_pole_arc_full`.
- **Missed by the census:** `profile/src/seg.rs` `arc_sweep_range`.

So one home that fixes the disposition can serve only the first
question. What every site shares, and what is spelled by hand at each,
is the margin: its sign, its rate, and its dimension door. Most sites
cross a parameter span through `Margin::levered`, against
`[tolerance.margins]` (iii) and that door's own doc. Certification
already uses `metered`.

**The final state.**
1. **One rule in `geom-core`.** `profile` sees only `geom-core`, and the
   sweep levers are not carriers. Signature:
   `turn(rows, extent, rate, band) -> Result<Turn, _>`, where
   `Turn::{Short, Whole, Over}` carries its `MarginDiag`. Every site
   matches exhaustively for the question it asks. If
   `geom_brep::pcurve_cache::Period` moves down cleanly with
   `WalkStop`/`BranchMiss`, the rule is `Period::turn`. Period's doc
   already claims every whole-period question. Moving it is a reshuffle,
   so it is logged, not asked. If it does not move cleanly, the rule is a
   free function and Period's doc is corrected.
2. **The rate is typed:** `Exact(r)` (a circle, an authored angle on its
   arm, a revolve's widest meridian) or `Bounded(inf, sup)` (an ellipse,
   a spiric, a chart image). The rule is `ConicArc::of`'s, lifted:
   - `Over` needs `h` at the inf rate definitely negative;
   - `Whole` needs `h` at the sup rate in band;
   - `Short` needs `h` definitely positive at the inf rate.

   Any other combination escalates, on the caller's straddle row. At
   `Exact` the two reads are one decision, bit-identical to today's
   verdicts, margins and K samples.
3. **The forward half is a TYPE, not a second call.** The extent enters
   as `Extent<T>`, and a reversed span does not typecheck. Constructors:
   - `forward(name, t0, t1, rate, band)` decides `0 < Δt` (hand-built
     loops, authored windows);
   - `width(lo, hi)` is a hull, max − min;
   - `decide_signed(...)` is for a signed input (revolve's angle, the
     cone apex loop's net turn), and refuses typed at `Zero`;
   - in `geom-brep`, a certified edge's extent comes from its
     certificate, which already decided `0 < Δt`.

   There is deliberately **no bare `magnitude(x)` constructor**: the
   absolute value of a reversed difference is exactly MESH-12's defect.
   `Extent` implements `Add`, for `props_rim_only_closed`'s sum of rim
   spans.
4. **Names stay per site.** Each caller passes its own
   `TurnRows { turn, straddle }`, as `ConicRows` does today. A `Bounded`
   site names its straddle row; an `Exact` site cannot straddle and
   passes none. That is type-enforced, so no name exists that can never
   fire. The rule's margin gets one row in
   `docs/predicate-dimension-audit.md`, and each site's row cites it. No
   name is renamed.
5. **A certified edge's turn belongs to its certificate.** Certification
   stores the `Short` or `Whole` it reached; `Over` never certifies. Four
   sites currently re-decide it on certified edges:
   - `bool_split_span_period`;
   - `bool_sphere_trim_meridian_span`;
   - `ConicArc`'s callers;
   - surgery's raw read.

   They read the stored turn instead, and their own decisions retire.
   Verify first that the band at those readers equals the band the edge
   was certified at (`restate_stored_edge` suggests edges are
   re-certified when the band changes). If it does not hold, those sites
   call the rule instead.

**What moves, deliberately.** Each of these is listed in the agreement
test as an expected delta, never hidden:
- Certification's ellipse and spiric arms, and props' cone ellipse,
  escalate on an inf-`Zero` / sup-`Negative` straddle that they admit
  today (`certify-admits-an-ellipse-wound-past-its-period-on-the-slow-rate`).
- Projected check 2 escalates on a sup-only overshoot, instead of
  refusing sign-certain (`AzimuthPeriodExceeded`), unless an inf chart
  rate exists.
- The `chart_region` excess forms flip their recorded margin's sign. No
  classification moves.
- The retired names leave the K stream.

**The charter's agreement test.** Keep each old spelling as an in-test
oracle (surgery's `old_misses` is the precedent), and run it against the
rule on the cases each site was written for:
- a full turn;
- `τ ± {0.5, 3, 20}·ε/rate` at both edges of the band;
- a zero-width hull;
- a reversed span, which must fail to compile or refuse at the
  `Extent` door;
- an ellipse with `Δ·inf` in band and `Δ·sup` definite.

**Two non-ratified doc statements to re-word in the same unit**, both
in `geom-core/src/predicate.rs`'s module docs:
- the rate-pair rule ("sup for an overshoot … can only refuse"), which
  should read: sup certifies smallness; a definite verdict past the band
  needs inf; disagreement escalates;
- the naming paragraph, which should allow a shared rule to serve under
  the caller's name.

**Unit cut.**
- **Unit 1:** the rule, the types and the harness, plus certification
  adopting the rule and storing the turn. Includes the rider and
  `ConicArc::of` as a caller.
- **Unit 2:** every remaining site switches over, and the certified
  readers read the stored turn.

Off-question findings from the pair, owed rows:
- surgery's raw read cites a "junction-end pick precedent" for
  bounds-lane reads, and nobody has found it ratified. Unit 2 checks
  that with `git log --all -S'junction-end pick'` before touching it.
- `profile/src/path/verbs.rs` `path_arc_sweep` gates a bare radian
  through `Margin::of` against the metres band. Unit 2 owns it as a
  site.

## Unit 1 landed (PR 5686, 2026-10-25)

The rule, its types and certification are built, along with `ConicArc::of`
as a caller, under a dual review (both APPROVE-WITH-FIXES, no MAJOR) and
a 13-item fix pass. What changed against the design:
- `Extent` is built by `forward`, `hull(values)` (not `width`),
  `decide_signed`, `Add`, and the certificate's `CertifiedTurn`, which
  carries its interval as well;
- rates are built by constructors;
- one speed floor is `geom::carrier_speed::speed_floor`, and the sup is
  `turn_defect_speed_hi(carrier, t0, extent)`.

**Unit 2 remains.** It moves every other site onto the rule:
- props' hand-built askers;
- topo's chart and hull askers, and projected check 2 (sup only: an
  escalation unless an inf chart rate exists);
- the authored askers (revolve, tube, path, `seg.rs`);
- the certified readers, which read the stored turn.

Read the PR 5686 body's per-site call shapes first.

## Closed (PRED orchestrator, 2026-10-25, PR 5710)

Unit 2 landed. Every site of the fact decides through
`geom_core::period`. The three questions have one home each:
`Period::{not_over, strictly_partial, whole_turn}`. Admitting an
undecided "not over" reading has one home too: `TurnUndecided::not_over_at_sup`,
on the sup reading's sign. The certified readers
(`bool_split_span_period`, `bool_sphere_trim_meridian_span`, surgery's
raw read) retired onto the certificate's stored turn. Each old spelling
is pinned against the rule as an in-test oracle. A full single review
returned APPROVE-WITH-FIXES with no MAJOR, followed by a fix pass.

Filed:
- `period-decide-callers-lever-a-parameter-gap` (P3);
- CHART's `azimuth-inf-arm-has-two-homes`;
- BAND's `surgery-cites-an-unratified-junction-end-pick-precedent`.

## Moved (2026-10-26)

PRED folded into PREDTAIL (Ev, in chat, 2026-10-26). Kept although
closed: ENCL's `hand-minted-invalid-gates-in-topo` and CLEAVE's
`topo-mints-indeterminates-outside-the-funnel` and `the-indeterminate-seal`
hand their `solid_contain` period sites to this row, and its Closed
section is the record of where those sites now decide
(`geom_core::period`).
