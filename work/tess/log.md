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

## TESS-4 merged (2026-09-22)

PR 3094 at `f2ca19bf0` → `9d3b4183e` after the fix pass (the review's
class: vacuity sentences outside the sweep — the Python cross-checks
and the compiled guide page; the guide sentence re-worded here as the
clause this change rotted, the Python siblings filed on LIB). TESS-4
and its row closed. Lane reclaimed. Four TESS units merged this
sitting; the slate's P0 remainder: `rim-free-loop-on-a-poleless-chart-
meshes-as-a-hole` (D), `tessellate-refuses-approx-face-without-caches`
(D), `lofted-circle-sections-…` (H), `one-element-grid-axes-drop-
schedule` (H), `rim-chords-…` (H, MESH-9 parked on it).

## TESS-5 dispatched; a survey on the Approx-face row (2026-09-22)

- **TESS-5** — the zero-WIDTH twin of TESS-1: a loop of meridians only
  on a chart with no pole (cylinder, torus; the cone's generators end
  at the apex, measured by the lane) walks to a zero-width polygon and
  meshes as a hole. The row says TESS-1's fact does not transfer and
  the unit has to find the statement; the brief asks for the structural
  fact first, measured across the chart kinds, and refuses on it —
  never on a width or a count. Middle tier unless the fact turns out
  to be a door PROPS owns, in which case it is announced. Reviewer
  probes from TESS-1's dual (`loops_the_meridian_guard_admits.rs`) are
  the starting fixtures.
- **Survey lane on `tessellate-refuses-approx-face-without-caches`**
  (P0/D): the fork is a pcurve-cache mint for straight carriers on an
  `Approx` chart versus a tessellation arm that reads the description;
  measure what each costs and who else reads the cache before a spec.

## The Approx-face row closed on a measurement (2026-09-22)

The survey found the wall gone: EXCH's LINE-carrier limb (PR 1798,
2026-09-17) mints the cap's straight-carrier caches, and the box
meshes, weighs and validates once `mint_pcurves` runs — the row (filed
09-04) and the fixture that skips the mint were what remained. The
fork it named needed no choosing; the mesh-side arm would have moved
the refusal from typed to silent. Closed with four filings: the fixture
pin (TINT), the refusal's five spellings and wrong noun (TESS, P3/E),
the LINE limb's unmeasured corners (PCERT), and SHELL's row told its
legs 2–3 lift. **CLASS, the third instance this sitting**: a row's
prose about another program's door goes stale the day that program
lands (the cap row's "import dead", the exact-zero row's mechanism,
this one). Measure-first before every dispatch stays the rule.

## TESS-5 green, in review (2026-09-22)

PR 3116 at `05063dc76`, full matrix green. The fact was not the row's
guess ("no rim and no pole" leaves the one-seam sphere and the cone
generator): the rim-free arm's u-extent is the spread of the loop's
iso-side OPENINGS, a column belongs to an edge, and only an opening
adds a column — so the guard asks for openings on two DISTINCT EDGES.
`TessellateError::SingleColumnCurvedFace { face, surface }`, raised by
`walk::require_two_columns`; four members closed (torus, cylinder
generator, one-seam sphere, cone generator); the hemisphere pair is
props' branch door's and pinned as such. One reviewer dispatched with
the over-fire hunt as the first claim: a native producer's rim-free
loop that uses ONE edge twice (the seam-form wall) would refuse under
this fact if any mints one.

Two of the lane's "looked off" go to the tracker after the review:
the cylinder member used to answer `MissingEntity { "degenerate curved
boundary" }` — kernel-defect vocabulary for a body statement — from
`curved`'s `polygon.len() < 3` count guard; and the build-slot registry
carries two dead exclusive/express holders that make every `-n` grab
exit 75 (the lanes fall back to blocking waits).

## TESS-5 merged (2026-09-23)

PR 3116 at `3d91c2b6d` → `5bc576ce6` after the fix pass. TESS-5 and
the rim-free-loop row closed; the residue the reviewer reached (two
coincident edges, sphere/cone) is its own P1 row. Five TESS units
merged this sitting. Lane reclaimed. P0 remainder is the three H rows:
`lofted-circle-sections-…` (remedies mostly outside `crates/mesh` —
survey first for the owner and the cheapest correct remedy),
`one-element-grid-axes-drop-schedule` (policy-shaped: S29's
sizing-policy conversation), `rim-chords-…` (MESH-9 parked on its
typed trigger).

## TOPO's set_face_surface seam, re-homed from MESH's log (2026-09-24)

PR 2594's lane wrote the two entries below on `work/mesh/log.md` on
2026-09-14; S-MESH left the tracker before the PR merged, and TESS
owns `crates/mesh/src/*` and shares `crates/mesh/tests/patch_memo.rs`,
so they land here with the row they name, now
`work/tess/a-chart-swap-whose-carriers-cannot-certify-leaves-a-face-nothing-can-re-derive`.
Signed (TOPO orchestrator).

### The memo suite's two re-keying rows, by seam (2026-09-14)

TOPO's `topo/set-face-surface-drops-rows-on-chart-change` (PR 2594)
makes `Body::set_face_surface` drop a face's pcurve rows when the swap
re-charts the face, the answer the loop-re-parenting doors took in PR
2549. Two rows in `crates/mesh/tests/patch_memo.rs` built their
"after" body by swapping a surface on a body that already carried
rows, and so read rows about the chart the face had left:

- `arena_keys_are_not_in_the_key_a_reminted_surface_key_hits_on_every_lane`
  re-keys every face of every corpus body to the surface it already
  had; the analytic curved faces (`rounded_prism`, `ball`, `cone`,
  `washer`, `donut`) lose their rows, because two keys holding an equal
  surface with no `GeomSource` read as two charts
  (`work/topo/two-provenance-free-keys-holding-one-surface-read-as-two-charts`,
  which now carries this caller as evidence). It calls
  `topo::mint_pcurves` after the re-key — the setter's own prescription,
  and on an equal surface it re-derives the rows that were there, so the
  row measures the memo's key exactly as before.
- `the_trimmed_nurbs_lane_misses_when_its_surface_changes` perturbs one
  weight of a loft wall's net. Those rows CANNOT be re-minted: the
  wall's boundary carriers are iso-curves of the original net and do not
  certify against the reweighted one. The row now saves the wall's rows
  before the swap and puts them back with `Body::attach_pcurve`, with
  the reason written beside it — the trimmed-NURBS lane cannot run on a
  rowless face, and the body this row needs is the old rows under the
  new fit, built deliberately rather than left behind by a silence.

No mesh source changed and neither row's subject moved. Signed (TOPO,
the set_face_surface lane).

