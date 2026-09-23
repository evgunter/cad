---
id: self-dot-has-no-gate-the-interval-square-one-cannot-see-it
kind: issue
title: the interval-square gate matches x*x and structurally cannot see x.dot(x) or dot(a, a)
status: open
opened: 2026-09-21
priority: P2
cost: E
---


**Found by DECIDE-1's static census** (the self-dot straddle; the
census is that PR's body). `scripts/gates/interval-square-allowlist.sh`
holds the interval-square discipline tree-wide, and its header writes
its own blind spots down honestly (KNOWN GAPS 1-5, and a five-shape
census for the five spellings its matcher cannot see). **The spelling
it does not mention is the one the whole class is named after**: its
`SQUARE_RE` matches `x * x` as an infix product, so

- `x.dot(x)` — a vector squared by calling the dot product with the
  same vector twice, which is exactly what
  `Vec2::norm_squared`/`Vec3::norm_squared`'s docs argue against and
  what M2 PR 4 removed from the norms; and
- `dot(a, a)` — the free-function form

go past it entirely. DECIDE-1 swept for both by hand and found one
production site at `Sym<Interval>` (filed on SHELL's slate as
`check-rigid-squares-a-column-by-multiplying-two-copies-of-it`), and
every other hit f64-only or under `#[cfg(test)]`. **That sweep is an
undated one-shot and nothing re-takes it** — the same sentence this
gate's own header writes about the backend widening measurement, and
the same defect: a hit list with no guard is a claim, not a receipt.

**What is owed:** a shape in the gate's census register for the
self-dot — `(?<![\w.])(PATH)\.dot\(\s*\1\s*\)` and the free
`dot\(\s*(PATH)\s*,\s*\1\s*\)` — counted as CANDIDATES with a
disposition each, exactly as the five existing shapes are, so that a
new `x.dot(x)` arrives as an unregistered candidate rather than
silently. The dispositions DECIDE-1 derived are in its PR body and are
the register's first entries. The existing shapes' known
under-counts apply here too and should be stated rather than closed: a
receiver repeated through a `let` alias (`let w = v; v.dot(w)`) is two
statements and outside any regex over one.
