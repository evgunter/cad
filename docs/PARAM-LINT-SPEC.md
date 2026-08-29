# PARAM-LINT — the parameter-coincidence lint (unit spec)

**Status: DRAFT — design conversation, awaiting Evan's sign-off.**
The next resident of the DISCIPLINES registry (DS5's grade-3 lint;
DS8's named next unit) specified against the real Expr substrate
(surveyed 2026-08-25, refs inline). This spec proposes resolutions
to DS-Q2 and to the dial-default question; those arms are marked.
Schedules nothing; the unit dispatches when Evan signs off.

## Grounding (survey facts the spec is built on, not choices)

- The expression sublanguage is `editor-core/src/expr.rs`: total by
  construction (no conditionals/iteration/user functions), dimension
  vocabulary {Length, Angle, Count, Scalar} with Count as D8's
  structural axis. **There is no sharing**: `Box<Expr>` trees, no
  Arc/interner/id-table — "both read `width/2`" is only ever two
  structurally-equal copies. The one sharing device in the language
  is `ExprKind::Param(ParamName)`, resolved against
  `Doc.params: BTreeMap<ParamName, DocParam>` at eval.
- `DocParam::Continuous { dim, value: f64 } | Count { value: i64 }`
  (doc.rs:38); nodes reference by name only; dimensions re-checked
  at every edit. Units are per-literal display metadata excluded
  from equality, keys, and evaluation — the lint compares
  dimensioned kernel-unit values and never reconciles mm-vs-in.
- Exact comparators exist (`Expr::bit_eq`, `DocParam::bit_eq`); no
  normalization/simplification/canonical-form pass exists anywhere.
- **Exprs do not enter content keys** (keys feed evaluated slot
  values, eval/slots.rs:4-6) — a lint reading them cannot perturb
  memoization, naming, or replay; only its own output ordering owes
  D9 (fix it: `BTreeMap` order over canonical name pairs).
- `eval<T: Decide>` is already scalar-generic (expr.rs:966); the one
  f64 pin is profile-program slot resolution (the C6 pin,
  eval/slots.rs:27-30).
- Edits: `SetDocParam` is create-or-replace only — **no parameter
  delete, no rename**; the refactoring pattern is elaboration
  (pure `Vec<DocEdit>`, caller applies all-or-nothing —
  `update_references`, refactor::split/inline precedents).
- Schema: a new persisted `Doc` field = one strict serde module +
  one field + a ledger paragraph at whatever the NEXT schema version
  is when the change lands (`SCHEMA_VERSION` clean-break mechanics,
  persist/mod.rs; `tests/schema_ledger.rs`). No number is reserved
  here: a version is claimed by an explicit read of main's constant at
  the claiming branch's final re-merge, never by a spec written
  earlier.

## PL1 — Scope v1: the parameter-table diagonal (a sharpening, not
## just base-rate control)

DS5 scoped v1 to "named parameters only" for signal-to-noise. The
substrate makes that the *semantically correct* scope, not merely
the pragmatic one: every derived expression evaluates through the
same `ParamEnv`, so **derived expressions comove under M10
automatically** — two structurally-distinct exprs that are equal as
functions of the parameters vary together with no declaration
needed. The independence-vs-comoving question that M10 actually
asks lives at the **leaves**: the diagonal {p = q} in `Doc.params`.
Same name ⇒ comoving; distinct names ⇒ independent marginals; the
lint's job is exactly to force the distinct-names-equal-values case
to say which of those it means.

Derived-expression coincidences (two slots' exprs structurally
distinct yet definitely equal at nominal) therefore matter only for
the edit-desync story, not for stackups, and are the recorded **v2
arm** — with two named costs when it comes: `ExprPath` findings are
positionally keyed and can silently re-point after a same-slot edit
(expr.rs:840-848), and profile-program slots resolve at the f64 pin.
v1 ships nothing there.

**Designed for future Expr sharing (Evan, 2026-08-25: sharing is
presumed coming; the design anticipates it).** When shared subterms
land (Arc/interned exprs), the v2 arm's rung ladder is already
shaped for them: a **shared node** is the structural rung — one
term, intent by construction, the PATHS-constructor analog — and
**equal-but-unshared copies** become the reporting rung with *share
the subterm* as the unify-analog menu arm (today's copies are
family-equal, so M10 is indifferent; the desync risk under editing
is what the arm repairs). Nothing in v1 keys on tree identity, so
sharing changes no v1 behavior: named parameters remain the
declared marginals and the M10 leaves after sharing exactly as
before (a shared subterm is an anonymous comoving intermediate, not
a marginal), and `DistinctRecord`'s `ParamName` keying is
sharing-proof.

## PL2 — Predicates and rungs (all exact; no funnel site in v1)

Candidate pairs: unordered pairs of distinct `ParamName`s of the
**same dimension** (comparing across dimensions is
dimension-laundering; Continuous-vs-Count never compares).
Canonical order: lexicographic name pairs, `BTreeMap` iteration —
D9 by construction.

- **Coincident**: `Continuous` pairs with equal `dim` and bit-equal
  `value`; `Count` pairs with equal `i64`. Exact — no ε, no
  `k_stats` site, nothing decided (the DS3/DS-Q3 posture: exact
  rungs stay out of the funnel). This is deliberate and is the
  whole v1 predicate.
- **Near-equality has no arm** (recorded lean, open to reversal):
  two params within ε_input of each other but not equal is a
  different smell (a modeling-tolerance question, not a stratum
  question), and giving it an arm would mint the lint's first
  decided predicate. If demand arrives, it enters as a separate
  finding class with its own funnel site.

Rungs, in the flush-mold vocabulary (`FlushRung` precedent —
names/flush.rs):

1. **structural** — the pair is one name (nothing to compare) or
   the sites share the expression verbatim: not in scope at the
   table level (distinct names are distinct leaves by definition);
   listed for the v2 expression arm, where structural equality of
   trees IS family-level equality (identical trees over the same
   params are equal at every assignment — the point-vs-family
   principle realized structurally).
2. **declared-distinct** — a recorded disavowal: "equal by
   coincidence, keep independent" (PL3).
3. **refusing/reporting** — coincident, no record: the finding,
   carrying both names, the shared value, and the two-arm menu.

**DS-Q2, proposed resolution (ratifies the recorded lean):** with
no normalizer in the substrate, `width/2` vs `0.5*width` (the v2
expression arm's case) lands in the reporting rung with *unify the
spelling* as the menu arm; the structural rung does not extend to
symbolic equivalence. Cheap to revisit if a canonicalizer ever
exists; the substrate argues it should not exist for this purpose.

## PL3 — Vocabulary, persistence, and the two repairs

**Declared-distinct is data; declared-same is a refactoring.**

- *Declared-distinct*: a document-level store
  `Doc.param_relations: BTreeMap<(ParamName, ParamName), DistinctRecord>`
  (canonical name order; record carries provenance per DS7's
  ladder). Document-level, not node-level, because the relation is
  about the table's leaves, not any consuming node — the
  `AppearanceMap` precedent. Persistence: strict serde module +
  a ledger paragraph at the next schema version, claimed the way the
  bullet above says. `Doc::bit_eq` and `diff.rs` each gain a
  clause.
- *Declared-same (unify)*: no record — the repair rewrites the
  document so the coincidence becomes structural: elaboration-style
  pure function returning `Vec<DocEdit>` (`SetExpression` replacing
  `Param(q)` with `Param(p)` at every referencing site, found via
  the `node_param_refs` walk), caller applies all-or-nothing;
  dimension re-validation comes free from the edit layer. **Small
  co-requisite**: a `DocEdit::RemoveDocParam` (with a
  still-referenced refusal), since today no parameter delete
  exists and unify would otherwise strand `q` as a live-but-unused
  leaf that the lint itself then stops seeing (an unused equal
  param is still an M10 marginal — stranding it is a lie by
  omission).

**Two-directional certification from day one** (the round-4
lesson, not repeated): every `DistinctRecord` must be consumed by a
live coincident pair —

- a record naming a dead `ParamName` is stale (the dead-key class);
- a record whose pair's values have **diverged** is stale in the
  vacuous direction: the disavowal is about nothing. Reported as
  prunable, never an error (a disavowal cannot be contradicted —
  there is no geometric claim in it; staleness is its only failure
  mode).

Finding type in the flush mold (rung enum + evidence + Display with
one story and the two-arm menu), rendered through the shared sink
(#981 part 1 when it lands; locally until then).

## PL4 — Doors and the dial

- **Report door** (always available, the `mixed_pins` posture —
  reports, never gates, called by nothing in apply/load/evaluate):
  `parameter_coincidences<P>(doc: &Doc<P>) -> Vec<ParamCoincidenceFinding>`
  — pure over the Doc, no Evaluation parameter (a recipe-layer
  discipline never needs one; this is what makes it grade 3 and not
  a `CheckId`), re-exported via `pncad::document`.
- **Enforce door** with the grade-3 dial: `require` refuses
  reporting-rung findings and stale records; `ignore` doesn't. The
  `auto-record` middle position is **deferred with DS-Q5** — its
  diff-basis/acknowledgment machinery is an editor-session design
  pass this unit should not improvise.
- **Dial default, proposed: `ignore`** (the report door always
  answers when asked). Ground: `require` is the ratified posture
  for kernel disciplines whose absence breaks construction; this
  stratum breaks nothing — its consumers are M10 and edit-
  robustness — and defaulting a brand-new lint to refusing would
  churn every existing document with coincidentally-equal params
  before anyone has declared anything. Flagged for Evan: this is
  the one place the spec chooses leniency over the house default.

## PL5 — The DS1 scaffolding obligation (#981 part 2)

This unit is where the grade-3 discipline machinery materializes —
built as shared shape, not lint-local: the rung-enum/evidence/
finding pattern, the two-directional record certification walk, and
the declare/undeclare edit vocabulary should be written so the
right-angle discipline (DS4, reserved) can instantiate them without
re-implementation. The predicates stay local per DS1. What must NOT
be built speculatively: verify-table machinery for geometric
contradiction (this discipline has none — a disavowal is
uncontradictable), the auto-record diff basis (DS-Q5), and any
funnel plumbing (v1 has no decided predicate).

## PL6 — The M10 contract (recorded for ERROR-DESIGN's consumer)

Same name ⇒ one marginal, comoving everywhere it is referenced.
Distinct names ⇒ independent marginals, **whether or not declared**
— the `DistinctRecord` adds no semantics to evaluation; it records
that the independence at an observed coincidence is intended, so
the lint stays quiet and a future reader (human or stackup report)
sees intent rather than accident. Derived expressions need nothing:
they comove through evaluation. This is why the record can never be
load-bearing in a build — the DS3 invariant holds for the whole
unit by construction.

## PL7 — Sizing, sequencing, acceptance

Sizing **M** (one PR): store + serde + ledger + bit_eq/diff
clauses; report door + enforce door; unify elaboration +
`RemoveDocParam`; findings/Display; tests (table-driven — equal
pairs report, declared-distinct silences, dead-key and diverged
records stale, unify elaboration round-trips and re-validates,
determinism, Count and Continuous arms, cross-dimension pairs never
compare); a small demo exercise through `pncad`. Sequenced after
#981 part 1 (the sink) if that has landed, locally otherwise —
soft ordering, not a gate.

Review claims to falsify: (1) no ε anywhere — grep-level and
behavioral; (2) DS3 — no config/record changes any evaluated body
or any content key; (3) the unify elaboration is atomic-by-purity
and leaves no stranded references at any failure point; (4) stale
detection is two-directional on the reviewer's own fixtures;
(5) report determinism cross-process; (6) e2e through pncad as a
first-time user.

## Open for Evan (beyond the two proposals marked above)

- **PL-Q1**: dial default `ignore` (PL4) — confirm or override.
- **PL-Q2**: DS-Q2 resolution as proposed (no symbolic rung) —
  confirm; ratifying it updates DISCIPLINES-DESIGN's ledger.
- **PL-Q3**: is `RemoveDocParam` acceptable scope-riding, or its
  own micro-unit first?
- **PL-Q4**: near-equal params staying arm-less (PL2) — confirm.