### The reweighted-net body is now a row on this slate (2026-09-14)

PR 2594's fix pass, on both reviewers' finding: the entry above
disclosed the reweighted-NURBS body — a face left rowless by a chart
swap whose carriers cannot certify against the new surface, with
nothing able to re-derive it — and a disclosure is not a schedule
(`work/README.md`). It is now
`work/tess/a-chart-swap-whose-carriers-cannot-certify-leaves-a-face-nothing-can-re-derive`,
with R2's measurement (two `Certify` on the hand-restored body) and
why the honest fixture needs a corpus body with an interior control
point, which `loft_prism`'s 2x3 net has not.

One row was ADDED to `crates/mesh/tests/patch_memo.rs` by that fix
pass: `a_rekey_keeps_a_spline_faces_rows_and_drops_an_analytic_faces`,
which pins both answers `set_face_surface` gives a re-key. It lives
here because this corpus is where minted spline-charted faces are —
`topo`'s own fixtures are analytic, so no row there can exercise the
predicate's shared-`Arc` rung. Signed (TOPO, the set_face_surface lane).
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `mesh/tests/common/witness_bodies.rs`, `mesh/tests/errors.rs`, `mesh/tests/loops_with_no_rim.rs`, `mesh/tests/mesh11r2_probes.rs`, `mesh/tests/mesh7r1_probes.rs`, `mesh/tests/mesh8r1_probes.rs` and 2 more. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. The PR also files `mesh-docs-say-every-face-mints-sense-true` on this slate: a doc here says every face is minted `sense: true`, which has not held since M5 S11. (TOPO implementer)

## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.

## 2026-10-06 — note from CLEAVE: a row moved onto your slate

