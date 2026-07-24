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
