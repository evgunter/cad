---
id: self-dot-has-no-gate-the-interval-square-one-cannot-see-it
kind: issue
title: The interval-square gate's KNOWN GAPS list does not name the self-dot (x.dot(x), dot(a, a)) it cannot see
status: open
opened: 2026-09-21
priority: P4
cost: E
---

`scripts/gates/interval-square-allowlist.sh`'s `SQUARE_RE` (`:213-214`)
matches an infix `x * x` and nothing spelled `x.dot(x)` or `dot(a, a)`.
Its header states KNOWN GAPS 1-5 and asks reviewers to eyeball `.dot(` on
possibly-zero vectors (`:56-58`), but never says the matcher is blind to
the self-dot, which is the spelling the class is named after
(`Vec3::norm_squared`'s docs argue against it).

The only production self-dot ever found at an interval scalar is filed on
SHELL as `check-rigid-squares-a-column-by-multiplying-two-copies-of-it`
(P2, open). A grep of `crates/*/src` for a receiver dotted with itself
returns 92 sites on `origin/main` (2026-11-22), nearly all f64, so a
census register of the self-dot would open with about 90 dispositions and
ask one more of every PR that writes `v.dot(v)` in f64 code, in the lint
job's gates step, for a class with one known instance.

Owed: a KNOWN GAP 6 paragraph in the header naming both spellings and the
`let w = v; v.dot(w)` alias form as invisible, with the grep a reviewer runs
to re-take the sweep. No new census shape.

What would close it: the header names the self-dot as a gap with its
re-runnable grep.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): A census shape would open with about 92 candidates: my grep of `crates/*/src` for a receiver dotted with itself, nearly all f64. It would also tax every future f64 `v.dot(v)` in the lint gates step, for one known interval instance (SHELL's `check-rigid-squares-…`, P2, open). Keep only a KNOWN GAP 6 line in `scripts/gates/interval-square-allowlist.sh`'s header (the matcher, at :213-214, cannot see `.dot`). `reform/self-dot-has-no-gate-the-interval-square-one-cannot-see-it.md`.