CLEAVE measured two lamina states (a two-edge self-slit ring in one plane face, and a zero-area
two-edge membrane face) that pass tiers 1–3 and 3′. It filed both rows on its own slate, but neither
is on CLEAVE's ground, so both now live with the program that owns the file:
`validators-accept-a-lamina-slit-or-zero-area-membrane-face` (validate.rs, check 4's wedge rule)
on RESTFRONT, and `tessellator-panics-on-a-self-slit-face-every-validator-passes` (mesh) on TESS.
The two refer to each other. JOIN's PR 4074 is reworking the planar lane's same-position dedup,
which the self-slit relies on, so read that PR before taking the TESS row.

Signed (CLEAVE orchestrator).
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): `mesh/nurbs_cert.rs` `whole_net_bound` matches `TensorCoeffs::derived_u/_v` (same values). `chords-carrier-gate-re-spells-check-direction` now points `chords.rs` at geom-core's `KnotVector::c1`. (ENCL orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4923 merged.** `geom_core::spline::weight_in_regime` (positive and finite) and `check_weight_values` are now the one weight predicate. Every knot-algebra plan that writes a weight (insert, refine, elevate, removal, surface twins) refuses with `WeightCollapse` through `written_weight`.
  - Your file's own spelling of the positive-and-finite check now reads `weight_in_regime`. Behaviour is unchanged. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4984 merged** (`2639b944`), unit 2 of the quotient ruling.
  - **Second order on the quotient form.** `C″`, `S_uu`, `S_uv` and `S_vv` now read the one Bernstein quotient bound over `W³`. Entry points: `geom_core::spline::{accel_pieces, cell_second_quotients, polynomial_area_pieces}` and `NurbsCurve3::accel_upper_bound`.
  - **Moved onto it:**
    - `geom-brep` `patch_bound` `rational_cells` (first- and second-order boxes, `m_sup`). The refinement and `coordinate_hull` are gone.
    - `mesh` chords' rational `sup|C″|`.
    - `offset_meters` `cell_normal` / `patch_regularity`, which now reads the area floor `PatchCell::m_inf`.
  - **Integral arm:** the area floor reads `A_u×A_v` at `2p−1` and certifies to degree 28. Past that it uses a conservative floor of 0.0.
  - **Refusals:**
    - a rational degree above 18 gives `PatchBoundError::DegreeAboveCeiling`, a typed recourse;
    - chords past the ceiling give `UnsupportedCurve`;
    - `QuotientRefused` is new (kernel defect);
    - `RefinedWeightLostPositivity` is gone;
    - `RefinementFailed` now means the integral insertion chain only.
  - **Mesh consequence:** a rational face above degree 18 now refuses `UnsupportedNurbsFace` in nurbs_cert, and offset refuses through `MeterError`.
  - **Goldens re-baselined, with causes in the PR body:** `m_sup`, chord counts, offset goldens, the budget and endpoint censuses, and meter ceilings.
  - **Perf:** the per-face bound is about 2× base on small rows and up to 11× on wide rational faces. Filed at `work/nurbs/second-order-quotient-pieces-cost-the-face-bound-eleven-times-main.md`. — (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5485 merged (`bea03a9f`). It closes **D303**: `mesh::sizing::ceil_count` refuses a step with its sign bit set, or NaN, as the new `TessellateError::StepNotPositive { span, step }`, worded as a kernel defect, instead of flooring to one chord. `+0.0` stays `ResolutionOverflow`, as the underflow end of a positive step. `+∞` stays `Ok(1)`, the zero-curvature reading from `nurbs_cert`'s `grid_steps`/`split_steps`. The doc states all four cases. The existing NaN unit expectation moved from `ResolutionOverflow` to `StepNotPositive`. `pncad-py` carries the new arm (tag `step_not_positive`). No golden moved. **Next on TESS ground (NURBS build C):** `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`, in two PRs. The first is a soundness fix: `NurbsCellGrid::cert` refuses any triangle whose UV box crosses a C⁰ crease cut, because its doc's max-of-cells argument needs C¹ across the cut, and interior grid triangles straddle u-cuts as well as boundary fans. The second meshes creased rational faces: an exact crease chord point on `IsoArc` boundary edges, plus a CDT crease constraint. (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5489 merged (`5b6ac199`). `mesh::nurbs_cert`'s `NurbsCellGrid::cert` now answers `+∞` for any triangle whose UV box strictly crosses a crease cut (an interior knot of multiplicity ≥ degree, in either direction). The max-of-cells argument needs C¹ across the cut, and interior grid triangles straddle u-cuts as well as boundary fans; the docs now say so. `NurbsCellGrid::from_cells(cells, (creases_u, creases_v))` takes the creases as an argument. `geom_core::spline::KnotStructure::crease_runs` is the one statement of where a crease is, and `KnotVector::c1` reads it (same verdicts). Nothing that meshes today moved: the C¹ gate still refuses creased faces. The follow-up (NURBS build C) relaxes the gate for rational creased faces and meshes lofted circles. (NURBS orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5511 merged (`5ad003f6`, NURBS build C). It is a pure refactor of `mesh::trimmed`'s retry loop: the body becomes named steps (`attempt`, `emit`), and two positional comments now name those steps. Bits are identical, and no golden moved. Next on TESS ground: the lattice-metric CDT for `lofted-rational-walls-refuse-at-some-deltas` (FULL). (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5519 merged (`e7f2d17c`, NURBS build C) and fixes main's demo tour. The Klein bottle's wall 8 pinned the C¹-gate refusal on the `circle`-lofted loop, which #5499 removed, so the tour panicked "NO LONGER REFUSES". Wall 8 is retired and kept live, as wall 7 is: the loop must tessellate at the stop's δ = 1e-2, and it gives 75324 triangles. `annulus` keeps `circle_split` while wall 5 (QUAD, tier 3) stands. `teapot.rs`'s `SPOUT_ARCS` prose is stale and gets reworded with `lofted-rational-walls-refuse-at-some-deltas`. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5531 merged (`845daf18`, NURBS build C, FULL). The trimmed NURBS lane's CDT runs in the **lattice metric**: one face-wide scale, a power of two nearest the geometric mean of the realized band aspects, applied at `cdt.insert` only (`cdt_point`, an exact multiply). Certificates, chords, creases, `sub_arc_joint` points and emission stay on unscaled UV. `is_malign` is two-sided on the residual over the metric; `SAFE_ASPECT` is unchanged and its docs now say the measured arm is the upper (raw-UV) one. Over the metric the column snap goes dormant: 0 bands across the tour against 2573 on main. That is the whole cause of the moved counts: d9 on 4 of 40 pairs (`loft_prism`, `swept_elbow`), the lily `finding_13` table (`lily_leaf_b` at δ 2e-3, 5374 → 5254) and the tour budget (+1.3%), all re-baselined and named. Lofted creased frustums and cylinders mesh at all 20 review δ (main: 9/20, 4/20, 5/20, 2/20 refused); the 8-arc controls are bit-identical. `Crossings` is now minted in `planar`. It closes `work/tess/lofted-rational-walls-refuse-at-some-deltas` and opens P3s `the-lower-malign-arm-is-unwitnessed` (witness or retire the snap) and `curved-lane-triangulates-in-raw-chart-coordinates`. Three new tess-lint rows are recorded on `work/budget/tess-lint-gate-red-on-main-23-findings`. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5551 merged (`72efad7f`, NURBS build C, FULL plus a delta review). **The column snap is retired** from `mesh::nurbs_cert::band_schedule`: the fixpoint, `SAFE_ASPECT`'s band role, `malign`, `BandCounts.snapped` and `step_v_at` are gone, and `band_schedule` loses its `patch` parameter. Over the lattice metric (PR 5531) it fired on 0 tour bands. On 15 + 13 adversarial uneven-station lofts × 20 δ, outcomes are identical on and off (worst ratio 0.4996), and the retired build is byte-identical to snap-off. The trade is cost only: up to 14% triangles on degree-2 adversaries and 1.616x on a degree-3 loft, 0 on the tour (recorded in `work/budget/…cost…`). The typed `CertificateExceeded` is the backstop, now guarded: `an_uneven_station_cylinder_refuses_typed` reds if the over-δ check is dropped. It closes `the-lower-malign-arm-is-unwitnessed`, the P0 `rim-chords-exceed-snapped-column-count` (0 of 4000 adversary rim decisions reached its configuration) and MESH-9. It files the cyl4 liveness item (TESS P2), `face-measure-carries-no-lattice-metric` (BUDGET P3) and the `snap_bands` column drop (BUDGET P3; the column stays, always 0). Its doc lines in TESS-BUDGET.md and the tess-lint and tess-meter docs are updated. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5560 merged (`f7970082`, NURBS build C, STANDARD, orchestrator-read). `mesh::trimmed`'s cylinder arm now takes PR 5531's `lattice_metric` on the realized aspect of its one lattice, `(Δu/nu)/(Δv/nv)`. The scale still applies only at `cdt.insert`, so the metric keeps one home. New row `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta` is red on main (`tilted_halves` R=5 20/20, R=20 19/20, R=100 7/20 refused) and green here. No golden moved: every trimmed cylinder in the corpus sits at metric 1, so its CDT input is bit-identical. It closes `trimmed-cylinder-lane-triangulates-in-raw-angle-by-height`, and `curved-lane-triangulates-in-raw-chart-coordinates` closes by measurement (that lane never refuses; the item carries the routing citations and the table). Next on TESS ground: cyl4's row refinement (FULL). (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5570 merged (`7369c4f1`, NURBS build C, FULL plus an orchestrator-read fold). `mesh::nurbs_cert::band_schedule` raises the rows of any band whose residual aspect over the lattice metric is below `1/√8` (`MAX_RESIDUAL_SQ` = 8): `ceil(nvc / (residual·√8))` through `sizing::ceil_count`. Columns, the metric and every other band are untouched.
  - **The line is derived over every split** of the sizing budget: a sliver on an off-row point (one, or two adjacent trim chord points) spans at most a box `a·s_u × s_v`, so its certificate is ≤ `a²/4·δ_s`, and `δ_s = δ/2`. The worst split is cap-bound (`mvv = 0`).
  - It fixes the uneven-station cylinder (cyl4), which refused typed `CertificateExceeded` at fine δ. 500/500 adversary meshes pass at worst ratio 0.4996. The tour cost is 0 (1704 faces, 476554 triangles, byte-identical sizing). cyl4 costs up to 14.3x (`work/budget/row-raised-bands-cost-grows-with-the-band-spread`).
  - The over-δ verdict is now `trimmed::certifies`, pinned by a unit row and end to end by `trimmed::tests::a_face_left_slivered_refuses_typed_through_tessellate` (a `#[cfg(test)]` `ROWS_RAISED` switch, unreachable outside tests). `lofted_circle::an_uneven_station_cylinder_refuses_typed` becomes `…_meshes_at_every_review_delta`. Two bound-level rows pin the line (threshold 9 and 16 mutants red them).
  - No golden moved. It closes `work/tess/cylinder-lofted-through-uneven-stations-refuses-at-fine-delta`. (NURBS orchestrator)
- 2026-10-23 — **Filed (ISO):** `loft-of-three-or-more-sections-at-degree-one-is-unmeshable` (P0 M): a degree-1 loft through ≥ 3 sections refuses at the C¹ gate's `Degree1Crease`, square profile included. It was found by ISO's measurement lane and is the degree-1 residue that the closed lofted-circle row recorded in prose. (ISO orchestrator)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-25 — **Seam (PRED):** PR 5717 (`mesh-sizes-each-lane-by-its-worst-admissible-triangle`, building `crates/mesh/README.md` `[mesh.sizing]`) is open on TESS and CHORD ground: `mesh::sizing`, `chords`, `curved`, `trimmed`, `nurbs_cert`, `cert`/`lib`/`memo` docs, and mesh count pins. `sizing::sizing_target` (δ/2) and `curved::SPHERE_SIZING_MARGIN` are gone; each lane sizes at the coarsest step its own theorem grants, each theorem pinned beside its step (`edge_chord_steps_meet_the_bound_with_equality`, `cylinder_rim_triangle_lands_at_delta`, `cone_grid_step_lands_the_worst_triangle_at_delta`, `sphere_grid_step_lands_the_worst_triangle_at_delta`, the torus pins at δ, and `nurbs_cert`'s residual-line row). The chord pass tightens sphere and cone circle edges to the face's grid step through `sizing::grid_boundary_step` (which absorbs `torus_boundary_step`). The NURBS lane keeps its lattice budget as `nurbs_cert::LATTICE_SHARE` = ½ with `MAX_RESIDUAL_SQ = 4/LATTICE_SHARE` (output unchanged: every NURBS budget row and d9 digest identical). The trimmed cylinder carries `trimmed::UNPROVED_HEADROOM` = ½ at its lattice and its edges' chords (`chords::edge_target`), filed as `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`; `the-nurbs-lattice-share-is-not-chosen-by-measurement` filed beside it. Closes `a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s` (main refuses typed, 1.34 δ; not a mis-mesh). Tour: 499232 → 462958 triangles. (PRED orchestrator)
- 2026-10-26 — **Set `ready`** (Ev, in chat). Every log entry since at least 10-12 is a seam note from another program (ENCL, NURBS, ISO, PRED), with nothing from this track's own orchestrator. A P0 track at about 107/30 with 46 open rows should not be hidden behind a dormant `active` claim. It is owed a priority-seam split when picked up. The P0 `loft-of-three-or-more-sections-at-degree-one-is-unmeshable` (filed by ISO, 10-23) is among its open rows.

- 2026-10-28 — **Picked up; `active`** (TESS orchestrator).
  - **The D10 hold.** No TESS row uses the hold's vocabulary. Every row was grepped for it, and the two INTENT rows that mention meshing are viewer and façade questions. Three rows sit close to it and stay on mesh ground. `a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt` was witnessed through a `PlacedUnion`, but its fix is `mesh::curved`'s CDT insert, so its row is built as a sphere face tilted by an ulp, with no placement. The two `two-coincident-edges-*` rows are the walk's and the census's answers; the carrier-identity reading leans on COINCIDE (noted on TESSWALK's log). The P0 loft crease is raised in ENCL's `patch_bound.rs`, a crossing announced at dispatch.
  - **The semantic-seam split** (`work/README.md`, Track size): 146 points → TESS 24 (reach: valid bodies the mesh refuses), TESSLIE 30, TESSWALK 28, TESSSIZE 24, TESSLATTICE 23, TESSWORD 13. The `plan.md` slate section lists the seams. The five successors are `ready`, and their bands 15900–16399 are recorded in `docs/MODEL-AB-LOG.md`.
  - **Closed:** `demo-tour-wall-8-no-longer-refuses-at-1e-12` was already fixed by PR 5519 (wall 8 retired, `klein.rs`). `a-thin-nested-rings-row-pins-untriangulated-where-the-mesh-now-meshes` duplicates the P1 main-red row on the same test, which is priced E and carries the fix.
- 2026-10-28 — **Dispatched** (TESS orchestrator).
  - `main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused` → `tess/thin-rings-repin`. **Tier: orchestrator's read.** It is a test re-pin, and the lane must show `check_mesh` sound on every new mesh before re-pinning.
  - `sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane` → `tess/sphere-tilted-circle-lane`. **Tier: dual (H → concurrent pair).** It is a new trimmed-lane arm with its own certified UV chord-step bound, which is logic where believing the claim takes more than reading it.
  - `loft-of-three-or-more-sections-at-degree-one-is-unmeshable` → **designers first.** The row leaves open which layer owns a C⁰ crease (the patch bound and mesh lane, or the loft's walls), so one Opus and one Fable designer weigh it. Blinding is on `analysis/design-fork/tess-degree1-crease`. An `[ev]` PR follows only if it is Ev's fork.
- 2026-10-28 — **Closed** `main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused` (P1, PR 5911, `tess/thin-rings-repin`). Since PR 5717's per-lane sizing, all six ops of the three nested thin rings (w 1e-3, 9e-4, 1e-4) mesh at δ 5e-3. Every mesh passes `check_mesh`, and the mesh volumes match the closed forms within 1.8e-5. `thin_nested_rings_build` now holds every op to `check_mesh`, and the dead `Known` / `untriangulated` machinery is removed. `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta` was re-measured and stays open. In its C-shape table, w = 0.04 and 0.02 at δ 0.1 now mesh, and w = 0.01 still refuses at δ 0.5 and 0.1. In its tube-alone table, six cells moved from refused to Ok and one, w 1e-4 opposed at δ 1e-3, moved from Ok to refused. (TESS thin-rings lane)
- 2026-10-28 — **PR 5920 (sphere arm), dual review adjudicated** (TESS orchestrator). R2 returned NOT-MERGEABLE-AS-IS and R1 APPROVE-WITH-FIXES, both on frozen head `832e7cd`.
  - **R2's MAJOR (unilateral, by execution):** the pole side takes its azimuth from the meridian image, so on some faces the trim polygon encloses the complement. `emit` silently flips it, `cert_sphere` certifies against the whole sphere, and the result is `Ok` and `check_mesh`-clean. 47 of 566 witness rows are > 5% off in volume.
  - **R1's MAJORs (unilateral, by execution):** a fold-over beside a trim chord at other δ (the cylinder arm too, predating the PR), and a shared-ear 4-use edge at 4× δ. Each comes back `Ok`.
  - **Bilateral:** `NEAR_POLE_ROW_SHARE` is tuned, not structural; refusals name the wrong edge; `sphere_widest_cos` duplicates `sizing::sphere_face_widest_cos`; `chords.rs` keeps a hand census.
  - Both reviewers disclosed seeing only the other's directory names while diagnosing the full disk; neither read any findings.
  - Fix pass dispatched off the union.
  - **Class filed:** `work/tesslie/a-witness-that-stops-at-check-mesh-cannot-see-a-wrong-region`.
- 2026-10-28 — **PR 5911 merged** (thin-rings re-pin; orchestrator's read). **[ev] PR 5913 opened** (degree-1 crease: Decision 1 agreed, Decision 2 a stated split; fork-log row 208). **PR 5923 opened** (the slab split, Decision 1). It waits on Ev, and a single FULL review was dispatched.
- 2026-10-28 — **PR 5923 merged: the P0 `loft-of-three-or-more-sections-at-degree-one-is-unmeshable` closes** (TESS orchestrator). Single FULL review, APPROVE-WITH-FIXES; the fix pass covered all of it. The positional slab-name placeholder (Ev's 1a) is re-homed to REFS as `loft-slab-and-section-names-are-positional`, P1 and open, not parked: REFS is the program building names-by-read, and the hold does not bind it. [ev] PR 5913 merged with Ev's rulings: Decision 1, and Decision 2 = S, as one clause in DESIGN [topology]. **Correction:** the design-fork row the entries above call 208 is row 209; 208 was taken on main first.
- 2026-10-28 — **PR 5920 merged: the P0 `sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane` closes** (TESS orchestrator). Dual review (H, concurrent) tallied 3 under blinded coding: one by R2, two by R1. Two fix passes and a delta review followed. The second pass's bulk-loaded CDT re-tied cocircular lattice points: 8 of 40 D9 pairs and 11 render cells were re-baselined, with the cause named. Its order assumption rests on spade's documented contract, which I checked. The row is `docs/dual-review-log/rows/PR-5920.md`. **Friction:** PR 5923's `test` job ran 25 min, over the 15-min bar, on a diff that seeds no core crate.
- 2026-10-28 — **Handed back: `ready`** (TESS orchestrator). The sitting's P0s are closed: the degree-1 loft (PR 5923) and the sphere face bounded by tilted circles (PR 5920). The P1 nightly-red thin-rings pin is closed (PR 5911). No dispatchable P0 or P1 row remains, and the one P1 is deferred by Ev. Banded and priced on the way out:
  - `a-notched-or-ringed-cylinder-wall-does-not-tessellate` P2/H (beside the ringed torus);
  - `cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm` P2/H;
  - `a-sphere-face-tilted-by-an-ulp-against-its-copies-fails-the-cdt`, `a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm` and `a-trimmed-sphere-face-holding-a-pole-inside-it-refuses` P3/M.

  The Ruled section of the closed loft row names the S-ruling follow-ons on CARVE, EXCH, MEASUREDOOR, TESSLATTICE and COINCIDE. The positional slab-name placeholder is REFS's P1 (`loft-slab-and-section-names-are-positional`), under Ev's couple-of-PRs bound.
