# Work-track proposal — 2026-09-03

Non-binding planning survey, the successor to the 2026-08-29 stream cut
(`docs/DOC-LEDGER.md` sweep 4 says where that one went). It reads every
open `kind: issue` file in `work/` — the 78 unowned ones under
`work/issues/` and the 133 that live programs hold — plus the 168 open
code-quality rows, rulings and findings, and proposes how the work that
no live program is going to carry divides into new programs
("tracks") under `work/`.

The cut is on two axes at once. **Territory**: each track owns a file
territory that no live program's `paths` cover, and where it cannot
(three places), the overlap is stated. **Class**: each track is
homogeneous in what it demands —

- **E (easy)**: the fix is written in the item or obvious; one PR per
  item; no open design question, no numerics or topology subtlety. An
  E track dispatches straight to implementers and takes the S-TCOST
  review posture: batched style review, no A/B row for infra-only or
  test-only units.
- **D (design)**: a question with several viable answers, a
  `DESIGN.md`/API-shape choice, or a body that says "conversation
  before unit". A D track is Ev-paced: it opens with `[ev]` PRs and
  hands the builds to an E or H track once ruled.
- **H (hard)**: the intended behaviour is clear but getting it right is
  technically difficult — geometry, certified-interval reasoning,
  topology invariants, tests that are hard to construct. An H track
  runs the full v6 dual with Fable specs.

Where a territory holds all three classes (the viewer, fillet), the
proposal splits the E lane from the D/H lane into two tracks and says
which lands first, or keeps one track and names the E openers as the
first units. Every item is cited by its `work/` file id.

A track graduates the way streams did: open `work/<id>/` by hand
(`program.md`, `plan.md`, `log.md`), claim the next free A/B band
(1500 upward, `docs/MODEL-AB-LOG.md`) in the opening commit, and
re-parent its items by editing their headers. Nothing here is a
commitment.

**All eleven graduated on 2026-09-03** (Ev, in-chat); each section
below carries its opening record, and `work/<id>/plan.md` supersedes it
as the charter.

## What changed since the August cut

- **Five programs closed on 2026-08-31** (GUI, GAUTH, S-BLEND, S-QA,
  PCURVE; sweep 5) and left their residue in `work/issues/`. The viewer
  (18 issues), fillet (9), the CI/workflow ground (~18) and the pcurve
  frontier (2) are unowned again.
- **Two programs are at exit today**: S-MATE (slate complete,
  `MATE-EXIT` on Ev since 09-01, 13 issue files) and M10 (M10-6 merged
  today, 7 issue files). `work/README.md` requires their issues to be
  re-homed before the directory sweep, and neither plan names a
  landing for more than one of them. S-CERT (2 units left, 20 issues)
  and VERBS (3 units, 20 issues) are next.
- **No program owns**: `crates/viewer/` (23.7k lines), `crates/quantity/`,
  37 `crates/topo/src` files (`body.rs`, `pcurves.rs`, `contact.rs`,
  `separation.rs`, `transform.rs`, …), `.github/workflows/`,
  `local-scripts/`, 23 of 41 `scripts/`, the `demos/` Python and shell,
  and root manifests. Owned by a code-quality track but by no program:
  `step-import`/`step-export`/`stl` (U), `tools/` (K), the sweep
  blend/fillet/chamfer files (T).
- **Of the code-quality letters, only K is live.** M, N, Q and R were
  never claimed as tracks; their rows are filed into them by S-CERT,
  S-BOOL and S-MESH lanes working the same files. T finished its lanes
  on 08-31 and has four `blend/` rows parked on a PR that has merged.
  U and V stopped 09-01 with named successor pickups (D343, D366; G4's
  trigger has since merged). The unlettered pile (63) is thematic, not
  territorial — the two themes with program mass are error-payload
  honesty (§3a of the survey) and CI-instrument blind spots (§3d),
  and both map onto tracks below.

## Territory already occupied

Live programs and what they keep (from `program.md` headers; globs are
`fnmatch` where `*` crosses `/`, so `crates/*/tests/*` is every test
file and `crates/geom-core/src/*` reaches `linalg/`):

- **S-BOOL** `topo/boolean/*`, `topo/splitting/*`, `crates/profile/*`,
  `editor-core/resolve/vdiff.rs`, `sweep/loft.rs`. Live: BOOL-12
  (review), BOOL-9/10 (spec), BOOL-4/5/6/7, BOOL-Q. Its keep_out cedes
  the germ arms, SPHSPH/CYLSPH, #1031 half B and #1076/#1077 to VERBS.
- **S-CERT** `geom-brep/props/*`, `offset_fit.rs`, `patch_bound.rs`,
  `geom-core/src/*`, `geom/src/*`, `bvh/src/*`. Live: CERT-M3, CERT-N3,
  then exit.
- **S-MESH** `crates/mesh/*`, `topo/coherence.rs`. Live: MESH-12
  (review), MESH-R, MESH-9 (parked).
- **VERBS** `geom-brep/intersect.rs`, `ssi*`, `offset*.rs`,
  `topo/{offset_*,shell,replace_face}.rs`, `sweep/revolve/*`. Live:
  CYLSPH (review), C5ARMS PR-2, CONE (unspecced); the log calls CYLSPH
  the last register unit.
- **SEAT** `crates/verbs/*`, `topo/{query,flush}.rs`,
  `editor-core/{verbs/*,names/geompred.rs,names/flush.rs}`. Live:
  SEAT-6, then the uncut per-verb migrations.
- **LIB** `crates/pncad/*`, `crates/pncad-py/*`, three design docs.
  Open-ended; its census B-family slate completed today.
- **S-TCOST** `crates/*/tests/*`, `crates/test-utils/*`,
  `scripts/{ci-filter.py,slowest-tests.py,base-test-listing.sh}`.
  Open-ended, census-driven, very live.
- **M10** at exit; **S-MATE** at exit; **PERF** a register with no
  units.
- **Code-quality Track K** `scripts/gates/*` (less two), `tools/*`,
  `docs/K-REPORT.md` — live.

## Housekeeping before any track opens

Cheap, and every later count depends on it.

- **Duplicates, one file each**: `render-lanes-checkout-merge-ref-vanishes`
  and `render-lanes-red-at-missing-merge-ref` are one finding (GH 1607);
  `pncad-py-python-feature-clippy-lane-is-red` and
  `the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row` were
  one (LIB landed the row and closed both on 09-03); LIB's `epsilon-has-no-type-of-its-own` and
  `step-writer-hardcodes-user-header-fields` duplicate code-quality
  `C13`/`C14` (which are parked on them); LIB's
  `facade-polygon-door-demoted-without-replacement` is `S79` #759;
  `D107` and `d107-release-profile-job-lives-in-nightly` are one
  question; `D102` and `bounds-tripwire-blind-to-named-alias` are one
  defect.
