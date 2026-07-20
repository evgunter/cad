---
name: cad-project-state
description: Greenfield Rust CAD kernel — DESIGN.md is the authoritative contract; M0/M1 complete; M2 in progress (PRs 1–2 of 7 merged, PR 3 implemented awaiting review — see docs/M2-LOG.md state snapshot)
metadata:
  node_type: memory
  type: project
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Greenfield B-rep CAD kernel in Rust (repo evgunter/cad). **docs/DESIGN.md
is the authoritative, ratified design contract** — read it before any
design or implementation work; do not re-litigate settled decisions
D1–D9. D1 now carries the M1 ratifications: half-edge structure (typed
`LoopBoundary`, computed mate, `outer ∉ rings`), the one-rule
orientation convention (interior-left ⇒ CCW-from-outside; GWB diagrams
are MIRRORED — never transcribe), the ten-operator set + `ring_move`
with site-enum addressing and the atomic/deterministic/postcondition
contract, and the three validity tiers (euler-valid / closed-solid /
geometric) with component-aware per-shell Euler–Poincaré.

**M0 complete (2026-07-16)** — `geom-core` (comparison-free `Real`,
trilean predicates, single-ε `Tolerance`, linalg) + `topo` skeleton.
**M1 complete (2026-07-16)** — half-edge topology + all ten Euler
operators + 12-pass tier-1 validator + `validate_closed`; cube and
holed box build through public ops only; raw builder is crate-internal;
`Body<Interval>` instantiates. PRs #15–#26 (see docs/M1-LOG.md; M0:
docs/M0-LOG.md). Notable: Mäntylä Program 11.6 erratum on record
(reading notes); replay-with-kills is per-arena (see D9); the
adversarial-review corpus of both milestones runs in CI
(`review_m{0,1}_pr*` suites).

**M2 in progress** (ratified #24 — ALL forks resolved in that
conversation: DXF bulge-chain profiles, winding invisible, full axis
support for revolve incl. pole collapse, D2's intensional EdgeGeometry
landing at M2 (option a), certified-conservative tessellation as an
export promise, no auto face-merging). Of the 7-PR sequence: **PR 1
merged (#27** — geom-curves/geom-surfaces closed-enum evaluators;
Real gained floor/reduce_periodic/copysign; branchless Duff basis; L7
allowlist moment resolved NOT-needed); **PR 2 merged (#28** — profile
crate, trilean validation, exact-order-band canonical form, K-hook);
**PR 3 implemented on `ev/m2-3-edgegeom`, adversarial review NOT yet
run** — geom-brep crate (EdgeGeometry certified-by-construction,
dihedral predicate, Newell, tier-3 validate_geometric, op signatures
with FaceSurface/EdgeCurveSpec). **docs/M2-LOG.md's "State snapshot
(handoff point)" is the resumption contract** — next moves: PR 3
review ∥ PR 4 (extrude) implementation, overlapped pipeline.
Remaining: 4 extrude, 5 revolve, 6 tessellation, 7 STL + K report +
exit. Mäntylä ch. 12–15 notes all archived in
`<main-checkout>/references/notes/`; the TOG 1986 boolean paper
(M3's second witness, text-layer PDF) is at
`references/mantyla-1986-boolean-operations-2-manifolds-tog.pdf`.

**Usability scoping (2026-07-19, branch `mngr/plan-gui`)**: DESIGN.md
gained "Beyond the kernel: the usability gap" (kernel-side client
services incl. incremental recompute + picking back-refs +
cancelation + selection stability; GUI = second kernel-sized project;
assemblies/drawings-HLR/feature-breadth; product infra) and a "Tabled
(far future)" section (interval-transcendentals reimplementation
moved there from the roadmap). **Sequencing stance: usable-as-library
before any GUI.** M2-PLAN PR 6 amended: mesh entity back-references +
appearance FINAL 2026-07-20 = full deferral, NO M2 artifact (contract
only, in DESIGN.md Band 1: attach in editor-core by stable names from
M4; PR 6 also gained Vertex keys in the back-refs and per-face patch
separability via the PR #32 orchestrator review).
**docs/GUI-DESIGN.md exists
(Evan-ratified 2026-07-19)**: G1 three-layer split (kernel /
headless `editor-core` — now in the crate table — / interaction),
edit-vocabulary-as-data, selection type = stable-name type.
Ratified: GQ1 (witness = authoritative branch selection;
`solution(constraints, params, witness)` pure; bifurcation = typed
error, distance-to-singularity margin; mechanism details are M4/M6
work; ezpz bit-identity audit item), GQ2 (per-node result DAG,
failures poison descendants only), GQ3 (all edits persisted,
snapshot+edit-log), GQ5 (typed quantities in the expression
sublanguage — supersedes the raw-meters reading; dimension-algebra
extent banked for M4/D8), GQ4 (one doc = one part recipe, multi-body
OK; refs document-local; cross-doc = assembly-era wrapper; **Evan's
uniformity principle: assemblies are recipe DAGs of the same
formalism — doc boundary is a namespace/versioning seam, so
GQ1-witnesses apply to mates verbatim; axis-3 binding semantics
RATIFIED in direction: Cargo.lock-style pinned-with-explicit-update,
details at assembly design**). GQ1–GQ5 all closed; remaining pre-M4
design work: GQ1 mechanism details + the selection-stability/naming
design doc. **DESIGN.md "Banked principles" subsection (ratified
2026-07-19 rounds 6–7)**: naming-localized-to-predicate-flips
(naming-doc pillar), content-keyed cache transfer (M2 PR 6 keeps
per-face patches separable), scalar-generic editor-core evaluation,
ε recorded-in-document + uniform-across-assembly + **change-ε as a
recorded SetTolerance edit (replay + D9 structural diff; any
predicate-verdict change = typed error, explicit resolution)**,
SE(3) mate-witness / pattern-index / corpus-at-M4 flags, and
**coincidence-is-structural-or-declared (pre-M3, Evan's strengthened
form)**: bit-equal descriptions never glue (unmargined-predicate
defect); coincidence intent is recipe data (shared key or declared
relation), detection is affordance-only, near-coincidence resolves
via explicit D7-style repair/adoption. Round 9 added six more:
fillet validity as reified predicates (pre-M5), SSI
completeness-by-subdivision contract (pre-M5), non-manifold boolean
results = typed errors (M3), expression language total-by-charter
(M4), two-layer DOF diagnosis (structural + GQ1 margin, M6),
bit-exact float persistence. All usability/GUI-conversation items
ratified; nothing pending. PR #32 (branch mngr/plan-gui) carries the
whole conversation — merge awaits Evan.

Key operational facts: **reference PDFs and notes live in the MAIN
checkout's `references/`** (git-ignored dirs don't propagate across
worktrees — NURBS book + Hoffmann were stranded in the original
session's worktree until 2026-07-16); the `interval` cargo feature
(geom-core AND topo) quarantines LGPL per issue #4 (closed; README +
rustdoc carry the consumer-facing note); x86-64 floored at x86-64-v3;
CI: fmt/clippy/test + ε matrix {1e-6,1e-9,1e-12} + interval lane +
`Real +` discipline grep. License dual MIT OR Apache-2.0; name still
pending (Q9). See [[cad-working-style]], [[orchestration-model]],
[[git-workflow]].
