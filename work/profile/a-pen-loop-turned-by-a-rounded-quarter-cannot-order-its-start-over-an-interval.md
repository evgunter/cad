---
id: a-pen-loop-turned-by-a-rounded-quarter-cannot-order-its-start-over-an-interval
kind: issue
title: A pen loop turned by a rounded quarter cannot order its start at Interval, even over a point box
status: open
opened: 2026-11-20
priority: P2
cost: M
---



Found by PROFILE's `a-pen-drawn-rectangle-with-a-boxed-side-refuses-line-span-over-any-box`
lane (2026-11-20), once the pair pass stopped refusing the rectangle.

A rectangle drawn with the pen, heading 0, `line(w)`, `turn(fl(π/2))`,
`line(1)`, `turn(fl(π/2))`, `line(w)`, `line_to(Start)`, validated
unguided at `Sym<Interval>` (`ConstructedProfile::validate`), refuses
`Escalated { site: Loop { loop_index: 0 }, gate: CanonicalOrderX }`
over every box of `w = 1 ± h`, a point box included:

| h | `canonical_order_x` margin |
|---|---|
| 0 | `[-8.88e-16, 2.00e-15]` |
| 1e-11 | `[-2.0001e-11, 2.0002e-11]` |
| 1e-6 | `[-2.0e-6, 2.0e-6]` |
| 0.05 | `[-0.1000, 0.1000]` |

The reading is `validate::lex_less`, called from `lex_min_index`, on
`v3.x − v0.x`, under `prove`'s exact-order band (zero `5e-324`). The
pen lands the fourth vertex as `x = w + cos θ + w·(cos²θ − sin²θ)` with
`θ = fl(π/2)`: the second turn rotates the first's ray (`path::turned`),
and the leg adds `w` times it to a position that already holds `w`. Over
the reals that is `cos θ·(1 + 2w·cos θ)`, about `6e-17`, so the true
order is decided but the value channel carries `w` on both terms and
the symbolic tier has no zero to prove. At `f64` the sum rounds to `0`
exactly, the x comparison reads Zero and y decides.

The same rectangle turned by `Real::pi() / 2` validates over every box
up to ±0.05 (`crates/profile/tests/a_boxed_rectangle_validates.rs`), as
does the four-corner spelling (inferred, not traced: the symbolic tier's
exact trig, `geom_core::sym::turn`, folds `cos(π/2)` to 0, so `x` is the
zero form and the comparison is a tie that y decides).
That row pins today's refusal:
`a_pen_rectangle_turned_by_a_rounded_quarter_cannot_order_its_start`.

What is open is a question, not a fix: the canonical order asks an
exact question of a coordinate the construction knows only to rounding.
Either the order may settle for a vertex that is minimal within the
band (it only picks the containment representative and the canonical
start's rotation, `lex_min_index`'s doc), or a rounded right angle
reads as `turn/4`, or the pen keeps its positions' dependency on a
repeated leg. Each changes a different contract.
