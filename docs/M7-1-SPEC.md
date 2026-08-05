# M7-1 spec — step-import skeleton + own-corpus round-trip (binding)

Mandate (docs/M7-PLAN.md unit 1): a NEW crate `crates/step-import`
that parses the AP214 subset `step-export` emits and adopts it per
DESIGN.md **D7** into first-class kernel bodies; acceptance is the
committed 14-fixture solid corpus round-tripping — censuses,
certified volumes, and validity — plus the `nurbs_wireframe`
disposition. This spec is binding: deviations are REPORTED
(numbered, with the executed blocker), never improvised silently.

## 0. The fence (absolute — two orchestrators share this repo)

- **No edits to `crates/step-export`** — src OR tests OR fixtures
  OR `.expect` sidecars. It is a normal *dependency* (dev- or
  main, your call, reported). If a round-trip disagreement makes
  an export fixture look wrong: STOP and report to the
  orchestrator — export-side changes are a design conversation,
  never a lane edit.
- **No CI edits.** The hosted matrix scopes `--workspace` (or a
  computed closure), so a new workspace member is picked up
  automatically. The only out-of-crate edit permitted is the ONE
  member line in the root `Cargo.toml`.
- **No edits to `scripts/check_step.sh` / `step_import_check.py`**
  (export-side oracle, unchanged), no `docs/M6-*` files, no other
  crates. FreeCAD is NOT needed by this unit — the oracle here is
  the kernel itself.
- Test layout: **at most two `[[test]]` targets** (e.g. one
  aggregate `roundtrip.rs` + one `parser.rs`). PR #179 is
  collapsing the workspace's per-file test binaries right now; do
  not mint a new pile.

## 1. Scope, in four legs