- **Stale-open, verify and close**: `chamfer-has-no-recipe-layer-door`
  (G16 landed `Node::Chamfer`), `sccache-trial-verdict-to-read` (verdict
  written, PR 1648 in review), `orthonormal-basis-poisons-vertical-planes`
  (the fix is on main at `vec.rs:407`; pin and close).
- **Stale parks**: Track T's `D322`–`D324` on merged #1360; `G4` on
  merged #1647; `shell-curved-wall-clearance-window` on merged M10-5.
- **Registers**: `m6-carried-items-register` re-homes each row to its
  owner or closes it; `m6-sense-gate-recorded-residuals` is an H item
  for the PROPS successor below.
- **The two exits**: the re-home tables at the end of this document
  give every S-MATE and M10 issue a landing, so `MATE-EXIT`'s
  ratification and M10's exit walk can sweep their directories
  without leaving residue behind.

## Proposed tracks

Eleven candidates, grouped by class. The first six sit on ground no
live program owns and can open today; the last five are successors that
open at a named program's exit and are listed so the re-home tables
have targets. Counts are items, with the class tally in brackets.

### Easy tracks

#### CIW — hosted CI, workflows and scripts (`ciw/`) — 17 items [E 15, D 2]

**GRADUATED (2026-09-03): opened as `work/ciw/`, A/B band 1500–1599.**

The S-QA ground, unowned since 08-31, plus the perf emitters. Territory:
`.github/workflows/*`, `scripts/*` less Track K's `scripts/gates/*` and
S-TCOST's three, `local-scripts/*`, `demos/*.sh`, `demos/*.py`,
`docs/perf-data/*/README.md` by courtesy of PERF, and one line of
`crates/viewer/Cargo.toml`. Cleanly disjoint from every live program.

