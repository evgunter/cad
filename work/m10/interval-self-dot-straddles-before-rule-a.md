---
id: interval-self-dot-straddles-before-rule-a
kind: issue
title: clause 1 refuses rule A on wide boxes because Vec::dot's v·v is an interval product, not a square
status: open
opened: 2026-09-05
refs: [M10-8]
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
