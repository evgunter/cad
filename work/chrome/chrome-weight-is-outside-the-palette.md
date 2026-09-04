---
id: chrome-weight-is-outside-the-palette
kind: issue
title: The tree's badge WEIGHT carries meaning no palette can tune, and no test sees any badge's colour
status: open
opened: 2026-09-04
refs: [1769, 1463]
---

Found by CHROME's style lane on PR 1769; judged a class question by the
fix pass on that PR rather than a missing assertion in it.

**Nothing in the repo sees a badge's colour or weight.** `git grep
badge()` finds it only in tests comparing the string, so the claim the
tree's attribution rests on — the row to act on takes the colour, a row
showing someone else's failure draws quiet — is eyeball-only. The
drawing is `crates/viewer/src/app.rs:2787-2805`.

**And half of that claim is outside the palette.** `ui.weak` is an
egui default: no theme, `colorblind-safe` included, can tune it, while
`theme.rs`'s `unresolved` is a palette colour held to the marks check.
The dichromacy carve-out (`crates/viewer/src/theme.rs:425-428`) exempts
`unresolved` from the safety claim by arguing every badge using it
carries its own words — an argument nobody has extended to a
distinction drawn in WEIGHT instead of hue. `ui.weak` is spelled 49
times in `app.rs` against 8 `colored_label`s, so this is the chrome's
general habit and not one badge's slip.

Two decisions, in order:

1. Does a semantic distinction drawn in weight belong inside the theme
   contract? If yes, the weak/strong split becomes a palette-supplied
   thing and the theme suite's checks reach it; if no, the carve-out's
   argument needs extending in `theme.rs` to say why weight is exempt.
2. Only then, a test worth writing: the render path would need a pure
   `RowStatus -> paint` function for a headless row to assert on, which
   is a change to how the chrome draws, not just a new assertion.

Signed: (CHROME orchestrator)

