---
id: circle-torus-clear-margin-reads-the-floor
kind: issue
title: The circle x torus subdivision reads its clear margin through the floor on |F|/|res|, which overstates the residual a 'definitely clear' piece is decided on
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [degree-2-subdivision-doors-carry-no-root-slack-meter, ellipse-edge-crossing-a-torus-has-no-root-lane]
---


Found by both dual reviewers of PR 3973 (`circle_roots.rs`, the clear
test, "unsure"), and measured by its lane.

## What

`circle_roots::certified_subdivision` reads a piece CLEAR when
`(|F(m)| − fall − noise)`, converted to metres of residual, is
definitely past the zero band. The conversion must give a LOWER bound
on `|res|` over the piece, which takes a CEILING on `|F| / |res|`. The
frame now carries one (`SubdivisionFrame::f_per_metre_hi`, capped by
`residual_reach`): the ellipse × torus door passes the torus's
near-surface ceiling `2r((2R + 2r)² + 3r²)` capped at `r/2` (outside
the shell `|g − r| ≤ r` about the tube, `|res| ≥ r/2`). The circle ×
torus door (`circle_torus::circle_torus_roots`) still passes the FLOOR
`2r(R² − r²)`, as main always did, which overstates the margin by up
to `((2R + 2r)² + 3r²)/(R² − r²)` — so a carrier whose true residual
dips within the band without crossing could read clear, and a `Miss`
be certified for it. Neither reviewer found such a `Miss`; whether one
exists is not measured.

## Measured (reviewer r1's differential, `R1N = 100`, 3600 poses)

Switching the circle door to the sound bound moves 674 of 3600
verdicts: 441 `Certified` → `Uncertain` and 233 `Miss` → `Uncertain`,
none the other way. That is too large a liveness cost to take inside
the ellipse × torus unit, so PR 3973 keeps main's reading for the
circle door and says so at the call site.

## What a fix needs

Decide the clear margin on a bound that is sound AND local: `Q` read
over the piece rather than the whole shell (the piece's own reach,
`|q(m)| + speed·w`), which the frame would carry as a closure. Measure
the 233 `Miss` answers against an oracle first: if any lies within the
band of the torus, this is a wrong answer (P0), not a liveness question.
