# GQ1 mechanism details: witnesses, branch selection, bifurcation (pre-M4 design doc)

Status: **PROPOSED** (design conversation; W-decisions ratify on
Evan's sign-off). This is the second and last "before M4 planning"
doc (DESIGN.md; the first was NAMING-DESIGN.md, #74). Grounding:
`references/notes/gq1-constraint-inventory.md` (ratified constraints
quoted; tensions T1–T6) and `references/notes/gq1-solver-litreview.md`
(branch-selection taxonomy A–D; certified-selection toolbox;
bifurcation theory; SE(3)). GQ1's ratified core is NOT reopened here:
witness = authoritative branch selection; `solution(constraints,
params, witness)` pure; continuation along the edit path banned;
bifurcation = typed error with distance-to-singularity margin;
witness refreshes at committed sketch edits; interval replay runs
interval-Newton contraction seeded from the f64 witness. This doc
pins the mechanism those sentences left open. Implementation is
M6-era (sketch solver); M4 needs the contracts (witness datum in
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

## W1 — The witness is the committed solved assignment (proposed)

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

## W2 — Selection is certification, not search (proposed)

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

## W3 — `WitnessBifurcation`: the typed refusal (proposed)

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
- Constructive upgrades (M6-era, recorded not promised): the
  Moore–Spence bordered system makes the fold point itself the
  solution of a regular square system — so distance-to-fold in
  parameter space can be computed and even certified when we want
  the margin in P rather than in configuration space; discriminant-
  chamber language ("params left the witness's chamber") is the
  clean spec the error text should use.
- K telemetry: `solver_branch_margin` is the first genuinely
  ill-conditioned predicate family the K funnel will see (T6) —
  K-REPORT's "K rarely binds" evidence is all well-conditioned
  construction; the M6 corpus must re-examine K here.

## W4 — Witness update policy: commits only, repair explicit (proposed)

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
  explicit `Rebind`; same trivial-or-loud ethos). An editor MAY
  offer it proactively when `solver_branch_margin` shrinks toward
  the band; the offer is UI, the edit is the semantics.
- Undo/redo and `SetTolerance` replays therefore need no special
  cases: the witness is ordinary recorded document state under GQ3.

## W5 — Composition with the result DAG (proposed)

The sketch node's result value carries the certified solved
assignment (it is what downstream consumers — profiles, MappedCurves,
future mates — read); `WitnessBifurcation` is a per-node failure
poisoning descendants only (GQ2 verbatim). Solve results are derived
artifacts under content-keyed caching: keyed by the bit-content of
(constraints, params, witness), transfer-by-equality applies — and
because `solution()` is pure in exactly those three, the cache key
is the correctness proof, the same argument as everywhere else in
D9-land.

## W6 — Certification schedule (proposed)

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

## W7 — ezpz audit criteria and the fallback (proposed)

The Q3 adoption decision (ezpz at M6) gets its bit-identity audit
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

## W8 — SE(3)/mates: contract verbatim, mechanism per-manifold (proposed)

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
ours to build at M6) is NOT this doc. Interface assumption recorded:
the decomposition plan is a pure function of the constraint graph
(generic, parameter-independent — DR-planning's own property), so
layer-1 outputs may key caches but never consult coordinates;
generic-configuration rigidity probes (the *other* witness) live
entirely in layer 1.

## Open after this doc

- The M6 implementation surface: operator choice (Krawczyk vs
  Hansen–Sengupta by cost profile), the ε-inflation schedule
  constants, bordered-system fold localization, the layer-1
  decomposition design, and the mate-solver mechanism (W8's proof
  obligation). None are M4 blockers.
- Whether `ReWitness` should ever be offered automatically in bulk
  (e.g. after a large certified-clean parameter change) — UI-policy
  territory, revisit with editor-core experience.
- The chamber-map upgrade (precomputed parameter-range intervals per
  van der Meiden — closed-form fold distances for decomposable
  steps) as a cheap early-warning margin — banked as an M6 option.
