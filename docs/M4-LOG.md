# M4 Implementation Log

Orchestrator's running log for M4 (parametric model layer). Ratified
work order: `docs/M4-PLAN.md` (#80, 2026-07-23, F1–F9 + Evan's F6
early-spike amendment). Binding pre-M4 design: `docs/NAMING-DESIGN.md`
(#74), `docs/SOLVER-DESIGN.md` (#79). Obligations grounding:
`<main-checkout>/references/notes/m4-obligations-inventory.md`.
L-numbering continues (counter at L7, unused since M0).

## Process conventions (inherited from M3 unchanged)

- One implementer + one adversarial e2e reviewer (falsification
  assignments, real consumer programs) + one fix pass per PR;
  reviewer suites promote as `review_m4_prN*`; self-merge with full
  writeups on green gate; genuine design forks wait for Evan.
- Branches `ev/m4-<n>-<slug>`, merge commits only; OUTPUT DISCIPLINE
  header in every agent spec; push after EVERY commit.
- Merge gate: `scripts/gate.sh <merged sha>` while hosted CI is down
  (keep ci.yml in sync); all topology-determining comparisons through
  Q1 trileans into `geom_core::k_stats`.
- Evan's sequencing principle (recorded at #80 ratification): he
  holds sequencing opinions only where order could affect the final
  design (stopgap-entrenchment risk); surface exactly those.

## PR 1 (editor-core: recipe substrate) — launched 2026-07-23

Binding spec `docs/M4-PR1-SPEC.md` (D1–D9): crate editor-core
(geom-core dep only), Doc-as-value + pure apply, RecipeNodeId
(monotone, never reused), F4 node vocabulary as data,
structural-vs-continuous as distinct typed slots, expression
sublanguage v1 (F1 lattice with same-dimension ratios REFUSED in v1;
F7 AST, no conditionals; scalar-generic evaluator with Interval
instantiation pinned), ExprPath {node, slot, path} with stability
tests, DocEdit v1 arms + reserved-arm plan, replay-identity + doc
diff. Acceptance: the die authored as a document through apply.

## STEP spike (F6, early per Evan's #80 amendment) — launched 2026-07-23

Parallel adopt-vs-in-house evaluation of ruststep/truck-stepio for
the AP203/214 analytic-subset EXPORT (import stays M7).

**F6 DECISION (2026-07-23, spike report
`references/notes/step-spike-report.md`): IN-HOUSE subset writer;
adopt nothing at runtime; ruststep (Part 21 parser) +
truck-stepio's importer become DEV-DEPENDENCY parse-back oracles in
tests.** Grounds, executed not estimated: (1) ruststep cannot write
STEP at all (serialization is its own open roadmap item, ruststep#13)
and its AP203 semantic layer failed on a minimal two-entity file;
(2) truck-stepio's writer ships conformance defects unfixable
through its API (resource-schema FILE_SCHEMA over an AP214 data
section; FACE_SURFACE where ADVANCED_BREP_SHAPE_REPRESENTATION
requires ADVANCED_FACE; hardcoded units/empty product/unwrapped
uncertainty; no analytic-surface printers — wrong for the M5
carrier story); (3) the spike's ~120-line prototype produced a
152-entity AP214 cube that an independent importer reconstructed as
exactly 6/12/8, already MORE conformant than truck's output;
generalized writer ≈ 450–650 lines M4 scope. Hybrid rejected: it
buys the trivial record-printing 30% while denying control of the
acceptance-critical preamble. **Open caveat for PR 7: no FreeCAD/OCC
tool exists on this machine, so the external-import acceptance is
NOT yet discharged — PR 7 needs a FreeCAD import run where one is
available.**

## Corner-aligned table investigation (Evan's #71 question) — 2026-07-23

Evan asked whether true corner-aligned table legs are possible yet
(demo ships the straddle workaround). Investigation (PR #82,
`crates/topo/tests/demo_tripwires.rs`): **(1) a single
corner-aligned leg NOW UNIONS tier-2-exactly** — the mixed
collinear+transversal seam class opened at the PR 5.5 fix pass and
nobody retried; pinned as a capability. **(2) Known gap (loud): the
result fails tier 3 `DescriptionNotAdjacent`** on seam edges lying
IN the shared plane (plane∥plane intersection degenerate — no honest
Intersection description exists for those edges); secondary tripwire
fires when the gap closes. **(3) The full table still refuses**
(second leg `NonMaximalFaces`): the first union's flush faces cannot
merge — equal-but-independent descriptions, ladder rung (b), by
design. Opener = M4 PR 5's Declare + GeomSource (declared rung then
glues); the primary tripwire fires with demo-upgrade instructions
when the second leg unions. The corner-aligned table is thus the
first concrete consumer of the M4 naming decisions.

## PR 1 conversation rulings (Evan on #81) — 2026-07-23

Evan's in-thread round produced three rulings for the fix pass:
(i) **non-finite floats: doors 1+2 land in PR 1's fix pass**
(construction-time refusal at literal/SetDocParam — best
diagnostics, poison never enters the document; eval-time finiteness
check at the T-erasure choke point with ExprPath in the error —
required because Div mints inf/NaN from finite documents; F3's
persist-time refusal stays as backstop); (ii) **Doc<P> genericity
deviation ACCEPTED** (Evan endorsed); (iii) **cast_precision_loss
suppression replaced by the std i32 hop** (i32::try_from →
f64::from, lossless by type; typed error outside ±2^31).

**PR 1 adversarial review (2026-07-23)** — suite `review_m4_pr1*.rs`
(15 tests, `ev/m4-1-review`), all execution-verified. ONE BLOCKER:
StableName refs escape apply validation (phantom-name Declare
inserts accepted; deletes strand Declares silently). **Orchestrator
ruling (spec D3 carve-out added)**: edit-time EXISTENCE validation
required (never-existed id = typo, refuse); delete-stranding stays
allowed = N5 dangling semantics (NodeGone at resolution, Rebind
repairs) with rustdoc. Non-blockers: CountToScalar(i64::MIN) panics
in abs (the ruled i32-hop closes it — regression test added to fix
pass); Doc PartialEq/diff are bit-blind (-0.0/NaN) — replay tests
and diff go bit-semantic (diff is the future SetTolerance-audit
substrate); ExprPath same-slot ancestor replacement silently
re-points stale paths (doc note; PR 5 GeomSource must not assume
detectability). SURVIVED under attack: replay bit-identity (subnormals,
-0.0, ulp pairs, delete churn), dimension checker (no smuggling hole),
purity to the bit, independently-authored die isomorphism +
node-granular diff exactness, Interval enclosures with healthy
decorations. Non-finite conduit premise execution-confirmed (Div
mints inf/NaN from finite docs; interval lane already refuses via
NaI). Fix pass dispatched (6 items); merge after gate.

**PR 1 MERGED (#81, `af5a94b`, 2026-07-23)** after the full cycle.
Fix pass landed all 6 items: DeclareNamesMissingNode edit-time
validation (+ N5 delete-strand rustdoc); non-finite doors 1+2
(door 2 via the sanctioned Decide probe — exact, no ε-dependence;
`eval` bound widened Real→Decide, D1-sanctioned, accepted);
i32-hop CountToScalar (closed the reviewer's i64::MIN abs panic;
CountToScalarOutOfRange typed); bit-semantic `bit_eq`/diff (sound
because door 1 makes NaN unrepresentable in stored docs; PartialEq
stays IEEE, documented); ExprPath staleness rustdoc; review suite
merged (38 tests total in the crate). Gate 11/11 on `b6393b7`;
post-merge confirmation gate on combined main `af5a94b`: 11/11
PASS (2742s) — the combined tree is fully verified. **PR 2 spec obligations banked from
this cycle**: wrap `EvalError::NonFiniteResult` with node/slot
context at the evaluation service; instantiate `Doc<P>` with the
real profile payload.

## PR 2 (evaluation service) — implemented, PR #83 open — 2026-07-23/24

Binding spec `docs/M4-PR2-SPEC.md`. Delivered on `ev/m4-2-eval`
(`a6e727d`): F2-verbatim Evaluation/NodeResult/NodeValue (Kahn
min-id order documented; poisoning descendants-only, `through`
walkable); full F4 wiring; 128-bit two-basis FNV-1a content keys
over bits (Merkle upstream); memo acceptance pinned (slot edit ⇒
2/54 recomputed, pip-depth ⇒ 48/8; memoized ≡ scratch by arena
fingerprint); CancelToken between nodes, typed partial, Epoch;
rayon idiom-1 with seq/par×scratch/memo fingerprint identity; die
(56 nodes) evaluates to exactly 7.8359375 at f64 + bracketed at
Interval. **Reported items**: (i) `topo::transform_rigid` LANDED
(no public rigid transform existed; spec's report-or-land clause;
rigidity door + per-edge re-certification, own suite) — reviewer
attacks first; (ii) die uses 6 pip masters + translation-only
Transforms (rotational placement can't hit the dyadic oracle —
cos π/2 ≈ 6e-17; rotation tested separately); (iii) memo.rs
repr_bits joined the bit-identity allowlist (hashes per D4, never
compares; retirement-scheduled note; both CI files); (iv) gate
caveat: harness killed gate.sh mid-row-9 — rows 1–8 attested from
the run, 9–11 completed green in the same runner/sha; the fix-pass
gate must be ONE uninterrupted 11/11 run. Adversarial reviewer
launched (R1 transform_rigid certificate honesty, R2 content-key
completeness incl. the un-hashed doc-ε flag, R3 memo soundness
under edit-back/delete-reinsert, R4 poisoning determinism +
cancelation memo hygiene, R5 parallel tearing, R6 die deviation +
τ-door margining, R7 interval lane + allowlist honesty).