Order: `main-latently-red-at-tier-all` (the viewer bin/lib rustdoc
filename collision reds every TIER=all run — rename first) →
`render-lanes-red-at-missing-merge-ref` (checkout by `github.sha` or
skip-with-reason) → `retire-render-automatic-matplotlib-fallback` (Ev's
ruling is recorded; strict step order in the body) →
`hosted-renderer-announces-itself-preview-only` →
`nightly-pin-reading-idiom-four-copies` (one `scripts/` reader) →
`mirror-parity-never-compares-flags` →
`python-suite-zero-test-guard-three-copies` →
`committed-conflict-markers-reach-main` (tree-wide marker grep) →
`bounds-tripwire-blind-to-named-alias` (close `D102` with it) →
`cache-rendered-cells-on-input-hash` →
`d107-release-profile-job-lives-in-nightly` (closes `D107`'s CI half) →
`rustdoc-gate-disagrees-with-workspace-doc` (align `doc-gate.sh` flags
with a contributor's `cargo doc`; the one-line topo link fix is Track
Q ground, by note) → `geom-brep-test-unused-edgedescription-import`
(with an `--all-features` clippy row) →
`the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row` (a
`--features python` clippy row on the gate; LIB clears the one lint) →
`perf-history-cannot-identify-its-host` → `sccache-trial-verdict-to-read`
(merge 1648, close).

Design, split out as `[ev]`: the class half of `main-latently-red-at-tier-all`
(what a main push re-gates after the F3 trim) and
`facade-guards-defer-to-rustdoc-json` (a nightly rustdoc-JSON scan or
declare the text scans permanent). `doc-gate-two-unread-axes` is a
measurement then a one-paragraph call. `rustdoc-gate-private-intra-doc-links`
stays parked on its trigger (a public doc set or Q9).

Overlap flags: `clippy-panic-gate-blind-in-macros` and the two
`gated-marker-*` items are Track K / S-TCOST ground (`scripts/gates/`,
`ci-filter.py`) and route there, not here. Code-quality's survey notes
K's entire open slate is this track's theme; absorbing K when its
current lane finishes is an option, not part of this cut.

#### CHROME — viewer chrome and coverage (`chrome/`) — 10 items [E 10]

**GRADUATED (2026-09-03): opened as `work/chrome/`, A/B band 1600–1699.**

The small viewer fixes and the coverage rows CI cannot see. Territory:
`crates/viewer/src/*` and `crates/viewer/tests/*` (the tests glob is
also S-TCOST's and Track W's, by declaration). Nothing else edits the
viewer; the only seam is with VIEW below, which edits the same files
later.

Order: `viewer-render-pipeline-creation-untested` (a lavapipe smoke row
constructing every `viewer::gpu` pass — the tripwire a shipped startup
panic proved necessary; check whether the `EdgePass` fix on
`claude/subagent-gui-integration-tests-i153yl` merged) →
`viewer-chrome-not-in-nextest-archive` (measure `--features app` in the
archive vs a small job; S-TCOST's keep_out on CI build knobs applies) →
`placed-union-has-no-session-op` → `probe-bounds-lacks-driven-slot-guard`
→ `pickindex-per-part-window-twins` (before any vertex-pick unit) →
`viewer-mate-tool-refuses-pattern-picks` →
`refused-mate-badges-every-instance-row` (if provenance must thread
through the solve, it becomes a CURVED rider) →
`doc-params-carry-no-display-unit` (mirrors `SetSlotUnit`; one persisted
field — announce on the persist-schema seam) → the viewer half of
`error-types-with-no-display-class` →
`viewer-first-light-on-real-hardware` (an Ev-run checklist, not a lane).

Sequencing rule: CHROME lands before VIEW's module split ratifies, or
its remaining items ride the split.

#### FIX — kernel and façade doors with the fix written (`fix/`) — 15 items [E 14, D 1]

**GRADUATED (2026-09-03): opened as `work/fix/`, A/B band 1700–1799.**

One-PR items whose body already contains the fix, on ground no live
program is working: the unowned `topo/src` files, `census.rs` and
`editor-core/{mate,assembly}.rs` once S-MATE's directory goes,
`editor-core/checks.rs`, `program.rs`, `refactor.rs`. This track spans
crates by nature; each item's fence is flagged and each PR stays one
item.

Items: `transform-rigid-refuses-described-nurbs` (refuse on
`is_placeholder()`, map control points; the point-map helper is Track N
/ S-CERT ground) · `census-decline-consults-one-face-of-pair` and
`interior-witness-budget-decline-untyped` (one sitting, same exhaustive
matches; SMELL-UV routed the topo half to S-BOOL — take it back
explicitly) · `tier-3-prime-findings-render-through-debug` (`validate.rs`
is Track P) · `subject-body-drops-the-declared-contacts` ·
`unit-admits-non-finite-direction-norm` (one line + red-first row;
`eval/wire.rs` is LIB/SEAT ground by keep_out) ·
`mate-contradiction-names-one-mate-twice` and
`pin-mismatch-recourse-emitted-twice` (the demo pin and Python pin flip
in the same change; `workspace.rs` is LIB's) · the kernel half of
`error-types-with-no-display-class` (`topo/{contact,readback}.rs`,
`mesh/types.rs` — S-MESH's, `quantity/fmt.rs`) ·
`no-parametric-loop-constructor` · `coherence-findings-have-no-consumer`
(a `CheckId::ChartCoherence` resident; the step-import diagnostics half
goes to EXCH) · `unify-discipline-machinery-onto-registry` step 1 only ·
`split-crossings-skip-pattern-mate-ends` · half (1) of
`mate-clocking-has-no-gui-path` (refuse the statically-contradictory
clocking rider at `AddMate`) · `boolean-error-has-no-fieldless-kind`
(`PathErrorKind` precedent; `boolean/mod.rs` is S-BOOL's glob, off its
slate) · `band-linear-spelling-not-swept` (pure spelling; profile,
sweep, tests).

The one D: `nested-pattern-mate-heads-refuse` is a one-sentence ruling
(nested heads compose, or ratify the single-level fence) then a small
PR either way.

Not here, deliberately: `boolean-declarations-has-no-geometric-producer`
(door-shape decision first; goes with CURVED), `direction-normalization-two-doors-one-home`
(SEAT's while open), the S-TCOST-shaped test items (routed back below).

### Design tracks

#### DOCM — the document model (`docm/`) — 16 items [D 13, E 2, H 1 after ruling]

**GRADUATED (2026-09-03): opened as `work/docm/`, A/B band 1800–1899.**

The editor-core document layer: the persisted recipe vocabulary, the
`DocEdit` set, document identity, and the frames and selectors the
viewer and the mate tool consume. Today three programs call this
ground "contended" or "a design conversation before a unit" (S-BOOL's
keep_out on the persist schema; LIB's on recipe doors, `evaluate`'s
signature and the resolver door; M10's on eval/schema outside the
analysis lane) and nobody owns it. This track is the owner. It opens
with `[ev]` PRs, one per question, and hands each build to FIX, CHROME
or VIEW when ruled.

Territory to fence: `editor-core/src/{persist/*,program.rs,doc.rs,
edit.rs,node.rs,names/role.rs,eval/parts.rs}` and `resolve/*` less
`vdiff.rs`. **This is a direct collision with code-quality Track V**
(whole of `editor-core/`; its `C6`, `D365`, `D366`, `D367` sit on the
seam) and adjoins LIB (bindings and the doors design doc), SEAT
(`verbs/*`, `names/geompred.rs`, `names/flush.rs`), M10's analysis
lane, S-MATE's `mate*`/`assembly.rs` (until its sweep) and S-BOOL's
`vdiff.rs`. The fence has to be drawn in the opening PR, with V ceding
the named files; that is the one thing this cut cannot do for it.

The questions, in the order they unblock the most:

1. **Persisted-variant compatibility** — what adding or renaming a
   persisted enum variant requires (forward-compat stance, migration).
   Unblocks `capend-top-bottom-contradicted-by-negative-extrude` and
   `fused-step-slot-aliases-arrival-spec` (both then E).
2. **Frames** — `sketch-frame-from-face` (already `needs_ev`: frozen
   `Datum::Frame` vs a derived-frame datum over a `StableName`, plus a
   carrier-kind interrogation door vs any-face wording). Unblocks
   `add-profile-mints-no-frame` and
   `add-profile-placement-on-picked-face-frame` (CHROME builds) and
   `no-door-mints-mate-frame-from-face` (LIB holds the plan; the
   hand-off S-MATE's keep_out required is recorded here).
3. **Operand selectors** — `split-side-and-pattern-instance-as-operand`
   (a part-selecting operand vs a projection node).
4. **Deleting from a chain** — `no-docedit-splices-a-deleted-node`
   (survivor policy per node kind, typed refusals, schema bump; the
   build is H).
5. **Document identity (LIBRARY-DESIGN A4)** —
   `save-a-copy-duplicate-id-bricks-store`,
   `memo-admission-and-resolver-state` (D→H build),
   `document-seam-no-in-session-change-detection` (store refresh shape,
   save-as warning, chooser vocabulary).
6. **Layer-3 identity across rewinds** —
   `layer3-recipenodeid-aliases-across-rewinds`: one rule for every
   holder (generation stamp, clear on replacement, or stable names);
   the viewer build goes to VIEW.
7. **Free-move commit** — `no-persistent-setplacement-session-op`,
   constrained by the G3 ratification.
8. **Revolve naming D1** — `revolve-pole-export-interior-on-axis-vertex`
   (what the pole export yields for an interior on-axis vertex now
   editor-reachable; one row per direction after).
9. **The instantiation seam** — `instantiation-seam-drops-mate-identity`
   (narrows the Q1 ruling's letter; carry `MintedDeclaration` across
   `PartValue`).
10. **The check registry's subject** —
    `check-registry-gathers-product-twice` (run_checks computes the
    product once; the fix edits `assembly.rs` and `product.rs`'s Dual
    arms — announce to M10's successor).
11. **A certified range query** —
    `certify-locally-valid-range-instead-of-sampling` (M10 residue: a
    slot-widening override, an indeterminate-means-subdivide verdict
    contract, pacing; the build reuses M10-3's driver).

E riders that need no ruling: `unify-discipline-machinery-onto-registry`
step 2 once the parameter-coincidence unit exists (DS8's rule), and
half (2) of `mate-clocking-has-no-gui-path` (a rotate-mate affordance
or documented roll conventions — a UX call).

#### VIEW — viewer architecture (`view/`) — 7 items [D 5, D→H 1, E 1]

**GRADUATED (2026-09-03): opened as `work/view/`, A/B band 1900–1999; dispatches after CHROME's slate.**

The viewer's structural questions, gated by one conversation. Same
files as CHROME (`session.rs`, `app.rs`, `pick.rs`, the `*tool.rs`
modules); disjoint from every program. CHROME's items land first.

Order: `viewer-session-god-module-split` (module boundaries for a
3,060-line `session.rs` and 5,520-line `app.rs`, `Refusal` delegation
discipline, gesture-safety as data, `Option<OpenTool>`; ratify into
`crates/viewer/README.md`, then an L-size mechanical refactor) →
`pick-priority-filter-vocabulary` (GQ7: a per-kind admission set;
where filters are offered and what the picture shows) →
`camera-fold-clears-status-line` (status-line ownership: typed/ranked
status vs badges) → `focus-marking-is-per-node-not-per-segment` (the
authored-step → canonical-segment map door beside the lowering is
`crates/profile` — S-BOOL's glob, by note — then a focused-slot state)
→ the viewer builds of DOCM's layer-3 rule and free-move answer →
`pick-index-built-on-ui-thread` (D→H: revise the GUI-3 §5 seam, then
tessellation and `PickIndex::build` on the `EvalService` worker with
cancel-and-restart; PERF cedes hover-picking to the viewer).

### Hard tracks

#### FILLET — blend completion, second pass (`fillet/`) — 11 items [H 4, E 3, D 4]

**GRADUATED (2026-09-03): opened as `work/fillet/`, A/B band 2000–2099.**

S-BLEND's residue. Territory: `crates/sweep/src/blend/*`,
`sweep/src/{fillet,chamfer,extrude}.rs`, `sweep/tests/blend*`, and
`crates/profile/src/{path/arc_fillet.rs,fillet_select.rs}` (S-BOOL's
`crates/profile/*` glob covers these for the PATHS lattice only —
announce, as S-BLEND did). VERBS' keep_out says fillet band and
surgery "stay ceded". **This track claims code-quality Track T whole**,
as S-BLEND did: its four live `blend/` rows `D322`–`D325` come with it
(the park on #1360 is stale), and Track M's `S90-impl` reaches these
files by a recorded cross-fence.

E openers, first: `fillet-nonpositive-radius-false-fact-refusal`
(chamfer's `NonpositiveSize` check on `fillet_edges`) →
`recourse-sentences-owe-followability-pin` (a composed pin that executes
every `FILLET*_RECOURSE` sentence — it also inventories which recourses
lie) → `bare-f64-margin-payload-family` (named payload shapes in the
fillet battery; the `edge_nurbs.rs` twin is Track Q ground).

H: `concave-closed-rim-has-no-band` (the material-adding closed-rim
band: relax `resolve_rim`'s convexity gate, orientation-agnostic
excise/merge walk; L) → `repaired-pole-rim-serves-no-closed-door` (one
planar host face carrying several rim arcs of one circle; the p4 probe
is the acceptance row) → `extrude-cap-rim-smooth-arm-noop` (construct
an input reaching the no-op arm or state the true unreachability
argument; S) → `fillet-ruled-spine-arms-no-surgery` (curved-support
trimline descriptions and chain terminations; blocked on the OQ6
run-out taxonomy — last).

D, as `[ev]`: `corner-config-tag-all-concave-trihedron` (already on
Ev), `nocornersidecandidate-has-no-producer` (delete the variant or
keep it with a stated reason), `fillet-refusal-describes-unbracketed-crossing`
(an attribution rule for the arc-carrier refusal channel), and the
consumer door `no-public-rim-arc-selector` (key shape: carrier circle
vs named entity; it lands in `topo/query.rs` or `names/`, which is
SEAT's — coordinate).

#### EXCH — exchange: STEP and STL (`exch/`) — 8 items [H 3, D 3, E 2]

**GRADUATED (2026-09-03): opened as `work/exch/`, A/B band 2100–2199.**

The I/O crates have had no program since M7 closed on 08-09; Track U
holds four rows there and LIB holds two duplicates of them. Territory:
`crates/step-import/*`, `crates/step-export/*`, `crates/stl/*`, and
the `ExtrudedPoint` rung of `topo/src/pcurves.rs::nurbs_iso_derive`
(unowned; shared with TRIM — TRIM owns the file and EXCH files its rung
as a row there, or the reverse, decided at opening). `geom-core/spline/compose*`
is S-CERT's glob and Track N's fence: the derivative channel is filed
as a row on whichever is live when the unit is cut. **This track takes
Track U's STEP/STL rows** (`D343`, `C13`, `C14`) and leaves U's
`pncad`/`pncad-py` rows to LIB.

H, in dependency order: `step-import-degree-one-line-promotion` (the
pcurve rung, then promote certified degree-1 carriers to
`Curve3::Line`; the certificate exists) →
`step-import-curve-recognition-named-exclusions` (a derivative channel
in `spline::compose` turns the turning witness into a certificate, then
open arcs, the general-quadric arm for ellipse, the helix implicit form;
L) → `rational-patch-flux-quadrature-budget` route 2 (an algebraic
cylinder-recognition certificate via exact spline-product hulls, so
M7-6 promotes rational walls to analytic `Cylinder`; issue 1195 is the
second beneficiary; explicitly unclaimed by S-CERT's Q4 ruling; L).

D→E, the option surface (LIB's category A, "plan to Ev first",
undrafted for two weeks): `stl-header-refuses-plausible-names`
(smallest; probably "fix the demos to carry a fallback") →
`step-writer-hardcodes-user-header-fields` (`C14`) →
`epsilon-has-no-type-of-its-own` (`C13`; sibling of the `D283` ruling —
answer the placement question before touching `Tolerance`; the
`geom-core` half is S-CERT's until its exit).

E: `D343` (typed payloads through `{:?}` in the two STEP crates) and
the step-import diagnostics half of `coherence-findings-have-no-consumer`.

### Successor tracks (open at a named exit)

These are not disjoint today because the program whose ground they sit
on is still open. They are listed so that the re-home tables below have
targets and so the exits can be planned. Each opens when its
predecessor's exit walk is ratified, or its predecessor extends its own
slate with the same list — either is fine; what matters is that the
items are not swept into `work/issues/` with no owner a second time.

#### CURVED — the curved-operand boolean remainder (`curved/`) — at VERBS' exit — 23 items [H 17, D 5, E 1]

**GRADUATED (2026-09-03): opened as `work/curved/`, A/B band 2200–2299. DISPATCHING since 2026-09-04 (VERBS' walk ratified at #1793); `work/curved/plan.md` supersedes this section.**

VERBS' Wave-2 claims that never became units, S-BOOL's ceded ground,
and S-MATE's kernel residue, as one program. Territory: the
declared-contact rungs, germ/pierce lanes and operand-reach arms of
`topo/boolean/*` and `topo/splitting/*`, `topo/chord_join.rs`,
`topo/census.rs` (S-MATE's today), `topo/boolean/{rest,carrier_eq}.rs`
(S-MATE's today), `geom-brep/{intersect,implicit}.rs` (VERBS' today).
**S-BOOL is live on the same globs**; the fence S-BOOL's keep_out
already states in prose (germ arms, SPHSPH/CYLSPH, #1031 half B,
#1076/#1077 are not its) has to be written into both `program.md`s:
S-BOOL keeps the PATHS lattice in `crates/profile`, BOOL-4/5/6/7 and
BOOL-Q; CURVED takes the rest. Code-quality Track Q's rows on these
files stay S-BOOL's BOOL-Q.

Lanes, each in its own order:

- *Declared tangency (kiss/cusp)*: `m9-3-semantic-residues` items 4–5
  (move `tangent_locus` out of `rest.rs`) →
  `dev1-cylinder-sphere-circle-locus-arm` (D: revise the DEV-1 set;
  residual-sign story is the gate) →
  `declared-cusps-second-order-wedge-arm` items 3–4 (definite tangency
  to the declaration ladder; the M9-3 join-lane emission; a red-first
  witness is live) → `torus-declared-rest-lane-banked` (the 0/2π kissing
  arm; routed by `MATE-7-TANGENCY-DESIGN`) → item 5's consumer sweep.
- *Torus lane completion*: `torus-operand-boxes-span-whole-ring` (retires
  lily wall 1) → `circle-residual-harmonics-needs-torus-arm`.
- *Germ and pierce*: `arc-aware-point-in-loop` (BOOL-2/3's ignored probes
  show wrong-not-conservative answers; with #1077) →
  `pierce-ring-has-no-join-arm` → `boolean-refuses-on-arc-carrier-not-arc`
  (door 2: a curve×curved-surface root finder and a curved-face ring
  lane) → `pinch-carrying-machinery-valence-4` (D→H: spec first; parked
  on SEAT-6).
- *Operand reach*: `slab-cut-cylinder-refuses-sector-side` (wire
  `enters_material_order2`) → `split-refuses-cylindrical-feature-box`
  (a seven-orders `CircularAxes` disagreement, likely wrong axes fed to
  the ellipse constructor) → `cosurface-disjoint-curved-walls-refuse`
  (D: is same-sense cosurface a `ContactClass` member).
- *Merge-door reach*: `cylindrical-rest-pair-hits-planar-merge` (filter
  non-planar pairs at the caller first, as an honest typed skip) →
  `coplanar-cap-pair-f7-repair-half-b` (Ev's steer: (a)-first if (b) is
  merge-shaped; two exemptions already falsified).
- *Containment doors*: `full-period-wall-has-no-containment-verdict` (E)
  and `curved-face-containment-lacks-cone-torus` share one function; if
  S-BOOL dispatches BOOL-4 first they are its riders, else CURVED's.
- *At-rest census strengths*: `overlap-lane-boundary-crossing-cuts`
  (the D3 cut schedule; the guard pin makes it self-checking) →
  `census-at-rest-two-boolean-lane-premises` stage 2 (parked by name on
  the C6 interference-fit era).
- Singles: `graft-copies-provenance-keys-verbatim` (a dead-ancestor
  bridge on `GraftMap`; the naive remap is measured wrong),
  `edge-chord-len-defaults-to-one-metre` (D→E, three-way choice),
  `boolean-declarations-has-no-geometric-producer` (D→E door shape,
  then a lift; `S79` parks on it), `plane-cone-elliptic-section-split-refusal`
  if VERBS-CONE is never dispatched, and the two `ssi*` drive-bys
  `plane-nurbs-ssi-misblames-control-net` and
  `ssi-lever-arm-min-fold-hides-poison` (E; `ssi*` is "Track Q ground
  behind PCURVE P-2" in two keep_outs — announce) with
  `ssi-chart-speed-usability-boundary` (D→E).

#### SHELL — shell, offset and transform (`shell/`) — at VERBS' exit — 9 items [H 5, E 2, D 2]

**GRADUATED (2026-09-03): opened as `work/shell/`, A/B band 2300–2399; dispatches at VERBS' exit.**

VERBS' Wave-3 leftovers. Territory: `topo/{shell,replace_face,
transform}.rs`, `geom-brep/offset*.rs` (VERBS' today; `offset_fit.rs`
is S-CERT's), `editor-core/clearance.rs` (an M10 deliverable that is in
no program's `paths`), `demos/tour` scenes by courtesy of Track X.

Order: `shell-needs-shellnaming-birth-channel` (E, M; unblocks LIB-G17
and must agree with SEAT's shell `VerbRecord` migration) →
`shell-offset-three-followups` items 2–3 and
`mint-offset-ignores-cone-mirror-nappe` (one nappe home both consumers
read) → `shell-of-hollow-body-thicken-every-boundary` (Ev's ruling is
verbatim in the body) → `transform-rigid-refuses-approx-face` (shape 1
or 2; the composition law is pinned) → `shell-offset-three-followups`
item 1 → `shell-curved-clearance-consumer` (D→H: where the curved
wall-clearance gate lives — a joint question with M10's successor) and
`shell-curved-wall-clearance-window` (its build; unparked by M10-5) →
`tour-hollow-tube-scene` (E; the tess-budget re-baseline coordinates
with S-CERT) → `tier3-approx-regrid-per-face-cost` stays parked on an
`Approx`-heavy fixture.

#### PROPS — enclosure certificates and interval honesty (`props/`) — at S-CERT's exit — 20 items [H 11, D 6, E 3]

**GRADUATED (2026-09-03): opened as `work/props/`, A/B band 2400–2499; dispatches at S-CERT's exit.**

Every S-CERT issue is residue: its slate is CERT-M3/N3 then exit, and
none of the 20 is on a unit. Territory is S-CERT's today
(`geom-brep/props/*`, `offset_fit.rs`, `patch_bound.rs`,
`geom-core/src/*`) plus `geom-core/k_stats.rs` (Track M) and the R
rows on the same files (`C3`/`D30` takeable since CERT-10). If S-CERT
prefers, this is its own second slate.

- *offset_fit*: `budgetexhausted-conflates-three-terminations` (E) →
  `offset-fit-mignitude-floor-on-norm-e` (H; the micron row is the
  instrument) → `patch-bound-offset-fit-recentring-origins` (D→H,
  measure first).
- *Sphere polar extent*: `rimless-polar-cap-refuses-degenerateface` →
  `two-face-sphere-split-measures-zero-volume` (both H; both keep
  CERT-1's three exact pole rows green).
- *Rational quad lane*: `quad-face-extent-trusts-caller-perimeter` (E) →
  `refine-dir-hairline-knot-insertion` (H; before the dial so floors are
  stable) → `quad2-rational-max-rounds-dial-decision` (D→E; a D2
  argument for Ev) → `purchasable-area-tightness-valve` stays parked on
  a consumer.
- *Props hygiene*: `props-two-eps-vocabularies-five-sites` (E),
  `props-refusal-cannot-carry-measured-overshoot` (a ruling on
  `bounds-allowlist.sh` — Track K's script).
- *Linalg interval honesty* (DL6 is ratified, so this is an audit):
  `normalize-overflow-yields-zero-axis` (D→E) →
  `certified-lane-non-real-contract-audit` (H, L; member by member) →
  `interval-orthonormal-basis-sign-hull` (D→H; M10-5's clearance
  workaround retires after) → `pole-branch-pick-two-integer-shift`
  (D→E; `chord_join.rs` is Track Q).
- *Verdict recording*: `three-per-node-verdict-shapes` (D→E, first) →
  `k-stats-escalation-channel-and-redo` (D→H, L; cross-crate).
- Also here: `m6-sense-gate-recorded-residuals` (four H gate
  extensions; residual 2 carries a design choice),
  `span-carries-its-knot-vector` (an `[ev]` ruling, then an L sweep if
  A or B), `lily-authoring-needs-shadow-vector-algebra` (D→E: which
  `Vec3` doors; closes `D79`), and `contribution-bounds-via-dual-interval`
  (M10 residue; waits on certification widths).

Not here: `k-report-baseline-fold-cert1-roster` and
`tess-budget-doc-finding-block-stale` are Track K's and go there.

#### TRIM — the NURBS trim frontier (`trim/`) — at CURVED's rim arms — 5 items [H 4, D 1]

**GRADUATED (2026-09-03): opened as `work/trim/`, A/B band 2500–2599. DISPATCHING since 2026-09-04 — the "rim arms" gate below was traced to `topo/pcurves.rs`'s own arms (this track's, not CURVED's); `work/trim/plan.md` supersedes this section.**

PCURVE's P-2 residue, the smallest and most blocked candidate.
Territory: `geom-brep/{pcurve_cache,nurbs_iso,edge_nurbs}.rs` (Track
Q's four, "behind P-2" in every keep_out), `topo/pcurves.rs` (unowned),
`mesh/trimmed.rs` and `topo/props.rs` by seam with S-MESH and PROPS.

`interior-iso-curve-de-boor-extractor` (H, L; a complete face cache for
an interior-column seam) → `general-pcurve-face-props-and-tess-refuse`
(H, L; cannot be measured until a whole body at rest exists — CURVED's
rim arms) → `loft-seam-carrier-exact-knot-compare` (D→H; tolerance-
structural compare with a soundness story, or an exact skin-fit
boundary row) → `clearance-window-tightening-needs-chart-boundary`
(M10 residue; the chart-boundary description this needs is this
track's) → `unify-edge-descriptions-on-pcurves` (S-CERT's file; check
its state). `docs/PCURVE-P2-SPEC.md` is still in `docs/` as an
unmerged spec and is this track's charter input.

## Surveyed and deliberately not cut

- **S-TCOST residue, back to S-TCOST**: `geom-brep-inline-canonical-frame-surfaces`,
  `body-hash-census-misses-rename-only-duplicates`,
  `blend-suite-fixture-and-oracle-copies`,
  `one-declaration-guard-one-home-in-test-utils`,
  `consider-proptest-for-randomized-sweeps`,
  `proptest-modules-in-src-ungated` (an `[ev]` ruling first),
  `malformed-ambient-eps-reds-review-m2-pr7-k`,
  `m10-4-bore-pin-row-red-at-interval-1e-6`,
  `m10-5-e2e-channel-slider-reds-at-eps-1e-6`, the two
  `gated-marker-*` items, `nextest-shard-count-needs-remeasure` and
  `skip-eps-battery-by-observing-oncelock` (both parked on their
  triggers). All sit in `crates/*/tests/*` or `ci-filter.py`; a new
  track would not be disjoint from a very live program.
- **S-MESH residue stays with S-MESH** (`one-element-grid-axes-drop-schedule`,
  `torus-grid-step-one-step-both-directions`,
  `rim-chords-exceed-snapped-column-count`,
  `approx-face-mesh-certifies-against-fit`,
  `rim-only-sphere-cap-panics-at-census`,
  `chart-azimuth-and-bbox-anchor-idioms`): its log says the next unit
  is a slate decision put to Ev, which is where these go.
- **S-BOOL's slate items** (`lattice-validate-collinear-junction-disagreement`,
  the PATHS arc vocabulary pair, `subdivided-profile-side-coplanar-walls-gate`,
  `containment-examination-is-extent-box-coarse`,
  `loft-stacking-trilean-is-end-to-end`, `vdiff-pruned-pair-shadow-exec-rung`,
  `revolve-wedge-rim-free-band-volume`, the `props/curved.rs` trio, the
  one-home consolidations, `void-birth-marking-at-insert-void` — Ev
  deferred it) stay; the heatsink pair is a `demos/tour` fix (Track X)
  homed on S-BOOL only speculatively.
- **LIB's feedstock stays** (the façade curation queue, the Python
  refusal-projection doors, the bindings test guards and CI rows). LIB
  is the most active program in the tree and names these as its own.
- **Track K's ground**: `clippy-panic-gate-blind-in-macros`,
  `k-report-baseline-fold-cert1-roster`,
  `tess-budget-doc-finding-block-stale`.
- **Register-shaped, not programs**: `decide-flagged-dimensional-debt-inventory`
  (eight `decide_flagged` sites across P/Q/V fences — a standing rider
  rule on whichever unit next touches each family, with the census
  count as the tripwire); `two-verb-seats-do-not-compose` (SEAT's
  charter issue; closes or re-scopes at its exit walk);
  `no-public-census-or-genus-query` (D→E, Track P; with `S79` and
  `genus-rings-helper-spelled-nine-times` — three files for one gap;
  fits FIX once the granularity is chosen); `direction-normalization-two-doors-one-home`
  (SEAT's while open); `viewer-first-light-on-real-hardware` needs a
  GPU, not a lane.
- **The error-payload honesty theme** the code-quality survey names as
  its strongest lift (`S19`, `D36`, `D343`, `D366`, `D284`, `D262`,
  `S394`, `S414`, `patherror-display-renders-float-noise`,
  `debug-in-prose-residue-after-finding-sink`, plus ~12 issues) is not
  cut as one program: its members are distributed above by territory
  (FIX, FILLET, EXCH, CURVED), because a sweep program over six fences
  is exactly what the K–X partition refuses. If Ev prefers one owner
  for the class, it is a code-quality track with a stated seam, not a
  program.

## Class demarcation, summarised

| track | opens | E | D | H | review posture |
|---|---|---|---|---|---|
| CIW | now | 15 | 2 | 0 | batched style review, no A/B row |
| CHROME | now | 10 | 0 | 0 | batched style review, no A/B row |
| FIX | now | 14 | 1 | 0 | batched style review; a row where kernel logic moves |
| DOCM | now | 2 | 13 | 1 | `[ev]` PRs; builds hand off |
| VIEW | after CHROME | 1 | 5 | 1 | one conversation, then standard |
| FILLET | now | 3 | 4 | 4 | full v6 dual; Track T claimed whole |
| EXCH | now | 2 | 3 | 3 | full v6 dual; Track U's STEP/STL rows claimed |
| CURVED | VERBS exit | 1 | 5 | 17 | full v6 dual; fence written against S-BOOL |
| SHELL | VERBS exit | 2 | 2 | 5 | full v6 dual |
| PROPS | S-CERT exit | 3 | 6 | 11 | full v6 dual; R and N rows claimed |
| TRIM | CURVED's rim arms | 0 | 1 | 4 | full v6 dual |

Counts are the items named in this document; a few appear in two
tracks where a body splits (a ruling here, a build there) and are
counted where the build lands.

## Re-home tables for the two exits

**S-MATE** (13 files; `MATE-EXIT` names a landing for one):

| item | to |
|---|---|
| `census-at-rest-two-boolean-lane-premises` | CURVED (parked, C6 era) |
| `overlap-lane-boundary-crossing-cuts` | CURVED |
| `dev1-cylinder-sphere-circle-locus-arm` | CURVED |
| `m9-3-semantic-residues` | CURVED |
| `torus-declared-rest-lane-banked` | CURVED |
| `cylindrical-rest-pair-hits-planar-merge` | CURVED |
| `census-decline-consults-one-face-of-pair` | FIX |
| `interior-witness-budget-decline-untyped` | FIX |
| `split-crossings-skip-pattern-mate-ends` | FIX |
| `mate-clocking-has-no-gui-path` | FIX (half 1), DOCM (half 2) |
| `nested-pattern-mate-heads-refuse` | FIX |
| `instantiation-seam-drops-mate-identity` | DOCM |
| `no-door-mints-mate-frame-from-face` | DOCM (with LIB's hand-off) |

Until CURVED opens, its six go to `work/issues/` with `refs` naming
this document, or to S-BOOL if it will hold them.

**M10** (7 files):

| item | to |
|---|---|
| `certify-locally-valid-range-instead-of-sampling` | DOCM |
| `clearance-window-tightening-needs-chart-boundary` | TRIM |
| `signed-penetration-depth` | CURVED (needs a certified point-in-solid) |
| `contribution-bounds-via-dual-interval` | PROPS |
| `k-stats-escalation-channel-and-redo` | PROPS |
| `three-per-node-verdict-shapes` | PROPS |
| `certified-lane-non-real-contract-audit` | PROPS |

**S-CERT and VERBS**, when they reach exit: every S-CERT issue is in
PROPS above except the two Track K items; every VERBS issue is in
CURVED, SHELL or EXCH above except `c5-plane-torus-cone-cylinder-arms`
(its own unit) and `tour-hollow-tube-scene` (SHELL).

## Overlap rules this cut respects

- One file territory per track, mirroring the K–X partition rule; where
  a track claims a code-quality letter it takes the whole letter
  (FILLET → T, EXCH → U's STEP/STL rows, PROPS → R and N) so the
  schedule stays single-owner.
- The three places the cut is not disjoint are named as such: DOCM
  against Track V and four programs (the fence must be drawn in its
  opening PR), CURVED against S-BOOL (the prose keep_out becomes a
  written fence), and CHROME/VIEW against each other (CHROME lands
  first).
- Successor tracks do not open while their predecessor is live; their
  lists are the predecessor's exit residue, and the predecessor may
  keep them as a second slate instead.
- Design rulings stay with Ev. A track may file instances and open
  conversations but not resolve one by implementing.
- Every item keeps its id; re-homing is a header edit
  (`work/README.md`), never a copy.

---

# Addendum — the 2026-09-04 re-home sweep and two more tracks

Non-binding like the survey above, and written into this file because
this is where a reader looks for how the tree was divided. **Executed
on Ev's direction (in-chat, 2026-09-04)**; `work/<id>/plan.md`
supersedes it as each track's charter.

## What the measurement found, one day later

`work/issues/` held **44 open files, 38 of them opened on 09-03 or
09-04** — after this document's cut. So the pile was fresh inflow, not
residue, and most of it already named its owner in its own `## Home`
section. **Re-homing, not a cut, was the bulk of the work**: 43 of the
44 moved onto an existing or new program's board, and `work/issues/`
now holds only four closed files and one item already being re-homed on
another branch (`lb13-guards-are-line-local`, on `ciw/rehome-lb13`).

Two things the cut left open did surface as tracks.

## TOPO — the topology core (`topo/`) — 17 items (3 issues + Track P's 14 rows)

**GRADUATED (2026-09-04): opened as `work/topo/`, A/B band 2700–2799.**

This document recorded that "no program owns … 37 `crates/topo/src`
files" and then cut eleven tracks without closing that gap. Re-measured
2026-09-04: **47 files in `crates/topo/src/` are in no open program's
`paths`**. Two `work/issues/` files name that ground as unowned in
their own `## Home` sections
(`validate-tier3-curved-boundary-containment`,
`no-public-census-or-genus-query`), and **code-quality Track P — whose
fence is exactly this territory — had never had a lane**: `smell/k-*`,
`smell/x-*` and `smell/t-*` branches exist, `smell/p-*` never has. The
rows were waiting on an owner.

Territory: an **enumerated** twenty-path list, not a glob, because
`crates/topo/src/*` crosses `/` under `fnmatch` and would double-claim
five programs' ground — and `scripts/work.py territory` is blind to a
double claim (see META below). Claims **Track P whole**, its
three-sub-lane partition inherited unchanged. Opener: `S331`
(`validate_pcurves` answers a clean bill on a body whose pcurve mint
just failed — a vacuous green through a public door).

The ~25 remaining `topo/src` files (`body.rs`, `entity.rs`,
`contact.rs`, `separation.rs`, the chart and `review_m1_*` readers) are
recorded as **unowned and not finished**, in the sense the
`geom-brep` seam gives that phrase: a row landing on one draws the
fence in the PR that mints it.

## META — the tracker and the process instruments (`meta/`) — 5 items (3 items + 2 registers custodied)

**GRADUATED (2026-09-04): opened as `work/meta/`, A/B band 2800–2899.**

The cut above divided the tree by territory and did not divide the
**instruments it used to do it**. `scripts/work.py`, `docs/prompts/`,
`docs/MODEL-AB-LOG.md` and `docs/DOC-LEDGER.md` are in no program's
`paths`; CIW's `keep_out` already ceded `scripts/work.py` to "the
tracker's own", naming an owner that did not exist. Three findings on
file are the cost: a `territory` check blind to the exact collision it
exists to catch, a **pre-registered A/B stopping rule passed about nine
times unnoticed** (109 v6 dual rows against a twelve-pair rule), and
two live FILLET spec acceptance clauses that instructed a coverage
reduction and red the gate if obeyed (that third one was closed by
CIW's `delete-config-trailer` hours before this program opened, and
stays closed in `work/issues/` — the class stays META's, with a worked
instance to point at).

**Checked against CIW first and there is no conflict**: CIW owns the
*runs*, META owns the *tracker and the briefs*, and the one touching
surface — `implementer-discipline.md` §2, what a run gates — is written
into META's `keep_out` as CIW's to amend without waiting. The fence
reads the same from both sides.

META also **custodies the two cross-program registers** this document
called "register-shaped, not programs" (`m6-carried-items-register`,
`decide-flagged-dimensional-debt-inventory`): it keeps them accurate
and routes their rows, and never executes one.

## Track W → S-TCOST

`work/code-quality/plan.md` recorded Track W's claimant as "ground is
`tcost`'s" and the seventeen rows never moved; no `smell/w-*` lane ever
ran. W's fence (`crates/*/tests/`, `crates/test-utils/`) and S-TCOST's
`paths` (`crates/*/tests/*`, `crates/test-utils/*`) are an exact match,
so the letter went to the program whole. **The claim was made on Ev's
direction rather than by the S-TCOST orchestrator**, which is not how
`work/README.md` expects a claim to happen; every row carries a
`## Claimed by` section saying so, and moving one back is a `git mv`.

## What this addendum does NOT cut, and why

- **The instrument-blindness theme** (11 issues: two `gated-marker-*`,
  `bit-identity-debug-only-gate-…`, `source-scanning-censuses-…`,
  `ci-draw-can-hide-a-compile-break`, `gui-wasm-build-is-not-gated`,
  `probe-interval-lane-has-no-clippy-row`, `rustdoc-d-warnings-…`,
  `detached-demo-workspaces-…`, `body-hash-census-…`,
  `clippy-panic-gate-blind-in-macros`) is the strongest single theme in
  the pile — every one is "a gate we built is blind and reports green" —
  and it is **not** a program: its territory is CIW *and* Track K *and*
  `crates/*/tests/` at once, which is what the K–X partition refuses.
  Split by fence: five to CIW, two to code-quality Track K, four to
  S-TCOST.
- **The error-payload honesty class**, which the survey above declined
  to cut and distributed by territory. That still holds, but it is
  worth recording that **five more instances arrived in a single day**
  (`clearance-refusal-names-one-face-twice`,
  `point-in-solid-refusal-names-faces-zero`,
  `debug-in-prose-at-blend-and-step-import` — split in two, both halves
  live panics — `run-on-whitespace-in-message-literals`,
  `mate-fault-accessors-wildcard-into-silence`). If Ev prefers one
  owner for the class, the survey's answer stands: a code-quality track
  with a stated seam, not a program.
- **The "one rule, two homes" class** (`nobodyroots-…`,
  `emit-blend-restates-…`, `mate-member-vocabulary-…`,
  `face-kind-read-…`, `loud-skip-marker-…`,
  `geom-brep-inline-canonical-frame-surfaces`) — same shape, same
  answer, distributed by fence.
- **Track J's ground stays a seam.** CIW's opening took most of it
  (`.github/workflows/`, `local-scripts/`) without claiming the letter,
  so `scripts/doc-gate.sh`, `gate-roster.sh`, `probe-suite-census.sh`
  and root `[workspace.lints]` are still "unowned, not finished".
  `rustdoc-d-warnings-breakages-outside-the-doc-gate` lands exactly
  there and went to CIW because `doc-gate.sh` is in CIW's `paths`.
- **Letters M, N, Q, R, U and V stayed in `work/code-quality/`.** Their
  claimants (`cert`, `bool`/`trim`, `mesh`, `exch`/`lib`, `docm` and
  seven others) claim them **through a unit that cites the letter**
  — `CERT-M3`, `CERT-N3`, `BOOL-Q`, `MESH-R` — rather than by moving
  rows, and V is explicitly shared by eight. P and W were different:
  both were single-owner by fence and neither had ever been worked.

## What arrived during the sweep, and how it was routed

Three issues landed in `work/issues/` on main while this sweep was on a
branch, and they are a fair test of whether the cut helps:

- **`perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger`
  → META.** `docs/PERF-PLAN.md` is cited by path from 29 tracked files,
  is not in the tree, and `docs/DOC-LEDGER.md` — the one document whose
  job is to say where deleted docs went — records no deletion of it.
  The item's own closing line reads *"Not a program's slate: the ledger
  is the repo's"*, naming a home that did not exist when it was filed.
- **`debug-only-counters-have-no-gate` → code-quality Track K.** It
  wants `scripts/gates/bit-identity-debug-only.sh` rewritten as a
  subject-list gate; K's fence is `scripts/gates/` less two. It is the
  **second** row on that one script, joining
  `bit-identity-debug-only-gate-ends-an-item-at-a-semicolon` — the two
  want the same file open at once and are one lane, which is the
  argument for the fence more than for either row.
- **`axis-flavoured-declarations-have-no-channel` stays in
  `work/issues/`, correctly.** Its fix is a new declaration source that
  is either placement-level (`BooleanDeclarations` — CURVED's) or
  frame-level identity through `GeomSource` (`topo/src/source.rs` —
  TOPO's), and choosing between them is the `[ev]` question. SEAT's own
  filing says it "sits outside SEAT's fence". This is what the
  directory is for under its new README: *issues that do not have a
  home yet, not a waiting room for issues whose owner is obvious.*

That README (Ev, 2026-09-04, `work/issues/README.md`) landed on main
independently of this sweep and states the rule the sweep executed.

## Programs that were unclaimed at this sweep

Recorded because the sweep had to check it and the answer should not
have to be re-derived. `git branch -r` per program prefix, 2026-09-04:

- **`props`** — 9 open items, opened 09-03, **zero branches**; its log
  still reads "No unit is cut and no branch exists yet." It opens at
  S-CERT's exit and S-CERT is down to `CERT-N3`.
- **`msolve`** — 5 open items (now 7), opened 09-04 by the FIX
  orchestrator on Ev's steer, **zero branches**. It holds the one live
  known-wrong answer in the tree (`mate-solve-is-transform-blind`, with
  characterization rows already on main written to go red when it is
  fixed).
- **`perf`** — zero branches **by charter**; a register with no
  orchestrator and no units. Not a gap.

Everything else has a branch dated 09-03 or 09-04. `bool`, `mesh` and
`lib` are quiet but **blocked rather than unclaimed** — `BOOL-12` is in
review on Ev, S-MESH's next unit is a slate decision put to Ev, and
LIB's log says its mechanical feedstock is spent.
