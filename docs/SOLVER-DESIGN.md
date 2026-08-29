# GQ1 mechanism details: witnesses, branch selection, bifurcation (pre-M4 design doc)

Status: **RATIFIED** (Evan, PR #79, 2026-07-23 — W1–W9 as proposed
plus the round-2 amendments: bulk certified-same-branch ReWitness,
drag-as-homotopy UI input, the worked elbow example, and the
GUI-DESIGN wall-mode-drag idea, all from the in-conversation round). This is the second and last "before M4 planning"
doc (DESIGN.md; the first was NAMING-DESIGN.md, #74). Grounding:
`references/notes/gq1-constraint-inventory.md` (ratified constraints
quoted; tensions T1–T6) and `references/notes/gq1-solver-litreview.md`
(branch-selection taxonomy A–D; certified-selection toolbox;
bifurcation theory; SE(3)).

*IMPLEMENTATION STATUS (added at the M4 8c exit sweep, 2026-07-27;
record unchanged):* the W-CONTRACTS landed at M4 PR 4 (#96) — witness
datum in recipes, typed error surface, purity boundary, solver
contracts as DATA ONLY (the W5 solved-assignment slot exists and stays
empty); the solver itself remains M10-era as designed. ERROR-DESIGN.md
(ratified 2026-07-27, #110) composes with W1–W9 as this doc
anticipated.

GQ1's ratified core is NOT reopened here:
witness = authoritative branch selection; `solution(constraints,
params, witness)` pure; continuation along the edit path banned;
bifurcation = typed error with distance-to-singularity margin;
witness refreshes at committed sketch edits; interval replay runs
interval-Newton contraction seeded from the f64 witness. This doc
pins the mechanism those sentences left open. Implementation is
M10-era (sketch solver); M4 needs the contracts (witness datum in
recipes, error types, purity boundary) — that is why this doc
precedes M4-PLAN.

## 0. Term hygiene (read first)

**Our witness** is recipe-stored *branch-selection data*: it answers
"WHICH point of the discrete solution set did the user mean?" The
geometric-constraint-solving literature uses *witness* for something
else — Michelucci–Foufou's **witness configuration** is a *generic
configuration* probing rigidity/dependence (it answers "is the fiber
0-dimensional?" at the diagnosis stage). The two roles are disjoint;
readers versed in GCS will misread GQ1 without this paragraph. Where
this doc needs the other notion it says "generic configuration".

The lit-review taxonomy in one line: shipping interactive solvers do
**seed-continuation** (previous solution seeds Newton; branch =
whatever basin you were in — SolveSpace documents "the initial
position of the sketch determines which [solution] is chosen", and
its official mitigation for flips is to drag slower). That is exactly
the history-dependence GQ1 bans. D-Cubed's documented no-hysteresis
contract (recorded chirality choices) and van der Meiden's
prototype-nearness are the stored-selection-data school — our school —
and interval-Newton certificates are how a selection becomes
*provable*. GQ1 = stored data + certified box; purity makes
no-hysteresis definitional rather than engineered.

## W1 — The witness is the committed solved assignment

```
SketchWitness = the full solved coordinate assignment (f64, raw
                kernel units) recorded at the last committed sketch
                edit, together with the params it solved under.
```

- **Why the assignment and not per-subproblem discrete tags** (DCM
  signs, ESM navigation vectors): tags are tied to a decomposition
  plan; when an edit changes the plan's shape there is no canonical
  transport of the tags (the literature is silent — lit §7.3). The
  assignment is plan-independent: any future decomposition can
  consume it. Root-count theory rules out the naive alternative
  (fibers are exponential — Borcea–Streinu ≈4ⁿ — so "index into the
  enumerated roots" is never on the table).
- **Discrete invariants are derived, never stored**: chirality signs,
  tangency sides, arc orientations are *computable from* the
  assignment via named trilean predicates when a subproblem wants
  them. One source of truth; no assignment-vs-tags aliasing.
- **The S2 pin, transferred honestly (T2)**: the edge witness kills
  aliasing by a canonical formula (witness = carrier(mid)) — that
  cannot transfer, because the sketch witness IS the user's choice;
  a canonical formula would erase exactly the information stored.
  The sketch-level pin is: **the witness is the committed solved
  assignment, residual-certified at commit** (D4 ¶2: residuals ≤ ε
  checked when it is recorded), **refreshed only by recorded
  commits** (GQ3 DocEdits — so which-assignment is always answerable
  from the document alone). What S2 actually forbids is a *free*
  witness — any point certifying against a loose contract; here the
  witness is pinned to a specific recorded event, and W2 makes the
  selection it induces provable rather than trusted.
- Persistence: f64 under D9 replay, shortest-round-trip formatting,
  bit-exact save/load (ratified persistence discipline applies
  as-is). Units erased before kernel scalars (GQ5).

## W2 — Selection is certification, not search

`solution(constraints, params, witness)` selects the branch by a
**certified basin test**, never a nearest-root search — there is no
nearness metric anywhere in the semantics:

1. Deterministic f64 Newton-family iteration from the witness
   assignment (libm-only, fixed iteration order, D9-clean) produces
   candidate x̃. Convergence tests are NOT topology-deciding (the
   ratified stance); nothing about x̃ is trusted yet.
2. **The certificate**: a Krawczyk / Hansen–Sengupta containment on
   an ε-inflated box **X** enclosing both the witness segment and x̃
   (Rump ε-inflation; operators per the lit review §3).
   `K(X) ⊆ int X` proves — in one computation — (i) exactly one root
   in **X** (existence + uniqueness), and (ii) the interval Jacobian
   is regular throughout **X** (no fold inside the box). (i) is
   provably-unambiguous branch selection: the witness lies in the
   certified uniqueness region of the returned root, so "the branch
   containing the witness's basin" is discharged constructively.
   (ii) is the **rigorous distance-to-singularity margin** — the
   certificate and the margin are the same computation.
3. **The margin predicate is a named k_stats trilean** —
   `solver_branch_margin` — whose sign is the certificate outcome
   and whose margin is the regularity bound (T3 discharged: branch
   identity is decided by this predicate, never by where the f64
   iterate happened to land; a small margin escalates instead of
   silently selecting). At T = Interval the same containment IS the
   ratified contraction-from-f64-witness — the interval lane proves
   what the f64 lane selected (Q-h: `solution()` is f64-only; the
   interval lane contracts, it does not solve — stated as ratified
   direction implies).
4. Certificate refusal (containment won't fire after the bounded
   inflation schedule) is TYPED, never a retry loop: near-fold
   (genuine bifurcation proximity) and far-jump (box had to grow
   until it clustered roots) both land in W3's error with their
   distinguishing evidence. The ratified "large-jump escalates on
   the same margin" falls out: a stale witness far from every root
   inflates into ambiguity and refuses.

Deterministic-parallelism note: any parallel residual/Jacobian
evaluation cites D9's two sanctioned idioms; the certification math
is fixed-shape by construction. The solver lives where SSI marching
lives — kernel numeric machinery under D4 certification, OUTSIDE the
total-by-charter expression language (T5, stated).

## W3 — `WitnessBifurcation`: the typed refusal

```
WitnessBifurcation {
  kind:        FoldProximity | AmbiguousBasin | ResidualFailure,
  margin:      the solver_branch_margin evidence (certified bound),
  implicated:  constraint/entity set from the near-nullspace of the
               interval Jacobian (highlightable in an editor),
  witness_age: params the witness solved under vs. params now,
}
```

- Vocabulary joins the ratified two-layer DOF diagnosis: this error
  is layer 2 ("degenerate/ambiguous configuration"), NEVER reported
  as over/under-constraint (layer 1, structural, float-free) and
  never as "solver didn't converge" (banned as a diagnosis).
- NAMING-DESIGN N5's reserved `Diagnosis::WitnessBifurcation` arm
  carries exactly this payload when a stable-name resolution fails
  through a sketch node; the verdict-vector diff machinery attaches
  the flipping `solver_branch_margin` instance (shared with the
  SetTolerance audit, as ratified).
- Constructive upgrades (M10-era, recorded not promised): the
  Moore–Spence bordered system makes the fold point itself the
  solution of a regular square system — so distance-to-fold in
  parameter space can be computed and even certified when we want
  the margin in P rather than in configuration space; discriminant-
  chamber language ("params left the witness's chamber") is the
  clean spec the error text should use.
- K telemetry: `solver_branch_margin` is the first genuinely
  ill-conditioned predicate family the K funnel will see (T6) —
  K-REPORT's "K rarely binds" evidence is all well-conditioned
  construction, so an in-band landing here is exactly K-REPORT's
  stated re-open trigger (#89 CLOSED — K = 10 is the permanent
  ratified default).

## W4 — Witness update policy: commits only, repair explicit

The stored witness changes at exactly one kind of event: a
**committed sketch edit** (ratified). Decisions this leaves open,
now pinned:

- **Parameter-edit rebuilds NEVER write back the witness.** Automatic
  write-back would be seed-continuation smuggled through the
  document — the witness would follow the edit path, and replaying a
  parameter sequence would differ from jumping to its endpoint
  (hysteresis, exactly what purity bans). Successive parameter edits
  all resolve from the same committed witness; a parameter far from
  the witness's chamber escalates honestly (W2.4).
- **The repair is an explicit `ReWitness` DocEdit** — recorded,
  replayable, undoable — adopting the current certified solution as
  the new witness (the direct analogue of NAMING-DESIGN N5's
  explicit `Rebind`; same trivial-or-loud ethos).
- **Certified-same-branch ReWitness is semantically invisible, so it
  may be automated in bulk (Evan, in-conversation on this PR)**: when
  the W2 certificate proves old witness and new solution lie in one
  uniqueness region (same branch), rewitnessing changes NO predicate
  outcome — it only recenters the stored point, improving future
  margins and shrinking future boxes. The W4 ban is therefore
  precisely on *silent/unrecorded* write-back, not on automation: an
  editor SHOULD bulk-ReWitness (recorded, e.g. piggybacked on the
  commit DocEdit) after certified-clean edits, and MUST ask only
  where the certificate refuses — the ambiguous cases W3 types.
  Purity is preserved because the recorded document, not the edit
  path, remains the sole input to `solution()`.
- **The user's drag path is legitimate UI input** for *authoring* the
  ReWitness proposal: purity bans path-dependence in the semantics,
  not in what the UI suggests. A drag is a user-supplied homotopy;
  if it stays fold-free (see the GUI-DESIGN "UI ideas" note: a
  default drag mode that refuses to cross the wall where
  `solver_branch_margin` → 0, with an explicit modifier key to
  cross), the homotopy uniquely identifies the endpoint branch, and
  the drag-end ReWitness needs no dialog at all. An explicit
  wall-crossing keypress is itself the recorded disambiguating
  intent.
- Undo/redo and `SetTolerance` replays therefore need no special
  cases: the witness is ordinary recorded document state under GQ3.

## W5 — Composition with the result DAG

The sketch node's result value carries the certified solved
assignment (it is what downstream consumers — profiles, MappedCurves,
future mates — read); `WitnessBifurcation` is a per-node failure
poisoning descendants only (GQ2 verbatim). Solve results are derived
artifacts under content-keyed caching: keyed by the bit-content of
(constraints, params, witness), transfer-by-equality applies — and
because `solution()` is pure in exactly those three, the cache key
is the correctness proof, the same argument as everywhere else in
D9-land.

## W6 — Certification schedule

A check family parallel to `WitnessMidpoint`, at the same kind of
gate:

- At **commit** (witness recording): residuals ≤ ε on the committed
  assignment (D4 ¶2 form, typed `ToleranceExceeded` on failure).
- At **replay/solve**: W2's certificate (the containment) — its
  failure is W3's typed error, its success certifies both selection
  and margin.
- At the **interval lane**: the ratified contraction-from-witness is
  W2 step 2 run at T = Interval; an indeterminate joins the
  subdivision-driver posture like every other interval refusal.
  Residual evaluation obeys the interval-square hygiene (powi(2),
  never x·x, for possibly-zero quantities).

## W7 — ezpz audit criteria and the fallback

The Q3 adoption decision (ezpz at M10) gets its bit-identity audit
criteria pinned now: (i) libm-only transcendentals (no platform
`std::f64` drift), (ii) no iteration order derived from hash maps or
pointer identity, (iii) bit-identical solve results across two
independent builds and across ≥2 platforms in a CI harness, (iv) a
deterministic, documented iteration/termination schedule. **Fallback
if the audit fails**: ezpz demotes to *seed proposer outside
`build`* (interactive previews may use it freely — preview is
allowed to degrade δ, never ε, and its output is laundered through
the commit-time residual certification anyway), while the in-`build`
path uses our own small deterministic Newton polish from the witness
— W2's certificate carries correctness either way, which is what
makes the solver-engine choice low-stakes: **certification, not the
iterator, is the contract** (the round-4 "solver output demoted to
witness" ratification, mechanized).

## W8 — SE(3)/mates: contract verbatim, mechanism per-manifold

T1 resolved by splitting what GQ4 transfers: the **contract**
(witness = branch selection; purity; certified selection; typed
bifurcation with margin) transfers verbatim to mates. The
**mechanism** is manifold-specific and NOT designed now — budgeted
per the ratified SE(3) flag. What this doc pins so M4 recipes don't
preclude it: the witness datum type is per-node and opaque at the
document layer (a sketch node stores an ℝⁿ assignment; a mate node
will store points of ∏SE(3)); the certification interface is
"certified-unique in a chart-box centered at the witness", with the
chart a deterministic function of the witness (exp-map chart at the
witness; quaternion double-cover quotiented; chart radius vs.
injectivity radius argued when the mate solver is designed —
chart-centered Krawczyk is sound but unpublished, flagged as our
proof obligation, lit §5/§7.5).

## W9 — Structural layer: out of scope, interface pinned (restated)

The two-layer DOF diagnosis is ratified: layer 1 (combinatorial
DOF/decomposition analysis — exact, float-free; no Rust DCM exists,
ours to build at M10) is NOT this doc. Interface assumption recorded:
the decomposition plan is a pure function of the constraint graph
(generic, parameter-independent — DR-planning's own property), so
layer-1 outputs may key caches but never consult coordinates;
generic-configuration rigidity probes (the *other* witness) live
entirely in layer 1.

## Worked example: the elbow

The two-bar linkage, smallest system with every phenomenon. A=(0,0)
and B=(d,0) fixed; C with |AC| = r₁ = 8, |BC| = r₂ = 6. Two branches:
elbow-up C⁺ and elbow-down C⁻ (reflections across AB). The fold locus
is r₁ + r₂ = d (elbow straightens, C⁺ and C⁻ merge); beyond it, no
real solution.

- **Commit (W1)**: the user drags C up and commits. Witness = the
  full assignment with C at (x, +y), residual-certified, recorded
  with d = 12.
- **Typed edit d → 13 (W2, the common case)**: Newton from the
  witness converges to elbow-up at the new params; the Krawczyk box
  around witness-and-candidate certifies one root + regular Jacobian.
  Margin healthy (the elbow is far from straight). Selection proved,
  zero interaction. If the editor bulk-ReWitnesses on the clean
  certificate (W4), the stored point recenters — semantically
  invisible.
- **d → 13.999999999 (W3, FoldProximity)**: the certificate margin
  enters the sliver band — the elbow is within ε-scale of straight.
  Typed `WitnessBifurcation { FoldProximity }` naming the two
  distance constraints as implicated. This is genuinely
  ill-conditioned geometry; asking is correct.
- **d → 15 (infeasible)**: no real solution (r₁ + r₂ < d); residual
  certification cannot pass; typed infeasibility, layer-1 vocabulary
  untouched (this is not "over-constrained", it is "no real
  configuration at these params").
- **d → 15, then back to 12 (no hysteresis)**: the stored witness
  was never rewritten by the failed excursion, so d = 12 reselects
  elbow-up, bit-identically. The seed-continuation contrast: a
  solver seeding from "wherever the sketch last was" can return from
  this excursion elbow-DOWN depending on numerical luck — the
  documented SolveSpace flip.
- **Wall-mode drag (the GUI-DESIGN idea)**: dragging C toward the AB
  line, the preview solver's margin shrinks; at the wall
  (`solver_branch_margin` → 0, the straightened elbow) the dragged
  point sticks, with the wall rendered. Drag ends → drag-end
  ReWitness, no dialog (the fold-free homotopy proved the branch).
  With the modifier key held, the user pushes through the wall: the
  explicit keypress records the intent, and the far side resumes on
  the mirror branch — a chosen flip, never a silent one.
- **AmbiguousBasin, for completeness**: a large typed jump in a
  near-symmetric sketch can leave the old witness between two
  well-separated roots — box inflation swallows both, the
  certificate refuses, and the error carries the candidate set for
  an explicit choice (Bidarra's doctrine, unchanged).
- **Downstream (W5/N5)**: an extrude of this sketch poisons (typed,
  descendants-only) while the sketch node carries
  `WitnessBifurcation`; a stable name referencing an edge born of
  C's position diagnoses its resolution failure with the
  `solver_branch_margin` flip via the shared verdict-diff machinery.

**How often do the asking cases fire?** Structurally: never inside a
chamber (the certificate fires and margins are healthy — folds are
codimension-1, and typical edits stay far from them); the asking
cases concentrate at (a) genuinely near-degenerate geometry, where
asking is honest, and (b) large typed jumps in root-crowded sketches.
Wall-mode drags eliminate the drag-borne cases by construction. The
honest quantitative answer is empirical and the instrument is already
built: `solver_branch_margin` sits in the k_stats funnel, so the M10
corpus measures exactly this distribution (the T6 obligation — the
first genuinely ill-conditioned predicate family K will see).

## Open after this doc

- The M10 implementation surface: operator choice (Krawczyk vs
  Hansen–Sengupta by cost profile), the ε-inflation schedule
  constants, bordered-system fold localization, the layer-1
  decomposition design, and the mate-solver mechanism (W8's proof
  obligation). None are M4 blockers.
- The chamber-map upgrade (precomputed parameter-range intervals per
  van der Meiden — closed-form fold distances for decomposable
  steps) as a cheap early-warning margin — banked as an M10 option.
