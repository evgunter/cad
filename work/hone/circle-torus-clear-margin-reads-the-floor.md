---
id: circle-torus-clear-margin-reads-the-floor
kind: issue
title: The circle x torus subdivision reads its clear margin through the floor on |F|/|res|, which overstates the residual a 'definitely clear' piece is decided on
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [degree-2-subdivision-doors-carry-no-root-slack-meter, 3973]
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

## Measured against the oracle (2026-10-11, GERM's half-angle meters lane, PR 4600)

The measurement "What a fix needs" asks for, on the door as it stands
(the floor still read). Circles grazing a torus `R = 1`, tube
`0.05 ≤ r < 1` (`R/r` from 1.003 to 20), at a point anywhere on the
tube, `ρ` 0.01–100 m, at any tilt, by a depth inside the zero band, in
its gap, or zero, judged by `conic_oracle::exact` at ε = 1e-6, 1e-9 and
1e-12 (`Band::new(ε, 10ε)`), 1,500 poses per ε whose true depth is
under `10ε`:

- **Nothing within the zero band is answered.** No `Miss`, no count,
  no `OnSurface` on a pose whose true depth is under ε, at any ε. So
  the P0 this item worries about did not appear.
- **The door does answer in the gap, and truly.** 244 of 4,500 got a
  definite answer (182 `Miss`, 62 certified counts; 97 / 92 / 55 at
  1e-6 / 1e-9 / 1e-12), every one true, the shallowest at 5.8ε true
  depth. The rate is flat in `R/r` (5–6% in each of `< 1.5`, 1.5–3,
  3–6 and `≥ 6`). The review of PR 4600, on its own pose family,
  saw 114 of 4,500 at 5.6–10ε, all true, most at `R/r ≈ 1`.

That is the liveness side the floor buys, not a wrong answer within ε.
(Corrected 2026-11-09: a definite answer between ε and `Kε` is not what
the trilean permits. `docs/DESIGN.md` `[verdicts]`: "margins within K·ε
escalate", and "soundness rests on escalate-never-guess". A `Miss` read
there is a guess.) The
row is `conic_quadric::in_band_sweep::a_circle_grazing_a_torus_is_answered_truly_or_not_at_all`
(slow set; red if any pose under ε is answered or any answer is
false). It does not settle the sound-and-local bound this item asks
for.

## The ceiling is read; L remains (2026-11-09, `onecopy/local-heights`)

Measured first. 60,000 random circles tangent to a torus (R 1, r 0.25), set off its surface by 0.5 to 15 picometres at ε 1e-12, over 3 seeds, each `Miss` judged by a dense and golden-section oracle of the true least distance:
- no `Miss` within the zero band;
- about 2,000 of about 3,200 `Miss` answers per seed have their true distance in the band's gap.

The orchestrator ruled (option C) by `[verdicts]`. The circle × torus door now reads its clear margin through the torus's ceiling, exactly as the ellipse × torus door does (`f_per_metre_hi: f_per_metre_surface`, `residual_reach: Some(r/2)`), so the two torus doors read one ceiling.
- Pinned by `circle_torus::tests::a_circle_passing_the_tube_in_the_bands_gap_is_no_miss`, which is red on the floor with 16 misses in the gap.
- **Liveness cost**, in that near-graze family on seed 7: `Miss` answers go from 3,187 to 509, so about 680 of about 1,190 true misses past the escalation now refuse.

**What remains is L**, the sound and local bound this row's "What a fix needs" describes: `Q` read over each piece, by its own reach, rather than the whole shell. The liveness cost above is the reason to do it.
