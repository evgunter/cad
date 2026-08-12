# LIB-U8a spec — quantities, units, formatter, and the checking parser (binding)

Mandate: LIBRARY-DESIGN §L5 U8's schema-free half, SCOPED BY LB8
(LIB-LOG): quantity newtypes at the D6 boundary, the unit table +
constants, a display formatter, and the expression TEXT PARSER
against the CURRENT AST. **U8b — display-unit STORAGE and the full
`25 mm` round-trip — is OUT**: per-literal unit storage touches
ExprKind::Literal/WireExpr/bit_eq/schema-v4, all owned by the
upcoming SWITCH unit; the schema breaks once, there. Measured
basis: `~/.local/share/cad-work/u8-census.md` — read completely;
cite, don't re-survey. Deviations numbered and REPORTED.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Every heavy cargo row
`local-scripts/with-build-slot.sh -- cargo ...`, synchronous FOREGROUND,
long timeouts (≤590000), one at a time; NEVER background or park.
Clippy default AND `--features interval` + discipline greps BEFORE
opening. Commit AND push per chunk. NO Co-Authored-By, no model
names. Merge origin/main before opening; confirm checks STARTED.
Other lanes are building — slot waits are normal.

## 1. Fence

In scope: a new home for the quantity layer (your measured call:
a small new crate vs a pncad module — D6 says the newtypes are
the PUBLIC API boundary and U9's Python types mirror them, which
argues for a crate the façade re-exports; decide, report),
`crates/editor-core` (the parser module — ADDITIVE only: new
module, zero changes to expr.rs's types/constructors/wire),
pncad prelude, `demos/tour/src/heatsink.rs` (migration below).
OUT: `crates/profile` (G2's lane), ExprKind/WireExpr/persist/
schema (the switch's), Doc::metadata conventions (U8b's), CI,
docs/M*-*, renders.

## 2. Deliverables

1. **Quantity newtypes at the D6 boundary**: `Length`, `Angle`,
   `Count` (the closed set; Scalar is bare f64) — hand-rolled per
   D6, canonical meters/radians underneath, constructed via unit
   constants (`25.0 * MM`-shaped in Rust; the census's
   step-import units.rs table is the prior art for the table —
   reuse its data shape, not its crate). NAME COLLISION (census
   risk 2): geom-core's `Length<T>` is the classify seam, a
   DIFFERENT thing — your API type must not collide in the
   prelude. Resolve by naming or by module discipline (e.g. the
   prelude exports the quantity types and NOT geom_core's seam
   type, which no library user should touch); state the
   resolution prominently in the PR.
2. **The unit table + constants**: mm/cm/m/in for Length,
   deg/rad for Angle, exact conversion factors as data (INCH =
   25.4 MM exactly; DEG = π/180 documented as inexact-by-nature),
   prefix handling per the census's lengths-take-prefixes rule.
3. **The display formatter**: a `fmt_real` SIBLING (not reuse —
   census §4): shortest-round-trip value rendering + unit suffix,
   the display unit passed as an ARGUMENT (no storage — U8b).
   parse(format(x, unit)) == x bit-exactly is the pin.
4. **The checking parser** (editor-core, new module): text →
   `Result<Expr, ParseError>` where every reduction goes through
   the existing fallible smart constructors (the parser can
   never mint what constructors refuse — the wire.rs strict-door
   philosophy at the text door). Grammar EXACTLY the AST's span
   (census §5): literals, unit-suffixed literals (`25 mm` →
   canonical-meters Length literal; bare integer → Count vs
   Scalar per the census's rule; bare real → Scalar), param
   idents, + - * / unary-minus with standard precedence,
   sin/cos/tan/atan2/min/max calls, parens, an explicit
   Count→Scalar promotion spelling (choose one, e.g.
   `scalar(n)`; report it — it becomes the round-trip's fixed
   point). NO extensions (no ^, no comparisons). Child order
   must match Expr::child's indices (persisted ExprPaths depend
   on it — pin with a descend test). ParseError is typed and
   carries the DimensionError where that's the cause.
5. **Heatsink migration**: the tour's hand-written
   canonical-meter literals (heatsink.rs:82-101,158,189) move to
   parsed or unit-constant expressions — byte-identical exports
   (the values are the same bits; this is a SAID change). The
   transcribed-decimal shape dies where it lives.
6. **Prelude**: quantity types + constants + parse door exported.

## 3. Acceptance

- parse/format round-trip property tests (proptest over the
  grammar, incl. dimension-refusal rows: `5 mm + 3 deg` refuses
  typed, `sin(5 mm)` refuses, `25 mm * 4 mm` refuses).
- Every DimensionError variant reachable through the parser and
  pinned.
- Byte-identity: tour exports unchanged at all three ε rows
  (heatsink's values are bit-equal by construction — verify by
  your own base-build diff).
- Batteries green; zero new [[test]] binaries.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-u8a-report.md`, per-phase figures.
Open, do NOT merge. Final message: PR number + report path only.
Genuine forks: report (the quantity-crate placement and the
promotion spelling are the likely spots), smallest faithful
reading, flag.
