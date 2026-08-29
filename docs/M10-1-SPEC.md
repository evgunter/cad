# M10-1 — distributions in the document (unit spec)

**Status: BINDING at dispatch** (orchestrator-authored; E1/E2 are
ratified design — `docs/ERROR-DESIGN.md` — and this spec adds no
open questions). Branch `m10/m10-1-distributions`. Sizing **M**
(one PR). Read `docs/prompts/implementer-discipline.md` in full
before starting.

## Grounding (substrate facts, verified 2026-08-29)

- `DocParam` is `editor-core/src/doc.rs:38`:
  `Continuous { dim, value: f64 } | Count { value: i64 }`, strict
  serde with `deny_unknown_fields`, exhaustive-on-purpose `bit_eq`
  (`doc.rs:72` — a new field must be spelled there or the compile
  breaks, which is the point).
- `DocEdit::SetDocParam { name, value }` (`edit.rs:77`) is
  create-or-replace; its apply arm (`edit.rs:1072`) already refuses
  `Continuous` with `dim: Count` and non-finite values, and
  re-validates every referencing slot.
- `SCHEMA_VERSION = 14` (`persist/mod.rs:374`); the migration table
  is EMPTY BY RULING (LQ7a; re-confirmed at plan ratification, Q4)
  — every bump is a clean break refusing typed with the regenerate
  recourse. Ledger paragraphs + `tests/golden/v*_golden.cad` +
  `tests/schema_ledger.rs` are the pattern. The version claim is
  race-prone across programs: take 15 by an explicit by-eye read of
  main's constant at the final re-merge (the v11 three-way-race
  lesson, recorded at `persist/mod.rs`'s v14 paragraph).
- `libm` is the workspace transcendental source (D9); `libm::erf`
  exists — no new dependency.
- PL6 (`docs/PARAM-LINT-SPEC.md`): same name ⇒ one comoving
  marginal; distinct names ⇒ independent; derived expressions
  comove through evaluation and need nothing.

## Scope

### 1. The `Distribution` vocabulary (E2 verbatim)

New type beside `DocParam` (editor-core; own module or `doc.rs`,
implementer's call):

```
Distribution = Band            { lo, hi }
             | Uniform         { lo, hi }
             | Normal          { sigma }
             | TruncatedNormal { sigma, lo, hi }
```

All fields `f64`, offsets **relative to the parameter's nominal**,
in canonical kernel units of the parameter's own `dim` (no display
units, no separate dimension field — the param's `dim` rules).
Strict serde, `deny_unknown_fields`.

`DocParam::Continuous` gains `distribution: Option<Distribution>`.
`Count` gains nothing — E11.3's "no distributions on structural
parameters" comes out **unrepresentable**, the house preference;
no refusal code path exists because no spelling exists.

### 2. Validation (typed, at every construction door)

Invariants: every field finite; `sigma > 0` where present;
`lo <= 0 <= hi` for the bounded forms (asymmetric legal; this IS
E2's nominal-inside-support rule, by representation). Enforced:

- at the edit door — new typed `EditError` variants in the
  `SetDocParam` arm, alongside the existing non-finite check
  (extend it to distribution fields);
- at the persistence boundary — join the shared-validator walk the
  non-finite check uses, so a hand-written v15 file with
  `sigma: -1` refuses at LOAD with the same diagnostics save
  refuses with. Never a silent best-effort load.

### 3. Equality, diff, persistence

- `bit_eq`: distribution compares present-vs-present bit-exact on
  its floats (`to_bits`), exhaustive match, both sides spelled.
- `diff.rs`: the params clause reports distribution changes (a
  param differing ONLY in distribution is a reported diff).
- **Schema v15, clean break**: ledger paragraph stating the format
  claim (a v14 reader handed a populated `distribution` dies on an
  unknown shape; a v15 file with all-`None` distributions is the
  degenerate carry), golden `v15_golden.cad` WITH a distribution
  populated on at least one param (all four forms across the
  corpus tests), `schema_ledger.rs` row, v14 refusal row.

### 4. The projection module (E1's three consumables, minus seeds)

New module `editor_core::analysis` (the analysis lane per E1: the
kernel and geometry lanes never see a probability; this module is
the one place distributions are read). Derived values only —
nothing here persists.

- `AnalysisPolicy { quantile_mass: f64 }` with a named default
  const (**0.9973 per parameter — the ±3σ convention**; E2's
  recorded policy dial). Request config, not a global.
- `analyzed_box(doc, &policy)`: per continuous param WITH a
  distribution, the bounded offset interval — the support for
  Band/Uniform/TruncatedNormal; the symmetric quantile interval
  `±z·sigma` for Normal, z derived from `quantile_mass` by
  **monotone bisection on `libm::erf`** (deterministic, no new
  dependency; its convergence error only moves mass between the
  analyzed and tail columns, never truth — E2). A param with NO
  distribution is FIXED: width-zero interval at nominal, mass 1
  (distributions are opt-in; the analysis varies exactly what the
  user declared variable — state this in the module docs, it is
  this unit's one elaboration of E1).
- `tail_mass(dist, analyzed_interval) -> f64`: Normal via erf;
  identically 0 for the bounded forms when the box covers the
  support; general sub-box handled (box intersect support).
- `box_mass(dist, sub_interval) -> Result<f64, MeasureUnavailable>`
  — the leaf-pricing door E6 will consume. **Band refuses typed,
  naming the parameter** (E2: "I know the limits but not the
  shape"; uniform is a different, stronger claim — never
  defaulted).
- Module docs carry the PL6 statement (independence semantics) and
  E1's boundary sentence (no probability below this module).

### 5. Out of scope (stated so nobody reads more)

Seed vectors (M10-4's); the driver (M10-3's); any kernel/geometry
crate change; any new node kind; Monte Carlo; display units on
distribution fields; `Joint` forms (E11.2).

## Review claims to falsify (handed to the dual reviewers)

1. **Zero evaluation impact**: no eval memo/content key changes; a
   distribution edit invalidates no memoized evaluation; every
   existing corpus document evaluates bit-identically at f64 and
   Interval with and without this diff.
2. **No ε anywhere** — grep-level and behavioral: distributions
   are exact data; the quantile bisection is reporting-lane f64
   with no Q1 predicate, no funnel site, no decided margin.
3. **Refusal completeness**: every construction path (edit door,
   load door, any test back door) enforces §2's invariants; the
   load door refuses a corrupt file with save-door diagnostics.
4. **Accounting honesty**: analyzed + tail masses sum to 1 (within
   stated f64 bounds) for every form × box combination the
   reviewer can construct; TruncatedNormal's tail is exactly 0 on
   its own support; Band prices nothing anywhere.
5. **Schema exactness**: v14 refuses typed with the recourse; v15
   goldens round-trip bit-exact; `bit_eq`/`diff` see the new field.
6. **e2e**: author a two-param document through `pncad` with a
   Normal and a Band, save, reload, read the analyzed box and tail
   mass as a first-time user; report the friction.

## Acceptance

Hosted CI green on the unit's own head (the schema suites and the
editor-core battery are the likely draws; state in the PR which
point the gate drew). Local scope per the standing calculus:
editor-core suite at default ε plus the interval lane if any
scalar-generic surface moved (none should). The PR body carries
the sweep-shape disposition for the exhaustive-match sites
(`bit_eq`, diff, strict modules) — one line per site the compiler
forced.
