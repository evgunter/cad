# M10 log — the error-propagation MVP

Narrative record; the plan is `docs/M10-PLAN.md`, the design record
`docs/ERROR-DESIGN.md` (E1–E11, ratified #110). Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction ("you'll be doing M10, error
propagation"), by a fresh orchestrator on a remote container. The
plan is a DRAFT design conversation — nothing dispatches until Evan
ratifies it; this entry records the operational facts that hold
either way.

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `m10/`** — unit branches
  `m10/<unit>-<slug>`, orchestrator branch `m10/orchestrator`
  (Evan authorized the prefix at opening; the harness-designated
  session branch `claude/m10-error-propagation-q3e7i8` is unused).
- **A/B ordinal band: M10 = 500–599**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit that
  opens this program, per that entry's rule. Implementer blocks are
  named `M10-B1, M10-B2, …` (the GUI precedent — `M10-<n>` are
  unit names).
- **This session runs in a remote container** (the GUI program's
  precedent, adapted for a smaller disk): no persistent
  `~/.local/share/cad-work`, no script monitors (PR watching via
  MCP subscriptions + scheduled self check-ins; away-channel
  etiquette followed by hand under the `(M10 orchestrator)` tag),
  GitHub through MCP rather than `gh`. Disk ~29 G free is the
  binding constraint: lanes are worktrees sharing one object
  store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane targets,
  review targets reclaimed at report time (Evan, at opening:
  subagents share fewer repo copies than the local-machine
  workflow assumes; the orchestrator checkout carries no target of
  its own). The build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, and push-early rules bind unchanged.
  The clone arrived SHALLOW; unshallowed with a blob filter at
  opening (ancestry checks and merges misbehave on shallow
  history — a successor in this environment should check
  `git rev-parse --is-shallow-repository` before trusting either).

**Sweep at opening** (what the plan's slate is grounded in, beyond
ERROR-DESIGN itself): #687 (`ContentBits for Dual` — the memo-seed
design question, the one lock left on `evaluate::<Dual64>`), #701
(`Enclosure` ungated in the bounds allowlist), the D1 ruling and
its hedge collected in DESIGN.md's roadmap entry, #1055 (shell's
curved wall-clearance window, aimed at M10's certificate),
CONTACT-DESIGN C5 (the gap-measure contract), PARAM-LINT-SPEC PL6
(independence semantics; DISCIPLINES' unit, not ours), PERF-PLAN's
M10 rows, and the codebase survey recorded in the plan's substrate
section (headline finding: the W2 sketch solver was never built,
so E8's solver walls are vacuous in v1 — plan Q1).

## Ratification (2026-08-29)

Evan ruled all five plan questions in-conversation the same day:
Q1 solver OUT, Q2 mass-prop Measures banked, Q3 MC lane rides
M10-6, Q4 clean breaks, Q5 the #1055 arm in M10-5 as a STRETCH
("possibly ambitious" — the follow-up-unit valve is the answer to
that hedge). Rulings folded, STATUS flipped, #1142 merged. Two
cross-orchestrator registrations landed on the plan PR before
ratification and are folded in the plan text: VERBS's consumer
demand for the clearance certificate (fixtures in-tree), and
PCURVE's measured poison-vs-widen datum, whose CLASS M10-D owns
(PCURVE files the class issue; the instance's mechanism stays
with P-1b).

**Next dispatches**: M10-D (orchestrator-led design pass, its own
design-conversation PR) and the M10-1 spec + block M10-B1 draw run
concurrently — M10-1 does not depend on M10-D.

## Seam: the C6 profile pin blocks E4/E6 on profile parameters (2026-08-29)

Found drafting M10-D, verified at the sites (`eval/slots.rs:27-30`,
`eval/wire.rs:440-482`): profile programs resolve parameter
expressions at f64 and lanes consume the elaborated segments via
`embed`, so a Dual seed on a profile dimension propagates no
tangent and an interval profile parameter does not widen the leaf
replay — silent zeros/points exactly where E4/E6 need signal.
PROFILES-V2 recorded the asymmetry and reserved it for Evan's
eyes; M10 is where the same parameter feeds both slot kinds for
real. Plan amended: unit **M10-P** added (design pass first, its
design PR waits for Evan); M10-3/M10-4 carry the
magnitude-parameter dispatch valve. Amendment self-merged as a
faithful elaboration (the ratified exit shape is unreachable
without it); the design fork itself is reserved for M10-P's PR.

## M10-1 dual review adjudication (2026-08-29)

Both arms APPROVE-WITH-FIXES on frozen 0e9ef0b4. R2 found the
unit's one silent correctness defect (deep-tail cancellation —
`1 − erf` bit-zeros the tail from ~8.5σ; unilateral MAJOR by
execution, tally candidate at merge); the carry-forward class and
the unpinned GUI fix were bilateral at different severities. Fix
pass dispatched as the adjudicated union to a FRESH same-arm lane
(the original lane's worktree had been reclaimed at report time,
which broke resume — reclaim implementer worktrees only after the
fix pass concludes, or accept fresh-lane fix passes and record the
executor covariate, as here).

Class findings given homes at adjudication (the standing rule):

- **"Priced" vs "set-theoretically forced" mass are one type**
  today: `box_mass(Band, covering) = Ok(1)` is measure-free and
  correct, but an E10 unresolved-mass budget over Band-only params
  would read "fully priced" while no shape was ever stated. M10-6's
  report/budget spec must distinguish the two (R2 MINOR-1).
- **"No distribution ⇒ mass 1" lives in prose, not API** — every
  consumer special-cases `None` by hand. M10-3's driver spec should
  give the fixed-param case a typed spelling (R2 style finding).
- **A distribution is invisible in the GUI** (`ParamRow` carries
  none), so users can hold state they cannot see; the census
  records Python's gap, nothing records the GUI's. GUI follow-up
  slate (R1 NOTE).

## M10-DI dual review adjudication (2026-08-29)

Both arms confirm CORRECTNESS with independent byte-identical
merge-base differentials (three lanes each); every finding is
guarding, coverage, or text. R1 (4 MAJ, by mutation testing and
witness re-measurement): three of four certifying policy arms
gut-to-green; the Dual arm's `Ok(())` grant + the unenforced E4
pairing hook (a silent deviation — DL3's own sentence); DL3
witnesses named wrongly (the real ones are cut_cylinder and
loft_prism, 2/18); the DL5 selection clause contradicting the
projection::mid counterexample class named sixty lines above it.
R2 (0 MAJ, APPROVE): the assemble-at-Dual availability inversion
pinned (same class as R1's second MAJOR), the digest-depth
overclaim, the DL5 surplus remedy clause. Fix pass dispatched
IMPLEMENTER-INHERITED (the worktree-kept rule worked — resume
succeeded). Tally candidates recorded at the row for the blinded
coding with class annotations (test/doc-class per the LILYWELD
3b precedent is the likely coding; not this log's call).

Durable homes at adjudication: R1's free DL6-class datum (an
EXACTLY coincident flush pair refusing `margin: Invalid` at plain
f64 — the class's third member, and its first on the default lane)
posted to #1143 with the fixture named; the E4 pairing hook is a
named obligation for M10-4's spec; `real.rs`'s ~270-line Bounds
ledger accumulation (both arms flagged the shape) banks as a
docs-home candidate rather than riding any unit.

## M10-P dual review adjudication (2026-08-29)

Both arms APPROVE-WITH-FIXES on frozen 55b1fd13, both re-deriving
the bit-identity fence independently on the TRUE merge base
(5fed0960 — the PR's provenance cited a stale one) and at greater
depth than the unit's own instrument; R2 additionally covered
Probe, which CI never runs. Every machinery claim held: guided ≡
plain at f64 (adversarial families included), no structure
selection at the lane scalar (the other-pocket consumption receipt
is the elegant proof), canonicalization structurally pinned with a
live-control decide-count row, ladders fork-proof by construction.
The MAJORs are reach/receipt/disclosure: the typed `Structure`
vocabulary covers 1 of the record's 11 decision classes vs the
claim (bilateral); the periodic-reduction class receipt missed
seven same-spelling sites and four same-shape floor-folds in topo
(bilateral — the class gets its own issue from the fix pass); the
interval-box door's unreachability through `evaluate` was
undisclosed (adjudicated: it IS M10-3's first spec bullet —
disclosure fix, not machinery). Fix pass IMPLEMENTER-INHERITED.

Durable homes at adjudication: the floor-based period-fold class →
its own issue (fix pass files, full both-spellings hit list); for
M10-3's spec — the interval parameter door is where R2's M1
friction dissolves, `ProfileLaneReplay` deliberately dropping lane
scalar payloads (R1 friction 2) should be revisited when the
driver wants the lane pass's refusal payloads, and the guided
enclosure being node-dependent (extrude widens, loft stays f64 by
C6/D9) needs stating in the driver's leaf semantics.

## Operational: disk math on this container (2026-08-29)

Measured under pressure: ONE workspace-wide interval battery in
debug costs ~15-16G of test artifacts — two concurrent lanes only
fit if at most one runs a world battery. Standing brief lines from
here: lane batteries run `CARGO_INCREMENTAL=0`, and scope to
touched crates (`-p ...`) unless the unit's sweep demands the
world — the hosted gate covers the rest (local-battery-scope's
time argument, now applied to disk). Reclaim-at-report remains
the transient's fix.

## M10-2 dual review adjudication (2026-08-29)

Split verdict on frozen e0cc0b20: R1 MERGEABLE-with-one-MAJOR
(rubric 4/3/4), R2 REQUEST CHANGES (4/2/2, 3 MAJ). Both reviewer
lanes were killed mid-review by the same account-limit wave and
resumed — a symmetric interruption, recorded for the row. The
transform/minted-carrier finding is bilateral and the center: a
measure over a transform-descended ref silently reads the UNMOVED
carrier (0.5 where the placed geometry sits at 0.75; 5 where it
sits at 95) and deviation 3's documented recourse — "the moving
node's own emission" — does not exist (wire_transform Arc-clones
the table through and mints nothing). R2 showed the interrogation
layer already takes `(ev, node, name)`, so the fix direction is
read-at-the-referenced-node, typed refusal as the fallback, plus
the schedule deviation 3 owed (Q6). R2's unilateral M1 is the
severity headline: `eval_measure` restates `expr::eval`'s
arithmetic without its non-finite door, so `13/s` at `s = 0`
measures `inf` and the assertion over it reports `Holds` — a false
PASS from the node whose job is certifying intent. The plane-gap
sign was bilateral at different severities (R1 MINOR, R2 MAJOR
with the role-swap-does-not-negate table over disjoint slabs);
adjudicated MAJOR — the plane arm folds sense (the carrier_eq S10
discipline) so g means material separation and role swap negates.
Fix pass IMPLEMENTER-INHERITED, adjudicated union: the 3 MAJ, both
probe suites adopted (six of nine closed-form arms had no
red-capable oracle in the PR's own suite — all six verified CORRECT
under both reviewers' independent oracles), the misattributed
parallelism refusal, the 1 m arm-floor honesty (docs must name the
floor as the operative sub-metre lever; the lever redesign banks on
chart_region.rs:804's standing criticism), the unreachable digest
arms (a corpus doc carrying Measure+Assertion makes them live), the
weak Python rows, and the accumulated prose/doc sweep (the
key-format bump-rule tension disclosed; this log's own
deviation-count line corrected by the fix pass).

## M10-3 dual review adjudication (2026-08-29)

Both arms on frozen 54a77ad9: R1 NOT-MERGEABLE (1 MAJ, rubric
4/4/4), R2 REQUEST CHANGES (2 MAJ; rubric requested post-report —
the lane omitted the triple and was resumed for it, the M9-3
missing-data shape avoided because the worktree-kept rule held).
The accounting composition is bilateral and byte-identically
diagnosed from independent fixtures: the mass columns are
UNCONDITIONAL (a leaf prices P(offset ∈ leaf), so the leaves
already sum to 1 − tail) while `total()`/`unresolved()` compose
`t·(1−tail) + tail` as if conditional — a Normal axis at the
default ±3σ totals 0.99730729, and the E10 honesty gate
under-reports unresolved mass by the whole tail, the unsafe
direction. Invisible to the shipped suite because every fixture is
bounded (tail ≡ 0) — the premise-excludes-the-failing-mode shape —
and both arms shipped deliberately-RED counterexample rows. R2's
unilateral second MAJOR is structural: `drive.rs`'s flip naming is
a SECOND verdict-diff engine using positional zip, the method
`resolve/vdiff.rs` ("built once", for exactly the f64-vs-Interval
case) rejects as unsound in its own 17-line argument; certification
stays conservative (unequal ⇒ refuse, no false certificate) but
FlipCrossing evidence can name permutation artifacts and miss true
flips. Adjudicated: name flips through the built-once engine, or
carry the confronting paragraph plus an honest best-effort label —
the missing paragraph is the defect either way. The claim-8 attack
(a definiteness misclassification that CERTIFIES) came back NOT
FOUND from both arms independently — the unit's one silent-defect
shape stood.

Fix pass IMPLEMENTER-INHERITED: the two MAJ, the bilateral
max_leaves 2× overshoot, the macroscopic pin's budget mismatch and
the ε/4-vs-ε/8 threshold correction, the irreproducible 26-hit
sweep receipt (both arms count 23 unique; R2's second sweep adds
~15 predicates the pattern missed), the containment positive arm
row (both arms constructed it — code correct, coverage absent),
deviation 3's missing schedule (an issue gets filed), the k_stats
narrowing vs E6's "every" disclosed as a deviation, the stale
deviation 7, and both probe suites adopted (the four red-by-design
rows go green with the composition fix).

Durable homes at adjudication: deviation 3's escalation-channel
issue is filed by the fix pass; the extra widening-class hits fold
into the corrected sweep receipt (the class's home remains #1191);
the verdict-diff triple-spelling (vdiff / drive.rs / verdict_summary)
gets a consolidation issue from the fix pass unless the MAJ fix
itself unifies.
