# M4 PR 4 binding spec — Naming part 2: resolution + the diff engine

Binding for the PR 4 implementer. Scope sentence: M4-PLAN item 4.
Ratified ground: NAMING-DESIGN N5/N7 (verbatim contracts), SOLVER-DESIGN
W1–W9 (contracts only — no solver implementation), DESIGN D8/D9.
Deviations use the REPORT mechanism (deviate → report prominently →
orchestrator rules at review).

## D1 — ResolveError/Diagnosis exactly as N5 wrote them

The N5 block is normative, not indicative:

- `ResolveError = Vanished{name, diagnosis, last_good: Option<Tombstone>}
  | Ambiguous{name, candidates, tie: TieWitness}
  | NodeGone{name, edit: RecipeEditRef}`.
- `Diagnosis = PredicateFlip{predicate, from, to} | StructuralParam{node,
  param} | RecipeEdit{edit} | Cascade{through}`.
- `Ambiguous` must interoperate with PR 3's `Tied` table entries and the
  documented `order_along` over-tie (a tie anywhere in the group widens
  candidates — never mis-binds; the candidates list IS the tied set).
- `Tombstone` carries entity kind, owning body name, and the mesh patch
  key of the last evaluation (GQ7 ghost-rendering payload). Selection
  state holds name + tombstone, never an arena key.
- Typed all the way: `ResolveError` is data in results, not a panic and
  not a string. Kernel never-panic discipline applies.

## D2 — The verdict-vector diff engine is built ONCE

- One engine, one crate-internal module: input = two evaluations'
  verdict logs (k_stats names + D9 replay guarantee both exist);
  output = the flip set, localizable to derivation paths.
- Consumers in this PR: (a) `Diagnosis::PredicateFlip` attribution for
  Vanished names; (b) `SetTolerance` apply = replay at the new ε +
  structural diff of verdicts, reporting exactly the flipped predicates
  (H4's ε machinery lands here; in-document ε *persistence* is PR 6 —
  build the apply/diff semantics against the ambient-ε mechanism now).
- The engine must not be specialized to either consumer: same call
  signature diffs (old-run, new-run) whether the cause is a parameter
  edit, an ε change, or a recipe edit. PR 6 reuses it untouched.

## D3 — Rebind, tombstones, and the EMPTY auto-menu

- `Rebind(name → selection)` DocEdit: explicit, recorded, replayable.
  Validation at edit time mirrors Declare's D3 carve-out (node
  existence now; name-level resolution at evaluation).
- The automatic-rebinding policy menu ships EMPTY (ratified). No
  "follow the merge", no nearest-neighbor, no silent fallback. Any
  convenience policy is a future ratification.
- Tombstones are created exactly when a name vanishes with a
  last-good evaluation available; they are evaluation artifacts (result
  DAG side), not document state.

## D4 — Hit-testing inversion

- Arena key → StableName over the M2 PR 6 back-refs, total for every
  key the evaluation exposes: the GUI never sees an arena key.
- Inversion is the bidirectional table read PLUS provenance walk where
  needed; it must be total on the corpus (assert in tests) and typed
  (`Unnamed` is a bug surfaced loudly, not an Option::None swallowed).

## D5 — Solver CONTRACTS only (types + document semantics)

- Opaque per-node witness datum on sketch-bearing nodes (GQ1 D1):
  serialized bytes + schema tag; editor-core stores/replays it, never
  interprets it.
- `ReWitness` DocEdit (single) + bulk certified-same-branch allowance
  (W-contract): the bulk form carries the certification obligation as
  data; enforcement arrives with the solver (M6). Shape it so M6 adds a
  checker, not a schema change.
- `WitnessBifurcation` typed error + its N5 Diagnosis arm.
- NO solver implementation, NO constraint evaluation. If a contract
  can't be typed without solver machinery, REPORT — do not stub logic.

## D6 — Banked obligations from PR 3 (all land here)

1. **Single-qualifier-flip localization fixture** (D5 deviation, R5):
   an edit flipping exactly one fragment qualifier (non-adjacent-
   partner configuration); assert exactly the names whose derivations
   pass through it change, counted.
2. **Declare name-level edit-time validation** (R6): upgrade `apply`
   to resolve Declare's StableName pairs against the referenced nodes'
   tables when they are evaluable; keep the documented carve-out for
   forward references.
3. **Vanished-diagnosis vs dropped fused-vertex identity** (R9): the
   kept-key-wins fusion drops the losing operand corner's identity
   with no N3-style retirement row — pin what `Vanished` reports for
   it and that the diagnosis is honest (Cascade or PredicateFlip, not
   a lie).

## D7 — CI invariants extended

- Golden-digest family gains resolution: same recipe + verdicts ⇒
  byte-identical ResolveError/Diagnosis output for the corpus's
  deliberately-broken documents (a small "diagnosis corpus" of edits
  that vanish names).
- f64/Interval agreement extends to diagnosis output.
- ε probes derived from ambient `Tolerance::get()` only (the matrix
  runs 1e-6/1e-12 rows). No hard-coded ε anywhere, tests included.

## D8 — Process

- Persistent clone under `~/.local/share/cad-work/` (never /tmp);
  sccache from first build; NEVER export RUSTFLAGS; commit+push after
  every coherent unit.
- Boolean-of-boolean fixtures stay excluded until issue #86 closes.
- fmt/clippy(-D warnings, default + interval)/workspace/interval lanes
  green before reporting. No gate, no PR from the implementer.

## D9 — Appearance-hook obligations (added post-PR 7 review, 2026-07-24)

- PR 7's `AppearanceLossCause` is the typed hook this PR enriches:
  `Ambiguous{at, ..}` → N5 `Ambiguous{name, candidates, tie}` by
  `table.lookup(name)` at node `at` (Tied entry = candidates);
  `AppearanceLoss::Vanished{candidates}` is a superset of N5's
  `Vanished` shape — N3 promises an offered candidate, N5's struct
  has no field for it; PR 4 resolves the tension by carrying offers
  inside `Diagnosis`/tombstone payload or wrapping (implementer's
  choice, REPORT which).
- **Banked (ruled at PR 7 review, A1)**: the operand→final paint gap
  — an attribute on an operand-node name resolves on the intermediate
  body only after a recipe extension; final-node consumers see
  neither paint nor loss ("resolves-anywhere" success criterion,
  upheld per N1 identity + N5's empty auto-menu). PR 4's resolution
  machinery must make the EXPLICIT repair ergonomic: when Rebind
  targets an appearance-carrying name, the suggestion ladder offers
  the final-node derivations wrapping it (FromA(x) etc.) as
  candidates. No automatic following.
