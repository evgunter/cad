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
