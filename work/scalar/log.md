# SCALAR — the log

## 2026-10-10 — opened

Cut out of FLUX by FLUX's orchestrator, on FLUX's priority seam:
FLUX measured 125.5 budget points against 30 and kept its eight
curved-arm rows. Every row here moved by `git mv` with its id and body
unchanged; the move priced the unpriced rows and re-priced legacy `D`.
— (FLUX orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): `geom_core::undecided!(subject, payload)` is the one home of the ': ' payload joint; `UndecidedRefusal` and topo's payload-only sites use it, byte-identically. (ENCL orchestrator)
- 2026-10-12 — Seam note from NURBS: PR 4834 (`nurbs/held-constant-weights`) adds a required method, `SpanLocate::held_constant(self) -> Option<f64>`, and implements it for `Probe` in `crates/geom-core/src/k_stats.rs` (+4 lines; it answers its value, as f64 does). The NURBS speed meter uses it to keep weights carrying a tangent or symbol off the net-only integral arm. (NURBS orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4840, merged at `8e772980bb`): `MarginDiag::tightens_below` withholds the tighten offer for a margin `theorem()` proves zero, on D4 ¶1 (i)'s ground: a proved-zero margin is not a size the user may intend, though numeric-first `decide_sign` would pass it at a smaller ε on Inexact lanes. `Indeterminate::reading` says 'a zero the symbolic tier proved, which no tolerance makes a size'; `IndeterminatePayload` renders a proven in-band enclosure as 'reaches into the ambiguity band'; only a proof removes Subdivide. Filed on sym: `an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked`. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4811, merged at `abff76bb25`): editor-core's undecided sentences render through `Indeterminate::undecided`. Evaluation, naming and selection are ended by one closed `decision::Decision` table with a kind column; evaluation constructors take a `Decision`, and an unknown name ends loud (`MissingRecourse`). D10 ground keeps main's text through held variants: `NodeErrorKind::MeasureEscalated` (measure distance/gap parallelism), `PlacementAxisEscalated` (`EditError::PlacementAxis`) and `PoseDirectionEscalated` (datum directions and frames, pose `degenerate`). All three classify as Escalated, and pncad-py `tags.rs` and viewer `tree.rs` gained arms. Left for after D10, on the ENCL row: the placement route compares the role word, the naming door's `&str` feed, `placement.rs`'s stale `# Errors` doc, and `datum_unit_norm` vs `eval_direction_norm` ending two ways. (ENCL orchestrator)
- 2026-10-12 — Seam note from DECIDE: DECIDE-10 (Ev's ruling on PR 4901; `docs/DECIDE-10-SPEC.md`) deletes the symbolic tier's label kind (`symbolic_zero` / `sign_gated` / `registered`) from every decision type. `Theorem`'s kinds go, `Zero::Proven` carries nothing, and `MarginDiag::jointly` is proven if both are. `topo::Discharge` becomes `{Numeric, Proven}`, the coincidence door's `Rung::MarginIdentity` / `CarrierVerdict` carry nothing, the K tokens `sign_gated` and `registered` retire (`k-lint` follows), and `drive.rs`'s summary gives one number for the tier. Branch `decide/10-delete-the-kind`.
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4943 merged. A sized decision's ending now depends on one classification of the reading, `MarginDiag::shows` (geom-core): Offer, NoSize, Floor, Unresolved, Past or Unreadable.
  - The decision's note (`SizedWords::no_size`, renamed from `at_zero`/`otherwise`; geom-brep `NoSize`, renamed from `AtZero`) appears only on NoSize: an exact zero, the refused side, or a theorem.
  - A positive reading below the floor gets the one `floor_note` sentence.
  - A reading that touches or straddles zero gets the lever alone.
  - The import door gives its sentence plus the lever, and never a decision note.
  - PR 4855's rejected-sign filter is gone.
  - New sized decisions take `no_size:`.
  - Tests that predict an ending should call `MarginDiag::sized_recourse`, not restate the rule. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4872 merged. A face whose weights sit at subnormal scale is now rescaled by an exact power of two (geom-core `spline::weight_scale::unit_scale_exact`) before its rational arm is certified, so its certificate equals the unit face's. `ssi::section` and the meters share the one checked helper. Any certified read of rational weights should divide by the weight floor after unit scaling, never after the norm. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 5003 merged. geom-core now has `EndingKind { Sized, Lever(&'static str) }`, with `EndingKind::end` and `Indeterminate::ending_by(kind, named)`, as the one by-kind dispatch for an escalation ending (lever-only through `Indeterminate::lever_ending`; missing row gives `MissingRecourse(named)`). profile `validate` and editor-core `decision` call it. `check_rigid` now refuses a non-finite linear entry as `RigidMotionError::NonFinite(MotionPart::Linear)` before reading any margin. (ENCL orchestrator)
- 2026-10-13 — **Seam (ENCL):** PR 5029 merged, touching `MarginDiag::sized_recourse_in_file` (`predicate.rs`) in strings and one local only; the `shows`/`within` logic is unchanged. The import door now says "is within … so the file cannot tell it from zero", and the partly-within twin says "may lie within … so the file might not tell it from zero". (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5023 merged. At a build, a refused reading past the band now ends like its in-band sibling (D4 ¶1 (iv)): the lever plus the decision's `no_size` note. geom-core `MarginDiag::shows` gains `Shows::Refused`. Touches `predicate.rs` (`Shows::Refused`, `SizedWords::sign_certain_recourse`). (ENCL orchestrator)
- 2026-10-14 — **Seam (NURBS):** PR 5114 (`nurbs/interval-scale-exact`, in review) adds one defaulted method, `Real::scale_pow2`, to `real.rs`. The Interval override `DInterval::scale_pow2` scales by an exact power of two without rounding in the normal range, and is outward below it. Any other `Real` implementor inherits the general product. (NURBS orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5101 merged. At rest, one definite refused reading ends one way whichever arm routes it: `StoredDefinite::Lever` gives the lever plus the note; `Contradiction` gives `KERNEL_OR_FILE_DEFECT_ENDING`, at rest and at the door past ε_in. In geom-core, `SizedWords` gains `refused: RefusedReading { AsShown, ContradictsStoredData }` (`predicate.rs`), and every other literal passes `AsShown`. (ENCL orchestrator)
- 2026-10-18 — Seam note from RESTREAD (branch `restread/one-edge-box-map`, PR 5308): `crates/geom/src/curves/boxes.rs`'s `spiric_arc_aabb` no longer has its own `Brk` construction. It reads `geom::spiric_period_box`, a `T: Real` construction that charges for its own rounding, at the scalar's brackets, hulled with the endpoints. Its doc's false "clamps `f_min` at zero" sentence is replaced. `Brk` and the other doors are untouched. (RESTREAD implementer)
- 2026-10-21 — **Seam (NURBS):** PR 5542 (CPAIR's one-line PR 3, `nurbs/seg-one-line`) re-baselines editor-core goldens that its new `segment_gap` read moves: kstats_bracket_rows' Profile log 147 → 159 (`PRE_PASS` 82, `LANE_OP` 77), asm2a_instantiate 806 → 818, m10_8's numeric tier 272 → 278 (shipped 264 → 270), m4_pr6's ε diff gains a `segment_gap` divergence (6 → 12), and m10_6's certifying keys are re-blessed (the verdict rows moved, not the filter). No verdict changed; every move passes unchanged with the gap read switched off.
- 2026-10-21 — **Seam (NURBS):** PR 5542 re-blessed its editor-core goldens on main `222b097c` after its fold (the gap now reads both axes): kstats Profile log 147 → 171, asm2a 802 → 826, m10_8 numeric 270 → 282 (shipped 262 → 274), m4_pr6 `segment_gap` divergence 12 → 24, m10_6 keys re-blessed (the verdict rows, not the filter). `a_pre_pass_that_escalates_before_failing_carries_the_escalation` now names `["segment_gap", "chord_side"]`: an in-band screen read rides the refusal beside its cause.
- 2026-10-21 — **Seam (NURBS):** PR 5552's FULL review found base reds on main, re-measured on `3c342076` and filed as `tcost/main-reds-editor-core-slow-set-and-the-k-probe-sweep` (P2): 24 editor-core slow-set rows (docm9_range, m10_* interval, revalue_corpus, sym11, sym_9), and two rows that abort `scripts/k_probe_sweep.sh` (sweep `ray_wall_margin_twins` on `curve_foot_branch`, editor-core `m4_pr8_k_probe`'s content key). TINT and SCALAR co-own the files.
- 2026-10-25 — Seam note from PRED `D292` (branch `pred/d292-directional-extremum`): `crates/geom/src/curves/boxes.rs` gains the directional extremum of a conic arc — `ArcWindow` (comparison-free crest-in-span test and extent), `conic_arc_extent_along` at the carrier's scalar and `conic_arc_extent_along_bracketed` (every bracket lifted to `Interval`). `circle_arc_aabb`/`ellipse_arc_aabb` are that body along the world axes, run at `Interval`; `Brk`, `extremal_angle_interval`, `axis_extremum` and `ANGLE_SLOP` are gone, and the bounds roster swaps `of` for the new sole-bound doors. Box bits move by outward-rounding ulps. (PRED orchestrator)
- 2026-10-26 — (tracker sweep) `the-box-driver-carries-no-part-resolver` (P2 M, parked on `intent-stage3-is-built`) arrives from FLUXHOLD, which closed: `crates/editor-core/src/drive.rs` is on this program's paths, as FLUXHOLD's plan named.
- 2026-10-26 — (easy-fixes lane) `unit-vec-escalations-end-a-poisoned-gauge-without-the-note`: `UnitVec3Error::Escalated` ends through `lever_ending`, pinned. The row stays open on `LeveredUnitError::Arm`, whose callers lever by both user-chosen and gauge lengths.

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): `m10_di_dual_corpus` (renamed `at_rest_census_verdicts_match_f64_at_interval`) and `geom-core/src/real.rs` (one comment) follow the at-rest check.
- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`, PR 5741) touches this program's ground. Every public number door takes a required predicate (`topo::Settle`: `Sign`, `AtMost`, `AtLeast`, `Contains`, `Resolution`) and answers `topo::MassMeasurement { enclosure, stop }`; a number is read through `MassMeasurement::met()`, which answers the enclosure where the resolution was met and a typed `Unmet` where it was not, and an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. A deciding predicate still undecided where no face can refine is the one quadrature refusal, `MassPropsError::Undecided { volume: (lo, hi), stop }`. An import hands out `StepImport::Solid { body: topo::CertifiedBody, .. }`: the body held with the gate's certificate of it (`body()`, `into_body()`, `measure(settle)`). `PropsError::QuadratureBudget`, `TargetUnreached`, `HeldCertificate`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone. Tests reading a number use `topo::test_support::pinned_volume(what, body, tol)` (1024·ε, asserting the resolution met). (QUAD unit C lane)
- 2026-11-02 — **Seam (ONEHOME, `onehome/run-owns-rounding`, PR 6380):** builds this program's `no-door-rejects-geometry-past-eps-f64-resolution` box check as ratified on PR 6329 (split 2 B): `topo::session_box::in_session_box`, over a body's vertices and edges' spans (read through `geom_brep::Reach::Span`), asked first in the at-rest gate's structural pass, refusing `ValidationError::OutsideSessionBox` (the gate's only finding, named by the op that wraps it). `Tolerance` now admits a run (ε, K) only where `(K − 1)·ε` covers `ADMISSION_FLOOR` = 16u · 1 km; `EPS_MIN` is that floor over `DEFAULT_K − 1` (≈ 1.97e-13 m), and `eps_min(k)` gives the least ε at another K. The row's "rejection at construction" is the finished-body gate's, not each building door's. (ONEHOME lane)
- 2026-11-06 — (ROUND orchestrator) Seam: ROUND's PR 6546 edits `crates/geom-core/src/predicate.rs`: `EndingKind::Sized` now carries its lever (as `Lever` does), with `lever()`/`with_lever()`/`under()`, a new `Indeterminate::under_lever`, and `UNNAMED_PREDICATE`. Callers outside profile: editor-core's decision table (one line). No SCALAR lane is live.
- 2026-11-19 — **Sitting opened** (SCALAR orchestrator). The track is `active`. Both `main-red` rows closed as stale: the d_tab rows and the Interval fence pass in the 11-17 nightly (run 44023697569) at every ε, re-pinned by the lanes that moved them. That nightly is the latest read of the slow set, because the `full suite, build once` job died before the suite on 11-16, 11-18 and 11-19. The four P1 rows are being re-measured against the tree before any dispatch: each is two months old and three of them carry a home or design question their bodies call open.
- 2026-11-19 — **The four P1 rows, re-measured** (three read-only explorers against main `00a5903654`).
  - `measure-carrier-holds-a-unit-chart-normal-by-prose` and `tangent-perp-takes-an-unchecked-unit-direction` are **parked** on ONEHOME's `carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`. Both waited on "the carrier rule changing", and Fork 274 (PR 7006) changed it. The alternative, minting a witness at each read now, was not taken: it adds site names and refusal paths the carrier change deletes. Each row's body says what is true of its sites today.
  - `dl5-has-no-arm-for-a-selection-whose-result-is-the-payload` is a **design fork**, re-priced from legacy `D` to `M +design`. DL5's two homes disagree: `docs/DUAL-DESIGN.md` (ratified, PR 1146) has no locally-constant clause, and `real.rs` (PR 1154's fix pass) has one. Two designers dispatched; draws on `analysis/design-fork/scalar-dl5-selection`.
  - `patherror-display-renders-float-noise` is a **design fork** (`M +design`). The row's two-helper picture is stale: `geom_core::Readable` landed after it (about 95 uses, exact round-trip, no grid), `profile::path::num` and `viewer::readout::number` still grid differently, and 150 to 200 Display arms use no rule. Three designers dispatched (drawn); draws on `analysis/design-fork/scalar-number-format`.
  - **Class**: a row whose body says "waits on X" in prose, with `status: open`, is read off the board as dispatchable for as long as nobody re-reads it. Both unit-direction rows sat `open` at P1 for two months on a trigger no `blocked_on` named.