**Leg A — Part 21 substrate.** A tokenizer/parser for ISO
10303-21 exchange structure sufficient for the writer's output:
HEADER (FILE_SCHEMA, and the DATA-section unit/uncertainty
context), DATA entity instances, complex (multi-record) instances
— the writer emits them for B_SPLINE_CURVE rational complexes,
SI_UNIT records, and GEOMETRIC_REPRESENTATION_CONTEXT. Reals
parse with `str::parse::<f64>` — the writer's printer round-trips
to identical bits (`crates/step-export/src/real.rs`), so exact
`==` comparisons downstream are legitimate. Errors are typed and
name the offending entity id/line: malformed syntax, dangling
references, unsupported entity types. No panics (`clippy::panic`
is denied in production code). The semantic layer is ours by
necessity, not preference: the F6 spike
(`references/notes/step-spike-report.md`, main checkout — the
directory is git-ignored) found no Rust STEP crate with a usable
AP203/AP214 semantic layer (Evan, #180 comment, 2026-08-04). For
the Part-21 *syntax* layer, hand-roll or adopt ruststep's
syntactic parser — decide against that report's findings and
report the choice; ruststep's parser and truck-stepio's
`in::Table` are precedented as dev-dependency parse-back oracles
(both satisfy the ~2-week release-age policy).

**Leg B — AP214 → kernel geometry.** The inverse of the writer's
identity mapping (`docs/CURVED-DESIGN.md` + the STEP writer identity mapping (M5 PR 13 record)):

- Surfaces: PLANE, CYLINDRICAL_, CONICAL_, SPHERICAL_,
  TOROIDAL_SURFACE → the kernel's analytic surfaces;
  `axis2_placement_3d` → kernel frame **field for field** (the
  export direction is an identity, so import is too — no
  re-derivation, no normalization that moves bits).
- Curves: LINE, CIRCLE, ELLIPSE, B_SPLINE_CURVE_WITH_KNOTS
  (incl. the rational complex instance) → kernel carriers,
  control points / knots / weights exact.
- CONICAL_SURFACE: STEP's `v` is axial; the kernel's is slant
  arc length — the fixed cos α reparameterization the writer
  applies must be inverted, and the apex placement convention
  (`radius = 0.0`) recognized.
- Units: the writer emits `SI_UNIT($, .METRE.)`; honor it (parse,
  don't assume — a foreign file in mm is M7-2's problem, but the
  unit record must be *read* now, refusing typed on units the
  subset doesn't cover).
- ε_in (D7): read `UNCERTAINTY_MEASURE_WITH_UNIT` and carry it on
  the import result as the default input tolerance, overridable
  per call. This unit only *records* it (own-corpus files declare
  1e-9 and adopt exactly); the healing ladder that consumes it is
  M7-2+. The field existing and being honest is this unit's
  obligation.

**Leg C — topology assembly + D7 adoption.** Rebuild the
`Body<f64>` through **topo's public doors** (the Euler-operator
vocabulary — `mvfs`/`mev`/`mef`/ring and hole operators — plus
geometry attachment; Mäntylä ch. 4–6 scans are in `references/`,
readable with poppler). ADVANCED_BREP_SHAPE_REPRESENTATION with
multiple MANIFOLD_SOLID_BREPs (kiss_assembly has 2 solids) and
inner voids must assemble. Adoption per D7: the parsed analytic
surfaces/curves become the *intensional* descriptions; caches
(pcurves etc.) are recomputed by the kernel's own machinery
(`mint_pcurves` and friends), NOT trusted from the file — where
the kernel's minting currently refuses (charts beyond
Plane/Cylinder at M6's current state), the imported body carries
what a natively-built body would carry, no more; state this
honestly in the suite rather than papering over it. `same_sense`
/ edge orientation must be honored, not healed — the corpus
contains reversed-sense faces (`same_sense = .F.` on 91 of 242
ADVANCED_FACEs across the corpus; composed_die and die_pips carry
42 each; the writer emits them faithfully and OCC-style silent
healing is exactly what D7 forbids).

**Leg D — acceptance suite** (the rows in §2, in at most two
test binaries; shared helpers in `tests/common`).

## 2. Acceptance rows (each a named test; all binding)

1. **Committed-corpus row**: every one of the 14 solid fixtures
   under `crates/step-export/tests/fixtures/*.step` imports to a
   body whose census (solids/shells/faces/edges/vertices) equals
   its `.expect` sidecar exactly, whose certified volume
   (`mass_properties`) matches `EXPECT_VOLUME_MM3 × 1e-9` m³
   within the quadrature's own certified bound plus the sidecar's
   print precision (state the tolerance derivation in a comment),
   and which passes the same validity ladder the corpus tests run
   (`validate` / `validate_closed` / geometric tiers) at default ε.
2. **Fixed-point row**: import(fixture) → `step_export::step_string`
   → import again: censuses and certified volumes identical run to
   run, and the SECOND export byte-identical to the FIRST (the
   writer is deterministic, so one adoption pass must be a fixed
   point). Byte-identity of the first re-export against the
   COMMITTED fixture is not required (entity numbering/traversal
   may differ) — but measure it and report which fixtures survive
   byte-identical and the divergence classes for the rest.
3. **Comparison discipline**: rows 1–2 compare counts, certified
   scalars, and structural invariants — never arena order against
   walk order (`docs/CURVED-DESIGN.md` + the STEP writer identity mapping (M5 PR 13 record)'s trap: the
   writer's traversal diverges from `Body::faces()` order on
   boolean results, which is most of this corpus).
4. **nurbs_wireframe disposition**: the curve-only fixture
   (GEOMETRIC_CURVE_SET in a GEOMETRICALLY_BOUNDED_WIREFRAME_
   SHAPE_REPRESENTATION) parses; the rational quadratic
   reconstructs with control points / knots / weights exact
   (`==`); no body is claimed and the suite says why (no solid in
   the file — a disposition, not a skip).
5. **Refusal rows**: (a) a hand-authored minimal file with
   `B_SPLINE_SURFACE_WITH_KNOTS` refuses, typed, naming the
   entity (the named M7 frontier — the S9 flip pattern will
   retire it when NURBS faces arrive); (b) truncated / malformed
   files refuse with parse errors, not panics; (c) a dangling
   entity reference refuses naming the id. Refusal messages
   follow the kernel's two-tolerance/fail-loud voice.
6. **ε_in row**: the import result exposes the header's declared
   uncertainty (1e-9 for every corpus file) and a per-call
   override exists; assert both.

## 3. Design constraints

- `Body<f64>` only. The file is decimal text; f64 is the honest
  carrier. A generic-T import is nobody's acceptance row; do not
  build speculative genericity (report if the lift turns out
  free).
- Fail loud (D4 ¶5): every refusal is a typed error naming
  entities; no silent guesses, no lenient re-interpretation.
  Ambiguity at ε_in scale is a typed error by D7 — with exact
  own-corpus input it should never fire; if it DOES fire on this
  corpus, that is a finding to report, not to widen away.
- New trilean predicates (if any) route through the standard
  `Decide`/K machinery — no bespoke thresholds. The import corpus
  is #89's designated re-open trigger: if k-lint rule 1 ever
  shows an in-band landing from these suites, report it as a
  headline finding; never retune.
- Comment density, naming, and error-message voice match the
  neighboring crates; module headers state each file's contract.

## 4. Local battery scope (iteration speed only; hosted CI is
the gate — memories/local-battery-scope.md)

`cargo check --workspace` once (the member-line edit is
workspace-visible), then the crate's own suites at default ε as
you build them. No interval lane locally (the crate is f64-only);
no FreeCAD; no CI mimicry. A known gate failure reproduces
locally first, targeted.
