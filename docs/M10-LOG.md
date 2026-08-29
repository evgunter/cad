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

## M10-1 MERGED (2026-08-29)

The program's first unit is in: distributions in the document, PR
#1147 at sample #39 (ordinal 500, the band's first). The dual
review's headline is R2's unilateral silent MAJOR — the deep-tail
`1 − erf` cancellation on exactly the number E2 forbids dropping —
fixed with one shared exterior/CDF pair that `quantile_z` also
reads, so the analyzed box always holds the mass the tail column
complements. The carry-forward class closed structurally
(`SetDocParamValue`); both probe suites promoted as merge parents.
Process findings recorded in the row: R2's rubric is missing data
(orchestrator worktree-reclaim broke the resume — the rule is now
"reclaim a lane's TARGET freely once its report is in; remove the
WORKTREE only when the unit fully concludes"), and the fix pass ran
on a fresh same-arm lane for the same reason.

## M10-DI MERGED (2026-08-29)

The Dual contract is implemented: the e4 door is OPEN
(`evaluate::<Dual64>` builds the whole corpus, value channel
bit-identical per-node), the policy seam is typed
(`AtRestOutcome`), the Enclosure gate fires, and the delegation
rule is the ledger's standing criterion. Sample #40, ordinal 501.
Both review arms independently proved correctness by differential;
the findings were all guards and honesty, the sharpest being R1's
mutation testing (three certifying arms could be gutted green —
now each is pinned to its validation door). Issues 687 and 701
close with this merge. The E4 pairing hook (DL3's own sentence)
is a NAMED obligation on M10-4's spec.

## M10-2 DELIVERED (2026-08-29)

Measure and Assertion exist. `Node::Measure { expr, refs }` is E3's
one dimension-generic sink — a `MeasureExpr` over `Primitive` leaves
that index the node's frozen `StableName` list, with the F1 lattice
asked (not restated) at every constructor. `Node::Assertion
{ measure, bound, dir }` is E10's persisted half, report-only by
construction: no op in the vocabulary takes a verdict as an operand,
so a `Violated` assertion cannot reach any downstream outcome.

The v1 primitive table ships with its scope stated: vertex x vertex,
vertex x plane, parallel plane x plane and parallel cylinder x
cylinder for `distance`; plane x plane and line x line for `angle`;
C5's three carrier pairs for the signed `gap`, sign convention
binding. Every other pair refuses typed naming the pair class. Two
trileans are consumed, both EXISTING funnel predicates at their
existing margin shapes (`bool_plane_parallel`,
`carrier_cyl_axis_parallel`); one new margined compare is minted, the
assertion comparison, and it took ledger row F16 with the honest
argument that E3 forecloses its own repair.

Schema v16 (claimed by an explicit by-eye read of main's constant),
populated goldens, prior-version refusal fixtures, load-door
re-checks for both node kinds. The binding census closed with a
SPLIT disposition, which is this unit's own judgement: the READING
door ships (`Value.measure` → `Measurement`, `Value.assertion` →
`Verdict`, both readable off a document authored elsewhere), and the
AUTHORING vocabulary is chartered as `B-MEASURES`. The friction the
R-series keeps finding is unreadable results, so that is the half
that shipped.

Two deviations, both stated in the PR: measure references are
CONSUMING DAG edges (nothing else can order a sink after the geometry
it measures), which makes deleting a referenced node a delete-door
refusal rather than N5 stranding; and `gap` takes its pair as two
carrier references in mating-role order rather than a `Declare` node
id plus a pair index. `min_clearance` does not exist here.