- 2026-11-19 — **Slate hygiene** (orchestrator's read; tracker only). The eight unbanded rows are banded and priced, each on the README's absolute definitions; three carry interval-drive readings (P2), one the mean-width margins (P2 H), the rest P3 or P4. `brk-has-no-certified-door` closed as stale: PRED's D292 deleted `Brk`. `plan.md` is rewritten to the present: it no longer copies the board's table, and it says to re-measure a row before dispatching it. The track's load reads 6/30.
- 2026-11-19 — **Fork: the delegation rule** (`dl5-has-no-arm-for-a-selection-whose-result-is-the-payload`; `[ev]` PR on `scalar/dl5-two-clauses`; `docs/DESIGN-FORK-LOG.md` row 292). Two designers, three reconciliation rounds: agreed on the rule, split on its home.
  - **Provenance**: DUAL-DESIGN DL5 is the draft Ev signed off whole on PR 1146, with no tangent condition. The locally-constant clause in `real.rs` is commit `7f3672ae5a`, a fix pass on PR 1154, which has no comment of Ev's. Nobody could check chat-side ratification.
  - **Where the roster's 25 `Selection` rows land, by both designers' read** (a lane re-reads each): `Choice` takes `map_refusal`, `Unclear`, `displaces`, `holds_enclosures`, the size and datum screens and the cost drivers; `Freeze` takes `mid`, the two projection macro bodies, `carrier_span` (confirmed: it returns `T::from_computed(lo)`, `T::from_computed(hi)`, and is filed `Payload`) and probably `knot_lines`; `Carries` takes `SectionFrame`, `curve_datum_errors`, `surface_datum_errors`. Classified from roster text alone: `sure`, `Least`, `inside_bound`, `curve_in_core`, `past_misses`, `exact_pair`, `exact_zero`.
  - **After the ruling**: the census re-sort is one implementing unit; `bounds_census.rs`'s `Why::Selection` doc keeps the old wording until it lands. The soundness question under the labels is filed as `census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection`.
  - **Class**: the reconciliation's home question changed hands in each of three rounds, each designer taking the other's last argument. A round that ends with the positions swapped is not progress toward agreement, and a third such round was the signal to state the split.
- 2026-11-19 — **Both P1 forks are with Ev**: PR 7154 (a number told to a person) and PR 7155 (the delegation rule). The track holds no other dispatchable P0 or P1 row, so nothing is dispatched until Ev answers; the orchestrator stays on the track to answer on those PRs. Next on an answer: one implementing unit per fork (the told-quantity type and its per-crate sweep; the census re-sort), each re-measured and tiered at dispatch.
- 2026-11-20 — **PR 7155 rewritten around Ev's question.** Both designers, shown the comment and the survey, dropped DL5.2 as a per-site condition: `from_computed` carries an unknown tangent at a dual (as `Sym`'s is opaque), a zero is claimed only through a listed stationary door pinned against finite differences, and a real tangent is computed at `T`. The build is `from-computed-claims-a-zero-tangent-at-a-dual` (P1 H), parked on the PR. The README states DL5.2 ahead of `Dual::from_computed`, which returns zero until that row lands. The home split of the first rounds is no longer put to Ev: with DL5.2 a scalar's behaviour, what is left to place is DL5.1, and the diff keeps it in the crate README.
- 2026-11-20 — **PR 7155 rewritten a second time, around necessity.** Ev: "i still don't see why we need to turn a computed f64 back into a T", and "still not sure if duals are, at least type-wise, only where they belong?" Two rounds, both designers counting in the tree. 114 lines (128 calls) are in a dual's reach by the enclosing function's own bound; about 9 produce a value the result is built on, and every other re-entry is a bound to decide against, a place to evaluate at, a free choice, or `f64`-built data. By type a dual is excluded only by `CertifiedBounds`/`CertifiedEnclosure`, which most certification interiors do not carry (`pcurve_cache/projected.rs` 1 of 40 generic headers, `spline_incidence.rs` 0 of 15): they are clear of duals by `AtRestPolicy for Dual` answering `None` and by who calls them. The recommendation is one door per kind of `f64` (DL5.2) and the certifying bound on certification interiors (DL5.3). The designers crossed on how to cut the count (by what the number is, or by what it feeds) and agree on the final state.
  - **Not checked by anyone**: the consumers of the roughly 55 place sites; whether every certification-interior item can take the certifying bound without splitting a construction caller; `props.rs`'s `T`-typed volume bracket fields; `Link::lowest_on`'s wrong sensitivity is still read, not run.
  - **Found on the way**: `Margin::<f64>::lift` is a re-entry and `Margin::value` is a public `T` exit; `edge_nurbs` `Schedule::station` holds `T` ends and interpolates in `f64`, where `spline_incidence`'s scheduled samples take a `from_f64` fraction against `T` ends; `ExtentBall::of_overlap` has no callers. These are in the build row.
