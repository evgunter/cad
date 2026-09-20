---
id: step-import-eps-in-ambient-two-dial-strand
kind: issue
title: recognition promotes at eps_in while selection and certify run at ambient: a column bent between the dials strands its edge
status: open
opened: 2026-09-04
---


Curve promotion reads the FILE's interpretation budget —
`recognize_curve::recognize(&payload, self.eps_in)` at
`crates/step-import/src/entities.rs:1154` — while the wall-column
candidate selection is banded at the AMBIENT ε
(`line_column_match(&iso, p_start, p_end, tol.eps())`,
`crates/step-import/src/adopt.rs`, the `Curve3::Line` arm of
`iso_curve_candidates`) and the certify doors run at ambient too.
Two dials, one edge: a carrier straight-within-eps_in whose wall
column (or vertex gap) sits between the two budgets — e.g. eps_in
coarse from the file header, ambient fine from the caller — promotes
to `Curve3::Line` under the coarse dial and then fails the fine-dial
selection band or the certify schedule, losing the bitwise candidate
promotion consumed. The failure mode is conservative (a typed refusal
or a down-ladder adoption, never a wrong body) and LATENT: every
committed fixture and corpus file exercises the dials equal (native
exports state ambient as eps_in; dm1's carriers certify with decades
of margin on both). Both EXCH-H1 reviewers' executed probes carry the
evidence shape. Fix belongs with EXCH's recognition lane: either one
dial for the whole promotion-to-adoption path, or a selection band
that reads the same budget promotion certified against.

Found by the EXCH-H1 fix pass (PR #1798), from the adjudicated review
union.
