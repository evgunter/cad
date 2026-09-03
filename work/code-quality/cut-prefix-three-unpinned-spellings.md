---
id: cut-prefix-three-unpinned-spellings
kind: issue
title: The budget cut-line prefix has three independent spellings across two cargo roots and a shell script, pinned in no direction
status: open
opened: 2026-09-03
refs: [D204]
---

## Was

unrowed. Raised by the `D204` lane's cross-root-constant sweep, which
was looking for the shape `D204` closes — a constant shared across a
cargo-root boundary with no pin in either direction — and found this
one beside it.

## Finding

The budget sweep's cut line is one string with **three independent
spellings**, and nothing holds any two of them to each other:

- `tools/tess-lint/src/lib.rs:478` — `pub const CUT_PREFIX: &str = "# tess-budget-cut:"`, the reader's half (`:518` strips it).
- `scripts/tess_budget_cut.sh:60` — `CUT_RE='^# tess-budget-cut: [0-9a-f]{7,40}(-dirty)? [0-9]{4}-[0-9]{2}-[0-9]{2}'`, the validator.
- `scripts/tess_budget_cut.sh:103,104` — the writer's own `echo "# tess-budget-cut: $commit $date"` and the strip `grep -v '^# tess-budget-cut:'`.

`D204` closed the same shape for `CHART_TAGS` by pinning it from the
meter's side, and `EXPECTED_HEADER` and `GROWTH_TOLERANCE` were pinned
the same way before it. This one is worse than those in one respect and
better in another: worse because the writer and the validator are *in
the same file* and still do not share a spelling, so a change to one is
not even a cross-root problem; better because a drift reds loudly at
parse rather than silently, since `tess-lint` refuses a sweep whose
first line it cannot strip.

**What makes it a finding rather than a tidy-up**: the regex is the only
one of the three that constrains the *shape* of what follows the prefix
(commit, optional `-dirty`, date), and the reader does not check that
shape at all. So the two halves disagree about what a cut line is, not
only about how to spell its prefix.

**Fence.** `scripts/tess_budget_cut.sh` is on retired Track J's unowned
ground (`plan.md`, *What this partition leaves out*), and
`tools/tess-lint/` is Track K's. A row landing on this draws the fence
first, in the same PR that mints it.
