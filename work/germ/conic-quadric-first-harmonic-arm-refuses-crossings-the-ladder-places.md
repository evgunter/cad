---
id: conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places
kind: issue
title: The conic x quadric first-harmonic arm charges the dropped second harmonic to definite crossings the ladder places
status: open
opened: 2026-10-04
priority: P2
cost: M
refs: [circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound, 4042]
---


Found by the lane that gave `boolean::conic_quadric` one arm switch
(`conic-quadric-doors-choose-their-first-harmonic-arm-two-ways`).

## What

`conic_quadric::conic_quadric_roots` takes its first-harmonic arm when
the second harmonic's amplitude `A₂` is in the zero band, and charges
`A₂` to both extremes' noise. `circle_roots::first_harmonic_roots`
then meters each root's slack as `speed·(noise/|R′| + …)`, so a
definite crossing whose slope `|R′|` is under about
`speed·A₂/zero` refuses `Uncertain`, where the ladder, which drops
nothing, certifies it. The arm is required for a constant residual and
for a tangency (the module docs); for a definite crossing it is the
weaker of the two.

## Evidence

The arm-switch probe of the lane above (a circle tilted against a wall,
over wall radii 1 mm, 1 m and 100 m, tilts 0 to 1e6 zero bands, grazing
inside and outside, crossing and coaxial; each answer checked against a
sampled true-distance oracle), on the poses whose tilt was definite or
in its gap while `A₂` read zero — the poses the tilt switch sent to the
ladder:

| ε | poses | ladder certified, first-harmonic arm `Uncertain` | of those with `A₂ ≥ 0.01·zero` |
|---|---|---|---|
| 1e-9 | 380 | 59 | 21 |
| 1e-6 | 220 | 15 | 8 |
| 1e-12 | 500 | 102 | 17 |

Every refused answer the ladder gave was true of the geometry; neither
arm answered wrongly. The rows with `A₂` under a hundredth of the band
are the rounding charge
(`circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound`);
the rest are this one — e.g. at ε 1e-9 a unit circle tilted 6e-5 m
against a unit wall (`A₂ = 0.9·zero`), crossing at `θ ≈ ±1.98`: the
ladder certified both roots on the wall, the arm refused.

The review of PR 4042 (`analysis/reach-review/4042`, `review.md`,
NOTE-1) measured that the losses are not only shallow crossings. The
arm's slack is `ρ·(noise + A₂)/√(−lo·hi)`, so with `A₂ = 0.5·ε` at
`ρ = 1 m` a crossing 1 cm deep each way already reads about `50·ε` and
refuses. Its band-edge sweep (`review_arm_edge`: `A₂` at 0.5–0.999·ε,
crossings 10.5–1e3·ε deep, 2,880 poses per ε) certified 0 of the 2,880
at ε 1e-9, deep crossings included; on every pose the ladder answered,
its answer was true of the geometry.

## Open question

Should the arm yield to the ladder whenever the ladder certifies? The
arm is required for a constant residual and for a tangency (the door's
module docs); for a definite crossing the ladder places roots the arm
refuses. Weigh it against the ladder's own misplacement along the
carrier (`hone/degree-2-subdivision-doors-carry-no-root-slack-meter`,
the PR 4042 measurements): a ladder answer is on the surface, not
necessarily near the true root.

## The shape of a fix

A design question. Two shapes: charge the dropped harmonic at the
root (`A₂·|cos(2θ − φ₂)|` bounded on the root's bracket) rather than
everywhere; or take the first-harmonic arm only for what needs it
(the constant residual and the tangency its extremes read in the band)
and hand a definite crossing to the ladder.

Moved from ROOTS to GERM on 2026-11-02, by `git mv`, when ROOTS merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.

## Decided (GERM orchestrator, 2026-11-20): the arm decides, the full residual places

Two designers (`docs/prompts/designer.md`), dispatched on this row with
the P0 `the-conic-quadric-door-certifies-first-harmonic-roots-past-the-zero-band`,
agreed in their first reports and through two rounds; no ratified text
changes, so it self-ratifies under the orchestration model.

- The first-harmonic arm keeps every **decision** on the extremes
  `c₀ ∓ A₁` with A₂ charged: miss, on-surface, tangency, constant
  residual, two roots. That enclosure is sound for any A₂.
- Where it reads two roots it **places** them on the full degree-2
  residual. Each root is bracketed where `|R₁| ≤ A₂ + noise` (two arcs
  on monotone pieces of the first harmonic `R₁`). The surface's own
  residual has definite opposite signs at the bracket's ends. The
  least slope `|R₁′| − 2A₂ − noise` over the bracket is decided
  definitely positive, which makes the "exactly two roots" count
  explicit where today it is assumed. The root is solved on the full
  residual and metered `speed · (|R̂| + noise)/slope`, without A₂,
  under the arm's root-slack row. The machinery is the cone door's
  `through_the_apex` deflated roots, lifted to one shared geom-brep
  home that both callers read.
- Rejected: charging `A₂·|cos(2θ − φ₂)|` at the root (still charges
  a known term); sending definite crossings to the ladder (two root
  finders for one pose class, and the ladder's own misplacement).
- Whether the first-harmonic arm or the ladder runs, when A₂ is past
  the band, is a method choice inside the arm, not a pose: the
  crossing-reading design (`curve-surface-crossing-has-no-kind-pair-table`)
  carries no escalating "A₂ in band" pose class. A₂ ≡ 0 by kind
  (circle × sphere) is a pose fact.
- Sequencing: after the ConicQuadric door moves into geom-brep (the
  crossing reading's U2), so this lands in its final home; the two
  never in flight at once. `two-first-harmonic-root-readers` lands as
  its second step (one shared decision reader, each caller places).

## Evidence: the arm's slack is realised to within 0.6% (2026-11-20, PR 7183's review)

A local sphere and wall fuzz, about 1.0M cases over 8 runs at ε 1e-6 and 1e-9. Its shape was small conics with A₂ in 0.2 to 3 bands, grazing and transverse, against spheres and walls of radius 0.05 to 2. It found no certified root past the band. But the worst certified root sat at 0.9936–0.99999 bands in every run: the first-order slack `speed·(noise + A₂)/√(−lo·hi)` is realised almost exactly, leaving no room for the second-order error it does not charge. Part 2 of this row's decided design, placing each root on the full residual and metering it without A₂, removes the A₂ term from the slack. Until it lands, a pose a hair past those draws could certify outside the band. Part 2's lane should pin the worst such pose as a row first.