- 2026-11-20 — **PR 7155, third rewrite, on Ev's row-by-row answer.** Ev took removing the lift for bounds and `f64`-built data, and asked: when is a place's output lifted at all, and is an unknown tangent a runtime exclusion where a compile-time one is wanted; why is a solved value plain `f64`; what finite differences meant; whether `from_f64` can be misused for any other door; and why certification interiors were not already closed to duals, "is there some reason that this keeps not happening?" Three more rounds. The designers traced 22 place sites to their consumers and every one ends in a decision or a parity, so a place is an `f64` argument and its result comes back on the value channel; the unknown tangent and the zero door are both withdrawn, and `from_computed` leaves the surface dual-reachable code can call.
  - **Why DL5.3 keeps not happening** (each fact read; that together they are the cause is the designers' judgement): Ev's rulings (PR 5689, "it was supposed to be impossible at the type level for a dual to enter into certification"; PR 6308) were each carried out at the door they were about. Behind the doors, the `Bounds` scope rule asks for the weakest bound that works, which is `Decide`, which a dual implements; the one bound that excludes a dual is also the bracket right, and `bounds-allowlist.sh` refuses it beside `Decide` in any file not ratified onto its list; the lane structs are declared `T: Real`; `AtRestPolicy for Dual` answers `None`; and no gate asks whether certification code can be instantiated at a dual. **Class**: a rule enforced by a gate that prices the safe spelling above the unsafe one is un-enforced wherever nobody is looking.
  - **Not checked**: about 9 place sites; that the value channel is cheap at a certifying scalar; that a compile-time-constant door covers every `constant` row of `from-f64-sites.tsv`; that every interior can take the marker bound.
- 2026-11-20 — **Ev ruled on PR 7155**, in stages. Taken: no lift for a bound or for `f64`-built data, the place form, the `from_f64` split, and DL5.3 whole, with the scope rule to be reworded ("weakest bound that works ... should be updated to have more of a focus on semantic correctness"; "the exclusion is free and only the bounds reading needs the extra check"). "you can get started on the fixes in 2 for sure, as well as all the parts of 1 that are like 'remove the lift'; the two things i had additional questions about can be resolved in a new pr". Held for that PR: what a solved value is, and any lift behind `CertifiedEnclosure`.
  - **The build is six units** under `from-computed-claims-a-zero-tangent-at-a-dual`, which keeps the solved-value question (`design`). Specified now: the `Certifies` marker bound and the scope rule; bounds that never become `T`; places on the value channel. Not yet specified: the `from_f64` split (after the bounds unit takes the lever scales off its list), `f64`-built data (one more read of the sweep path's origin), the finite-difference check (Ev asked whether it sits behind a `debug_assert`).
  - **Load**: four `H` rows beside the told-quantity build put the track over its 30 points. Splitting it is a sitting's work (`work/README.md`, Track size); the DL5 build is the seam, a program of its own if it outlives this sitting.
  - **On the two held items the designers read further**: neither found a site that needs a lift behind `CertifiedEnclosure` (the loft has a `T` route in `LoftBuild::in_t`; the fitted pcurve image stays `f64` data), and each walked a solved value scalar by scalar. They differ on what its door is, and that is the new PR's question. `spline_face::settle` returns a box's midpoint as a point where the isolation certified the box.
- 2026-11-19 — **Fork: a number told to a person** (`patherror-display-renders-float-noise`; `[ev]` PR on `scalar/told-quantity`; `docs/DESIGN-FORK-LOG.md` row 291). Three designers, two reconciliation rounds, converged.
  - **What the tree holds, re-measured**: `profile::path::num`/`num_toward` (grid: finer of ε/10 and a relative 1e-9; all of `crates/profile`'s refusal sentences); `viewer::readout::number` (ε/10 and a relative 5e-4, a 22-character bound); `geom_core::Readable` (exact round-trip, 76 to 95 uses by three counts, and `MarginDiag::readable` through it); `quantity::fmt::render_shortest` (a private copy of `Readable`'s rule). An explorer's scan counted 150 to 200 error `Display` arms rendering a scalar with `{:e}`, bare `{}` or `{:?}`, largest in geom-brep, topo and editor-core; no designer verified the count.
  - **Held until Ev answers**: `user-facing-number-format-is-not-one-across-payloads-endings-and-escalations` (parked on the fork's row) and REROUTE's `refusal-floats-outside-the-spline-stack-render-through-f64-display`. Both move computed numbers onto `Readable`, which keeps the noise.
  - **Not checked by anyone**: whether production code or a test reads a scalar field of a refusal payload (it sizes what an opaque field costs); whether `PathError<T>` and its siblings keep `T`; the PRs behind `16650cf9b8` (#990), 3402 and 5838.
  - **For the building lane**: signed zero is pinned both ways today (`num` prints `-0`, `MarginDiag::readable` folds it); `Stackup::content_key` hashes a `Readable` rendering, so that site stays exact; `Shows::ending`'s "tighten the tolerance below {} m" prints an exact bound and needs the offered rounding once a grid applies; `readout.rs`'s `tolerance` doc cites `work/props/…`, a path that no longer exists; `props::render_number`'s doc and `props::written_text`'s disagree on whether it serves sentences.
- 2026-11-19 — **Both P1 forks are with Ev**: PR 7154 (a number told to a person) and PR 7155 (the delegation rule). The track holds no other dispatchable P0 or P1 row, so nothing is dispatched until Ev answers; the orchestrator stays on the track to answer on those PRs. Next on an answer: one implementing unit per fork (the told-quantity type and its per-crate sweep; the census re-sort), each re-measured and tiered at dispatch.
- 2026-11-20 — **Ev answered PR 7154**: "the rules are fine, but this level of detail should definitely not be in DESIGN.md". The rules are a section of `crates/geom-core/README.md`, "A number told to a person", beside the recourse rules; DESIGN keeps one `[one-way.places]` row and the companion row. The orchestrator read the answer as covering the reader's-notation paragraph and said so on the PR. Next: the build, as a unit, re-measured at dispatch.
- 2026-11-20 — **Ev questioned PR 7155's premise**: "it sounds like `Dual`s are ending up where they don't belong again? when is 'A computed f64 turned back into a T' valid?" Neither designer had counted, so an explorer read every `from_computed` call outside tests (157, read not executed): 57 a dual cannot reach, about 60 decided on or reported only, 9 constants, about 10 on a first-order argument in a comment, 10 wrong, 10 undetermined. No gate covers `from_computed`, and `from-f64-allowlist.sh`'s error message recommends it as the fix.
  - **The wrong ones**: `topo::Link::lowest_on` is live at `Dual<f64>` through the stackup (filed on ATREST, P1); the loft's enclosure arm (6 uses, `sweep/src/loft.rs`), `geom_brep::curve_pair::polish` and `geom`'s `on_carrier` `polish` fire only at `Dual<Interval>`, which nothing outside tests builds.
  - **Undetermined**: `periodic_nurbs.rs`'s `restated`/`reduced` (does a period ever carry a tangent), `ExtentBall::of_overlap` (a pub function with no caller), `SplineWall::outward` (projects on the value channel at `f64`, which goes round `project`'s `CertifiedBounds` door).
  - **Unreachable only by its callers**: `carrier_span` (`T: Bounds`; all seven callers certified).
  - **Where duals are built**: `editor-core/src/stackup.rs` at `Dual<f64>` and nowhere else outside tests; `drive.rs` holds no dual, so E7's pruning has no consumer today.
  - The fork went back to both designers with Ev's comment and the survey; the question put is what re-entry should be so the code enforces the answer. **Class**: two designers and three rounds agreed a rule over a population nobody had measured, and Ev's first question was the measurement.
- 2026-11-20 — **PR 7154 merged** on Ev's ruling. `patherror-display-renders-float-noise` is restated as the build (`H`, three units: the type and `profile`'s move onto it; the notation-taking rendering; one sweep per crate) and is the track's one dispatchable P1. PR 7155 (what a re-entered value carries at a dual) is with Ev; its build row `from-computed-claims-a-zero-tangent-at-a-dual` is parked on it.
- 2026-11-20 — **Dispatch: `the-told-quantity-type-and-profiles-refusals-on-it`** (unit 1 of the told-quantity build). Letter H, tier dual: a public vocabulary type every sentence-writing crate will hold, with rounding rules that are easy to get subtly wrong. Units 2 (the notation-taking rendering) and 3 onward (per-crate sweeps) stack on it once its type has survived review; they are not dispatched alongside, because the review can still reshape the type they would build on.
- 2026-11-20 — **Ev questioned PR 7155 again**, on the rewritten recommendation: "i still don't see why we need to turn a computed f64 back into a T. your table of valid cases looks like cases where it doesn't cause an issue if we do this, but none of them obviously look to me like cases where it's necessary", and "still not sure if duals are, at least type-wise, only where they belong?" The survey sorted the 157 re-entries by harm and never by need, so nothing measured answers the first; on the second, a dual is kept out by `CertifiedEnclosure`/`CertifiedBounds` alone and about 100 re-entry sites admit one by type. Round 7 went to both designers with Ev's words and two things to count in the tree: which dual-reachable re-entries the code cannot do without, and where the types admit a dual that has no meaning there. **Class**: a rule about when a thing is safe was put to Ev twice before anyone asked whether the thing is needed.
- 2026-11-20 — (tracker sweep) **FRAME merged in.** It held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size). Its 10 live rows moved here by `git mv`, ids and bodies unchanged, each with a dated Moved note; its charter and plan are the `## From FRAME` sections of `program.md` and `plan.md` (`docs/doc-ledger/frame-merges-into-scalar.md`). (tracker sweep)
- 2026-11-20 — (tracker sweep) To SCALAR's orchestrator: rows were added to this track while it is held (Ev, in chat, 2026-11-02: adding rows to an existing orchestrator is safe). None is P0 or P1, so the sitting's slate is unchanged. (tracker sweep)

- 2026-11-20 — `the-told-quantity-type-and-profiles-refusals-on-it`
  (PR 7159) reported with hosted CI green on head 75501c0244. Dual
  review dispatched: two reviewers concurrently on that frozen head,
  one brief (`review-brief-7159.md` in the forks scratch), ten claims
  to falsify plus the style lane; slot-order byte on
  `analysis/model-compare/scalar-2026-11-20`. Held, not dispatched:
  `a-place-is-an-f64-argument-answered-on-the-value-channel`. The
  build mutex is width one with two implementers and two reviewers on
  it, and the fair-pair rule says to reduce simultaneous work rather
  than narrow a reviewer; the places unit also edits files the two
  DL5 units in flight edit. It goes out when one of them lands.

- 2026-11-20 — **PR 7159's dual review is in and adjudicated.** Both
  reviews: APPROVE-WITH-FIXES; the rounding core survived
  exact-arithmetic sweeps on the parsed-float reading. Taken into the
  fix pass (`fix-list-7159.md` in the forks scratch, twelve items): an
  `ArcLen` leg's `len / r` told as given (float noise back in a
  sentence and out through `as_given`); `Told::beside` false for an
  offered or enclosure member; `Debug` and `PartialEq` as ways back to
  a computed number; the field gate failing open on shapes its blind
  list does not name, and its count pin; four unpinned or loosened
  contracts in the suites. Nothing argued down. The orchestrator's
  brief was wrong on one claim and both reviewers corrected it:
  `Band::offer_deciding` has no production caller outside `geom-core`.
  - **Class — a role is a constructor name nothing checks.**
    `authored_*` makes any scalar "given", and `as_given()` then hands
    it back; every per-crate sweep will make that choice at dozens of
    sites. To be filed by the fix pass; it shapes how unit 3's sweeps
    are briefed (each lists the provenance of every given it tells).
  - **Class — a suite re-baselined onto the function under test.**
    Seven consumer rows replaced `m / k` with `Band::offer_deciding`.
    A re-baseline brief has to say that the new expected value is
    derived independently.
  - **Friction**: both reviewers lost 25 to 35 minutes at a stretch to
    the width-one build mutex and ran fewer mutants than they meant
    to; neither was narrowed by the orchestrator, so the pair is fair.

