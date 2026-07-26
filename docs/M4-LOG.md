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

**PR 2 MERGED (#83, `8aec775`, 2026-07-24)** after review + fix
pass. Review: ZERO blockers across R1–R7 — transform_rigid survived
direct residual measurement (worst 3.3e-15 under 32 composed
rotations; typed refusal at 1e9-scale degradation); stale-ε memo
attack structurally unreachable in v1 (three walls; keys carry
eps/k for PR 6); memo/parallel/cancelation all bit-sound. Review's
one find: NaN/inf TRANSLATION bypassed check_rigid (refused
obliquely via certification) — fix pass added the NonFiniteMap
door (component-wise x·0 probes, k_stats-named). First gate run
FAILED the 1e-6/1e-12 rows: both review band tests had hard-coded
default-ε probes — made ε-aware (probes from ambient Tolerance);
the ε matrix thereby proved it catches ε-brittle TEST code too.
Final gate 11/11 uninterrupted on `11bca72`. **Rulings**: τ−ulp ⇒
Full stands as designed band semantics (flagged to Evan for
awareness); doc-ε/ambient-ε unification re-audit = PR 6 obligation.
**Banked for later PRs**: PR 6 re-audit (doc ε joining the content
key if ops ever read it); Instances consumer lands with its first
real user; incidental find — exactly-touching union refuses typed
at the kernel (pre-existing M3 envelope, fixture knowledge).
Next: PR 3 (naming part 1 — RolePath enums, eager name tables, CI
invariant).

**Rotation-exactness question (Evan, #83, 2026-07-24)** — "is
inexact sin/cos a problem in general?" Answer on the thread: no in
every load-bearing layer (Q1 margins + D4 certified bounds never
depend on exactness; D9 = bit-identical replay not exact values;
test `==`-sharpness degrades to brackets off the dyadic grid); the
one design-relevant case — rotation-induced intended coincidence —
is exactly what N6 GeomSource-through-transforms solves (intent is
recipe data, not float luck). **Banked idea (not scheduled):
signed-permutation fast path in `transform_rigid`** — quarter-turn
rotations are exactly-representable signed permutation matrices;
a bit-checked recognize-and-permute path would keep dyadic models
exact through 90/180/270° turns, preserving ==-sharp oracles.
Principled, small, optional.

**Transform re-certification contract sharpened (Evan, #83)**: Evan
correctly observed universal re-certification success is impossible
without sub-ε nudging (rejected — value-fudging; an explicit
D7-style repair op is the honest shape IF a need is ever
demonstrated). Ruled contract = two classes: provable survival for
bodies with residual slack > the map-application noise bound
(O(ulp·scale) — virtually everything the pipeline mints, ~six
orders of headroom at defaults), and an irreducible sliver class
(residuals within noise of ε) that refuses TYPED. **PR 3 fix-pass
items banked**: (i) transform_rigid rustdoc upgraded from
"empirical" to the two-class statement with the noise bound;
(ii) a deliberately-marginal fixture pinning the sliver class
refuses typed at the ε boundary. **Demo note**: the demo die stays
translation-authored (Evan: it's how a human would do it) — applies
to the PR 8 demo-as-recipe rebuild.

**Re-mint-at-transform SHIPPED (PR #84, merged 2026-07-23, main
`098c4c1`)**: `transform_rigid` now re-mints Intersection witnesses
construction-fresh from the MAPPED carrier at the pinned mid
parameter (`carrier'.eval(sample_param((CERT_SAMPLES-1)/2))`) —
the same S2/WitnessMidpoint formula construction uses — instead of
mapping the stored witness. Fresh headroom every transform (bit-zero
mid residual), no chain ratcheting, per-edge local (no cross-entity
dependence — Evan's locality concern), D9-clean (pure function of
mapped carrier + pinned param). Sliver refusals unchanged: typed
errors, no nudging (ruled: not until demonstrated need). The
`unreachable!` in the old `map_description` is gone (total
`map_mapped_curve`; never-panic discipline). Pins:
`m4_remint_transform.rs` (bit-equality on real boolean output under
a rounding-hostile map; 5-transform chain stays fresh; determinism)
+ `m4_remint_headroom.rs` (0.9ε marginal witness certifies but
keeps consumed slack under map-the-witness even for identity; the
re-mint formula certifies with fresh headroom). Gate 11/11 on
`b363fb0` (ff from main ⇒ gated sha ≡ merge content). Banked item
(i) (rustdoc two-class statement) is DONE via #84's module docs;
item (ii) (sliver-class typed-refusal fixture at the ε boundary)
stays banked for PR 3's fix pass.

**PR 3 implementer casualty + recovery (2026-07-23)**: the naming
implementer hit the Fable usage limit mid-final-verification
("Table looks honest. Running the full editor-core suite"). First
assessment said its worktree died and the uncommitted editor-core
half was lost — WRONG: isolation-worktree subagents live under
`<main checkout>/.claude/worktrees/agent-<id>/`, not the session
scratchpad, and the worktree (with all uncommitted work) survived
at `/home/evan/projects/cad/.claude/worktrees/agent-ab07efef…`.
Evan's question ("can the agent not be resumed from transcript?")
prompted resuming the original agent instead of the freshly
spawned replacement; the resumed agent found its own worktree,
committed the editor-core half (`6345291`: StableName/RolePath
made real, N2 discriminators, bidirectional NameTable filling the
PR 2 slot, per-op wire emission, structural Declare hashing),
merged the remote branch (which had picked up main `098c4c1` via
the stood-down replacement's `90f7a4e`), and pushed — branch head
`f9fd585`, everything on the remote. **Lessons, now standing
policy**: (i) implementers commit+push after every coherent unit
(no batching to a final push); (ii) a dead subagent's first
recovery move is RESUME-FROM-TRANSCRIPT (it knows what it wrote),
and its isolation worktree under `.claude/worktrees/` likely still
holds the files. Kernel half remains `3d93561` (crates/topo,
+271/−19). Verification still to finish; then review cycle.

**Parallel tracks opened (Evan's parallelization question,
2026-07-23)**: two independent tracks launched alongside the PR 3
verification tail. (1) **STEP-export half of PR 7 pulled forward**
(branch `ev/m4-7-step`, clone `~/.local/share/cad-work/step-export`):
in-house AP214 subset writer per the F6 decision, spike code
(preserved at `~/.local/share/cad-work/step-spike/`) as seed,
ruststep/truck-stepio as dev-dep parse-back oracles; FreeCAD
external-import acceptance stays open (no tool on this machine).
Needs no naming — consumes finished kernel bodies only. (2) **M5
design prep** (branch `ev/m5-curved-design`, clone
`~/.local/share/cad-work/m5-design`): DRAFT `docs/CURVED-DESIGN.md`
in the NAMING/SOLVER-DESIGN house style — curved intersection
representation + D4 certification for SSI curves, pcurves, certified
marching, analytic-pair dispatch, fillet predicate reification,
TangencyLocus, D7 boundary, BVH trigger — plus open-questions-for-
Evan list. Explicitly a design conversation: NO self-merge; Evan
pushback precedes ratification. Serialization kept where real:
PR 4 stays behind PR 3 (builds on its tables/discriminators), PR 5
behind PR 4; PR 6 core could overlap PR 4 but both live in
editor-core — contention not worth it. Appearance half of PR 7
unblocks at PR 3 merge (StableName) and can then run beside PR 4.

**FreeCAD oracle installed (Evan-approved, 2026-07-23)**: FreeCAD
1.1.2 headless (`freecadcmd`, extracted AppImage — apt only carries
0.18/2019) at `~/.local/share/cad-work/freecad/`. Smoke test: the
F6 spike's in-house AP214 cube imports as a VALID solid, exactly
6/12/8, volume 1.0. PR 7's open external-import caveat is now
DISCHARGEABLE on this machine; the STEP implementer was directed to
wire it as an admesh-pattern check script (env-var locator, loud
hermetic skip when absent) run against cube, die, and a
boolean-result body.

**PR 3 implementation COMPLETE (2026-07-23, `ev/m4-3-names` @
`afa6c0b`, pushed)**: the resumed implementer finished — recovery
total, nothing redone. D1 StableName real (runtime kind tag — kind-
heterogeneous collections + F3 serialization); D2 full closed
RoleSeg vocabulary (composition via boxed operand StableNames per
N1; section-face completion-order index argued recipe-covariant via
split_join_order exact-order sort; transform contributes no segment
— key-stable arenas); D3 discriminators through k_stats::decide,
ambient band, SideVerdict{+,−,Mixed,On}, in-band ⇒ typed
Naming(Escalated), no bare indices; D4 bidirectional NameTable in
NodeValue (Arc), injectivity/totality typed, kernel emission gaps
fixed kernel-side (SplitNaming, BooleanNaming incl. pre-remap
reduction_contacts, GraftMap edges, ChordJoiner fragment rows,
provenance accessors); D5 all four CI-invariant tests (golden die
digest 0x8d53_0dcf_2954_07bb; no-flip motion ⇒ zero table changes
across 56 nodes; f64/Interval table equality; node-granular flip
localization) — single-qualifier-flip fixture deferred (reviewer/
PR 4 item). 17 tests across 4 suites; full workspace + interval
lanes green; fmt/clippy clean. Typed-refusal deferrals: Merged
variant lacks evaluate-level fixture; sphere-like all-on-axis
revolve poles; tied-upstream-through-downstream; >1 section chord
per operand face. Found-and-fixed in verification: same-operand
collinear chord descent; provenance chases stop at the operand
table (was breaking the PR 2 rich-doc test); zip vertex_merges
dead-partner rows (A-priority documented). Implementer supplied a
10-item reviewer attack list. **Adversarial review launched**
(clone `~/.local/share/cad-work/pr3-review`). Fix-pass queue
already holds: sliver-class ε-boundary typed-refusal fixture
(banked from #83).

**M5 CURVED-DESIGN draft OPEN as PR #85 (2026-07-23, design
conversation — waits for Evan, NO self-merge)**: 917-line draft in
the NAMING/SOLVER house style. C1-C12 spine: intersection-locus
ladder (analytic > exact conics > fitted-with-certificate); fitted
certificate = residual + Bernstein hull sup-norm honesty +
uniqueness tube (W2 transferred: selection is certification);
march-then-certify SSI (Hoffmann stepper untrusted, ℝ⁴ tracing,
box-exclusion exhaustiveness, singular tracing refused); per-half-
edge pcurve caches certified in meters; total (kind,kind) dispatch
table; cache-structure-is-f64-lane (C6); TangencyLocus + sector
predicates, M5 booleans transverse-only; analytic-first fillets
with reified predicate list; in-house MIT interval ring (inari
stays quarantined); BVH doubling as SSI exhaustiveness structure;
NURBS substrate scope; 9-seam refactor inventory. OQ1-OQ9 for Evan
(conic extent, certificate staging, exhaustiveness gate placement,
pcurve-vs-carrier primacy, curved-3′ deferral, fillet scope,
tangency symmetry, interval-ring sign-off, Q5/curvo closure).
T1-T6 tensions flagged not relitigated (D4 two-strength
"certified"; LGPL boundary text; witness selection as proof
obligation; D9 discipline on iterative numerics; K=10 revisit
likely fires; CDT bulk-load trigger). Sources visually read:
Hoffmann §6.2-6.3 complete, NURBS Book §7.3-7.4 + §9.4.4,
Vida-Martin-Várady taxonomy pages.

**PR 3 adversarial review returned (2026-07-24): needs-rework; fix
pass ruled and dispatched.** Review ran 13 uncommitted probes; 4
found real breakage. Findings + rulings (R1-R13): R1 BLOCKER —
chase_b false refusal on a bar pierced through two A-walls (middle
B fragment dies pre-graft; `?` bypasses the chord_kind rescue the
A lane gets) ⇒ B-lane fall-through parity + both-orientation
fixture. R2 MAJOR — split through existing operand vertices
refuses ("crossing vertex without SplitEdge birth record") ⇒ ruled
pass-through-with-side role (derivable from birth data per D2;
STOP-and-report fallback if not). R3 MAJOR — head fails `cargo
fmt --check` (3 files; fmt commit predated last two commits). R4
MAJOR — Merged lane dead code end-to-end until PR 5 declare
threading ⇒ kernel-level synthetic merge_groups unit test + REPORT
line; full fixture PR 5. R5/R6 report-only deviations (single-
qualifier-flip fixture → PR 4; Declare name-level validation → PR
4). R7 name_pattern totality/multi-body-master hardening now
(latent). R8 Merged-collision code comment (PR 5 discriminator).
R9 doc fix: "kept-key identity wins" (not A-side; zip_seam keeps
outer vertex) — Vanished-diagnosis check banked for PR 4. R10
order_along over-tie accepted (widens Ambiguous, never mis-binds).
R11 companion names-only golden digest. R12 displaced doc comment.
R13 pre-existing kernel PANIC on legal double-subtract
(attach.rs:173 assert_eq, reproduced on origin/main) — filed as
**issue #86**, not PR 3's; blocks boolean-of-boolean fixtures
corpus-wide. EXTRA: the banked #83 sliver-side ε-boundary
typed-refusal fixture rides in this fix pass (test-only, beside
m4_remint_headroom.rs). Verified CLEAN under attack: transform
pass-through vs #84 re-mint (no keys minted/killed), Mixed-verdict
covariance, section completion-order covariance, reduction_contacts
key spaces (clone/graft/revert key-stable), order_along boundaries,
refusal doors all typed + no hard-coded ε (grep-verified), identity
boundary on the rich doc. Fix pass dispatched to the resumed
implementer in its surviving worktree.

**STEP export implementation COMPLETE (2026-07-24, `ev/m4-7-step`
@ `b7b382a`, pushed; review launched)**: `crates/step-export` —
runtime deps kernel-only (F6 "adopt nothing" honored; ruststep +
truck-stepio dev-only oracles). AP214 ed.2 (schema names the AP the
data uses), unprefixed SI_UNIT METRE (kernel is metres, no .MILLI.
lie), LENGTH_MEASURE-wrapped uncertainty = ambient ε at write time,
ADVANCED_FACE under ABSR, orientation adds no new conventions
(same_sense=.T. both levels per M1 outward-normal + he_plus
contracts). Part 21 real printer: shortest-round-trip digits
normalized to Part 21 grammar, exact-bits parse-back proptested;
NaN/∞ typed refusals. Multi-shell solids classified by per-shell
exact divergence-theorem volume (positive → independent MSBs; void
shells → typed VoidShellUnsupported — BREP_WITH_VOIDS needs a
designation the kernel doesn't record). 12-variant typed error
enum; byte-identical determinism (fixed default timestamp).
18 oracle tests green (cube 6/12/8 bit-exact corners, die 11/24/16,
kiss assembly 2×MSB; refusal doors end-to-end); workspace 122/122.
**FreeCAD external-import acceptance DISCHARGED**: all three
committed fixtures import VALID with exact counts + volumes
(admesh-pattern `scripts/check_step.sh` + step_import_check.py +
.expect sidecars; FREECADCMD env discovery, loud hermetic skip,
REQUIRE_FREECAD=1 hardening; new ci-local.sh row + ci.yml job).
Adversarial review launched (clone `~/.local/share/cad-work/
step-review`) — attack surface includes the ambient-ε-in-golden-
bytes vs 11-row-matrix interaction, HashMap-order determinism,
orientation proof beyond OCC healing, near-zero shell-volume
classification.

**PR 3 fix pass COMPLETE (2026-07-24, `ev/m4-3-names` @ `136d3db`,
pushed; gate running)**: all rulings landed. R1 chase_b made
infallible — broken B chains return the non-resolving key and route
to the chord_kind rescue (lane parity); reviewer reproductions
committed both orientations + subtract variants
(m4_pr3_names_rework.rs). R2 derivability CONFIRMED — new
`RoleSeg::OnToolVertex{side, of}` (identity via null-pair copy row,
side via recorded side assignment; both halves side-tagged so the
mirror-lane original/copy swap can't silently flip a name). R4
kernel-level Merged-lane unit test (synthetic merge_groups; sorted
+deduped constituents). R7 name_pattern: typed multi-body-master
refusal + per-instance totality. R8/R9/R11/R12 done (names-only
companion digest 0x015e_a22f_fd6d_b11d; full digest unchanged by
the rework — no die names moved). EXTRA: m4_remint_sliver.rs pins
the refuse side of the transform two-class contract (past-ε typed
ResidualExceeded, mid-band never-accepts; ambient-derived
magnitudes) — both sides of the #83 contract now pinned. Meta-
review spot-check: chase_b Result→infallible conversion verified at
both call sites. fmt/clippy/workspace/interval all green. Branch
still ff from main (098c4c1) ⇒ gated sha ≡ merge content. Gate
launched on 136d3db; PR body drafted with the three REPORT lines
(Merged eval-fixture → PR 5; single-qualifier-flip → PR 4; Declare
name-level validation → PR 4). PR + merge on gate green.

**STEP review returned (2026-07-24): mergeable-after-fixes; fix
pass dispatched.** No writer correctness defect — reviewer
independently confirmed orientation via exact rational signed
volume computed from emitted file text (cube +1.0, die +0.875,
kiss +0.875×2) and verified the mapping against the M1 interior-
left/he_plus contracts. S1 MAJOR (evidence gap, not a bug): the
orientation axis is invisible to all committed oracles — OCC
ShapeHealing silently rectifies a fully inverted shell (flag-flip
experiment: valid=True, POSITIVE volume), truck compares counts
only ⇒ ruled a committed parse-based signed-volume oracle + flag-
flip negative control (dyadic fixtures ⇒ f64 exact). S2 "exact
volume" doc softening + band-headroom argument. S3 check_step.sh
double-import (positional arg → env var). S4 stale "11-row" text
(matrix is 13 rows now). S5/S6 latent-door one-liners. Verified
CLEAN: real printer exact-bits over 40M random patterns + power-
of-two ulp neighbors; ε-golden hazard does NOT fire (explicit
uncertainty pin; green at 1e-6/1e-9/1e-12); D9 no-hash-iteration
grep + non-dyadic byte-identity; all F6 conformance grounds; all
refusal doors; hermetic skip matrix; +83 dev-only deps, licenses
clean. **Security-flag triage: FALSE ALARM** — the "deleted test
file" was the reviewer's OWN uncommitted probe (zz_ convention),
never in git history (verified --all/main/head); tree pristine at
b7b382a. **Ops note (reviewer finding 7)**: cross-agent shell
cross-talk observed — another agent's cargo/git commands briefly
executed in the review clone's cwd via the shared shell snapshot;
no tampering (HEAD pinned, status clean, goldens green), but
reviews should pin HEAD + re-verify goldens at report time, as
this one did. Meanwhile PR 3 gate still running on 136d3db.

**STEP fix pass COMPLETE (2026-07-24, `ev/m4-7-step` @ `6ef1b8d`,
pushed)**: S1 orientation oracle committed — minimal Part 21 parser
over emitted text (all three orientation data honored) computing
per-shell signed volume with exact `==` pins (cube +1.0, die
+0.875, kiss +0.875×2; dyadic-fixture exactness documented) +
flag-flip negative control reading exactly −1.0 (catches what OCC
heals and truck's counts miss). S2 "exact"→headroom-not-exactness
honesty rewrite. S3 STEP_FILE env (double-import fixed, re-verified
live). S4 gate.sh stale-proof wording. S5/S6 latent-door one-
liners. 123 workspace rows green; step-export green at ε=1e-6/
1e-9/1e-12; FreeCAD acceptance re-run green. Meta-review: diff
tightly scoped (7 files, oracle + docs/scripts only). **Merge
sequencing ruled**: PR 3 merges first; THEN ev/m4-7-step merges
current main and gates the actual post-PR 3 merge state (STEP
depends on topo, which PR 3 touches — gate the real candidate, not
the stale base). PR 3 gate mid-matrix on 136d3db.

**PR 3 MERGED (2026-07-24, PR #87, main `90aae39`)**: gate 11/11
uninterrupted on `136d3db` (352s wall). First gate attempt was
aborted by a WSL crash that tore four cached test binaries in the
gate runner's warm target ("Exec format error" — torn writes, NOT
test failures); purged by ELF-magic scan and re-run clean. Crash
also took the away-channel monitor (re-armed) and pushed disk to
5G free — reclaimed ~24G by deleting finished agents' build caches
and completed review clones (probe files preserved at
`~/.local/share/cad-work/review-probes/`); gate warm cache and
Evan's main-checkout target left intact. Naming part 1 is on main:
StableName/RolePath, eager bidirectional name tables, N2
discriminators, the D5 CI invariant, kernel emission, OnToolVertex,
chase_b lane parity, both sides of the #83 transform contract.
**STEP sequencing executed**: origin/main (90aae39) merged clean
into `ev/m4-7-step` as `0768f78`, pushed; gate launched on the
actual post-PR 3 candidate. STEP PR + merge on green. Next after
STEP: PR 4 spec (resolution + diff engine) — carries banked items:
single-qualifier-flip fixture, Declare name-level validation,
Vanished-diagnosis vs dropped fused-vertex identity.

**STEP export MERGED (2026-07-24, PR #88, main `0063f00`)**: gate
all-rows PASS on `0768f78` (the actual post-#87 candidate), 1016s
wall, including the new `step import (freecad)` row live in the
gate. PR 7's STEP half is done — F6 discharged end-to-end
(in-house AP214 writer, oracles dev-only, FreeCAD acceptance
green, orientation-sensitive text oracle closing the OCC-healing
blind spot). Merged-branch hygiene: step-export clone +
PR 3 implementer worktree removed. Appearance half (PR 7's
remainder) unblocked by #87's StableName — implementer launched.
Next orchestrator task: PR 4 binding spec (resolution + diff
engine; banked: single-qualifier-flip fixture, Declare name-level
validation, Vanished-diagnosis vs dropped fused-vertex identity).

**Appearance half COMPLETE (2026-07-24, `ev/m4-7-appearance` @
`7d7861d`, pushed; adversarial review launched)**: appearance.rs —
Attr/AttrKind (Rgba8/Label/Visibility, float-free, additive),
AppearanceMap in Doc (bit_eq covered), SetAppearance/ClearAppearance
DocEdits (Declare carve-out validation; Clear works on stranded
names as the repair path), Evaluation.appearance total post-pass
(works on canceled prefixes), typed AppearanceLoss with six causes
(the N3/N5 hook PR 4's Diagnosis attaches to; Vanished carries
structural merged/unmerged candidate offers). Appearance excluded
from content keys — appearance-only edit recomputes ZERO nodes
(pinned). Flagged conservative choices: tied names accepted at
edit, refuse typed Ambiguous at resolution (no auto-pick); NO
inheritance (patterns don't inherit master; transforms pass through
by N1 identity, not policy; boolean FromA/FromB wrapping means
pre-boolean attributes don't paint post-boolean faces); edge/vertex
= typed AppearanceWrongKind; fail-loud clears. 13 integration + 3
unit tests; fmt/clippy/workspace/interval green (131 rows × 2
lanes). Reviewer attack list includes: tie-across-ε honesty,
whether FromA/FromB non-inheritance is the right "survives
recompute" reading (flag-don't-assume), Clear-replay determinism
under upstream edits, PR 4 hook-shape compatibility (Ambiguous
{at,width} vs N5 TieWitness; Vanished vs N5 payload).

**Evan engaged CURVED-DESIGN #85 (2026-07-24)**: inline comments on
most OQs + two top-level questions (march-then-certify failure
path; whether C6 cache-knots needs explaining). DECIDED by Evan in
review: OQ1 (b)-staged-via-(a); OQ2 hull bounds day-one AND tube
always; inari temporarily OK on default path as in-house-ring seed;
D7 waits; OQ6 WIDENED to three-convex-edge corners (he wants the
die-with-pips demo upgrade as the acceptance target); OQ8 in-house
ring approved; OQ9 lean DESIGN.md Q5 revision. Needing elaboration:
OQ3 gate placement, OQ4 pcurve-vs-carrier, C8 refusal-name
vocabulary, OQ7 (his counter-proposal: keep TangencyLocus separate,
maybe renamed narrower), T1 clarification (two-strength residual —
resolved-tension not feature), T3 innocent-operand example, and his
K tangent ("much larger K? GUI-indistinguishable slivers are
probably mistakes") — likely its own issue. Original design agent's
transcript lost to the WSL crash; fresh responder launched on the
surviving clone (doc is self-contained) to reply inline + top-level,
revise the doc with DECIDED markers, and report.

**#85 conversation serviced (2026-07-24)**: all 15 inline threads
replied + top-level answer posted; doc revision `c0c74ea` pushed
(+114 lines, DECIDED markers for OQ1/OQ2/OQ5/OQ6-scope/OQ8/OQ9 +
the inari transition allowance; T1 discharged). Notable design
movement: OQ7 adopts Evan's two-level counter-shape — verdict-mark
on every definitely-tangent edge + must-carry enforcement keyed on
jet-determinacy (exempts G2 joins by the predicate itself) —
proposed rename `TangentIntersection{s1,s2,witness}`, to land as a
D2 sharpening. OQ4 gained a new invariant-shaped argument
(carrier-primary keeps the edge parameter chart-neutral). K
question spun out as **issue #89** (kernel-K revisit gated on the
M5 exit K-snapshot per T5; scale-relative sliver lint at the
document layer). Remaining for Evan, all 👍-sufficient and
watchlisted: OQ3 (gate placement rec (a)), OQ4 (carrier-primary),
OQ7 (reshape + rename), OQ6 run-out vocabulary, C6 (top-level).

**Appearance review returned (2026-07-24): mergeable-after-fixes;
ruled and dispatched.** No blockers; all lanes green incl. both ε
rows. A1 MAJOR was a design ruling, RULED: the paint-operand-then-
extend-recipe case (attribute resolves on the intermediate node
only; final node shows neither paint nor loss) is UPHELD as
correct v1 semantics — N1 identity + ratified N5's EMPTY auto-menu
(appearance auto-following a face through a boolean would be the
banned silent rebind). Required: document "resolves-anywhere"
success criterion + commit the reviewer's probe as a pinning
fixture; the operand→final gap banked in M4-PR4-SPEC D9 (Rebind
suggestion ladder offers wrapping derivations; GUI attributes the
displayed node's names). A2 dedupe Ambiguous losses per name. A3
drop the u32 saturation. A4 hook-mapping recorded in PR4-SPEC D9
(incl. the N3-offers-vs-N5-shape ratified-doc tension, PR 4
resolves). A5 nested-Merged empty-offers doc line. Verified CLEAN
under attack: cross-ε tie honesty (monotone verdict refinement ⇒
silent owner-swap impossible — for a name to stay Unique while
changing owners a definite verdict would have to flip sign),
transform identity is the same Arc (bit-identical), content-key
exclusion (zero recompute pinned; DocDiff has no eval consumers),
Clear-replay determinism (no poison path; ids never reused), all
six loss causes reachable, totality on canceled prefixes,
entity-injectivity prevents attr merging, B11 no-global-uniqueness.

**CURVED-DESIGN fully decided (2026-07-24)**: Evan 👍'd all four
inline replies + the top-level answer (verified per-comment via the
pulls reactions API — which exposed and fixed a monitor gotcha:
inline-review-comment reactions live under pulls/, not issues/).
Every fork resolved: OQ3 in-op gate, OQ4 carrier-primary, OQ6
run-out refusal vocabulary (zero constructor surface), OQ7 Evan's
two-level shape + TangencyLocus→TangentIntersection rename (D2
sharpening at ratification), C6 + C3 failure path. Doc folded at
`9860b87` (on top of my `d05579b` sign-off note); review-status now
reads "merging #85 = whole-doc ratification"; closing comment
posted and watchlisted (👍 = ratify, orchestrator merges).
Post-merge obligations flagged by the responder: flip DRAFT header
to ratified-record; DESIGN.md ratification pass (D2 sharpening +
Q5 lean revision + TangencyLocus spelling sweep); inari quarantine
boundary-text update; LGPL-before-publish exit condition.

**CURVED-DESIGN RATIFIED and MERGED (2026-07-24, PR #85, main
`b7e3962`)**: Evan "lgtm!" + 👍 on the closing comment. Header
flipped DRAFT→ratified-record before merge. M5's design record is
on main: C1-C12 ratified with grounds, T1-T6 flags,
post-ratification obligations listed in the header (DESIGN.md
D2-sharpening pass incl. TangencyLocus→TangentIntersection sweep +
Q5 lean revision; inari quarantine boundary text; LGPL-before-
publish). The pre-M5 design-doc program is COMPLETE (NAMING,
SOLVER, CURVED all ratified). m5-design clone retained for the
DESIGN.md pass; watchlist clear.

**#86 fix kicked off (2026-07-24, Evan's request on #88)**: no
overlap with live work (topo vs editor-core). Investigator briefed
diagnose-before-fix: (a) upstream boolean-of-boolean bookkeeping
bug — make the double-subtract succeed; (b) honest-envelope typed
refusal; (c) wrong assert. Prefers (a); reviewer's preserved probe
as seed; branch ev/issue86-attach-panic. If (a), the R13
boolean-of-boolean fixture ban lifts.

**PR 4 launched EARLY (2026-07-24, Evan's push on #88: "does PR 4
need to wait on 7?")**: re-examined the contention — only spec D9
(appearance-hook enrichment) truly depends on PR 7's types; the
rest is independent, and the DocEdit-enum overlap is a trivially
additive conflict. Implementer launched on ev/m4-4-resolution off
current main with D9 sequenced LAST (merges main when it gets
there; stops-and-reports if PR 7 somehow hasn't landed). Lesson:
"same crate" is not by itself merge contention — check the actual
dependency and conflict surface before serializing.

**Appearance fix pass VERIFIED + #86 report in (2026-07-24)**.
Appearance: all rulings done at `32d5645` (resolves-anywhere
documented twice + reviewer's probe committed as the ruled-gap pin;
per-name Ambiguous dedupe with deterministic defining-`at`; width
kept as usize — candidate set derivable via table.lookup, noted per
A4 mapping; A5 scope paragraph). Four lanes green with captured
exit codes (default/1e-6/1e-12/interval, 131 rows each). Main
(docs-only CURVED-DESIGN delta) merged in as `9f2cf6f`; GATE
launched on the true candidate. **#86: diagnosis (a) CONFIRMED
with causal trace** — merge_coplanar_faces absorbs a coplanar slot
floor whose surface stays alive via 4 edge-description refs;
re-description drains 4→0; the last remove_curve_if_orphaned
stranded the surface (the assert is a genuine invariant and
stays). Fix: curve removal cascades description_surfaces through
remove_surface_if_orphaned — the exact dual of the
descriptions-keep-surfaces-alive rule; closes the same latent gap
in kemr/kev/kef/split_edge; kef's killed_surface reports through
either door. Committed fixture: crossing-slots double-subtract
SUCCEEDS + tier1/2/pseudomanifold + transform shake-out, both
lanes; reproduced the exact panic pre-fix. Implementer opened PR
#90 itself (minor process deviation, harmless). All lanes green.
Adversarial review launched (attack: cascade removing a
transiently-orphaned surface mid-Euler-op would be strictly worse
than the leak). R13 boolean-of-boolean exclusion lifts when #90
merges — seed the naming corpus with the fixture's document shape
(bank for PR 5/8).

**Ops (2026-07-24, Evan's asks on #88)**: (1) hourly agent check-in
heartbeat armed + saved as standing memory — sweep every agent lane
per tick, nudge silent ones (two lost wake-on-completion events
today motivated it). (2) Hosted Actions diagnosed: runs trigger but
every job dies in 2s with "recent account payments have failed or
your spending limit needs to be increased" — still true after Evan
added budget; told him: retry/fix the payment method and check the
Actions spending limit is > $0 (budget and spending limit are
separate knobs). Also flagged: the ci.yml step-import job has never
executed hosted — expect a shake-out iteration when Actions wakes.
gate.sh remains the merge gate until a full green hosted run.

**Demo-montage refresh kicked off (2026-07-24, Evan on #90)**:
crossing slots endorsed as the first boolean-of-boolean demo;
ideation agent launched — inventory the montage, flag weak entries
(donut named), propose 8-12 candidates with capability-showcase
grounds and envelope-honest feasibility tiers (NOW / post-#90 /
post-PR-5 / M5-tripwired), file as a pick-list issue for Evan.
No implementation until his picks.

**Demo refresh proposal filed (issue #91, 2026-07-24)**: 12
candidates, top-5: cross-lap joint (Evan's crossing-slots, post-
#90), shadow-silhouette solid (first-ever `intersect` demo, 3-way
post-#90), project-box enclosure (longest boolean-of-boolean
chain), parametric heat-sink strip (first M4-layer demo: one
recipe, three fin counts, counted downstream-only recompute —
feasible NOW), cutaway section (first `split` demo, re-minted
transform separation). Donut → half-torus grab handle (keeps the
montage's only torus). Inventory surprises, fix-regardless: (a)
the tour still validates boolean stops via the pre-M3-PR-6a
upgrade_edges_to_intersections clone hack instead of
validate_pseudomanifold + contacts; (b) NOTHING in the demo suite
exercises STEP export; (c) topo::intersect and topo::split have
never appeared in any demo. Envelope note candidates respect:
gate_planar refuses booleans if ANY operand face is curved (arcs
and booleans stay separated until M5); flush-plane contact refuses
until PR 5. Awaiting Evan's picks on #91.

**Second WSL crash — disk full (2026-07-24)**: disk hit 100%
(169M free), crashing WSL again; killed the appearance gate
mid-interval-row, both monitors, the PR 90 reviewer, and the demo
agent mid-task. Recovery: freed to 18G (deleted appearance-review
clone, issue86/appearance/demo-ideas targets; gate cache at 30G
kept — in use); ONE torn binary purged from the gate cache
(m4_pr2_wire, ELF-magic scan); appearance gate relaunched; all
monitors re-armed + NEW disk watchdog per Evan's request (warn
<15G, critical <8G, 5-min poll) — saved as standing memory; PR 90
reviewer, demo agent, and PR 4 implementer all resumed from
transcripts with warnings that crash-window test results are
ENOSPC-suspect and must re-run. Root cause: five parallel lanes ×
5-8G targets + 30G gate cache on a 251G disk. Standing mitigation
now in the disk-watchdog memory (delete finished lanes' caches at
seams — this session had been doing it only at merge seams, not
review-completion seams).

**#91 revisions posted (2026-07-24)**: Evan's three pushbacks
answered with evidence. (1) C2 shadow-silhouette: his curvature
concern diagnosed as actually a COPLANARITY constraint — with
decoupled dimensions the 2-way H×T prism intersect passes tiers
1/2/3′ with exact dyadic volume, and the 3-WAY
intersect-of-intersect SUCCEEDS ON MAIN PRE-#90 (chamfer crossing
is coplanarity-free); naive coincident-plane variant refuses typed
— kept as a PR-5 Declare before/after narrative. (2) C9 reshaped:
donut's unique content is the torus SURFACE KIND (vase/pulley
cover full-revolution); new C9 = rope-groove sheave (ring-torus
groove + bore, genus 1) — enabled by finding the vase's "off-axis
arcs refuse as toroids" comment is STALE (revolve mints ring-torus
walls today; only horn/spindle refuse). (3) FreeCAD headless
render WORKS: freecadcmd + QT_QPA_PLATFORM=offscreen +
Gui.updateGui() renders our die.step to clean 1600×1200 shaded
isometric, no display — proposed as montage render path (F6
dogfooding; matplotlib kept as the tessellation-proving fallback;
~1 min startup ⇒ batch stops in one session). Issue #91 body
updated + evidence comment posted. Picks still open.

**Gate FAIL false alarm (2026-07-24): OOM, not code.** Appearance
gate run 2 failed ONLY the f64 ε=1e-6 row, in 4s, with a bare
"Terminated" mid-suite (5/7 m4_pr2_eval tests already ok; same
suites pass at 1e-9/1e-12/interval-1e-6). Cause: ~5G-RAM WSL
instance running the gate + two agent batteries concurrently —
OOM kill. Machine now quiet (batteries done); gate re-running.
RAM-contention corollary added to the disk-watchdog memory.
**PR 90 review returned: mergeable-after-fixes** — core cascade
fix verified (panic reproduced on revert; trace corroborated via
kef's debug postconditions; structural proof: the cascade fires
only in states the pre-fix kernel already flagged as bugs — no
wrong-removal window constructible; D9 clean). Required: F1 kef
cascade-door test (reviewer's ready-made probe — the door hunk
currently has ZERO coverage and a demonstrable pre-hunk misreport)
+ F2 interval-lane parity in the fixture (tier 3′ + transform);
doc-drift fix rides along. Follow-up issues to file: unreported-
kill channel on kev/kemr/split_edge result structs; corrupt-input
misattribution note. Fix pass dispatched.

**PR 7 COMPLETE — appearance MERGED (2026-07-24, PR #92, main
`9764dd4`)**: gate all-rows PASS on `9f2cf6f` (443s, quiet
machine after the OOM false alarm). Both PR 7 halves are on main
(#88 STEP + #92 appearance). The A1 ruling (paint does not follow
a face through a boolean; resolves-anywhere; explicit Rebind
repair path) is flagged in the PR body for Evan's awareness.
PR 4's D9 unblocked — implementer notified to merge main and
finish; #86 agent cleared to run its F1/F2 battery. Appearance
clone removed (hygiene). M4 status: PRs 1, 2, 3, 7 merged +
re-mint (#84) + CURVED-DESIGN ratified (#85); PR 4 nearing
completion; PR 90 (#86 fix) in fix pass; remaining: 5, 6, 8.

**A×Z verdict + new kernel gap filed (2026-07-24)**: Evan's A×Z
suggestion refuses typed today — NOT the coplanarity gap, a new
one: resolve_roles_geometric's vertex-only anchor probing fails
when an isolated seam hexagon (cookie-cutter null-face pair, no
struts) leaves every flanking fragment bounded entirely by seam
vertices (all OnBoundary ⇒ no anchor ⇒ JoinDesync). Structural for
these letterforms; H×T dodges via surviving original corners. Fix
sketch: probe region-interior points (also covers nested islands
= A's true counter). Filed as an issue with the agent's exact-
fraction oracles as the ready acceptance fixture; C2 ships H×T;
A×Z upgrades when it closes. Demo agent's kernel instrumentation
verified reverted, clone clean, disk 21G.

**Hosted CI fully GREEN (2026-07-25, PR #94 merged)**: Actions is
back (Evan fixed billing) and PR 94's run passed ALL 10 jobs
including the first-ever hosted `step import (freecad)` execution
— the apt-has-no-freecad shake-out fixed by installing the
checksum-verified 1.1.2 AppImage (version-matched to the local
oracle, cached via actions/cache). First fully-green hosted run
since the billing outage began 2026-07-22. Open question for Evan
(flagged on #90): whether gate.sh retires to a local convenience
now that hosted CI is authoritative again — recommend keeping
gate.sh as the pre-merge gate for wall-clock (hosted run ≈ 7 min
rows in parallel — actually comparable; the real difference is
gate.sh needs no billing and runs pre-push). Decision deferred to
Evan; both stay for now.

**#92 follow-ups from Evan (2026-07-25)**: (1) black-box appearance
metadata — agreed additive; BANKED AS PR 6 SPEC ITEM (schema v1
freeze is the decision point; opaque Custom{key, bytes} arm or
metadata map; F3/bit_eq constraint = bit-exact serializable,
trivially satisfied by bytes). (2) joining painted objects: no
errors by construction (resolves-anywhere; worst case = paint not
propagating until Rebind); industry standard is silent topological-
naming color-following — the N5-banned shape; our flow = displayed-
node rendering + PR 4 one-click Rebind offers + open N5 door for a
ratified carry policy. GUI-DESIGN note added on main (55a31d8);
answered on #92.

**PR 4 implementation COMPLETE (2026-07-25, `ev/m4-4-resolution` @
`260e620`, pushed; adversarial review launched)**: D1-D9 all done.
N5-verbatim ResolveError/Diagnosis/TieWitness/Tombstone; one
cause-agnostic diff engine (vdiff.rs, per-predicate sign
populations; attribution prefers name_frag_* discriminators, else
deterministic-first — flagged for review); Rebind with the Declare
carve-out rewriting Declare pairs AND appearance-store keys;
merge-time call RULED UPHELD: RebindAppearanceCollision typed
refusal (repair = ClearAppearance first — no silent survivor-
picking); hit-testing total + typed Unnamed; solver contracts as
data only; all three PR 3 banked obligations; diagnosis goldens +
f64/Interval agreement; D9 done post-#92-merge — Vanished offers
ride NEXT TO the byte-verbatim N5 error (wrapping choice), and the
A1 ladder end-to-end test pins paint→union→suggest→rebind→resolves
on the FINAL body. Batteries green (139/139 both lanes + ε rows;
pre-crash battery matched). Honesty note: agent used broad pkill
once during crash cleanup (~19:28), possibly racing a gate cargo
spawn — that gate completed normally, no impact; switched to
PID-scoped kills. Reviewer launched (attack list: over-tie
candidates fidelity, attribution determinism/honesty, ForeignNode
totality, collision-refusal bypasses, offers golden coverage,
ladder completeness/non-invention, tombstone last-good pinning).

**Merge-gate policy CHANGE (2026-07-25, prompted by Evan: "how is
gate.sh free?")**: honest accounting — gate.sh costs 30G cache,
serialized wall-clock, RAM/CPU contention on a 5G box, and
contributed to both disk crashes; hosted Actions (green since #94)
runs the same matrix parallel on GitHub hardware ~7 min on the
PR's merge ref. NEW POLICY: hosted Actions PR checks are the merge
gate; gate.sh demotes to documented billing-outage fallback. The
running PR 90 gate is the last old-regime run (no double-verify);
PR 4 onward merges on green checks. Gate runner's 30G target gets
deleted after PR 90 merges (cold rebuild accepted for rare
fallback use). Demo implementation launches post-PR-90-merge:
Evan confirmed everything viable-now is picked (C2 H×T, C4, C5,
C9, C10 + fix-regardless trio; C1/C3 unlock at the merge).

**SESSION HANDOFF SNAPSHOT (2026-07-25, orchestrator seam at
Evan's suggestion)**. Merged this session: #84 re-mint, #87 naming
part 1, #88 STEP export, #92 appearance (PR 7 complete), #94 ci
freecad fix, #90 issue-86 cascade fix (main `604e5dc`); #85
CURVED-DESIGN RATIFIED. Policy changes, all logged above and in
memories: hosted Actions = merge gate (gate.sh fallback, header
updated 12e256b, cache deleted — disk 57G); monitor suite scripted
in scripts/monitors/ (install to ~/.local/share/cad-work/monitors/,
arm 3 persistent Monitors at session start); hourly agent sweeps +
disk watchdog standing; clone placement never-/tmp; push-per-unit.
IN FLIGHT FOR SUCCESSOR: (1) PR 4 review — reviewer (static review
complete, verdict pending its held-then-released test lanes) will
deliver a self-contained report; process rulings → fix pass by the
PR 4 implementer if needed → merge on green Actions checks. Branch
ev/m4-4-resolution @ 260e620, PR not yet opened — open with a
writeup covering the D1-D9 report in the log above (incl. the
UPHELD RebindAppearanceCollision ruling + the wrapping choice for
Vanished offers). (2) Demo implementation NOT yet launched —
Evan's picks: everything viable now (C2 H×T, C4, C5, C9, C10,
fix-regardless trio) + C1/C3 (unlocked by #90's merge). Launch one
implementer per the #91 issue body; FreeCAD render path per the
#91 evidence comments; normal review pipeline, Actions gate. (3)
PR 5 spec next on the critical path after PR 4 merges (GeomSource
+ Declare; corner-table tripwires + the #91 flush-plane demos are
its acceptance showcases; R13 boolean-of-boolean fixtures now
unblocked for its corpus). (4) Parked: #93 resolver gap (ready
fixture; schedulable between seams), #89 K-revisit (M5 exit), PR 6
banked items (doc-ε re-audit; black-box appearance metadata —
Evan's #92 ask), Q9 name shortlist (Evan's call). Away-channel:
reply on the PR/issue threads; sign-off watchlist at
~/.local/share/cad-work/signoff-watchlist.txt (empty).

**PR 4 review returned (2026-07-25): mergeable-after-fixes; ruled,
dispatched.** Report persisted at ~/.local/share/cad-work/
pr4-review-report.md (self-contained + probe appendix). P1 Finding
2 (fix): rebind suggestions counted SideOf partner MENTIONS as
wrapping (6/12 phantoms in the band-cut probe) + no kind filter
(offered names Rebind itself refuses) ⇒ separate suggestions
walker + kind filter + probe inverted to pin. P2 Finding 1
(RULED): cross-group population-cancel can empty the FlipSet and
reach the RecipeEdit-lie fallback — ruling: widen the vdiff
blind-spot docs AND add a qualifier-delta rung to the diagnose
ladder (N2 verdicts are embedded in names; compare the vanished
name's own qualifier vector against same-node siblings in the new
table ⇒ honest PredicateFlip from recorded data); true-no-evidence
keeps RecipeEdit re-documented honestly. P3 positive tests
(RecipeEdit arm, no-prior Vanished, P2 pin). P4 hardening rides
(tombstone debug_assert, exhaustive for_each_inner, parallel
verdict-log probe adopted). Recorded no-action: candidates
degeneracy verified compliant; two-ε diff waits for PR 6's
recorded ε; content-key tag bump BANKED for PR 6 persistence;
Merged offers pin waits for PR 5 corpus. Verified clean: verdict-
log substrate (memo-transfer log identity, decision outcomes
bit-identical), Ambiguous fidelity, tombstone last-good, W-datum
opacity, D6.1 single-qualifier counted, goldens cross-lane. Fix
pass dispatched (report will persist to pr4-fixpass-report.md);
merge gates on ACTIONS per the new policy. pr4-review clone
removed (report + probes preserved).

**Successor orchestrator picked up (2026-07-25, post-/clear
continuation — monitors and subagents inherited live).** Fix pass
RECEIVED: `e6f78b7` pushed; P1 (walk_names Partners::Include|Skip
+ kind filter, probe inverted to pin), P2 (blind-spot docs widened;
qualifier_delta rung fires on clean single-SideOf-entry deltas with
unanimous signs, serves no-prior path too; REPORTED boundary
ACCEPTED: Mixed/On have no honest single-Sign reading in N5's
payload — those deltas fall through to the honestly-re-documented
fallback, negatively pinned), P3 (three positive tests incl. the
EMPTY-FlipSet population-cancel shape), P4 (tombstone debug_assert,
exhaustive walk_names, parallel verdict-log pin). Local battery
running as confirmation only. **PR 4 OPENED: #96** with the full
D1-D9 + review-outcome writeup; merge gates on green Actions
(checks running; watcher armed). **NEW ISSUE #95** (fix-pass
REPORT): memo transfer can reuse stale name tables — names embed
minting node ids (N1) but content keys exclude them (D8), so the
names half of a memoized value is not a pure function of its
content key; reachable by re-pointing an input to a bit-identical
twin. Ruled: "accept + document" rejected (silent recipe-lie, the
class R9/N5 forbid); recommended disposition 1 = re-derive the
naming half on mismatched transfer (store minting context beside
the memo; geometry reuse unchanged); sign-off comment posted and
on the watchlist (comment 5077325409); lands in PR 5 spec or a
small dedicated PR after Evan's 👍. **DEMO IMPLEMENTER LAUNCHED**
(clone ~/.local/share/cad-work/demos, branch ev/m4-demo-refresh):
scope = Evan's picks C1, C2 H×T (+3-way), C3, C4, C5, C9, C10 +
tier-3′ modernization; retirements donut/openbox/voidbox-panel;
wedge STAYS (C6 not picked); A×Z explicitly out of scope (banked
acceptance fixture); no-coincident-planes design rule and FreeCAD
render path per #91 evidence; report to demo-refresh-report.md;
review pipeline after. Next: PR 4 merge on green → PR 5 spec
(GeomSource + Declare; #95 disposition folds in on sign-off;
corner-table + lap-joint tripwires and #91 flush-plane demos are
its acceptance showcases; R13 boolean-of-boolean fixtures
unblocked).

**PR 4 MERGED (2026-07-25): #96 → main `4ad1dca`** on a 10/10 green
Actions matrix (incl. step import freecad); implementer's local
confirmation battery came back ALL GREEN post-merge (139/139 both
lanes + PR-4 ε rows; archived as a #96 comment). Reviewer
TaskStopped; pr4 clone deleted (disk 127G free). **PR 5 SPEC
COMMITTED (`docs/M4-PR5-SPEC.md`, b7aec91)**: D1 GeomSource
N6-verbatim (with the PR-1 ExprPath re-point caveat made binding),
D2 declared-rung migration (merge_faces/plane_eq to source
comparison, rung (b) untouched), D3 bit_identity to debug-only +
empty production allowlist (tripwires stay armed; interval.rs entry
renamed), D4 Declare threading via the PR 3/4 resolution machinery
(N5-typed refusals on unresolvable intents), D5 first real Merged
rows (R4 fixture, R8 discriminator, PR-4 F10 offers pin), D6
acceptance showcases (closure corpus certify, corner-table primary
tripwire fires + secondary REPORTed, #91 flush-plane Declare
before/after, R13 corpus seeding), D7 #95 disposition-1 CONDITIONAL
on Evan's 👍 (else skip + REPORT), D8 #93 out of scope, D9 standing
process. **PR 5 IMPLEMENTER LAUNCHED** (clone
~/.local/share/cad-work/pr5, branch ev/m4-5-geomsource off main
incl. #96), with cross-lane RAM discipline (demo lane runs in
parallel; both agents pgrep-wait before every cargo invocation).
Two lanes in flight: demos (ev/m4-demo-refresh) + PR 5.

**#95 ruling REVISED (2026-07-25, Evan's pushback engaged)**: Evan
intuited disposition 2 (separate keying = pure function + state
lookup); stress-testing proved him right for a STRONGER reason —
disposition 1's one-level context check fails the grandparent
re-point case (X's input g→g' twins: X re-derives names, N's
input-id vector unchanged → N keeps a table embedding X's OLD
names; staleness is recursive). Verified at 4ad1dca: memo lookup is
per node id, so no intra-run twin sharing and no own-id needed in
the key. Revised ruling on the thread: disposition 2 with the
recursive key naming_key(N) = H(content_key(N), [(input_id_i,
naming_key(input_i))...]); naming-key mismatch reuses geometry +
re-derives names (or re-runs the op if emission isn't separable —
REPORT which). Spec D7 updated in place; new sign-off comment
5077393718 on the watchlist (old 5077325409 removed). Extra
regression pin: grandparent case.

**#95 RATIFIED (2026-07-25)**: Evan 👍'd the revised ruling
(disposition 2, recursive naming key). PR 5 D7 green-lit to the
implementer (sequenced last in its plan); watchlist entry
auto-cleared by the monitor.

**Usage-limit outage + recovery (2026-07-25, ~08:00–15:30Z)**: both
implementer lanes died at the Fable limit mid-morning; Evan
re-logged-in ~15:30Z; both resumed from transcript per the standing
ladder — NOTHING lost (both had pushed; demo lane's push came from
the hourly-sweep nudge minutes before the kill — the discipline
paid for itself). WSL RAM mystery solved by an investigator agent:
the ~5.7G ceiling is WSL2's DEFAULT 50%-of-physical rule (11.75GB
host), no explicit limit; `memory=10GB` added to
C:\Users\evgun\.wslconfig at Evan's ask, EFFECTIVE ONLY at his
next `wsl --shutdown` (he'll restart after current work concludes);
memory banked — keep 5G discipline until `free -h` shows ~10G.

**DEMO REFRESH IMPLEMENTATION COMPLETE (2026-07-25,
`ev/m4-demo-refresh` @ `b91f1de`, pushed)**: all #91 picks landed —
C1 crosslap (JoinDesync-class refusal narrated + THIRD tripwire
planted pinning that exact class), C2 H×T + 3-way with
naive-refusal/decoupled-pass pair + 3 shadow-proof renders, C3
15-op project box (exact dyadic V after EVERY op), C4
split-of-boolean cutaway (no fallback needed — works on main), C5
recipe heat sink (recomputed-1/reused-4 + 135/135 names ASSERTED),
C9 torus-groove sheave (Pappus closed-form rel 2e-16), C10 one-
session FreeCAD STEP renders (14 planar bodies; montage.png 17
panels regenerated + visually inspected), tier-3′ modernization
(clone hack deleted), retirements done (wedge kept). Kernel diff =
exactly one test file. REPORT items: STEP lane planar-only until M5
(curved stops refuse typed, narrated); Boolean-of-Pattern not
wireable in F4 (possible future F4 item); crosslap refuses
JoinDesync not NonMaximalFaces; FreeCAD offscreen pitfalls fixed
in-script (incl. discovering the OLD montage had mid-animation
captures); table narration drift (coplanar-touch + inset-overlap
now union exactly — kernel caught up post-PR 5.5). Adversarial
reviewer LAUNCHED (10 falsification claims, e2e execution
required; report to demo-review-report.md).

**Demo review returned (2026-07-25): MERGEABLE.** All 10
falsification claims executed clean — oracles independently
recomputed by Fraction integration (C2 = 1593/512, 3-way =
12321/4096), sheave Pappus re-derived symbolically, C5 asserts
proven real by tamper test, FreeCAD renders pixel-deterministic
across sessions, sentinel correctly absent on simulated mid-run
failure, kernel diff exactly one test file. Five MINOR + four NOTE
findings; fix pass dispatched with rulings: M1 doc overclaim
(projectbox 3′-once reality), M2 STEP arm must assert
Unsupported*-class + planar-must-export (the silent F6-hollowing
hole — the one finding with teeth), M3 README shadow-PNG caption,
M4 narrate_naive becomes a real 4×DescriptionNotAdjacent pin
(fires like a tripwire when Declare glues it), M5 pin the 135, N2
honest Empty label; N1 implementer's discretion (tighten gate vs
reword caption, REPORT which); N3/N4 accepted. Merge on green
Actions after the fix push. Review report:
~/.local/share/cad-work/demo-review-report.md.

**Evan's demo review notes (2026-07-25, in-chat, PR #98 held
open)**: six revisions dispatched to the demo implementer —
(1) pulley likely redundant vs sheave: surface-kind audit; prefer
folding a conical zone into the sheave and deleting pulley (cone
was pulley's unique content per #91); (2) wedge dropped or
replaced with a more interesting partial revolve, implementer's
pick; (3) T resized to H's bounding height in both silhouettes;
(4) silhouette3 diamond resized to genuinely shape the y-shadow
(current one only bevels the H — not acceptable); (5) montage
trims to silhouette3-only and full-heat-sink-only (other scenes
stay in the tour script, montage:false); (6) cutaway view/cut
plane rotated so the section actually reveals the interior.
MERGE OF #98 IS HELD until the revision pass lands, renders
regenerate, and I visually re-inspect the montage. Also: rustfmt
row on #98 was a formatting-only miss in demo_tripwires.rs;
orchestrator fixed directly (3ec8b18) — mechanical, no content.

**DEMO REFRESH MERGED (2026-07-25): #98 → main `493ce7b`** on a
10/10 green matrix at revision head `7cbd781`; closes #91. Final
shape after Evan's in-chat review round (montage: "looks great"):
13 panels; pulley DELETED (sheave gained conical rim shoulders —
one part now carries plane+cylinder+cone+torus, census asserted);
wedge → quarter-turn chute (C-channel × 270°, Pappus rel 0.0);
letterforms equal-height; third silhouette shape = letter C
(diamond dropped — only beveled the H); montage trimmed to
silhouette3 + heat-sink-9 only (other scenes stay in the script);
cutaway restaged to face the viewer. Orchestrator visually
inspected montage/shadow-y/cutaway; Evan approved the montage.
Main now carries THREE Declare-sensitive pins (crosslap tripwire,
corner-table tripwire, narrate_naive 4×DescriptionNotAdjacent);
PR 5 implementer briefed on which should fire (1-2) vs which
flipping would signal a regression (3). demo-review clone removed.
Away-channel note: the demo review + revision cycle ran while Evan
was in-chat; both reports live under ~/.local/share/cad-work/.

**PR 5 implementation COMPLETE (2026-07-25, `ev/m4-5-geomsource` @
`a68e8e5`, pushed, main/#98 merged in; adversarial review
launched)**: D1–D7 all done. GeomSource as topo side records with
lowered pure-data fields (R1 accepted: layering necessity;
Minted{index} = per-evaluation mint-order identity per the spec's
own caveat); plane_eq/merge migrated to (GeomSource, orient) with
the debug bits-agree assertion; tripwire allowlist EMPTY (memo.rs
retained on its bit-hashing non-consumer justification — R3
accepted); Declare threading LIVE end-to-end (die = 21 declared
pips through 77 nodes, exact oracle); first eval-level Merged rows
— PR 3 R4 + R8 and PR 4 Finding 10 all discharged (R8 collision =
loud typed refusal, per-group discriminator BANKED — R4); D7
landed as disposition 2 recursive naming key, naming miss = full
op re-run (R8 accepted — emission not separable; D9 makes re-run
bit-identical), both pins in. **R2 accepted and flagged for the PR
body: the retirement is a designed NARROWING** — undeclared
value-equal flush booleans now refuse typed at the coincidence
door (the M3 bit rung was doing real, now-forbidden work); whole
corpus + demos migrated to declared intent; goldens re-pinned with
row-shape verification. **Corner-table PRIMARY WIRE FIRED** —
four-leg corner-aligned table tier 3 GREEN, demo stop ships it;
SECONDARY GAP CLOSED (in-plane seam edges consumed by the declared
glue) — this supersedes the 2026-07-23 corner-table gap note
above. **Crosslap wire could NOT fire (R7)**: the mate is a pure
REST contact — M3 envelope (iii) join-stage gap, same frontier as
the declared corner-flush REST pin; wire re-armed honestly as
crosslap_rest.rs pinning both doors; join-stage REST lane BANKED
(with #93 as the other join-stage kernel item). R9 accepted
(ContactRecords carry same-operand vf rows; docs updated). R10 =
pre-existing tour panic at ε=1e-6, filed as #99. Battery:
1252/0 + 1252/0 + interval 1396/0, fmt/clippy clean, tour green
default ε. Reviewer attack list: false-declaration laundering
(both doors + orient/transform/carried paths), rung-(b) no-silent-
widening, GeomSource composition + Minted determinism under
parallel schedule, D7 tamper + re-run bit-identity + memoized
recursion, R5 emit_topo machinery (PRIME: junction naming,
skip-and-record, N4 totality), N5-verbatim Declare doors,
tripwire-catches-smuggled-consumer, golden re-pin shape honesty,
all showcases executed. First review with the fixed code-quality
rubric (PR 5 = pre-experiment Fable reference row for
docs/MODEL-AB-LOG.md).

**PLANNED WSL RESTART (2026-07-25, ~12:00 local)**: pausing for the
10GB RAM bump (memory=10GB already in .wslconfig; Evan runs `wsl
--shutdown`). Pre-restart state flush: PR 5 reviewer TaskStopped
CLEANLY mid-build (clone ~/.local/share/cad-work/pr5-review @
a68e8e5 checked out, build partial — safe to resume; NO findings
yet, report not started). RESUME CHECKLIST for this orchestrator
(or a successor reading cold): (1) verify `free -h` shows ~10G —
then the 5G sequential-battery discipline RELAXES to two parallel
lanes max (update prompts accordingly); (2) re-arm the three
monitors from ~/.local/share/cad-work/monitors/ (install step
already done); (3) resume the PR 5 reviewer by SendMessage — point
it at its clone, tell it the machine now has 10G, re-state: attack
list unchanged, battery rows may run with more parallelism but
still check pgrep first; (4) sign-off watchlist is EMPTY; (5) two
other open items: #99 (tour ε-panic, between-seams), demo renders
regeneration owed to the PR 5 fix pass (table + montage — PR 5
changed stop 8 but didn't re-render; fresh table.png verified
correct by orchestrator + Evan pre-restart). In flight NOTHING
else: PR 4 merged (#96), demos merged (#98), state-sync #97
merged, PR 5 implementation pushed @ a68e8e5 with report
delivered; only the review cycle remains.

**Post-restart parallel dispatch (2026-07-25)**: RAM verified 9.7G
total / 8.2G available (memory=10GB took); monitors re-armed ×3;
PR 5 reviewer resumed from transcript (told: possible torn target/
from the mid-build stop — rebuild fresh; two cargo lanes now
allowed). **#93 IMPLEMENTER LAUNCHED between seams** — first
MODEL-AB row: difficulty M logged pre-flip, draw 197 → FABLE arm
(row 1 in docs/MODEL-AB-LOG.md). Branch
ev/issue93-seam-region-anchors off main; binding constraints:
anchors through reified predicates only (k_stats funnel), no
envelope widening (flush-plane pins must stay refusing), A×Z ×3
variants become acceptance fixtures with independently-derived
exact volumes, name-table goldens must not move (STOP+REPORT if
they legitimately must). **PR 6 SPEC DRAFTED**
(docs/M4-PR6-SPEC.md, DRAFT until PR 5 merges): D1 snapshot+edit-
log versioned text with explicit migration chain, D2 Ryu bit-exact
floats + NaN/inf typed refusal (-0.0 is data), D3 the-recipe-is-
the-save (no tables/keys persisted), D4 recorded ε + SetTolerance
= replay + PR-4 diff (discharges Finding 6), D5 content-key tag
bump (Finding 8, one line), D6 three CI rows (round-trip identity,
ε-diff golden, corrupt/unknown-version typed refusals), D7 Evan's
#92 black-box appearance metadata at the schema-freeze point
(opaque bytes, never interpreted), D8 scope walls, D9 standing
process + A/B protocol.

**#99 dispatched between seams (2026-07-25)**: A/B row 2 —
difficulty S pre-flip, draw 220 → FABLE (two fable draws so far;
fair coin). Branch ev/issue99-tour-eps-panic off main. Charter
framing in the brief: panic is always a bug — outcome must be
green run or typed refusal naming the profile; honest root-cause
split (demo data vs kernel escalation) required, minimal kernel
diff if kernel-side, REPORT prominently. Three lanes now active
(PR 5 review, #93, #99); memory comfortable (8.1G available at
dispatch). PR 6 spec fully ratified after two D7 rounds with Evan
(final: MetaValue tree, serde-native boundary, v-field
convention).

**PR 5 review returned (2026-07-25): mergeable-after-fixes; ruled,
fix pass dispatched.** Report at pr5-review-report.md. F1 MAJOR:
the declared-merge SKIP lane emits Ok bodies failing tier-3
DescriptionNotAdjacent ×4 (declared-identity classification
rewrote edge descriptions anticipating a glue the skip never
performed; reviewer probe = ordinary flush-caps/offset-walls union
with only TRUE declarations, both doors) — ruling: on skip,
descriptions must match ACTUAL unmerged adjacency; probe adopted
as pin; typed refusal only if honest description impossible. F2
skipped-outcome write-only → plumb public + test; F3 Err(_)
catch-all launders tier-2 diagnostics → preserve real reasons; F4
Declare eval-door gaps → adopt BothOperands probe + NodeGone
deleted/foreign + Ambiguous payload; F6 pure-seam-vertex arm zero
corpus instances → targeted fixture; F7 bit_identity doc ¶
contradiction → one line. F5 NOTE accepted (verified-at-use
semantics per contract; DESIGN wording banked for PR 8). Reviewer
verified clean by execution: false declarations refuse at both
doors, rung (b) holds, transform/revert + parallel-mint bitwise
identity, D7 pins + re-run bit-identity, smuggled consumer fails
CI-local, die golden re-pin survived independent 552-row
reproduction. Quality rubric (A/B reference row, fable): 1 MAJ / 3
MIN / 3 NOTE, ONE silent deviation (skip-lane tier-3 posture
unstated), ratings idiom 4 / tests 4 / docs 4. Fix pass also owes
the stale demo renders (table/montage regeneration). MODEL-AB-LOG
reference row updated when fix pass concludes.

**#99 implementation complete (2026-07-25, ev/issue99-tour-eps-
panic @ 4745e9f)**: root cause = DATA bug — bracket fillet via
point rounded to 1.146 vs exact tangent apex 1.5 − 0.5/√2
(1.1464466…), arc carrier 2.315e-6 off tangency = genuinely inside
the carrier_line_circle escalation band at ε=1e-6; the kernel's
typed escalation was CORRECT, the demo's .expect made it a panic.
Fix: exact-tangency constant (post-fix margin ~1.1e-16, definite
Zero at all supported ε), ZERO kernel diff, regression pin runs
the tour binary at default/1e-6/1e-12. Four pre-existing
demos/tour clippy lints flagged (on main, untouched). Compact
blinded review launched (same rubric — A/B row 2 needs comparable
treatment; review scaled to surface, not rigor).

**Docs/memory audit merged (2026-07-25, Evan's ask)**: 6 files
edited (cad-project-state de-rotted — live status now delegated to
this log's tail; CLAUDE.md milestone reference made rot-resistant;
orchestration-model defers model choice to the A/B protocol;
disk-watchdog/freecad-oracle/git-workflow refreshed; MEMORY.md
index resynced), 1 deleted (boolean-consumer-findings — historical,
facts in M3-LOG, action items discharged). Borderline rulings:
KEEP multi-agent-capabilities (spawn mechanics still load-bearing),
KEEP orchestrator-handoff's mngr caution (breakage was real, cheap
to keep), name-candidates deletes when Q9 closes, monitor-arming
consolidation in orchestration-model DEFERRED to the next audit
pass. Report-only flags accepted as historical (M4-PLAN gate.sh
convention text — condition lapsed naturally, policy change is
logged; M4-PR5-SPEC "5G box" — harmless, fix pass told directly).
Reaches main with the next state-sync PR.

**#99 review APPROVE (2026-07-25); PR #100 opened.** Reviewer
re-derived the apex + both margins in exact rationals, verified
the escalation band at predicate level (genuine data bug, not band
papering), reproduced the old panic byte-for-byte on main, and
tamper-tested the pin. 0 MAJ / 1 MIN / 3 NOTE; rubric idiom 5 /
tests 4 / docs 5 — A/B row 2 FILLED (first complete experiment
row). The 1 MINOR (failure-message tail reversed) orchestrator-
fixed directly (f4460a5, pin re-run 3/3). #100 merges on green
checks. Review clone for #99 removed after merge.

**#100 MERGED (2026-07-25): main `9b07465`, closes #99** on 10/10
green checks. #99 lanes + docs-audit clone removed (pr5-review
clone RETAINED until PR 5 merges, in case the fix pass needs the
reviewer resumed). Remaining in flight: PR 5 fix pass, #93.

**SESSION-LIMIT FLUSH (2026-07-25, 98%, reset ~25min; wakeup
scheduled)**: PR 5 fix pass COMPLETE @ 386a900 (F1 skip-lane
re-description at both doors, F2/F3 skip records public+honest,
F4 door tests, F6 arms pinned ×2, F7; battery 1256/0 + 1256/0 +
interval 1400/0; renders regenerated incl. corner-aligned table
montage). **PR #102 OPENED** — checks watcher NOT yet armed (limit).
#101 declared-tangency discipline RATIFIED in-session (5-point
shape; between-seams unit, coin flip at dispatch). WAKE CHECKLIST:
(1) arm PR 102 checks watcher → merge on green → clean pr5/
pr5-review clones → state-sync PR to main; (2) check #93 lane
(was mid-battery — likely died at the shared limit; resume from
transcript, clone ~/.local/share/cad-work/issue93, branch pushed
@ 40670fa+); (3) answer Evan's large-K demo lint question
(recommendation drafted: fold into PR 8's K-telemetry wiring —
sweep demo scenes alongside Band 4 corpus, |m|/ε percentile
threshold from K-REPORT baselines, advisory-first; #99's 2.3ε
margin at 1e-6 is the motivating catch); (4) A/B: PR 5 reference
row fillable from review rubric (1MAJ/3MIN/3NOTE, 1 silent dev,
4/4/4); #93 row awaits its review; (5) #101 + tangency unit and
PR 6 dispatch (spec ratified) queue after PR 5 merges.

**Post-limit wake (2026-07-25)**: monitors verified alive (no
re-arm needed); PR #102 checks watcher armed; #93 lane resumed
(had died 500-then-529 while finalizing its report — work done,
report pending). A/B PR 5 reference row filled. Large-K demo lint
(Evan's question): RULED — fold into PR 8's K-telemetry wiring;
the Probe sweep gains the demo scenes alongside the Band 4 corpus,
lint statistic = |m|/ε per K-REPORT's normalization, threshold
from the K-REPORT baseline percentiles, ADVISORY first run then
gating once the baseline is pinned; tooling-level threshold only
(no kernel ε). Motivating catch: #99's margin was 2.3ε at 1e-6 —
in-band there, but the lint would have flagged it as a bottom-
percentile margin at EVERY ε row, before any band was entered.
Added to PR 8's obligations.

**PR 5 MERGED (2026-07-25): #102 → main `75166b8`** on 10/10 green
checks. The M4 naming stack is now END-TO-END: GeomSource identity,
bit_identity out of production, Declare threading live, first real
Merged rows, corner-aligned table shipped, #95 recursive naming
key. pr5 + pr5-review clones removed (disk 120G). Seam actions:
state-sync PR; PR 6 + #101 dispatches (coin flips below); #93
merges main + reviews when its report lands.

**Seam dispatches post-PR-5 (2026-07-25)**: state-sync PR #103
opened (watcher armed). #93 report received — three stacked
join-stage repairs (anchor tiers via point_in_face-certified
centroids, Newell ring-winding predicate, rehome_rings), all A×Z
variants green with independently derived oracles; merge-under of
PR 5 dispatched with the flush-pin ruling (pin the live post-merge
outcome; undeclared flush refusing = correct N6; add declared
success arm if cheap). **PR 6 implementer LAUNCHED** (A/B row 3:
L pre-flip, draw 221 → fable; branch ev/m4-6-persistence).
**#101 implementer LAUNCHED** (A/B row 4: M pre-flip, draw 218 →
fable; branch ev/issue101-declared-tangency; 5-point discipline
binding). FOUR consecutive fable draws — blocked-randomization
proposal noted in MODEL-AB-LOG, pending Evan.

**#104 filed (2026-07-25): PartialPath authoring algebra — CONCEPT
status per Evan** ("not a mandate; needs to harmonize with
whatever we already have"). Tangency-by-construction typed path
builder; harmonization analysis in the issue: composes with #101
(authoring layer lowering to its flag/verify document contract,
reusing its junction predicates), closure = the care point
(tangent closure overdetermined for one arc → constructor family),
schema v1 unaffected (profiles-as-programs = determined v2 lift
via F3 migrations; ladder rung below M6 constraint sketches). Open
questions listed (LoopBuilder relationship, sugar inventory,
multi-loop, constructor-quantity exactness contract). No
implementation until ratified. #101 implementer nudged
(structure-only): junction classification as a small callable API,
5 points win any tension.

**#104 sequencing settled (2026-07-25, Evan delegated the call)**:
schema v1 = explicit geometry + #101 flags; profiles-as-programs
DECLARED as the v2 end state (not optional) via the F3 migration
chain, lift determined by the flags. Grounds: v1 must not freeze
around an unratified algebra (closure family, multi-loop, sugar,
exactness contract all open); the algebra gets exercised by real
authoring before entering the file format. Evan's philosophy point
(program = truer recipe form) accepted — it's what upgrades v2
from option to commitment. Recorded on #104.

**#93 merge-under complete (2026-07-25, @ 51d6244)**: clean merge
with PR 5, post-merge battery green (1260/1405 baselines + 5
acceptance tests), flush/rung-(b)/goldens unchanged, and the
REPORT-item pin landed exactly per ruling — coupled flush A×Z
refuses UndeclaredCoincidence undeclared, succeeds EXACTLY
(2562165/950272) with all six pairs declared, through the new
anchor tiers: PR 5's narrowing and #93's fix compose as designed.
Adversarial reviewer LAUNCHED (blinded, rubric; attack list:
oracle re-derivation, envelope non-widening + fresh opposite-side
case, Newell-predicate soundness + Interval escalation, centroid
boundary certification, doubly-nested rehome case, golden
stability, declared-arm truth). Three lanes now: PR 6, #101,
#93-review.

**#101 implementation COMPLETE (2026-07-25, ev/issue101-declared-
tangency @ b76834f, pushed)**: all 5 ratified points — per-junction
classifier reusing existing carrier predicates verbatim (zero new
ε), UndeclaredTangency with repair menu, TangencyContradicted
(verified never trusted), LoopBuilder::fillet (bit-exact on dyadic
right angles, declares by construction) + declare_tangent,
persistence keys the flags. Migrations: bracket → constructor, 11
corpus fixtures declared, free arcs verified transversal +
untouched, no unintended definite-Zero found. Batteries green
(1267/1412 new baselines; tour + eps_regression 3/3). Rulings
R1-R6 ACCEPTED: R1 same-carrier = identity not tangency (declared
cocircular/collinear joints refuse with same_carrier:true; two-arc
circles stay legal — spec refinement, credit to implementer); R2
cusps classify Tangent (smoothness = direction question, additive
later); R3 pub(crate) + future public wrapper for #104; R4
line/line fillet scope (arc-leg = sugar follow-up, noted on #104);
R5 intent judgments (reviewer spot-checks two); R6 DESIGN.md sync
→ PR 8 exit-sweep obligation. Blinded reviewer LAUNCHED (rubric;
attack: narrowing holes at every door, verification laundering via
in-band, same-carrier gate edge cases, fillet degeneracies,
migration intent honesty, free-arc classification, predicate
discipline). Foreground-battery clause now standard in prompts.

**#93 review returned (2026-07-25): APPROVE CODE, CORRECT REPORT;
two MAJOR claim-findings, both filed.** All four oracles
independently re-derived exact; old winding probe's unsoundness
empirically confirmed; no envelope widening (two fresh flush
constructions bit-identical main vs branch); Newell predicate
sound with typed Interval escalation; batteries exact-match. F1:
implementer's unreachability claim FALSIFIED — general-position
depth-2 nested island reaches the anchor-exhaustion arm (typed
refusal, load-bearing) → #106 completeness gap (ready fixture).
F2 (HEADLINE): **main 75166b8 silently returns V=22.5 vs exact
22.4375 on a depth-2 chain, all tiers passing — fail-loud violated
in general position; the #93 branch computes it exactly** → #105;
push notification sent to Evan. Fix pass dispatched (correct the
claim, adopt both constructions as pins — #106 as typed-refusal
pin, #105 as exactness pin; stale titles; in-clone battery rerun
mandatory per the cwd incident). Quality rubric (A/B row 1
pending fix pass): idiom 4 / tests 4 / docs 4.

**Ops incident (2026-07-25, caught by hourly sweep): resumed
subagents' cwd resets to the orchestrator worktree** — five stray
cargo processes found building MY worktree (wrong tree, green-but-
invalid numbers); killed PID-scoped, stray target/ deleted (35G→
freed, disk 87G), all three lanes corrected (cd-per-command now
mandatory), lesson banked in subagent-death-recovery memory with
the /proc-cwd sweep check.

**Session crash + recovery (2026-07-25 ~16:20 → 21:21 PDT, cause
unknown — Evan doesn't know either; not disk (87G free at the
time), possibly WSL or host)**: whole Claude process died; all
monitors and three subagent lanes orphaned. NOTHING LOST —
push-per-unit held again: #93's fix-pass commit a628040 pushed
pre-crash (docs corrected, #105/#106 pins, minors); PR 6's
bb16238 pushed; #101-review clone intact. Recovery: monitors
re-armed ×3; all three agents resumed from transcript with the
cd-per-command + foreground-battery rules restated; #93 verifying
its final battery then done, PR 6 filling report placeholders,
#101 review continuing. Reports outstanding: #93 final numbers,
pr6-report placeholders, issue101-review-report (not started).

**#93 PR OPENED (2026-07-25 evening): #108** (closes #93 AND #105
— the main-silently-wrong bug this branch fixes). Post-crash
re-attestation clean: full in-clone battery at a628040 (145
suites: 1263 @ 1e-6 and 1e-12, 1408 interval), issue93_nested_
islands 3/3 (#105 exactness pin, #106 typed-refusal pin with
retire-on-closure instructions, depth-1 control), docs corrected
(exhaustion arm load-bearing), O(n²) guard hoisted. Checks watcher
armed; merge on green. A/B row 1 FILLED (fable, M: 2 claim-level
MAJ / 1 MIN / 2 NOTE, 0 silent, 4/4/4, moderate fix pass). #106
stays open as the coverage-gap tracker — NEXT OPUS DISPATCH per
protocol v2 block 1.

**#101 review returned (2026-07-25 evening): discipline HOLDS at
every door** — 17 executed probes clean across narrowing holes
(direct/builder/recipe/embed/multi-loop), verification laundering
(declared+in-band → typed Escalated with repair menu), R1
same-carrier gate (near-cocircular escalates), migration intent
spot-checks (definite-Zero proven by classifier), free arcs
(validate undeclared), predicate discipline (zero new ε,
bit-identical margins, interval escalation executed); fillet
corner bit-exactness re-derived IEEE-step-by-step. 1 MAJOR:
oversized fillet radius silently validates (arc never approaches
the corner; sugar doc claim falsified — the review's one silent
deviation) → RULED: refuse via TangentJointOutOfRange when a
tangent point falls outside its leg; degenerate-radius test
family; reviewer probe inverted to pin. 1 MINOR (float_bits
usize::MAX sentinel collision → tag the key space), 3 NOTEs. Fix
pass dispatched. Quality rubric (A/B row 4 pending fix pass):
1/1/3, 1 silent dev, idiom 5 / tests 4 / docs 4.

**PR 6 implementation COMPLETE (2026-07-25 evening,
ev/m4-6-persistence @ aebc39e, pushed)**: D1-D9 all landed —
schema-1 JSON text format (REPORT: chosen for ryu floats +
tooling; serde_json float_roundtrip feature LOAD-BEARING, caught
real last-ulp parse drift day one), migration chain, full recipe
persisted (Expr rebuilt through dimension checkers on load,
ProfileDesc hand-wired keeping kernel crates serde-free, hex
witness bytes, structural appearance keys), recorded-ε wiring with
per-node evaluate refusal + the FIRST REAL two-ε diff through the
PR 4 population core (Finding 6 discharged), content-key tag 1→2,
three D6 CI rows built FIRST and wired both lanes, MetaValue tree
per final D7. Battery: 150 suites, 1280/1280/1425, all D6 rows
green. Blinded reviewer LAUNCHED (12-item attack list: float
round-trip property attack incl. building WITHOUT float_roundtrip,
non-finite smuggling doors, corrupt/truncation refusals, replay
identity with an independent all-vocabulary doc, ε-conflict doors,
MetaValue canonicality, serde-free layering grep, edit-log
exercised-not-vestigial, CI tamper checks, tag-bump memo check,
#101 SCHEMA-COLLISION SCOPING — sequencing: #108 → #101 → PR 6
fix pass merges main + extends schema v1 to tangent_joints before
freeze). A/B row 3 pending review.

**#108 MERGED (2026-07-25 evening): main `6f7d79a`, closes #93 +
#105** on 10/10 green checks. The join-stage seam-region gap is
fixed, main's silent-wrong-volume bug is dead (exactness pin in
place), and #106 remains the tracked coverage residue (next OPUS
dispatch). issue93 clones removed. Merge order proceeding: #101
(fix pass running; merges main before PR) → PR 6 (review running;
fix pass merges main + extends schema v1 to tangent_joints).

**PR 8 SPLIT (2026-07-26, Evan's question — he's right)**: 8a =
corpus + latency reporting (implementer, L, gate PR 6+#101); 8b =
K-probe + large-K lint (implementer, M, gate 8a merged so the
lint baseline is stable); 8c = DESIGN.md exit sweep + trim + exit
walk (ORCHESTRATOR + Evan sign-off — design-ratification work per
the M0-M3 convention; runs LAST, never self-attesting; not an A/B
row). Spec restructured in place.

**PR 6 review returned (2026-07-26): REQUEST CHANGES — 2 MAJ / 2
MIN / 3 NOTE, all narrow; every REPORT claim verified true**
(float_roundtrip proven load-bearing by removal; independent
14-edit-variant doc round-trips bit-identical at 3 ε + interval;
ε-diff golden exact + tamper-fails; layering holds; memo works
under tag 2). MAJ-1: NaN with all-ones bits skips the save-door
walk (float_bits loop marker = real NaN; second sentinel-collision
this milestone — pattern flagged) → save accepted NaN, wrote null,
unloadable file. MAJ-2: duplicate JSON keys last-wins silently in
serde-derived maps (no-silent-loads violated). MIN-3: no committed
golden v1 fixture (drift CI-invisible — fixpoint-only row). MIN-4:
tangent_joints extension scoped precisely (wire field + key/bit_eq
/content-key sight + embed carry + corpus row). Fix pass dispatched
TWO-PHASE: doors+golden now; tangent_joints after #101 merges.
Quality (A/B row 3 pending): 2/2/3, 0 silent (5 reported devs),
idiom 5 / tests 4 / docs 5.

**Sentinel-disease structural ruling (2026-07-26, Evan: "deserves
proper types")**: float_bits' in-band magic delimiters are the
shared root of #101's usize::MAX alias AND PR 6's NaN-marker
alias (+ the door blind spot) — ruled: retype the key-encoder
stream as TAGGED TOKENS (tag byte + payload; Marker/Float/Index),
no in-band sentinels anywhere; both classes become unrepresentable.
Rides PR 6's existing content-key tag bump (keys process-internal
per D3, zero persistence impact). Added to PR 6 fix pass phase 1
with a STOP-and-REPORT scope valve.

**#101 PR OPENED (2026-07-26): #109 @ b4f0eed** (post-merge of
#108; 147 suites 1283/1283/1429, eps_regression 3/3). A/B ROW 4
FILLED: #101, fable, M — 1 MAJ / 1 MIN / 3 NOTE, 1 silent dev
(the falsified fillet doc claim), idiom 5 / tests 4 / docs 4,
fix-pass moderate (fillet Result + fit predicate + 6 pins + key
fix), ~880k tokens incl. crash resume, checks watcher armed.
Merge on green → then PR 6 phase 2 ping.

**#109 MERGED (2026-07-26): main `1f3be61`, closes #101** on 10/10
green. The declared-tangency discipline is live: profiles refuse
undeclared definite-Zero tangency, declarations verify, the fillet
constructor authors exact tangency with fit gating. #101 clones
removed. PR 6 PHASE 2 GO sent (merge main + tangent_joints schema
extension + regenerated golden + full battery). Remaining to M4
code-complete: PR 6 merge → 8a → 8b; then 8c exit sweep
(orchestrator + Evan). #106 (Opus row) in flight, not a gate.

**Side-chain dispatches (2026-07-26, Evan's ask)**: (1) IN-HOUSE
INTERVAL TRANSCENDENTALS (the tabled DESIGN.md post-M7 item pulled
forward): standalone workspace-excluded crate, proven libm error
pads + monotonicity/extremum handling, decoration semantics per
the M0 poison ruling, differential containment/tightness harness
vs inari-as-dev-dep-oracle, build/perf evidence; kernel UNTOUCHED
(adoption = later ratified decision). A/B ROW 5: difficulty L
pre-assigned, arm = FABLE (block-1 remainder, no draw). Branch
ev/interval-transcendentals. (2) M6 PRE-DESIGN: ERROR-DESIGN.md
DRAFT (error-propagation MVP — distributions, Dual sensitivities/
stackups, interval-over-parameter-box checks; composes with W1-W9)
per the NAMING/SOLVER/CURVED pre-milestone pattern; zero-cargo
lane; design conversation with Evan later. Branch
ev/m6-error-design. DEFERRED deliberately: REST-contact join lane
(collides with #106's region — after it merges); arc-leg fillet
sugar (filler). Four lanes now: PR 6 fix pass, #106 (opus),
interval-core, m6-design.

**computable revival (2026-07-26, Evan)**: his prior computable-
reals library (~/projects/computable — state/bounds/refinement
formalism, dyadic bounds, refine_to) joins the interval-core lane
as a potential SECOND ORACLE (inari-independent tight enclosures
for the containment harness); Evan upgraded scope from read-only
to fix-up-with-purpose ("inspiration to revive it"). Lane rules:
guest conventions (its AGENTS.md/STYLE.md), new branch, no history
rewrites, never push its default; scope ladder rot→bugs→
transcendental additions with a size-it-first valve so the
interval crate stays primary. Honest-fitness paragraph still
required (computable reals ≠ fixed-precision interval lane;
possible M6 clearance-check exactness role).

**#106 implementation COMPLETE (2026-07-26, ev/issue106-depth2-
coverage @ ccda018, pushed) — FIRST OPUS ROW.** Tier-4
Anchor::RegionVertexChord (anchor-vertex-to-region-vertex chord
midpoints, existing point_in_face certificate, no new predicate/ε),
justified via the no-Steiner triangulation theorem. #106 refusal
pin retired to exactness 13/4 per its baked instructions; #105 +
depth-1 green; depth-3 (205/64) and comb (405/128) probes pinned
at live outcomes. Notable HONESTY: self-ablation revealed the comb
probe already built pre-fix — docstring corrected rather than
credit claimed; residue enumerated with NO unreachability claim
(the #93 lesson landed). Battery 1265/1265/1411, goldens unmoved.
Blinded reviewer LAUNCHED (attack: anchor-only chord-existence gap
vs the theorem's some-vertex guarantee, boundary-midpoint
certification, oracle re-derivation, ablation-claim verification,
tier-ordering verdict-log identity).

**ERROR-DESIGN.md DRAFT delivered (2026-07-26): PR #110 opened as
a design conversation (WAITS for Evan — the pre-milestone design
class; no self-merge).** E1-E11: distributions as document-layer
ParamDef metadata (kernel sees boxes/seeds only), Measure sink
nodes, chamber-certified-or-local_only Dual sensitivities,
certified-interval worst-case gates with RSS advisory (refused
under Band), read-only analysis lane composing with W1-W9 (leaf-
box W2 certificates; no crossing witness walls — a flagged
narrowing of Q1's union sentence). Five spiciest calls front-
loaded in the PR body; sign-off comment on the watchlist. PR 6
delta re-check dispatched to its reviewer (tagged-token retype +
tangent_joints are post-review deltas — mid-flight-changes
convention); PR + merge gate on APPROVE-DELTA.

**#111 filed + dispatched (2026-07-26)**: the az-render lane found
a REAL mesh bug — A×Z bodies are exact-kernel-perfect but
tessellate non-watertight (CDT centroid-parity keeps an exterior
needle triangle on 1-ulp-noisy collinear seam boundaries;
BoundaryEdge refusal; δ-independent; suspected unreified in-band
decision in the parity/culling path). Render UNBLOCKED via the
STEP lane (all-planar body); scene ships with the mesh refusal
PINNED tripwire-style (cites #111, retire-instructions) — no
silent skip, no weakened shared check. #111 fix dispatched as its
own kernel unit — A/B ROW 8: difficulty M pre-assigned, arm =
OPUS (block-2 remainder). Suspected-class survey (other unreified
CDT decisions) included report-only.

**#110 design round 1 (2026-07-26, Evan)**: two forks — (1) Real-
trait lineage ("Interval was a quasi-stand-in for uniform; where
does moving measures out of Real leave the original design?") —
answered: the Real channel is the PER-LEAF ENGINE, not demoted;
measures can't ride the scalar channel because dependency makes
distribution arithmetic WRONG (not interval-loose — no conservative
direction; p-boxes/Fréchet collapse under the kernel's shared-
parameter workload); the measure prices leaves of INPUT space only,
so derived-quantity correlation never needs representing; Interval
= the sound integration kernel for any input measure = the original
purpose completed. E1/E2 to restate lineage explicitly. (2)
Truncation hesitancy — CONCEDED: mandatory truncated support
replaced by TAIL-MASS ACCOUNTING (unbounded supports welcome;
analysis box is the knob; outside-mass carried as an explicit
additive term in every result; truncation = optional sugar).
Awaiting Evan's reaction before the drafter reworks.

**PR 6 PR OPENED (2026-07-26): #112 @ a4997ff** — schema v1 freeze
(the M5 gate). Delta fix closed MAJOR-DELTA-1 (save-door joint
bounds, both sites) + two symmetry-sweep holes beyond the ask
(save-side EditReplay verification; to_value duplicate-key
refusal). Full trail in the PR body. Checks watcher armed; merge
on green → then 8a dispatch (block 3 draw). A/B row 3 rubric on
merge: 2 MAJ / 2 MIN / 3 NOTE + 1 delta MAJ, 0 silent (5 reported
devs), idiom 5 / tests 4 / docs 5.

**PR 6 MERGED (2026-07-26): #112 → main `78fe760` — SCHEMA V1
FROZEN, the M5 gate is through.** Matrix now carries three
persistence rows (1e-6/1e-9/1e-12) hosted. pr6 clones removed.
A/B row 3 final: fable, L — 2+1 MAJ / 2 MIN / 3 NOTE, 0 silent
(5 reported), idiom 5 / tests 4 / docs 5, substantial fix passes
(tagged-token retype + strict maps + goldens + tangent_joints +
save-door symmetry sweep). **#106 review APPROVE 0/0/4** (opus row
5 final: 0/0/4, 0 silent, 4/5/5, NO fix pass — first
zero-fix-pass unit of the milestone; NOTEs banked for 8a latency
data); PR #113 opened, watcher armed. **8a DISPATCHED** — block 3
draw: (opus, fable) → 8a = OPUS (A/B row 9, L pre-assigned);
branch ev/m4-8a-corpus off 78fe760. Remaining after 8a: az PR
(review executing), #111 (in flight), 8b (fable, gated on 8a),
8c (orchestrator + Evan).

**#113 MERGED (2026-07-26): main `0251da7`, closes #106** on the
FULL 13-row matrix (branch refreshed post-#112 before merge — the
stale-10-row-green trap caught and dodged; watcher pattern now
requires ≥13 rows). Depth-2+ nested islands classify; the join
envelope's typed residue shrank honestly. #106 clones removed.
**Interval-core lane delivered** (ev/interval-transcendentals @
02c6147): proven-pad crate, 3.2M dual-oracle cases (computable
REVIVED — zero rot, wired as second oracle 4/4), harness caught a
real 1-ulp 2Prod underflow bug in itself, 93× build / 1.4-135×
runtime evidence vs the gmp stack. Ops note: lane accidentally
committed 252MB of target/ blobs in two pushed commits (removal
COMMIT, no history rewrite — verified via reflog); orchestrator
authored ev/interval-transcendentals-v2 (identical tree, clean
history; original branch retained as the record) — the PR lands
from v2. Blinded review LAUNCHED on v2 (pad-math per-function
derivation checks, war-story reproduction incl. fix-revert,
harness-direction integrity, degrade-boundary bombardment).
Adoption = separate ratified decision (M5-PLAN candidate).

**#114 MERGED (2026-07-26): main `1f7e1e9`** — the A×Z scene +
render land (13-row green); az clones removed. A/B row 7 final:
fable, S — 1 MAJ (fallback-lane crash) / 1 MIN / 2 NOTE, 0 silent,
idiom 4 / tests 4 / docs 4, small fix pass, ~263k tokens. Open
PRs now: #110 only (ERROR-DESIGN, Evan's careful pass). In
flight: 8a (opus), #111 (opus), interval-review. M4 close =
#111 merge + 8a + 8b + 8c.

**#111 implementation COMPLETE (2026-07-26, ev/issue111-cdt-needle
@ d77ce5d, pushed) — opus row 8.** Root cause SHARPENED beyond the
issue: the parity test POINT was wrong (spade center() rounds
~5e-17, 10× the needle's half-thickness — reification alone could
never fix it; exact parity on the rounded point still answers
inside). Fix STRUCTURAL: even-odd flood fill over the CDT face-
adjacency graph seeded at the outer face, toggling on odd
constraint-crossing multiplicity — integer traversal, zero float
comparisons, no new ε; watertightness across boundary edges by
construction; revolve slit preserved via odd-multiplicity;
try_add_constraint refusal kept typed + atomic. 6 pins incl.
exact-chart unit replay + all three A×Z variants watertight +
az_intersect ADDED TO THE EXTERNAL ADMESH GATE (passes). Battery
1289/1289/1435 (baseline math reconciled vs #109). Survey: two
same-class signed_area<0 winding flips reported not-in-band (
reviewer to verify the bound). Post-merge follow-up owed: flip
the demo scene's #111 retire-pins. Blinded reviewer LAUNCHED
(flood-fill attack surfaces: seeding uniqueness, shared-sub-edge
multiplicity, slit variants, disconnected interiors, depth-3
holes; watertight-theorem-vs-bookkeeping; root-cause re-derivation;
winding-flip band argument).

**Interval-crate review returned (2026-07-26): VALUE-SOUND, PROOF
PROSE NOT — fix pass dispatched.** No containment violation in
3.7M dual-oracle cases; war stories fail-on-revert exactly; no
second atan2 trap (15-config hunt); degrade boundary honest;
re-land tree-identical. 1 MAJ: §2 claims libm atan2 = 1 ulp but
libm's own CI table says 2 — PAD_ULPS=4 survives ONLY via the
sharper bit-distance argument (atan2 margin 1), which must be
WRITTEN as the proof before "certified" is honest. MINORs: report
misquoted its own harness (cos p50, powi p99 = 2^50+2 needing
honest explanation, 3.7M not 3.2M), hull() decoration divergence
SILENT (doc honesty 3/5 — milestone low), ops.rs zero coverage,
irreproducible build-time claim (measured: 48s CPU / 203MB RSS).
Quality (A/B row 6 pending fix pass): 1/4-ish MIN/…, idiom 5 /
tests 4 / docs 3. Plan: after fix pass, the crate PRs to main as
workspace-excluded tooling (zero kernel risk); ADOPTION stays an
M5-PLAN ratified decision.

**Interval-crate fix pass complete (2026-07-26, v2 @ 8c30882); PR
#115 OPENED** (crate as workspace-excluded tooling; adoption =
M5-PLAN decision). §2 rewritten as the real proof (source facts
re-verified at libm-v0.2.16; Lemma P3 with negative-order-reversal
explicit; margins: sin family 2, atan2 1 — future functions with
CI bound >3 flagged); lucky-original kept as war story #4; powi
2^50 tail explained + regime-split; D7/D8 divergences documented
AND pinned; ops.rs 5 units + 300k×4-op differential lane; build
claims restated as measured (1.6s withdrawn). Final: certify
12/12 (~5.8M asserts / 4.0M cases), edges 9/9, computable 4/4,
kernel untouched. Watcher armed (13-row floor).

**Second limit outage + recovery (2026-07-26 ~11:21-23:30 PDT)**:
Fable limit hit mid-day; Evan re-logged-in evening. Pre-limit
landings survived pushed: #115 checks went green during the gap →
MERGED on wake (main `12851a2` — the interval crate lands as
tooling); **8a COMPLETED pre-limit** (ev/m4-8a-corpus @ 172ebe9:
8 documents / 160 nodes, coverage ASSERTED with fails-on-unlisted-
kind, 6 == mass pins with derivations, PR 6 rows rewired onto the
corpus, latency lane report-only with counted-reuse asserts — die
14.4s→1.0s incremental, cone 3/77; TWO findings: depth-2 chain
pinned-refusal with promotion path since #106 postdates its base,
and PR 6's kitchen-sink fixture carries a silently-Failed boolean
its fingerprint comparison can't see). #111 reviewer died at the
limit → resumed. **8a blinded reviewer LAUNCHED** (coverage-
totality tamper, 3 oracle re-derivations, both findings verified
incl. promotion-path execution, Recorder fidelity, cone-by-hand,
no-gate grep, old-fixture coverage shrinkage).
