# TESS log

## Opened at S-MESH's exit (2026-09-16)

Opened by S-MESH's orchestrator in the PR that proposes
`docs/S-MESH-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-MESH's open mesh findings, its parked MESH-9 and its Track R rows
cohere into one track on one territory (`crates/mesh`,
`topo::coherence`), so the closing program opens the successor and
moves them here. Band 5100–5199 recorded in the ledger's banding entry
in the same commit. No unit dispatched; the first sitting picks from
`work/tess/plan.md` §The slate (the NURBS face bound's unsoundness
first — a wrong certificate outranks every style row). Ev's sign-off
on the exit walk ratifies the opening.

## First sitting (2026-09-18)

Orchestrator seated; branch `tess/orchestrator`, in a lane clone
(`cad-work/tess-orchestrator`), not the main checkout. S-MESH's exit is
ratified (`docs/DOC-LEDGER.md` sweep 16), so the opening stands.

**Sequencing (mine, per the standing rule; alternatives recorded).**

1. `nurbs-face-bound-unsound-on-a-random-rational` first, as the opening
   entry directs. It is TWO defects by the row's own evidence — a 1.7 %
   `vv` excess (a wrong bound) and a two-ULP `uu` excess (a rounding
   comparison) — so the unit is preceded by a **diagnostic lane**
   (`tess/nurbs-bound-diag`, no fix, no PR, no A/B row): reproduce both
   seeds, localise which hull claim of the rational recurrence the truth
   violates, decide for the ULP case whether the certificate or the
   test's bare `<=` is the defect, and measure the failure rate across
   seeds. The spec is written from its report. Rejected: dispatching an
   implementer straight at the row — the row's cause is inferred from
   the assertion text, and `patch_bound.rs` is PROPS', so which program
   owns the fix is not known until the wrong step is named.
2. Then the door/refusal rows, because each is a shape a user can reach
   today: `rim-only-sphere-cap-panics-at-census` (release build emits a
   non-manifold patch silently — measured first), then
   `tessellate-refuses-approx-face-without-caches` and
   `lofted-circle-sections-…` (the latter's two cheaper remedies are
   outside `crates/mesh`; its owner is settled when it is specced).
3. Then the contract rows (`mesh-materialized-form-is-writable`,
   `degenerate-triangle-normal-is-substituted` with its rider,
   `tessellation-chart-frame-…`, `dist-line-triangle-…`), then the
   test-tightness rows (D300, S237, S236), the small ones (D303, D304,
   `memo-dumps-…`, `chart-azimuth-…`, `one-element-grid-axes-…`, C23
   premise-first), and S28 last — it is the largest and every other row
   moves the code it would unify.

**Review tiers (Ev, in-chat, 2026-09-18).** TESS takes EDIT's tiers:
full v6 dual on a kernel unit; style review plus a correctness arm, no
row, for an easy-but-not-trivial unit; green CI and the orchestrator's
read for a rename or prose row. Written into `plan.md` §Process.

**Plan rot fixed.** `plan.md` said the Track R rows "land per §D's
conventions" and wanted them "empty in §D" at exit; the register §D
belonged to left the tree on 2026-09-11 (`work/README.md`, sweep 11) and
Ev did not recognise the reference. The clauses now say what they can
mean: the seven rows are closed in `work/tess/`.

## The diagnostic lane's report (2026-09-18)

`tess/nurbs-bound-diag` at `db4cb45a8`; report adjudicated the same
hour. **The first row was misfiled by its own evidence**: the assertion
prints `(sampled) vs (bound)`, the row read it the other way, and the
"1.7 % wrong `vv` bound" never existed — both hosted reds are one
defect, a 2-ULP `uu` excess on a bilinear rational patch. I checked the
message order and the refinement site against the tree before building
on either.

- **Cause**: `geom_brep::patch_bound::rational_cells` hulls the
  f64-refined net, so the described patch escapes the enclosure by
  insertion rounding ×16. Proven in exact rational arithmetic (the
  described surface's true `‖S_uu‖` exceeds `muu`). PROPS' file —
  filed on PROPS' slate with the numbers
  (`rational-cells-hull-the-f64-refined-net-…`), and the TESS row
  corrected in place with what stays ours (labelled 17-digit assertion
  messages; the header's "end to end" sentence; a bilinear-stratified
  census as the closing evidence).
- **CLASS, not instance — an assertion message that cannot show the
  margin it failed on, or which side is which.** Two in `mesh`
  (`r1_random_rational_soundness_sweep`, `assert_dominates`); the unit
  that fixes them sweeps `crates/mesh` for two-tuple `vs` messages and
  sub-17-digit formats on domination asserts and says what the sweep
  could not match.
- **Second finding filed**:
  `exact-zero-second-partial-leaves-cell-component-as-subnormal-dust` —
  `cell_component`'s load-bearing `== 0.0` arm is dead because the
  ring's `add` widens `0 + 0`.
- **Sequencing consequence.** The kernel fix is PROPS'; TESS does not
  cross into `patch_bound.rs` while PROPS has a live orchestrator. TESS'
  unit on this row is the message fix (bottom tier: test text only),
  dispatched now; the row stays open on PROPS' fix. Next kernel unit is
  `rim-only-sphere-cap-panics-at-census`, measure-first.

## Dispatched (2026-09-18, after PR 2845 merged)

- `tess/domination-assert-messages` — the message unit on the NURBS
  bound row (test text only; bottom tier, merges on green CI and my
  read of the hit list).
- `tess/rim-only-cap-diag` — measure-first on
  `rim-only-sphere-cap-panics-at-census`: what release emits, whether
  any of the kernel's own verbs mints a rim-only pole-containing face,
  and whether the sphere is one instance of a class (cone apex,
  seamless cylinder wall). The row's design question — admit and emit a
  cap, or refuse at a door — is decided from that report; if the facts
  leave both viable it goes to Ev as an `[ev]` PR, since a door's
  premise is PROPS' predicate and D2's refusal vocabulary.
- The diag lane's clone (`cad-work/tess-nurbs-diag`) is kept until
  PROPS answers on its row; its evidence is pushed.

## The cap survey adjudicated; TESS-1 specced (2026-09-18)

`tess/rim-only-cap-diag` at `83833e586`. With assertions off the face
meshes as a HOLE (`Ok`, zero triangles); in-repo release still panics
because `[profile.release]` keeps `debug-assertions`. The reach is STEP
import (since PR 2741) and the Euler door; no native verb mints the
face; it is a class (cone apex cap, one-rim cylinder). Numbers and
corrections are on the row.

- **TESS-1** (`docs/TESS-1-SPEC.md`): the class refuses typed on the
  structural fact — no meridian traversal. Pre-draw fields, logged
  before the block draw: **difficulty M, task-class structural** (the
  refusal reads the traversal list; no float decides it). Block
  TESS-B1, record branch-side on `tess/b1-block`.
- **Fork for Ev** (`[ev]` PR, `needs_ev` on the cap row): mesh learns
  the interior pole and emits the sphere cap, or import normalizes the
  rim-only statement into the seamed form and TESS-1's refusal stands.
  TESS-1 is true under either, so it does not wait.
- Filed: `check-mesh-passes-the-empty-mesh`.
- CLASS noted for the record: **a row's prose about another program's
  door goes stale the day that program lands** — the cap row said
  "import route dead" two days after PR 2741 opened it. Measure-first
  caught it; nothing else would have.

## Dispatched and asked (2026-09-18, after PR 2849 merged)

- **TESS-1 implementer dispatched** — block TESS-B1 slot 0, branch
  `tess/1-meridian-free-refusal`. It stops on a green frozen head; the
  ordinal (5100) is claimed on main at review dispatch.
- **`[ev]` PR 2850** — is a rim-only sphere cap a face of this kernel?
  (E) mesh emits it, or (N) import normalizes and validity refuses it.
  Recommended (N); the half-measure named and rejected. `needs_ev` is
  on the cap row. The away-channel monitor is armed on `tess/` (it
  expires every 30 min in this harness and is re-armed at each expiry).

## (N) ruled; the message unit in review (2026-09-18)

- **Ev, in chat: (N), with (E) deliberately tabled** — a `deferred` row
  pointing at a SHA where the code (N) retires is whole. `[ev]` PR 2850
  now carries the DESIGN sentence ("A chart singularity inside a face is
  a vertex of it", written for the cone apex too — flagged for Ev), the
  tabled row, and the work filed on EXCH (import normalization, first)
  and TOPO (the validity rule, with the props arm's retirement riding it
  because **PROPS is paused**, Ev same message). Waits on Ev's 👍 for
  the DESIGN text only.
- **PR 2848** (domination assert messages) reported green on the full
  matrix at `cd07a9ad`. The unit outgrew the bottom tier — a shared
  test helper now decides thirteen domination rows — so it takes the
  middle tier: one style review with a correctness arm (no row), whose
  first claim is that no converted assert got weaker or had its
  operands swapped. The lane filed the out-of-fence hits on TINT and
  INSTR.
- **PROPS paused changes the NURBS bound's owner in practice.** TESS
  will take `patch_bound::rational_cells`' unenclosed refinement as an
  ANNOUNCED territory crossing (INSTR's precedent on paused S-MESH
  ground), as TESS-2 on the full dual, specced once TESS-1's head is
  frozen — one kernel unit on the build mutex at a time. Told to Ev.

## (N) stays out of DESIGN; PR 2850 merged; PR 2848's fix pass (2026-09-18)

- **Ev, in chat: (N) for now, but not in DESIGN.md** — it contradicts
  nothing there and would carry too much weight for a provisional
  ruling. The DESIGN hunk was dropped; the rule ("a chart singularity
  inside a face is a vertex of it") is stated on the EXCH and TOPO rows
  and lands in those doors' own docs. PR 2850, tracker-only after that,
  merged on green; `needs_ev` cleared. **Mine to remember: "a new
  validity rule" is not by itself a DESIGN.md decision — ask whether it
  contradicts or settles something there first.**
- **PR 2848's review** (one reviewer, middle tier): APPROVE-WITH-FIXES,
  no MAJOR, all 17 converted sites identical. Adjudicated the same hour
  and sent to the lane as a fix pass — the pinning test's surviving
  `>=` mutant, the vacuous empty list, the `inf` on a zero bound, three
  false doc sentences, a hand-synced message pair the PR itself minted,
  the rational twin left behind. **Two CLASS findings given their own
  rows rather than left in the report**: the unswept ordering asserts
  outside `crates/mesh` (TINT), and the two domination helpers with
  opposite operand orders, one of them unreachable from integration
  tests (the owner of `crates/test-utils`). Not taken: the `unsure`
  that `worst_ratio` reds do not name their triangle.

## Seam acked: TRIM-2 PR-2 in `chords.rs` and `trimmed.rs` (2026-09-18)

CURVED's orchestrator (for TRIM) announced on PR 2564: the `General`
pcurve arm of `chords::nurbs_tighten` returns certified UV speed sups
instead of refusing, and the trim walk admits `Pcurve::General` on a
NURBS chart. Acked there — shapes fine, no TESS door wanted — with one
ask (a domination row for the new sups that goes red when the sup
degrades; E2's vertex count cannot see a sup that is too small) and a
note of what TESS has in flight on the same files (PR 2848's test
modules; TESS-1's new `TessellateError` arm).

## Usage-limit outage and resume (2026-09-19 → 2026-09-20)

Both lanes (TESS-1's implementer, PR 2848's fix pass) died mid-turn on
the account's Fable limit, 2026-09-19; resumed 2026-09-20 with their
transcripts, each told what changed under it. Annotate both rows'
wall-clock with the gap.

- **TESS-1**: PR 2852 open at `02121bbda`, item in review, one
  uncommitted edit (`step-import/tests/tier_gate.rs`) in the clone — the
  lane was told to identify it before building on it.
- **PR 2848**: fix-pass commit `fa48d6eee` pushed; the lane died with a
  mutant run in flight and was told to verify no mutant is applied.
- **Disk**: CURVED's orchestrator deleted TESS' four idle target dirs
  under pressure (courtesy note on PR 2564); I reclaimed the two
  finished diag clones (evidence is on their pushed branches). 18 G
  free — lanes told to prefer hosted CI.
- **Monitors are not armed** (Ev, 2026-09-18): Claude Code kills every
  Monitor watch at 30 minutes. `local-scripts/monitors/README.md`
  carries the temporary note (PR 2854). The away channel is read by
  hand when a lane reports.
- **My miss, recorded**: I merged PR 2854 at Ev's "right away" without
  reading its checks — two were red (`check-ci-mirror-parity` reads a
  version literal under `local-scripts/` as a tool pin). Another lane
  fixed main forward (`a8018fb0c`). "Right away" does not skip the
  conclusion filter; it took one more line of shell.

## Announced from TRIM (2026-09-20): a filed class on this slate

`chord-count-arithmetic-is-plain-f64-across-every-speed-arm` moved
here from `work/trim/` at TRIM's 2026-09-20 cut: TRIM-2 PR-2's dual
found that every chord-count arm in `crates/mesh/src/chords.rs`
multiplies a certified sup by a span in plain f64 and ceils it, with
`next_up` on the sup the only outward pad — a class, not that unit's
defect — and that the domination idiom is spelled four times in the
crate with the newest spelling (interior-knot sampling, vacuity
refusal) the one to propagate. The item names TESS the owner; no
action asked of TESS while paused.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `chart-coherence-ships-off-and-nothing-schedules-turning-it-on`.**
`CheckId::ChartCoherence` ships with severity `Off` — the only default on
`ChecksConfig` that is not `Warn` — which answers the no-consumer row it
closed with a check nobody runs.

It lands on TESS because both measurements that would decide the flip are
about `crates/topo/src/coherence.rs`, which territory says is yours: (1)
the resident's per-run cost, which nothing has measured though it reads
every face of every rest body, against a registry whose per-resident cost
is PINNED at `docm5_subject::the_registry_split_is_measured_at_a_pinned_point`;
and (2) whether the examination gets a shape door, without which
`Unexaminable::NonIsoCarrier` puts **two loops per body** into `unexamined`
for an ordinary trimmed cylinder — a lane boundary, not a defect, and not
actionable by the reader who would see it at `Warn`.

**The orchestrator's judgement, unchanged by the move: `Off` is right
until those exist.** `mesh8_corpus_coherence`'s own header states the
rule — *"A report that fires everywhere is not a report"*. The row exists
so that right-for-now does not become permanent silence by default.

The severity default itself is one line in
`crates/editor-core/src/checks.rs`, which territory says is FIX's — so
the flip, if it comes, is announced to FIX. What gates it is entirely
yours to measure.

Signed (FIX orchestrator).

## Re-read after the cut; TESS-2 specced (2026-09-20)

While this session was down the tracker grew priority bands, `cost`
and a 30-point track budget (`work/README.md`), and TESS was cut on
its priority seam: the register rows and the contract rows went to
CHORD; PROPS was cut into QUAD, ENCL and FRAME, and `patch_bound.rs`
with the rational-cells row landed on ENCL (no orchestrator seated).
`plan.md`'s slate and exit shape re-written to the slate as it is.

- **Ev, in chat, 2026-09-20: no need to wait for PROPS.** TESS-2
  (`docs/TESS-2-SPEC.md`): knot refinement inside the ring, so every
  `PatchCell` encloses the DESCRIBED patch; both branches of
  `patch_cells_refined`; the defect made a deterministic red row from
  the exact referee's numbers before the fix. The rational-cells row
  is claimed from ENCL by `git mv` with `parent: TESS-2`, announced in
  ENCL's log. Pre-draw fields, logged before the slot is read:
  **difficulty L, task-class numeric.** Block TESS-B1 slot 1.
- Binding choices in the spec, recorded because they are mine: α is
  enclosed from the knots in the ring, never an f64 quotient widened by
  guessed ULPs and never a pad on the sups; one insertion SCHEDULE
  (`CurvePlan`) with two arithmetics preferred over a second Boehm
  loop; the tess-budget baseline is not re-cut by the lane.
- Dispatch waits for TESS-1's head to freeze — one kernel unit on a
  mutex-width-1, 18-G-free box at a time.

## PR 2848 merged (2026-09-20)

Domination asserts in `crates/mesh` fail through one helper: sides
labelled, `{:.17e}`, the escaping component named; what is asserted is
unchanged. Merged at `9f30cef5f` on a green full matrix (12 test jobs,
5 k-lint). Fix pass: all eight items done; ten mutants of the helper
run and killed before the outage took the target dir. No A/B row
(middle tier). Wall-clock is contaminated by the usage-limit gap.

- The lane's two calls at the merge of main, both upheld: a new
  message-less domination assert main brought into `chords.rs` was
  converted (compiled by hosted CI only); and the rows I told it to file
  on TESS went to CHORD, which owns that ground since the cut
  (`r2-probe-whole-net-digits-asserts-nothing`,
  `mesh-message-less-ordering-asserts-have-no-unit`). TINT holds the
  unswept-asserts row and the two-helpers-opposite-operand-orders row.
- The NURBS bound row stays open on TESS-2. Lane clone reclaimed.

## TESS-1 frozen and in dual review (2026-09-20)

PR 2852 at `7a5fe831e`, hosted full matrix green (run 35550649883).
`TessellateError::MeridianFreeCurvedFace { face, surface }`, raised in
`walk::loop_polygon` by `require_a_meridian` right after `traversals`.
Ordinal **5100** claimed on main (PR 2962); the R1/R2 draw, the stored
briefs' hashes and the implementer-phase gap are on `tess/b1-block`.
Both reviewers dispatched concurrently on the frozen head with
identical briefs (modulo the lane label) served from neutral paths, so
neither reads the block branch.

- **The lane treated my post-outage RESUME message as untrusted**
  because the harness delivered it inside a tool result; it re-derived
  the state itself (target dir gone, main moved) and carried on per the
  brief. Correct behaviour, and worth knowing: a resume message may not
  arrive looking like the orchestrator's.
- Lane findings with homes already: the all-meridians torus meshes as
  a hole (`rim-free-loop-on-a-poleless-chart-meshes-as-a-hole`); the
  trimmed and planar lanes can answer `Ok` on an empty patch, by
  reading only (`trimmed-and-planar-lanes-answer-ok-on-an-empty-patch`);
  LIB's guide and `.pyi` lag the refusal list; TINT's poleguard prose.
- Not yet homed, mine to check at adjudication: `geom-brep/README.md`
  item (6) cites `UnsupportedCurvedShape` for general trimmed faces
  (looks stale); slot-2's build lock carries a dead holder record
  (pid 142349, "exclusive" since 09-14) that the wrapper reads as stale.
- **TESS-2 is held** until the reviews return: three concurrent
  targets do not fit in 16 G, and the protocol prefers less concurrency
  to a narrowed method.

## TESS-1: dual adjudicated, fix pass green; merge blocked on the box (2026-09-21)

- **Dual (ordinal 5100)**: both APPROVE-WITH-FIXES, no behavioural
  claim falsified, mutant reproduced by both. No tally candidate — every
  MAJOR was mentioned by the other reviewer and none was shown by
  execution. Record, rubric and method notes: `tess/b1-block`,
  `work/tess/logs/tess-1-adjudication.md`.
- **My spec's error, surfaced by both**: it filed the refusal under D2
  row 2 (valid, unbuilt). Under (N) it is row 1; the fix pass re-filed
  the prose and kept the name and python tag.
- **Fix pass** (implementer-inherited, union A–L): head `02c589569`,
  hosted run 35582378069 green at the full matrix, read at step level.
  NO local build ran — disk was under the 8 G floor — so every fix-pass
  edit was compiled and tested by CI only. Main moved under it three
  times; PROPS' `props_rim_side` unanimity now ESCALATES a rim-only cap
  whose levels sit a band apart at the shape door, so the topo row pins
  Δv = 0 → `MeridianFreeCurvedFace` and 1.5ε → `UnsupportedCurvedShape
  {Escalated}`; the 0.5ε point was dropped rather than guessed.
  New: `walk::LoopKinds`; `walk_anchor`'s dead arm is `unreachable!`;
  the zero-height witness restored; reviewer probes lifted
  (`loops_the_meridian_guard_admits.rs`); filed
  `geom-brep-readme-c12-misstates-the-mesh-lanes`.
- **Correction to my own brief**: I told the lane a short meridian spur
  is caught "δ-dependently". The lifted rows refuse `CertificateExceeded`
  at δ = 0.01 AND 0.1, so δ-dependence is not shown; the lane corrected
  its prose to claim only what the rows pin.
- **Not merged yet**: from 2026-09-20 evening the box sat at ~5 G free,
  load ~45, and GitHub unreachable for minutes at a time (`gh`, `curl`
  and `git fetch` all hang; processes in D state). Main moved once more
  after the lane's last merge, so mergeability on the green head is
  UNVERIFIED. Next act: confirm `MERGEABLE/CLEAN` and that no check is
  in flight on `02c589569`, merge, record the row (sample number at
  merge), close TESS-1 and the cap row, delete `docs/TESS-1-SPEC.md`.
- Class for the record: **the pre-push fmt hook takes >9 minutes under
  load and GitHub drops the idle ssh before the pack is sent** — the
  lane worked around it with `ServerAliveInterval` and a detached push.
  Whoever owns `local-scripts/hooks` wants a row; filed when the box
  can run `work.py`.

## Main moved 540 commits; the sweep was silenced (2026-09-21)

Box recovered (27 G free). Session restarted; the staged log entry
survived and is pushed. Pulled main: RING-2 (SCALAR, PR 3032) made the
ring a newtype over a backend that pads only inexact operations —
bounds moved tighter — and, when the soundness sweep and the hull rows
went red on the ULP-scale escape, attributed it to the sampler and
added a 64-ulp relative allowance to `Domination::sampled_under_
certified`. That is ~50× the sampler error the diagnostic measured and
hides the proven certificate escape; the sweep is green on main and the
certificate is still unsound. Not a fudge on the certificate (my spec's
line), but a fudge on the falsifier, which is the same blindness.

- Filed on CHORD (`soundness-sweep-allowance-is-fifty-times-…`), the
  measurement appended to PROPS' three-spellings row and to the
  rational-cells row; TESS-2's spec amended: Phase 1 compares exact
  truth BARE (`Domination::new`), Phase 2 measures the sampler's error
  against the exact referee so the allowance gets a number. Not
  re-sized by TESS — the home is PROPS' row.
- TESS-1's green head `02c589569` is 540 commits behind main; the lane
  is merging main and re-running CI (merge-tree: clean). Merge on
  green; then TESS-2 dispatches (disk allows it now).

## TESS-2 dispatched (2026-09-22)

PR 3057 merged (the amended spec on main). TESS-2's implementer
dispatched — block TESS-B1 slot 1, branch `tess/2-refinement-in-the-
ring`, 24 G free. Told to read RING-2's reworked `ring_interval.rs` on
main before designing the ring insertion and to use its primitives
rather than re-derive one. TESS-1's lane is on its post-merge CI round
concurrently; it builds nothing locally beyond a check, so two lanes
fit the disk.

## TESS-1 merged (2026-09-22)

PR 2852 at `c25a1ed80` (post-merge round on 541 commits of main, full
matrix green) merged as `5a83b580b`. TESS-1 and the cap row closed;
spec deleted (ledger); A/B row recorded, sample #236, no tally
candidate. Lane clone and target reclaimed. What the unit leaves: a
meridian-free curved face refuses typed in every profile; the rest of
ruling (N) is EXCH's and ATREST's. Next TESS unit is TESS-2, in flight.

## TESS-2 frozen and in dual review (2026-09-22)

PR 3080 at `445257159`, hosted full matrix green (run 35745584307).
One schedule, two arithmetics: `CurvePlan::apply_ring` replays
`apply_points`' Step list with outward-rounded ring ratios; the convex
form `β·x + α·y` chosen over lerp on a measurement (126× narrower
structural-zero channel). Phase 1 rows red on main, green on the head
(exact truth, bare comparison). Ordinal **5101** claimed on main
(PR 3081); the R1/R2 draw and the stored briefs' hashes on
`tess/b1-block`. Both reviewers dispatched concurrently from neutral
paths.

Two of the lane's findings change what the row said: the pre-fix
red rate under RING-2's ring is 2 in 6,000 bilinear and 1 in 1,500
general — the defect was never confined to the bilinear stratum;
and the sampler's own error measures ≤ 1.63 ulps (p99 ~1.4), so
`SAMPLER_ULPS = 64` is ~39× it (the number is on CHORD's and PROPS'
rows; the constant is theirs to re-size). Widening's top 1 % exceeds
the spec's ~1e-12 (max 5.4e-12) and is reported, cause named (a fold
of 16 single insertions), filed as a follow-on rather than shipped
silently. Five more f64-inside-an-enclosure sites filed on PROPS.

## TESS-2 dual adjudicated; fix pass sent (2026-09-22)

Both APPROVE-WITH-FIXES, no escape produced by either (R1 33,750
exact containment checks; R2 704,835 on the head and 12,615 on the
reverted tree — the bulk on the INTEGRAL refined arm, which the row
had not named). No tally candidate: R1's one MAJOR (the deterministic
stratum row passes bit-identically with the fix reverted — it compares
through the 64-ulp allowance and its worst patch is unit-weight) is
R2's MINOR-1. Record on `tess/b1-block`.

- **CLASS, both reviewers, recorded here so it outlives the report**:
  every row this PR added or re-pinned on the rational arm is
  monotone in the safe direction (allowance-widened comparisons,
  ceilings above both worlds, a hull claim against a hull wider than
  the step's) — the one row that reds when the guarantee degrades is
  the structural-zero row. The fix pass makes the stratum coverage red
  pre-fix (exact literals, bare), floors the dust re-pin, and pins the
  convex form's bulge instead of claiming it cannot.
- Also converged: the PR body's post-fix digits were not taken on the
  head; the referee's round-down argument is not what `Decimal.sqrt`
  does (the literals are right — both certified them exactly); the
  sampler-error method was deleted and its figures sit on two rows.
- Ordinal 5101 on main (PR 3081). Fix pass sent, union A–K; two review
  targets reclaimed; 26 G free.

## TESS-2 merged (2026-09-22)

PR 3080 at `2aaf73402` merged as `9bd61de7e` on a green full matrix
with main not moved under it. TESS-2, the rational-cells row and the
NURBS-face-bound row closed; spec deleted (ledger); A/B row recorded,
sample #239, no tally candidate. The fix pass found and fixed a second
defect (`apply_ring` on a longer line than its plan). Lane reclaimed.
What TESS leaves here: the patch bound encloses the described patch on
both arms; the sweep's own allowance is CHORD's row with the measured
numbers. Slate next: `tessellate-refuses-approx-face-without-caches`,
`lofted-circle-sections-…`, the two rows TESS-1 filed, and
`check-mesh-passes-the-empty-mesh`.

## TESS-3 and TESS-4 dispatched (2026-09-22)

Two cheap rows, middle tier (style review with a correctness arm, no
A/B row), on disjoint files, concurrently:
- **TESS-3** — the exact-zero row. RING-2 made the ring's `add` the
  backend's with exactness witnesses, so `0 + 0` may now be `[0, 0]`
  and the row's mechanism closed by someone else's change; measure
  first, then pin the exact zero (an `== 0.0` row, not `< 1e-100`) and
  the `split_steps` degenerate arm it decides. `nurbs_cert.rs` is
  shared with CHORD — announced in the PR.
- **TESS-4** — `check_mesh` on zero triangles: what the validator
  claims (closed 2-manifold, vacuously true of nothing) against what
  its callers read it as (the mesh of a solid), decided from a survey
  of which producers can legitimately hand it an empty mesh.
The three rows TESS-1 filed priced (P0/D, P1/D, P4/E).

## TESS-4 green, in review (2026-09-22)

PR 3094 at `e10e8e6b`, full matrix green. `check_mesh`'s contract is
the mesh of a SOLID: `MeshError::NoTriangles` and `EmptyPatch { face }`,
decided before the edge census, D2 row 1 — because `Body::new()` is
tier-1/2 valid ("validates vacuously") and `tessellate` on it
legitimately produces the empty mesh. `tessellate` stays count-blind
(TESS-1's rule is about refusals; a validator re-deriving from the
emitted mesh is the other thing). No caller wanted `Ok` on nothing;
`MeshError` has no exhaustive consumer. Filed on EXCH: the STL writers
turn a zero-triangle mesh into a valid `solid` file and no shipped
caller validates first. One style reviewer with a correctness arm
dispatched. Undecided and named in prose only: whether
`tessellate(&Body::new())` should answer at all — the reviewer is
asked whether that is a deviation owing a row.

## TESS-3 merged (2026-09-22)

PR 3098 at `45077d84d` → `cd0019ec2`. The row was already closed by
RING-2 and nobody had measured it; TESS-3 measured and pinned it
(`== 0.0`, the affine arm's observable, the rational dust bracket).
Diff was docs and four test rows, so it merged on green CI and my read
of the assertions — the bottom tier, not the middle one it was
dispatched at. Filed: `chords-m-bound-zero-arm-is-dead-…` (the same
collapse open-coded twice in `chords.rs` with no exact-zero case).
Lane reclaimed. TESS-4's review is in flight.
