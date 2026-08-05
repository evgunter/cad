# M6 log

M6 = the main-path curved completions (renumbered 2026-08-03, Evan,
PR #169; plan assembled from #161 + #169 — see the orchestrator's
M6-PLAN). This log records units as they land, newest last.

## Unit 1 — the in-place edge-blend composition surgery (head of
## queue per Evan's #169 ruling)

**Scope**: close M5-walk row 8's carried item — THE COMPOSED DIE
(filleted blank + 21 pips + filleted pip rims in ONE body) — via the
surgery the PR 12 review sized at one reviewed unit: in-place face
split along STORED trimlines + ring carry-through + rim-edge →
torus-band replacement. Optional rider (taken): the circle-carrier
definite-miss bound retiring door A's unconditional arm.

**The mechanism** (`crates/sweep/src/fillet/surgery.rs`; routed from
`fillet_edges` whenever the whole-body door refuses, so the M5
whole-body path stays bit-preserved):

- *Open chains* (single convex plane–plane links ending at
  fully-requested trivalent corners): per support face, one strut
  `mev` per boundary vertex to the corner ball's foot + one trimline
  `mef` per edge carve the face into the SHRUNK face (same `FaceKey`,
  same surface/sense via S12 parent-sense inheritance, **rings
  kept**) plus per-edge strips; `kef` across each dying sharp edge
  merges strip pairs; per corner, three arc `mef`s + two `kef`s + one
  `kev` fuse the corner triangles into the sphere octant (the F2
  order-free chart pick, extracted to `octant_chart` and shared with
  the whole-body builder — code motion, arithmetic unchanged).
- *Closed chains* (circular plane–sphere rims): the plane side struts
  to the widened trim circle exactly as above (the ring loop's hole
  WIDENS — the fillet eats into the flat face); the sphere side
  SPLITS the revolve-seam meridian edges where the sphere trim circle
  crosses them (no struts into the cap), then one trim `mef` per
  half-cap; rim-edge `kef`s + strut `kef`s fuse the annulus. A curved
  face must be ring-free (`props`' closed-form inventory — the
  donut's own representation), so the closure vertex dies by a
  fan-merging `kev` that leaves one meridian remnant as the band's
  SLIT: a double-traversed minor-circle `Seam` edge, with the band's
  torus chart seamed at that azimuth (`u_ref` is conventional data,
  D2; Seam certification demands the seam lie in the chart's u_ref
  half-plane).
- *Exactness discipline*: the trim-circle carriers are the rim
  carrier's own frame SCALED (same axis, same u_ref, same parameter
  window; reversal by negating axis + window, never endpoint atan2 —
  π-arc safe); feet and split targets are evaluated ON those scaled
  carriers at the rim's own parameters, so azimuths are inherited,
  not reconstructed. Final pass: surfaces/senses first, then every
  new edge re-described `TangentIntersection` (trims/arcs) or `Seam`
  (slits), then a whole-body `mint_pcurves` re-mint (the input's
  caches are stale the moment the first strut lands).
- *The one new decision*: `fillet3_ring_clearance` — exact
  circle-vs-line / circle-vs-circle clearance between a support
  face's rings and every blend trimline, decided BEFORE mutation,
  typed refusal `FilletError::RingClearance`, two-tolerance on every
  arm (trio-pinned). Everything else is structural. In practice
  predicate 2's sampled screen fires first on the same length; the
  exact form is what the carry-through soundness rests on (sampling
  can overestimate a gap, the closed form cannot).

**The rider (door A)**: `geom_brep::circle_residual_extremes` — the
implicit residual of a circle carrier against a sphere (exact first
harmonic) or cylinder (degree-≤2 harmonic amplitude bounds) over the
whole circle, in meters — and a new `bool_circle_curved_clearance`
trilean in the boolean's curved-face arm: margin `max(lo, −hi)`
positive ⇒ the circle is definitely one-sided ⇒ no wall crossing;
zero/negative keeps the typed pierce frontier; in-band escalates.
Ellipse/NURBS carriers keep the M5 unconditional door. Judging the
full circle for an arc is conservative in the safe direction.

**The composed die** (`crates/sweep/tests/m6_surgery.rs`): cube ∖
21-ball group cut → 12 box edges filleted in place (r = 0.12, all 21
rims carried as rings) → all 21 rims filleted in one call (r = 0.02,
42 arcs → 21 closed chains → 21 torus bands). One body: 129 V / 195 E
/ 89 F, tiers 1–3 green, certified volume on the DERIVED closed form
— Steiner blank − 21·(cap + rim-torus term), the rim term derived by
Pappus over the removed cross-section (triangle minus two circular
segments, both disks tangent at the sphere-side trim point) — at
1e-9 relative with `volume_pad == 0`, watertight under `check_mesh`,
bit-replayed, and in the CI-gated STEP fixture corpus
(`composed_die.step` + sidecar: FreeCAD imports it VALID at
89 faces / 245 edges / 129 vertices, volume 952914984 mm³ —
FreeCAD's own measure reproduces the Pappus closed form to sub-mm³
on a ~0.95 m³ body). The tour gains the `diecomposed` stop
(montage cell 17), green at ×3 ε alongside the two M5 stops it
joins. Deviation 1 is FLIPPED at both doors
(`m5_pr12_die.rs::deviation_1_flipped_*`, S9 pattern — history
kept): door B COMPOSES via the surgery; door A marches past the
retired pierce arm to its REAL frontier, the containment stage's
`PartialSphereFace` door (typed, the M5 PR 9c class — the blank's
octants are trimmed sphere faces with no chart-trim extent). The
rider also un-shadowed a WORKING door: nested sphere∪sphere now
answers (union = the outer ball) through the whole-sphere
containment arm the unconditional pierce refusal used to mask —
probe 7 flipped to pin the answer instead of the mask.

**Deviations, numbered.**

