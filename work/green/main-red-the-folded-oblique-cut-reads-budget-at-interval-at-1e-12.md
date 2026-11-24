---
id: main-red-the-folded-oblique-cut-reads-budget-at-interval-at-1e-12
kind: issue
title: Main red at ε = 1e-12: a_folded_oblique_cut_reads_violated_off_its_corners reads Violated at f64 and a Budget refusal at Interval, so its one-class assertion fails
status: open
opened: 2026-11-24
priority: P1
cost: M
---


## Finding

From `embed/conic-symmetry`'s fix pass (2026-11-24), on its branch with
main `83bbe105b6` merged.

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all --ignore-default-filter -E 'test(=self_intersection_proves_injectivity::a_folded_oblique_cut_reads_violated_off_its_corners)'`
fails at the row's class comparison: "the folded cut: the Interval body
reads the f64 body's verdict class", left `"Violated"`
(`Witness { a: 3v3, b: 4v1, … d: 7.850462293418877e-16 }`), right
`"Budget"`. It passes at the default ε and at 1e-6.

It is not that lane's: the row fails identically with the lane's one
production change switched off (`one_stated_conic`'s ellipse arm taking
the identity alone, main's reading). The row passed at 1e-12 on that
branch with main `1cf86c9e96` merged (23 Nov), so it moved between
those two mains.

**Checked on bare main (EMBED's conic-symmetry lane, 24 Nov):** it
passes at `1cf86c9e96` and fails at `83bbe105b6` (f64 `Violated`,
`Interval` `Budget`), so a commit in that range turned it;
`git log 1cf86c9e96..83bbe105b6` touches the suite file in `ae95ea6c5`
and `8c32db16b`, and neither was bisected. It is still red at 1e-12 on
main `17e78d9c90` (EMBED's landing runs).