- 2026-11-20 — **Ev on PR 7176**: "this broadly sounds good", then
  "how are these used downstream, more concretely? it seems like it
  would be confusing for something running at interval to have some
  of its values be exact in a way that, taken literally, would
  constrain the others." Round 15 went to both designers to trace each
  solved value forward. They agree: three or four functions store the
  value in the body (a split parameter, a re-anchored edge end), five
  only evaluate at it (places under DL5.2, no door), and
  `editor-core::drive` runs these at `Interval` in production. One
  found that the proposal conflicts with DESIGN `[scalars]` ("a
  continuous value that varies over the box is no choice ... so the
  lane builds it") and that a split parameter already means two things
  by carrier kind. Put to Ev on the PR as: the point stays and
  `[scalars]` is amended, or the lane builds an enclosure; both lean
  the first. The orchestrator's sketch overcounted the door's users
  (nine; it is three or four) and said so there.
- 2026-11-20 — **Ev asked, in chat, what note would have made the
  illegitimate lifts visible without his pokes**, and said the text
  about types being semantically correct is not landing. Answered,
  and proposed as PR 7189 (`[ev]`, own PR since it binds future work):
  a paragraph in DESIGN `[unrepresentable]` that a signature is a
  claim (a bound says where code has a meaning; a conversion says what
  makes its result true; one sentence per admitted type; no gate
  taxes the narrower bound), and three rules in
  `docs/prompts/designer.md` (need before safety; count before ruling;
  the per-type sentence). No designer round, stated in the body.
- 2026-11-20 — **PR 7155 merged** on Ev's ruling (DL5.1 to DL5.3). **The two held questions are with Ev** as a new `[ev]` PR on `scalar/solved-value` (`docs/DESIGN-FORK-LOG.md` row 293; the same two designers, labels redrawn). Two more rounds; they agree. A solved value is not a conversion: an `f64` search proposes a candidate and a check at `T` admits it, so the door takes the candidate and the residual and runs the check, bounded `Certifies`. A closed form stays in `T`. Nothing is owed at a dual, which reaches no solver. No site needs a lift behind `CertifiedEnclosure`. The finite-difference check is recommended as a suite row, not the `debug_assert` Ev asked about: the release profile runs debug assertions, and a difference quotient disagrees with the tangent at a kink.
  - **Dispatched**: `certification-interiors-are-closed-to-a-dual-by-a-marker-bound` (H, dual) and `a-bound-to-decide-against-never-becomes-a-t` (H, single FULL), concurrently; they share files and are logically independent (one changes bounds, the other bodies). `a-place-is-an-f64-argument-answered-on-the-value-channel` (H, dual) waits for the build slot the told-quantity lane holds: the mutex is one wide and four concurrent builders would each crawl.
  - **Friction**: a full lane clone took over ten minutes with two running beside a build; `work.py set` and `work.py lint` each exceeded two minutes once under the same load. Not measured further.
- 2026-11-20 — **Ev ruled on PR 7176.** The door stands ("this broadly sounds good"), for values the body stores only; at an enclosure scalar the admitted value is one number checked over the family, and DESIGN `[scalars]` now says that is the one continuous value a lane takes from `f64`. Ev on the alternative (the lane builds an enclosure of each member's root): "leaving it at A is definitely fine for now although B might be needed to support booleans later", since over a box wider than the tolerance the crossings rarely meet at one place. Filed as `a-stored-solved-parameter-is-an-enclosure-of-each-members-root` (P3, design, measure first). The door is unit `a-solved-value-the-body-stores-is-admitted-by-its-check`, after the marker-bound and places units. DL5.2's place row said "a split"; it now says "the point a cell is subdivided at", since a parameter an edge is split at is stored.

- 2026-11-20 — **PR 7176 merged** (Ev's ruling, above). **PR 7189**
  approved by Ev after four changes ("this looks good!"); merges on
  green.
- 2026-11-20 — `a-bound-to-decide-against-never-becomes-a-t` reported
  as **PR 7185**, hosted CI green on head 2b6ba7b1a4. Single FULL
  review dispatched on that frozen head (`review-brief-7185.md`, nine
  claims; reviewer byte on the analysis branch). The implementer chose
  `Margin<T, A = ()>` with the bound applied by the run's scalar at
  the decision through a hidden `Decide::allowing`; whether that is a
  new door from an `f64` to a `T` is the review's first claim. It left
  five families of bound unmoved and filed each; one is a rule
  question (a bound a `Sym` identity is proved over has no allowance
  spelling). Incident, reported by the lane: one pattern `pkill` run by
  mistake, believed to have hit only its own shell. The places unit
  stays held: it shares files with 7185, whose shape the review can
  still change.

- 2026-11-20 — **PR 7189 merged** (Ev: "this looks good!").
- 2026-11-20 — **PR 7159's fix pass is in**, head e7bd8eb468, hosted
  CI green. All twelve items done; it found a second site of the
  MAJOR's kind itself (a `Via` leg's derived bulge told as authored).
  The pass rewrote more than it patched (about 1,200 lines across
  `told.rs`, `profile` and the gate; the grid now respects float
  spacing for a large length; equality is of what is told; a pair's
  radius lever is an index to its largest corner), so it does not
  merge on the orchestrator's read: a delta re-review went to one of
  the two reviewers, continued in its own lane, with eight claims.
  Only one, because the other's clone is in use by PR 7185's review;
  the pair's comparison was taken on the frozen head and is not
  affected. The row will say so.

- 2026-11-20 — **PR 7185's review is in: NOT-MERGEABLE-AS-IS**, one
  MAJOR, fix pass dispatched to the same implementer
  (`fix-list-7185.md`, seven items). At `f64`, `Interval` and `Dual`
  the allowance was measured bit-identical to the lifted spellings.
  At `Sym` it changed what is proved, both ways: a bound of exactly
  zero compared equal to "no bound" and proved a `Zero` it should not,
  and the scaling doors made a form-zero margin opaque, losing proofs
  the five mate sites had. Also taken: the `spline_trim` floor reorder
  turns a clean "says nothing" into an in-band refusal; `new_scaled`
  reads a negative arm as degenerate; two bounds left that fit the
  form. Nothing argued down.
  - **Class — "none" decided by a value inside a type that says
    "some".** `Allowance == NONE` was a float comparison. Whether a
    thing is present is a fact of its type (DESIGN `[unrepresentable]`).
  - **Class — one rule written twice, half of it moved.** The spline
    floor exists in `topo` and `geom-brep`; the sweep moved one and
    they now clamp in different orders. A sweep's brief should ask for
    the twins of each site it moves.
  - **Open, recorded on the fix list**: whether `Margin<T, A = ()>`
    earns its spread (nineteen funnel signatures) against a door that
    takes the `f64` and returns a sign. The fix pass is told to say so
    if it does not, and not to rework it unasked.

- 2026-11-20 — **PR 7159's delta re-review is in: APPROVE-WITH-FIXES,
  no MAJOR**, on head `e7bd8eb468ec`. The first pass's fixes are
  closed; the rounding core held over 70,336 floats against an exact
  rational oracle with the grid restated independently; the three
  items the implementer argued down stand. A second, small fix pass
  went to the same implementer (`fix-list-7159-delta.md`, six items):
  a read-back refusal tells a derived chain vertex as given
  (`profile/src/path.rs:5351`); a `where` clause with parentheses
  hides a whole type from the gate; a line pin does not stop a pinned
  name being rebound to a scalar; two test rows; one backwards
  sentence in the PR body. Nothing argued down. After it the unit
  merges on green and the orchestrator's read: the remainder is
  mechanical.
  - **Class — a list of checked sites is the checker's own claim.**
    The first fix pass was asked to re-read every given-role site and
    list them; it listed the read-back point "authored, kept", and
    the first review had passed the same site unread. The filed row
    "a role is a constructor name nothing checks" is the durable
    home; unit 2's brief asks for the provenance of each given, not
    a list.
  - **Class — a pin that names a line holds the line's text, not what
    its names are bound to.** Inline captures (`{verb:?}`) make the
    pinned text independent of the value formatted. A gate's header
    must state what its pin holds.

- 2026-11-20 — **PR 7159 merges**: the second fix pass took all six
  items (head `07451c1117e7`, green run 44813916029); read by the
  orchestrator, no third review. Every position a refusal locates is
  now told as derived, and the seven `{:?}` sites go through
  `identifier()`, so the gate holds no pins. The comparison row is
  `docs/model-compare-log/rows/PR-7159.md`. Unit 1 of
  `patherror-display-renders-float-noise` is closed; units 2 and 3+
  (the per-crate sweeps) are next on it, their briefs asking for the
  provenance of each number told as given.
- 2026-11-20 — **Main-red (EXCH):** filed `main-red-endpoint-census-moves-interval-rs-on-the-told-reading` (P1) on this slate. PR 7159's `told_reading` added two endpoint reads to `interval.rs` that `certified_endpoint_census` (slow set) has no disposition for. ONEROUND's PR 7146 moved the same roster line. (EXCH orchestrator)

- 2026-11-20 — **Unit 2 of the told quantity is specced and
  dispatched**: `a-refusal-is-told-in-the-readers-notation` (letter M,
  tier dual, so its reviews run in sequence), into the lane PR 7159
  freed, branch of the same name. Chosen over a per-crate sweep
  because it touches `viewer`, `quantity` and `told.rs`, none of which
  the two DL5 units in flight touch, and because the sweeps should
  build on the notation door rather than be revisited by it. It also
  measures, without changing, how many error types would cross
  clippy's size line with an inline `Told` (the boxing row). The
  positions row and the per-crate sweeps stay open behind it.

- 2026-11-20 — **PR 7185 merges**: its delta re-review is APPROVE, no
  finding above NOTE, on head `840cfead10` (green run 44828404802).
  Against the merge-base spellings the allowance measured identical at
  `f64`, `Interval`, `Dual` (value and tangent) and, in sign and
  `proven()`, at `Sym`; the reviewer could write no `f64 → T` exit.
  The one such step left is the decision's own, private to
  `predicate.rs`. The implementer's two argued-down items were upheld.
  The four notes are on
  `the-margin-allowance-parameter-spreads-to-every-funnel`. The
  comparison row is `docs/model-compare-log/rows/PR-7185.md`.
  - **Class — "a `T` or a bound" as a local enum.** `chain_parity::Gap`
    and `spline_trim::Floor` are two instances, the second grown by the
    fix pass. A third mixed site gives the pair one home.
  - **For briefs on a contended slot**: a local compile that times out
    is not a pass; the fix pass sent two type errors to CI that way.
  - The places unit (`a-place-is-an-f64-argument-answered-on-the-value-channel`)
    is dispatchable now that this has landed.
- 2026-11-20 — (ONEROUND orchestrator) **The endpoint census's `interval.rs` line: ONEROUND is fixing its read now, on `oneround/less-sin-census`.** EXCH filed one main-red row on each of us for the same roster line (`main-red-endpoint-census-moves-interval-rs-on-the-told-reading` here; ours is `less_sin`'s series test, from PR 7146). The test stays red until all three reads are counted, so a fix for one cannot go green alone. Our lane dispositions its own read and, if your two (`told_reading`'s `Reading::enclosure(self.0.lo(), self.0.hi())`) are still unrostered on main when it pushes, counts them too with the disposition your row proposes, after reading them; it then says so here. If you have a lane on it already, a note back and ours merges yours first.
- 2026-11-20 — (ONEROUND orchestrator) **The endpoint census's `interval.rs` line is fixed on main for all three reads (PR 7223).** Your two (`told_reading`) are counted with the disposition your row proposed, which held on the lane's reading: both ends go to `told::Reading::enclosure`, which has no accessor and only renders. So `main-red-endpoint-census-moves-interval-rs-on-the-told-reading` has nothing left to do but close, if you agree with the sentence on the roster. One thing the lane saw and did not file, as a question for you rather than a defect: an uncertified enclosure with ordinary ends is told to a person as its ends, with nothing saying it did not certify.
- 2026-11-20 23:30 — (GERM orchestrator, seam note) **Main red in the nightly slow set, bisected to PR 7159** (`bd8b2c9bb5`, `scalar/told-quantity-type`). `offer_rows::every_offered_tolerance_passes_just_below_it` offers tolerances whose margins don't pass just below them, on three rows; the offered values are the margins rounded to four significant figures. Filed as `main-red-offer-rows-false-offers-since-the-told-quantity-type` (P1) on this slate. The bisect table is in the row.
- 2026-11-21 — **Seam note from SECT:** PR 7193 (`sect/certificate-by-support`, Ev's ruling in PR 7182: a certificate this process derived is read by its bit-identical support) adds one named door to `geom-core` `k_stats`, a replay of a stored `Recorded` into the open frame and sink, so a certificate read in place of its derivation logs exactly the derivation's verdicts and escalations (the ONEHOME drive's lane parity needs it). `Detached`/`Recorded` stay non-`Clone`; `splice` is unchanged. Review the door on the PR if it touches your work. (SECT orchestrator)