1. *`die_composed` is not corpus-expressible.* `Node::Fillet` is
   every-edge BY DESIGN, and every pipped body carries co-surface cap
   MERIDIAN seam edges, which the battery honestly refuses
   (`TangentialEdge`, margin exactly 0) at any radius. The composed
   die at the recipe layer needs an edge-SELECTION vocabulary that
   does not exist — the N4 fillet-naming emitter, banked in
   `eval/wire.rs::wire_fillet`'s docs since PR 12. The document sits
   BESIDE the registry (the M5 `die_fillet` precedent) with the
   refusal pinned executed in
   `editor-core/tests/m6_composed_node.rs`; the standard
   corpus/persistence/latency rows for the surgery are therefore
   BLOCKED on that vocabulary, and the surgery's live rows ride the
   sweep suites, the interval lane, the STEP fixture corpus and the
   tour instead.
2. *Door A composes only to the containment stage.* The rider's
   definite-miss verdict is real (the far pairs clear; the two sweep
   strategies re-agree on disjoint balls, retiring the divergence
   `die_pips`' docs predicted would retire), but blank ∖ pips then
   refuses typed at `PartialSphereFace` — trimmed sphere faces have
   no whole-chart containment extent. That door pre-exists (M5
   PR 9c) and is now reached honestly instead of masked. The
   composed die COMPOSES via the surgery (door B), which is the
   unit's mandate.
3. *Surgery front-door scope*: multi-link open chains (junction
   carry-through), run-outs at partially-requested corners, concave
   (material-adding) blends, and non-circle-carried rims refuse
   typed, each naming its gap. One-edge (single self-loop) rims also
   refuse (`a closed chain of fewer than two links`).
4. *Torus/cone/ellipse carriers keep the unconditional pierce door* —
   the rider covers circle carriers against spheres/cylinders (the
   closed harmonic forms); the rest still refuse without a clearance
   verdict, stated in the arm's docs.

**Battery.** Touched crates all green at default ε:
sweep + topo + geom-brep = 121 test binaries; editor-core
(m4_pr6_roundtrip / m4_pr8_corpus / m4_pr8_k_probe / m4_pr8_latency /
m6_composed_node) green after the deregistration; step-export all
binaries green (byte-golden with the new fixture; the reversed-face
pins reconciled with #170's die_pips addition at 89 = 5 + 42 + 42).
New rows: 7 (m6_surgery) + 3 (m6_rider) + 1 + loud-skip
(m6_surgery_interval) + 1 (m6_composed_node) + 1 (geom-brep
circle_residual_extremes enclosure/tightness) = 13. Flipped rows: 3
(deviation 1 renamed with history, S13 probe 7, the die_body subset
row's context note) + 3 count/table updates in m5_pr13_curved.
Multi-ε: m6_surgery + m6_rider at 1e-6 AND 1e-12; the interval lane's
one-pip composed die BRACKETED (enclosure width ≤ 1e-6 m³ around the
closed form); tour ×3 ε green. `scripts/check_step.sh`: 13/13
fixtures OK under FreeCAD 1.1.2 (composed_die included). Clippy
clean on sweep/topo/geom-brep/editor-core/step-export + demos/tour;
fmt-all clean. Interval-square tripwire: every new `src/` square is
`powi(2)` (test-file f64 oracles use plain products, per the F6
scoping).

Opened 2026-08-03 at the M5 close (#169). Plan: docs/M6-PLAN.md
(assembled from the #161/#169 ratifications).

**M6 OPENED (2026-08-03).** Plan seeded (assembled from
#161+#169 ratifications — nothing newly proposed; self-merge
class, Evan reviews retroactively). Dispatches: **M6-1
composition surgery (FABLE — block-20 draw byte 161 = (fable,
opus); difficulty M logged pre-draw)** and **montage curation
(OPUS remainder; difficulty S logged pre-draw)**. Surgery spec =
the PR 12 review sizing + walk row 8, embedded in the brief
(the geometry is all landed; the unit is topological).

**Curation COMPLETE (2026-08-04, a7ca822): all seven items.**
16-cell sheets (identical rosters, banners both baselines);
old-die + bracket cells retired (off_sheet lever); rocker on;
two-peg recorded as considered-not-built (waits on cylindrical
declared contact); die_pips in the STEP corpus (48F/138E/71V,
rtol 1e-9; +2 degenerate pole edges per pip vs ball's +4 —
a dimple keeps one pole); the stale-comment CLUSTER (four
sites) fixed. Self-caught bug flagged for review (the dropped
registry row — coverage-total caught it). Infra finds: fmt-all
under load-18 takes minutes → SSH keepalives added to the
lane's pushes (hook never bypassed); render.sh fallback +
rm-before-render hazards recorded. **Light review DISPATCHED.**
Surgery lane still building (the hard unit; checkpoint-push
nudged).

**Curation MERGED as #170 (2026-08-04): the 16-cell
superimposable sheets are live; die_pips in the STEP corpus
(its two corpus-growth pins updated with derivations at the
gate — the reversed-face count and the exactness table). A/B
row: M6-curation (opus, S, APPROVE 0-findings).** Lanes
cleaned. Sole open lane: M6-1 surgery (the composed die).
Evan-notify: the surgery lane will fold this merge (die_pips
fixture + sheet rosters touch its tour-stop work).

**M6-1 surgery COMPLETE (2026-08-04, 031ca25): THE COMPOSED
DIE EXISTS** — every verb of the old shape-(v) criterion on ONE
body (129V/195E/89F, 21 rings; Pappus-derived closed-form
volume at zero pad; watertight; STEP byte-golden;
FreeCAD-valid; sheet cell 17 both montages). Rider TAKEN:
circle_residual_extremes + bool_circle_curved_clearance —
nested spheres now ANSWER, the predicted strategy divergence
RETIRED, door A marched to its real frontier
(PartialSphereFace). Discoveries: revolve-seam half-caps forced
the meridian-split design; props' ring-free law forced the
donut-style Seam slit + chart re-seam; **the composed die is
corpus-INEXPRESSIBLE (dev 1): Node::Fillet is every-edge and
pipped bodies carry co-surface cap meridians (TangentialEdge at
margin exactly 0) — the recipe-layer edge-SELECTION vocabulary
(banked N4 emitter) becomes an M6 unit candidate.** New
predicate fillet3_ring_clearance (trio pinned). **Full
adversarial review DISPATCHED** (independent Pappus
re-derivation; Euler choreography attack; Seam-slit legality
probe; rider falsification; merge-base divergence check).

## HANDOFF SEAM (2026-08-04): successor orchestrator briefing

Predecessor (cad-implement-m5-7plus session) hands off at the
M6-unit-1 seam by Evan's request. State at handoff: M5 COMPLETE
(35 PRs, exit walk 13/7/0, #89 closed at K=10 permanent); M6
open with unit 1 (the composed die) merged or merging (check
gh pr list), curation merged (#170); NOTHING else in flight;
all clones deletable after verifying push-clean (clean-lanes.sh).

**Immediate work order:**
1. Verify the M6-1 surgery PR merged; clean its lanes
   (m6-surgery, m6-surgery-review) via scripts/clean-lanes.sh.
2. Dispatch M6-2: the SSI generic-T lift (M6-PLAN unit 2;
   blocker map = PR 9c dev 2 + the S13 NURBS re-gate; its
   acceptance owns the non-vacuous fitted-cache row). Block-21
   A/B draw; difficulty logged FIRST.
3. Then per plan order: loft/sweep assembly (unit 3, carries
   the analytic-chart pcurves + NURBS-face STEP + shape (iii)
   completion), edge-selection fillet vocabulary (unit 5 — the
   N4 emitter substrate; makes the composed die persistable),
   census/contact design doc (unit 4, design-only), hygiene
   items as lulls (k-lint floor refresh, canary-gated latency
   refresh, internal-tangency fixture).

**Standing process (verbatim-binding; sources in memories/ and
the M5/M6 logs):** one implementer + one blinded adversarial
reviewer + one fix pass; binding specs/contracts BEFORE
dispatch; OUTPUT DISCIPLINE headers; the foreground clause +
"THAT NOTIFICATION WILL NEVER ARRIVE" + blocking-is-fine-
parking-is-not; cwd-reset guard every prompt AND resume;
scripts/new-lane.sh for clones (activates the pre-push
fmt-all hook); iteration-speed local scope (memories/
local-battery-scope.md — no CI mimicry; sweep-shaped units get
the lighter scope); reviewer local runs = unique-signal only,
pins ride the gate; push-per-unit (checkpoint-push on long
builds); NO Co-Authored-By trailers in lane commits (blinding);
watchers carry the CONFLICTING guard (a conflicted PR gets NO
check runs — silent); mergeable never left UNKNOWN; A/B blocked
pairs, difficulty logged pre-draw, blinded reviewers, fix pass
inherits the arm (docs/MODEL-AB-LOG.md; block-20 consumed;
block-21 next); state-sync PRs at seams; two cargo lanes max;
monitors armed at session start (scripts/monitors/ — install
then run from ~/.local/share/cad-work/monitors/; the watchlist
parser fix is IN, entries are whitespace-separated); hourly
sweeps check working-tree mtimes AND cargo/freecadcmd before
nudging; disk watch (~30G/lane); S9 flip pattern for retired
refusals; two-tolerance INCLUDING definite arms; powi(2)
tripwire + Bounds allowlist live in the discipline job.

**Open with Evan:** Q9 (name), #131 (cusps), the PATHS
cusp-variant split; the I1-I3 long-term ideas parked in
docs/LONGTERM-IDEAS.md (I1(0) = the #89 sliver lint).

**Watch out for:** usage-limit outages (#8-#13 this epoch; the
recovery ladder = resume-from-transcript with commit-first
instructions; if a transcript is GONE, fresh finisher per
memories/resume-vs-fresh-subagent.md — happened once at the
surgery fix pass); waiter-parking (5 catches this session —
the sweep+nudge net holds); away-channel self-echoes (your own
comments come back — not Evan); consultations go to a FRESH
issue/PR, never a merged-PR thread; corpus-growth pins (new
fixtures legitimately move corpus-wide counts — update with
derivations, don't relax); the box's CPU can pin at base clock
after sleep (canary: 10M-iteration python sum ≈0.9s healthy,
≈15-19s pinned — Lenovo Vantage fixed it once, no restart
needed).

(Hygiene note for a lull, 2026-08-04: the interval-square
tripwire has false-positived twice on `a * a.dot(x)` — vector ×
projection. A negative lookahead excluding method-call
continuations (`\1\b(?!\s*\.)` in the grep -P pattern, both
ci.yml and ci-local.sh) would retire the class; the two named-
binding restructures stay as they are.)

## Successor orchestrator online (2026-08-04)

Handoff received; #171 (M6-1 surgery) and #172 (briefing) verified
merged; lanes verified already clean; monitors installed + armed
(away-channel, disk watchdog, hourly check-in); CPU canary healthy
(0.75s).

**New Evan instructions at handoff (in-chat, 2026-08-04), both
A/B-EXEMPT and assigned OPUS by Evan's ruling:**
(a) **Bazel verdict** — decide whether the project should adopt
Bazel; if yes, implement it and measure whether it speeds up CI
(CI time is compilation-dominated). Verdict phase dispatched
(analysis-only, no builds); implementation waits for a free
cargo-lane slot and an orchestrator sanity-check of the verdict.
(b) **Globe lily** — a montage piece, dual-purpose: kernel stress
test (careful numbered list of inexpressible / bad-API findings —
expected to be rich, given unit 3 hasn't landed) + creative
exercise with stylistic judgment encouraged. Lane dispatched
(ev/m6-globe-lily; PR held for orchestrator review, not
self-merged).

**M6-2 dispatch prep**: difficulty M logged pre-draw; block-21
draw byte 59 → (opus, fable): **M6-2 = OPUS**, fable remainder
owed to the next A/B-eligible dispatch (expected: unit 3
loft/sweep assembly). Spec substrate exploration in flight;
binding spec precedes dispatch per standing process. The two
cargo-lane slots are M6-2 + globe-lily.

**M6-2 DISPATCHED (2026-08-04)**: binding spec docs/M6-2-SPEC.md
merged to main via #173 (docs-only state sync); implementer on
lane m6-2-ssi-lift, branch ev/m6-ssi-lift, PR held for
adversarial review.

**Bazel verdict DELIVERED (2026-08-04): NO** (report:
~/.local/share/cad-work/bazel-verdict-report.md; no builds run).
The measured case: baseline post-#167 run = 17.6 min wall /
~79 billed min, wall 100% compile-gated — but dependency caching
already fully hits (Swatinem, 225 pkgs warm), and **96% of the
8.6-min build job is the 261 test binaries** (249 declared
`[[test]]` targets ≈330 lines each, each re-monomorphizing the
generic kernel and relinking the graph). Bazel would rebuild
those same actions: touching geom-core invalidates 249/249,
the mid-tier crates 71%; median PR cache-hit on the expensive
actions 0–29%. gmp/mpfr is NOT a CI cost (quarantined excluded
workspace, 0.3–0.5 min). rules_rust pre-1.0 + a third
hand-mirrored build config = real carrying cost for a cache that
misses. **Ranked alternatives** (saving ÷ effort): (1) collapse
249 test crates → ~12 aggregators (−7 min wall, ~−14 billed
min/run, ~half a day, mechanical; nextest sharding unaffected);
(2) `CARGO_PROFILE_TEST_DEBUG=line-tables-only` (one line);
(3) mold/lld; (4) 8-vCPU runner for the two build jobs (billing —
Evan's call); (5) sccache-GHA (same funnel, same misses); cranelift
explicitly NOT recommended (D9 bit-identity risk). Disconfirming
test named: land (1)+(2) and re-measure — if the build job doesn't
drop 8.6 → ~2.5 min, the 96% measurement was wrong and Bazel
reopens. **Orchestrator ruling**: alternatives (1)+(2)+(3) are the
follow-up phase under Evan's add-on (a) (the goal is CI speed;
same lane, still Opus/A/B-exempt, CI-infra class per rows 16/41
precedent — validated by hosted-CI timing, not a blinded lane);
QUEUED until a cargo-lane slot frees. (4) left for Evan.
**Evan APPROVED all four including the bigger runner** (#173
comment, 2026-08-04: "yes, please go ahead with those including
the bigger runner"). Split into: Phase A (config-only, no cargo —
line-tables-only + mold + 8-vCPU on the two build jobs; same
agent, branch ev/ci-speed-config, PR with before/after CI
timings, orchestrator merges) dispatched immediately; Phase B
(the 249→~12 test-crate collapse) held for a free cargo-lane
slot. Also relayed to the globe-lily lane per Evan's same-thread
ask: commit+push renders the moment they're generated.

**CI-speed Phase A MERGED as #174 (2026-08-04)**: mold +
line-tables-only on the two build jobs, job-level env (rust-cache
hashes CARGO_*/RUST* into its key — step-level knobs would desync
the fingerprint forever; one cold rebuild paid at merge). Measured
on the PR's own full-matrix runs: wall 17.6 → 14.2 min (−19%),
billed ~79 → ~67 (−15%), default-build compile 514 → 320 s warm
(−38%), interval −20%; same 261-binary archive in and out.
Item 4 NOT landable: larger runners need an org on Team/Enterprise
and the repo is User-owned — landed as the BUILD_RUNNER repo
variable (unset ⇒ ubuntu-latest), with the recorded caveat that
larger-runner minutes bill SEPARATELY (the report's
billing-neutral assumption was wrong). Runner is 2 vCPU (not 4 —
provenance step prints nproc), which RAISES Phase B's expected
value. Deviation accepted: --no-verify push (fmt hook invokes
cargo; diff was yml+sh only). One GHA flake noted (download-
artifact hang, cancelled+rerun clean) — watch, unrelated.
Evan told on #173. Phase B (249→~12 collapse) still HELD for a
cargo-lane slot; stale #167 watchlist entry cleared.

**Globe lily COMPLETE (2026-08-04): PR #175 open, 27/27 green,
adversarial review dispatched into the freed lane slot.** Eight
closed analytic solids (torus-segment stem turtle, two
truncated-zone lanterns with cone mouths, three extruded two-arc
crescent leaves), every one through the full ladder; sheets grown
to 18 cells both lanes; lily_lantern STEP fixture = the corpus's
first no-pole spherical face (degenerate-edge normalisation 0 —
the control for that pin); reversed-face pin 89→91 with
derivation; +10,462 K-probe samples; `wall_probes` pins seven
refusals LIVE. **The fourteen walls** (full list = PR #175
description): findings 1-2 (tangent curved contacts — G1 tube
unions, flower∪stem) corroborate UNIT 4; finding 6 (bare revolve
un-filletable: seam meridian TangentialEdge margin 0 — no
booleans needed) independently corroborates UNIT 5; findings 8-10
(no sweep/loft Body door, no taper, no petal membrane)
corroborate UNIT 3. New API-ergonomics items for later triage:
revolve axis in sketch coords with SILENT placement risk (11),
unchecked frame orthonormality (12), tessellation δ budgeted per
body by ring radius not feature size — stem 76k triangles vs
lantern 2.3k at 0.53% volume error, visible in the render (13),
no near-tangency distance query (14). Positive: analytic export
round-trips with kernel censuses unchanged (no OCC
normalisation), volumes to ≤1.4e-14 of closed forms. Phase B
STILL held — it restructures test crates across every crate and
would conflict with BOTH open lanes; it dispatches only after
M6-2 and the lily merge.

**Lily review returned (2026-08-04): APPROVE-WITH-FIXES,
0 MAJ / 2 MIN / 4 NOTE — every substantive claim confirmed by
execution.** Highlights: all 8 closed-form volumes independently
re-derived (lantern BIT-IDENTICAL; the leaves' apparent 3e-13
adjudicated to the REVIEWER's formula conditioning — acos- vs
asin-form — with an fsum'd Simpson oracle; the PR's 1.4e-14 is
tight and honest); the silent-placement risk (finding 11) proven
by a mutation witness no existing test catches — the adopted
stored-geometry G1/placement probe fails loud on it; the seven
walls fire verbatim at the claimed sites; the 89→91 pin derived
not fudged; committed renders regenerate pixel-identical.
MIN-1: `wall()` variant-blind (Err-ness only — drifted refusals
would stay green); MIN-2: README "every one carries .F." false
(17 of 20). NOTE-2 is an INHERITED main-side item for the
orchestrator ledger: committed montage legacy cells 1-8 are not
reproducible from the committed per-scene PNGs (FreeCAD-rendered
cells vs matplotlib-fallback files) — identical on main,
pre-existing. Fix pass dispatched to the implementer lane
(adopt review/lily probes + MIN-1/2 + NOTEs 1/3 cheap takes).

**CPU PIN LIVE (2026-08-04, caught by the review's NOTE-4,
confirmed by orchestrator canary: 19.66s vs 0.75s at session
start)** — the box is at base clock, builds ~20×. Evan notified
(terminal push + #173 comment; Vantage poke needed). Lanes
continue, slowly.

**M6-2 implementation COMPLETE (2026-08-04): PR #176 open, all
six spec-§4 acceptance rows reported MET, NINE numbered
deviations (none silent), blinded adversarial review
DISPATCHED** (spec-conformance + rubric; assigned attacks:
the EnvelopeStatement/OnLocusHull adjudication against the walk
row's C2 clause, dev 1's f64-Newton-under-Bounds Interval
semantics, a reviewer-planted second-species corruption, the
Box3 bracket seam, the Copy-drop ripple, sweep completeness).
Implementation shape: Box3 lifted at the SEAM under sole-bound
`T: Bounds` (no allowlist entry needed); projection lifted with
f64 Newton + T residuals (dev 1); certify closure generic with
`SsiCertificate<T>`; `Pcurve::Fitted(Arc<NurbsCurve2<T>>)` with
`PcurveFittedLane` static split (f64/Probe/Interval certified,
Dual refusing typed) and `PropsQuadLane` gaining it as a
supertrait (dev 2/3); `EnvelopeStatement` making the
envelope's claim-form explicit (MapResidual*/OnLocusHull);
UnsupportedCarrier retired via S9 flip; at-rest row + planted
wrong-carrier corruption + Interval enclosure row in
topo/tests/m6_2_fitted_at_rest.rs; vacuity pin renamed to
no_export_corpus_body_carries_a_nurbs_carrier_or_face.
Battery targeted under the CPU pin (dev 9; canary 9.7s at the
time) — hosted CI is the gate.

**M6-2 review returned (2026-08-04): APPROVE-WITH-FIXES,
1 MAJ / 4 MIN / 5 NOTE, 0 silent deviations (clause-by-clause
spec diff), all nine reported deviations UPHELD.** The
adjudications that matter: (1) OnLocusHull is HONEST by the walk
row's own letter — the row's text cites the SsiCertificate
machinery whose limb-2 hull bound has been sup|f_S∘C| since M5
PR 7; the statement enum ADDS honesty. The real residue is now
PINNED by a reviewer probe: a between-samples image displacement
(exact basis locality, all 9 schedule samples bit-identical,
~1e-3 m drift between them) certifies cleanly — the documented
statement boundary; every current consumer of between-samples
images refuses typed. (2) Dev 1's f64-midpoint Newton is SOUND:
certificates claim residuals AT the structural point, evaluated
at T — widened inputs widen and refuse; never understate; the
split-impl would have certified nothing extra while breaking
cross-lane bit-identity of the selected pair. (3) The corruption
rows have teeth — foreign-arc rejected for the RIGHT reason;
reviewer's second-species corruption (sub-interval cache, honest
numbers) caught by the loop-continuity walk, now pinned as the
net that catches it. MAJ-1 = hosted CI RED on one unused import
(topo test), which SKIPPED the whole hosted interval matrix —
row 1's hosted evidence missing; the fix is one line + green
re-run, but the gate is the gate. Fix pass dispatched (inherits
the arm): import + header contradiction + dead sentinel arm
(typed-error preferred) + string continuations + domination-row
on_locus_max + adopt review/m6-2's three probes.
**Banked follow-up (pre-existing, M5)**: probe_tube_chart's uv
pad divides by an UPPER speed bound while its comment claims the
wide-pad direction — flag from this review, not this PR's to fix.

**Lily MERGED as #175 (2026-08-04).** Fix pass: probes adopted
by merge (authorship kept); all seven wall pins variant+payload
strict with three-outcome structure (pinned narrate / MOVED
panic / retired panic); the fix pass CORRECTED THE REVIEW twice
with cross-checked methods (the .F. claim is 13/20 — the review
forgot the four pre-existing zero-carriers; the stored
minor_radius is 56 ulps off, not 4) and caught an
excessive_precision clippy red the probes would have hit at the
gate. NEW finding 15: naming CurvedBooleanUnsupported's payload
type forced a geom-brep dep in demos/tour — topo does not
re-export its own error payload types. Findings now FIFTEEN.
Lanes cleaned (globe-lily 1.9G, lily-review 444M; the review
lane's modified renders were its NOTE-2 regeneration evidence,
discarded after checkout). CPU still pinned (canary 21.4s at
merge). A/B: exempt add-on lane (Evan ruling), no row.
Ergonomics ledger for triage at the next planning seam:
findings 11 (silent revolve placement — world-coordinate axis
or tube_along_arc), 12 (unchecked frame orthonormality),
13 (tessellation δ budgeted by ring radius not feature size),
14 (no near-tangency margin query), 15 (error payload
re-exports).

**tube_along_arc RATIFIED as a unit-3 rider (Evan 👍 on the
#175 design reply, 2026-08-04)** — plan amended in place.
Findings 11 + the minor-radius drift close there; ledger items
12-15 remain for the next planning seam.

**#176 fixture disposition (Evan's design probe on the PR
thread, 2026-08-04)**: Evan questioned the Leg C
refit-a-quarter construction. Orchestrator adjudication after
reading the fixture + APIs: the refit is defensible (public
doors re-derive everything — fit provenance never enters the
certificate's trust chain; it reproduces the fit_branch OQ4
idiom; hands the corruption row its second arc) but NOT the
best available — SsiBranch already carries its own fitted
pcurves and split_at/insert_knot exist, so the fixture can
knot-split the kernel's OWN pair, which is strictly closer to
the walk row's intent; the current comment's knot-split
contrast is also wrong (splitting preserves shared
parameterization; only [0,1] renormalization differs). Added
to the open fix pass as item 7 (split preferred; refit-kept
fallback requires the honest comment; certified numbers must
not move — stop-and-report if they do). The scaffold caveat
stays documented either way: the row re-anchors to a
constructor-built body when the banked join lane lands.
Awaiting Evan's 👍 on the disposition (watchlisted).
Evan APPROVED the split disposition in comment form and amended
the fallback ladder: before any refit fallback, consider letting
the stored fitted parameterization run [0,L] / general [a,b]
with the bounds stored as data (relayed to the fix pass as the
middle rung; each descent requires a concrete stated blocker).
**Evan also asked the fixture question OF ALL NINE deviations**
("that deviation was the one i was most unsure about, but there
were others that felt off too") — a read-only design audit is
DISPATCHED: per deviation, the actually-available alternative
space (the split_at-discovery pattern), verification of each
justification's factual claims against the code, verdict
RIGHT / RIGHT-BUT-MISDOCUMENTED / SHORTCUT / FORK with cost.
Orchestrator rules per item on its report; forks escalate to
Evan.

**Phase B UNBLOCKED EARLY (Evan, in-chat, 2026-08-04: start on
what the live work can't affect).** Scope carve-out replaces the
blanket hold: collapse every crate EXCEPT those whose test tree
#176's diff touches (agent computes exclusions from the PR diff;
expect topo + step-export at least); excluded crates follow in a
small second PR post-merge. sweep (60 targets) + editor-core
(51) ≈ 45% of the win, zero overlap. Validation under the pin:
cargo check --tests per crate + nextest list roster
reconciliation (count-exact before/after); hosted CI is gate and
measurement. Branch ev/ci-test-collapse, same agent.

**Design audit RETURNED + RULED (2026-08-04, posted to #176)**:
7 of 8 audited deviations RIGHT (devs 4/5 explicitly
anti-shortcuts; dev 1's audit note: the SPEC's split-impl
suggestion was the inferior ask — Decide has exactly one method
and Band-routing the structural ε's would add an Indeterminate
Newton arm and pollute the K census). Two finds: dev 7
RIGHT-BUT-MISDOCUMENTED ("Copy is load-bearing" asserted, not
demonstrated — containers are Debug/Clone-only, flows move) →
fix-pass item 8 (honest doc rewrite or compile witness); dev 9
scope gap (local battery omitted geom-brep interval row) → merge
gate includes explicit confirmation the hosted interval shards
run the geom-brep suites. Fix pass = items 1-8 + dev-9
confirmation; merge on fully green matrix. Evan's instinct
("others felt off too") found exactly the two real soft spots.

**M6-2 MERGED as #176 (2026-08-04): WALK ROW 2 IS NON-VACUOUS.**
Fix pass discharged all 8 items + the dev-9 hosted confirmation
(interval shards run 17 geom-brep binaries incl. pcurve_conic).
Item 7 landed at ladder rung (a): the at-rest fixture's carrier
is now cylinder_sphere_ssi's OWN marched-and-fitted curve
restricted by split_at (exact knot insertion; PcurveCache
already stores general [a,b] bounds, so the sub-arcs keep
natural [0,0.25]/[0.5,0.75] domains — the normalization blocker
never bit). The chart image stays fixture-interpolated for a
VERIFIED reason: the ℝ³ implicit lane returns pcurve_a/b = None
(finish_r3) — no kernel-minted image exists to restrict; the
scaffold caveat is documented (row re-anchors when the join
lane lands). MINOR-2 took the typed error (2-line ripple);
shift_branch answers Option (clippy::panic is denied — the
louder-than-clone legal form). One self-caught overclaim
stop-and-reported: the cross-scalar envelope identity assertion
was falsified by the hosted ε=1e-6 row (the tube ladder's
extent evaluates at T and can select a different rung) — now
thinness + on_locus_max dominance with the reason documented.
Blinding note, resolved: three reachable commits carry the
harness trailer (two via the main merge = #174's, one the
reviewer's own probe commit); all 11 implementer commits clean;
no blinded party still active; no history rewritten. A/B row
RECORDED AT MERGE (the M5 readout's discipline). Lanes to
clean; unit 3 next (FABLE, block-21 remainder).

**CONCURRENT M7 ORCHESTRATOR (Evan, in-chat, 2026-08-04)**:
Evan starts a second orchestrator on another account (this
account's Fable limit expected today). Protocol ratified in
chat and recorded in memories/concurrent-orchestrators.md +
the briefing ~/.local/share/cad-work/handoff-prompt-m7.md:
static 1+1 cargo-slot split (cargo-slots.txt), M7 scope fence
(new import crate + tests + M7-PLAN only; export-pin changes
via design-conversation PR), separate sign-off watchlist, A/B
continues with M7-prefixed blocks, GitHub as the
cross-orchestrator channel.

**M6-4 (contact design doc) STARTED EARLY (Evan, in-chat):**
design-only, no lane — Fable design agent drafting
docs/CONTACT-DESIGN.md (C-numbered proposals; census by local
geometry; declared contact as data; ball-and-socket /
interference / cylindrical / G1 tube chains worked; M8
signed-clearance co-design; OQ5 disposition). Orchestrator
meta-review then design-conversation PR — WAITS for Evan.

**M6-4 COMPLETE (2026-08-04): CONTACT-DESIGN RATIFIED (Evan 👍
on #178's affordance comment) and MERGED.** C1–C8 as written
(one orchestrator meta-review fix before the PR: the identity
lemma scoped to its true strength — whole-carrier from a shared
patch for analytic kinds only; per-span for piecewise-rational,
span-partial coincidence escalates). OQ5 is CLOSED — the
CURVED-DESIGN OQ5 entry updated with the closure record.
Unit 4 done in ~5 hours wall from Evan's start-it-now call,
zero cargo-lane cost (design-only). Fresh from the doc:
the two-peg demo's vocabulary now exists on paper (waits on
the C7 join-lane implementation, banked); M8's gap contract
is pinned.

**CI-speed effort CLOSED (2026-08-04): #179 MERGED — the
disconfirming test PASSED.** 251→24 binaries (12 aggregators +
guards); compile step 514s → 320s (#174) → 88s (#179), inside
the predicted 90-120s; wall 17.6 → 6.9 min (−61%), billed ~79 →
~59 (−25%). Roster provably a superset-rename (MISSING=0,
EXTRA=12 = the per-crate every_suite_file_is_aggregated guards
answering the autotests=false silent-drop hazard). One CI-caught
issue (six self-re-exec probes' --exact filters; fixed
layout-independently via module_path!()); scoped duplicate_mod
allows in aggregators only; sweep WAS in #176's exclusion set
(the brief guessed wrong; single-PR fold after #176 merged).
Wall is no longer compile-gated — remaining billed cost is test
EXECUTION, out of scope. Lane cleaned; final table on #174.

**M6-3 DISPATCHED (2026-08-04): FABLE (block-21 remainder),
difficulty L (logged pre-assignment), lane m6-3-loft, branch
ev/m6-loft-assembly, spec docs/M6-3-SPEC.md; brief carries the
#179 aggregator-layout note. Slot 1 claimed in cargo-slots.txt.**

**Fable-limit outage #1 this session (2026-08-04 ~22:20 →
2026-08-05 ~01:30, ended by Evan's /login).** M6-3 implementer
killed mid-Leg-D edit; lane push-clean through e680e81 (Legs
A–C + Band-4 corpus row); resumed from transcript with a
Leg-D-onward brief. The concurrent-M7 arrangement did its job:
M7-2 shipped and M7-4 spec'd during the outage. CPU pin FIXED
during the gap (canary 0.87s).

**Accumulated ratifications during the outage:**
- **UNIT 6 ADDED (Evan 👍 on the #184 triage): the curved
  sense-flip tier gate** — sized S-M, sequenced after unit 5
  (Evan: "no strong opinion on sequencing — do as you see
  fit"). STRENGTHENED by the M7-2 review addendum on #184: the
  props torus arm never consumes sense_sign (inside-out torus
  certifies POSITIVE volume, bit-identical — executed,
  review/m7-2 a1_* probes). Negative controls available
  in-tree: step-import's adopted flip probes.
- **KERNEL_* sidecar fields APPROVED (same 👍)** with the M7
  orchestrator's three refinements locked on-thread:
  full-precision KERNEL_VOLUME_MM3 via the round-tripping
  printer (staleness row asserts bit-exact), KERNEL_* = NATIVE
  census with the kiss_assembly 1/2-vs-2/2 divergence
  documented on KERNEL_SOLIDS, no seam/pole accounting. Queued
  as an S lull unit on my side; M7's consumer switch follows.
- **THE #89 RE-OPEN TRIGGER FIRED (2026-08-05, reported by the
  M7 orchestrator per protocol): the project's first IN-BAND
  LANDING** — fixture cone_trunc (FreeCAD-authored, mm-scale),
  a props_rim-class predicate, from the M7-2 foreign corpus —
  exactly the source the K-REPORT predicted. Nothing retuned;
  Evan owns the re-open decision. M6-side consequence: the
  stale 1.5e-3 k-lint baseline floor refresh (already a named
  M6 hygiene pickup) is now LOAD-BEARING for reading the
  landing's context — promote it up the lull queue.

**FABLE BUDGET NEARLY EXHAUSTED (Evan, in-chat, 2026-08-05):
wind-down mode.** M6-3 implementer instructed to wrap at the
nearest coherent seam (finish only mid-flight work, minimal
battery, push, open an honestly-PARTIAL PR listing unfinished
legs verbatim, stop — no merge). Its adversarial review is
DEFERRED until budget returns (reviews stay Fable per protocol;
the PR sits open, unmerged). Orchestrator entering low-activity:
monitors stay armed, hourly sweeps continue (cheap), no new
dispatches, no spec/exploration work. Remaining M6 queue on
resume: finish M6-3 (remaining legs as a follow-up dispatch) →
review+merge → unit 5 → unit 6 → KERNEL_* fields (S) → hygiene
(k-lint floor FIRST — load-bearing for the #89 landing readout —
then latency refresh, internal-tangency fixture, montage NOTE-2,
tripwire lookahead) → exit walk. The M7 orchestrator
(separate account) is unaffected and continues. from the substrate
exploration (which read post-#176 origin/main): six legs
(builder with EdgeGeometry::IsoCurve + exact iso-pcurve lane;
two tier-3 flips with the placeholder/described discriminator;
volume-only flux with rational walls refusing typed — shape
(iii) is a POLYLINE loft; B_SPLINE_SURFACE_WITH_KNOTS both
forms; analytic-chart completion routing closed-form-harmonic
vs Fitted/OnLocusHull per class; tube_along_arc rider with
bit-exact storage pin). Dispatch (FABLE, block-21 remainder,
difficulty L logged pre-assignment at task creation) WAITS for
the ev/ci-test-collapse PR to merge — the collapse is
restructuring the exact test trees this unit touches, and my
one cargo slot (under the 1+1 split) is occupied by it.

**M6-3 PARTIAL SEAM REACHED (2026-08-05, PR #192 open, UNMERGED,
branch dbf9e82): Legs A-C + the Leg-D writer half DONE** — the
loft/sweep builder (IsoCurve seams, Pcurve::IsoLine +
MapResidualIsoHull, both tier-3 flips via is_placeholder /
Seam-idiom exemption, the exact i128-rational Newton-Cotes flux
door at 0.3s, B_SPLINE_SURFACE both writer arms, Band-4
loft_prism at V=9 bracket-pinned, tour narration live). NOT
STARTED (listed verbatim in the PR): Leg D fixture half, Leg E
(chart completion), Leg F (tube_along_arc), §7 remainder,
dedicated Interval rows. Six numbered deviations — note dev 1:
NURBS surface AREA is SUPPLIED (fixed-resolution hull enclosure
with honest area_pad) because check 7's meter consumes it — the
spec's own anticipated reportable case; the eventual review
should attack that enclosure. Dev 3 ReversedStacking refusal;
dev 6 = the wrap itself. Full battery green at default ε; one
pre-existing aggregated-binary flake noted (tolerance_init,
moot under nextest isolation — a #179 follow-up candidate).
RESUME PLAN: a follow-up dispatch finishes the listed legs on
this branch (warm lane m6-3-loft held, slot 1 idle), then ONE
adversarial review covers the whole unit, then merge + A/B row
(fable, L).

## SOLE-ORCHESTRATOR PICKUP (2026-08-05)

The M6 orchestrator is done (Evan, in-chat); the M7 orchestrator
is sole orchestrator and picked up PR #192 per Evan's explicit
instruction. Split wound down: both cargo slots, M6 files
unfenced, single watchlist (memories/concurrent-orchestrators.md
updated). State found: M6-3 partial delivered as #192 (Legs A-C
+ D-writer, 6 numbered deviations, early wrap on a budget call,
held for review; hosted CI 6 RED: clippy expect() in
m6_loft_body.rs, discipline job, interval build+archive, shard
1/2 at all three ε rows) — the M6 session died between the PR
opening and its log entry, so this is the delivery record.
COMPLETION DISPATCHED: fresh implementer (the dead session's
agents are unresumable), FABLE — the arm the M6 orchestrator
assigned at block-21 (fix pass will inherit it); warm lane
m6-3-loft reused; brief = fix the 6 reds first (targeted local
reproduction), then the PR body's NOT-started list verbatim
(§4 remainder, Leg E analytic-chart completion, Leg F
tube_along_arc rider, §7 sweep incl. the lily.rs stale line,
§9.3 interval rows); deviations numbered from 7; ONE adversarial
review of the whole unit follows completion. M7-4's fix pass
runs concurrently on slot 2.

**M6-3 completion DELIVERED (2026-08-05): PR #192 fully green
(27/27 on dd4131d), every spec leg + every NOT-started row
landed; whole-unit blinded review DISPATCHED.** Six red jobs
root-caused and fixed first (clippy/interval test-side allow;
quad.rs x*x → powi(2); CARGO_TARGET_TMPDIR create_dir_all at
both dump sites). Walk row 4's closer landed: cone/sphere/torus
charts certify and mint closed-form classes; ball/cone/donut +
die octants carry stored pcurves AT REST; sphere general
circles via certify_fitted's Circle-carrier rational-chain arm
(OnLocusHull) pinned f64+Interval. tube_along_arc rider: inputs
bit-exact (56-ulp drift retired ==), Pappus both scalars.
Deviations 7-9 (executed blockers): mint-side fitted wiring
banked (oblique trihedron legally uncached), tour SceneBody
Stop banked (no NURBS tessellation lane), merge_coplanar_faces
re-mints. **Discovered + fixed in passing: F5** —
pcurve_chart_radial_moving's r²-scaled metering (the collision
that made rim-dim defer it dissolved when this unit rewrote
pcurve_cache.rs) — the FreeCAD corpus ceiling moved 1e-8 →
1e-5, table rewritten composed with the rim-dim retirement.
Banked dimensional unit SHRINKS to F3+F4. Completion ~642k
tokens, ~7h wall under the pin (incl. one phantom-CI
misdiagnosis corrected by the sweep — the "unbuilt" pushes had
built and failed). Review attacks: Leg E derivations + walk
row 4 falsification, the F5/ceiling composition, dev 1/3/9
adjudication, cross-unit composition with step-import, the
two-hands quality-seam probe.

**M6-3 review returned (2026-08-05): APPROVE-WITH-FIXES,
0 MAJ / 5 MIN / 4 NOTE, rubric 5/4/4, all 9 deviations honest,
no gate weakened.** Leg E's cone-nappe/sphere-pole/torus algebra
independently RE-DERIVED BY HAND — all correct (the pole-start
d̂-flip/σ compensation exact); V=9 re-derived and oracle-exact;
the 1e-8→1e-5 ceiling move composes correctly (freecad suite
executed at 1e-6/1e-7 certify + 1e-4 obligation arms); hosted
27/27 verified. MINORs all honesty/coverage class: two
error variants shipped untested (reviewer probes now fire
them), an executed-false comment (pole-crossing meridian arcs
DO certify), a "does drift" comment measuring 0 ulps, the
stale F5 audit row, a stale step-import chart-list comment.
Notable NOTE banked with the PcurveFittedLane item:
merge_coplanar's re-mint would silently drop FITTED caches on
a merged fitted-cache body (latent, no current path). Quality
seam between the two build phases DETECTABLE BUT WEAK: all four
honesty defects sit in completion-phase scope; the partial's
files yielded none under the same scrutiny. Light fix pass
dispatched (inherits fable). Review ~231k tokens, ~2h under
the pin.

**M6-3 MERGED as #192 (2026-08-05): WALK ROW 4 IS CLOSED and
loft/sweep bodies are LIVE.** Fix pass discharged all items
(stacking probes adopted by --no-ff merge; the pole-crossing
comment now tells the demonstrated truth; 0-ulp measurement
stated; F5 audit row FIXED(M6-3) + the adjacent F6 bullet
honestly advanced rather than left stale; step-import's chart
list current; the merge_coplanar fitted-cache hazard named at
its site). Hosted 27/27 on 427e54d. A/B row RECORDED AT MERGE
(fable; the partial's tokens are unrecorded with the dead M6
session). Lanes cleaned (11G freed). M6 remaining: unit 5
(edge-selection fillet vocabulary), the proposed sense-flip
unit (#184 item 1, latitude given), hygiene (k-lint floor).
The merge UNBLOCKS: M7 unit 3 (NURBS-face import — the export
side now emits B_SPLINE_SURFACE_WITH_KNOTS both arms),
KERNEL_* sidecars (15 incl. loft_prism), F3+F4 dimensional
unit, typed-margin design draft.

**Stranded-state salvage (2026-08-05, branch-cleanup sweep at
Evan's ask)**: the dead M6 session's last three commits (71
M6-LOG lines — the Fable-limit outage record, the accumulated
ratifications, wind-down mode, and the M6-3 partial seam with
its resume plan) never reached main; salvaged by chronological
keep-both merge. STATE CORRECTION the salvage surfaces: **M6
unit 6 (the curved sense-flip tier gate) is RATIFIED** (Evan 👍
on the #184 triage during the M6 session's outage — not merely
sequencing latitude as this log's pickup entries assumed);
sized S-M, sequenced after unit 5, strengthened by the torus
addendum, negative controls in-tree. Also confirmed by the
salvage: the pickup's completion independently followed the
stranded resume plan (follow-up dispatch on the warm lane →
one whole-unit review → merge + fable A/B row) to the letter.
The k-lint floor refresh keeps its promoted lull-queue spot
(the landing it was to contextualize is now retired by #197,
but the floor is stale on its own terms).
