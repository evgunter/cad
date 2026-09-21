---
id: interval-self-dot-straddles-before-rule-a
kind: issue
title: clause 1 refuses rule A on wide boxes because Vec::dot's v·v is an interval product, not a square
status: closed
opened: 2026-09-05
refs: [1828]
priority: P1
cost: H
closed: 2026-09-21
---

**Found by M10-8's R1 review (NOTE-9), by execution.** Rule A of the
atom algebra (`sqrt(X)² = X`, `geom_core::sym::algebra`) is applied
only where clause 1 of the theorem holds — the expression has a real
value on the whole box, which the numeric channel certifies before the
identity test is asked. On a wide box that clause is refused for the
arc family's own shape, and the refusal is the NUMERIC channel's, not
the algebra's:

- `Vec::dot(self, rhs)` (`crates/geom-core/src/linalg/vec.rs:69`,
  `:187`) computes `v·v` as `Σ vᵢ · vᵢ` — a product of two
  INDEPENDENT copies of one enclosure. For a component that straddles
  zero, `[-a, b] · [-a, b] = [-ab, …]`: a spurious negative lower
  bound, where the square `[0, max(a², b²)]` is exact
  (`RingInterval::sqr`'s docs, `crates/geom-core/src/ring_interval.rs`,
  carry the same argument for the certification ring).
- `sqrt(v·v)` over that enclosure is a domain violation: the interval
  `sqrt` clamps and records `Trv`, `sign_within` answers
  `MarginDiag::Invalid`, and the tier does not ask the form
  (`crates/geom-core/src/sym.rs`, `Decide for Sym<T>`: a domain
  violation is the numeric channel's own answer). Rule A never runs.
  R1's row `r1_rule_a_never_fires_on_a_straddling_argument`
  (`crates/geom-core/tests/m10_8_r1_sym_probes.rs`) pins that this is
  the SOUND outcome — the expression has no real value on half the box.

So the family's reach on a wide box is bounded by the value channel's
dependency problem one level below the algebra: `v·v` is a square by
CONSTRUCTION, and `powi(2)` (`crates/geom-core/src/interval.rs:399`,
tight across zero — `powi_is_tight_across_zero`) would give the exact
`[0, …]` where the product gives `[-ab, …]`.

## What is owed, and what is not

- MEASURE, on the M10-8 documents and the slab, how many clause-1
  refusals on `sqrt(v·v)` shapes a `dot`-as-square would remove, and
  whether any ceiling moves. Not done in M10-8's fix pass: `dot` is the
  kernel's, every consumer's f64 bits ride on its association, and a
  change there is a D9 conversation (`x*x` vs `powi(2)` at f64 are
  bit-identical only where `powi` is implemented as one
  multiplication — `interval-square-allowlist.sh` exists because this
  was litigated once).
- If the measurement moves something: a `Vec::norm_sq` door that
  squares component-wise through `powi(2)`, used by the carrier
  constructors, with the f64 bit-identity pinned across the swap; the
  allowlist gate re-read for the new site.

## Re-homed at M10's exit sweep (2026-09-13)

Here because the row is about rule A's reach — clause 1 refusing before the algebra
is asked. The FIX is one line in `crates/geom-core/src/linalg/vec.rs` (`powi(2)`
for `v·v`), which is PROPS' file and its linalg interval-honesty lane's ground: the
seam is announced there and PROPS may simply take the row.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## CLOSED by DECIDE-1's census and measurement (2026-09-21)

**The mechanism does not reach the certification path.** Measured, not
assumed, in both halves (`docs/DECIDE-1-SPEC.md`; the two tables are the
PR body's, and the PR is this row's record):

- **The static census.** No site in `crates/*/src` multiplies an
  enclosure by ITSELF at `Interval` or `Sym<Interval>` on a
  certification path a measured document takes. Every length on that
  path is `norm_squared().sqrt()`, and `norm_squared` has squared
  component-wise through the tight `Real::powi(2)` since M2 PR 4 —
  which is the very door this row asked for ("a `Vec::norm_sq` door
  that squares component-wise through `powi(2)`, used by the carrier
  constructors"). `Sym::powi` hands the value channel to `T::powi`, so
  a `powi(2)` on a `Sym<Interval>` is that tight square.
  `scripts/gates/interval-square-allowlist.sh` is green on the branch
  and is the standing receipt for the adjacent scalar spelling
  `e * e`; the hand sweep for `x.dot(x)` and `dot(a, a)` — which that
  gate structurally cannot see — turned up one production site at
  `Sym<Interval>`, `topo::transform::check_rigid`'s three unit-column
  residuals, whose consumer is a `sign_within` and not a `sqrt` and
  which no measured document decides. It is filed on SHELL's slate
  (`check-rigid-squares-a-column-by-multiplying-two-copies-of-it`), and
  the gate's own blind spot on that spelling is filed on GUARD's
  (`self-dot-has-no-gate-the-interval-square-one-cannot-see-it`).
- **The dynamic measurement.** Zero clause-1 `Invalid` refusals on the
  six documents at ε = default, `1e-6` and `1e-12`, at the nominal and
  at ceiling + δ. Every replay that was blocked at all was blocked by
  exactly one predicate, and every one of those was `Indeterminate` —
  a real margin the band could not classify (`assert_bound`'s
  dependency-widened enclosure on the plate, `arc_diameter_clearance`
  and `dihedral_wedge` on the annulus, `carrier_matches_mapped_source`
  on the link and the bracket) — never a domain violation. The
  instrument is `crates/editor-core/tests/decide_1_self_dot_interval.rs`
  and its ceilings reproduce `m10_10_pins_interval`'s pinned table at
  every ε.

**The pins that stand** for the SOUND outcome on a hand-spelled
product are R1's, unchanged:
`r1_rule_a_never_fires_on_a_straddling_argument` and
`r1_rule_a_decides_zero_at_every_width_and_off_it_widens`
(`crates/geom-core/tests/m10_8_r1_sym_probes.rs`). `Vec::dot`'s
association is untouched, as the row's own D9 note requires.
