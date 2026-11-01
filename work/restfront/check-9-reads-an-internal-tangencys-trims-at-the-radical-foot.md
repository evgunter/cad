---
id: check-9-reads-an-internal-tangencys-trims-at-the-radical-foot
kind: issue
title: check 9's arm 5 reads both arcs' trims at the radical foot of a decided internal tangency, which near-concentric carriers put about g·r/d off both circles, so arcs a centimetre clear of each other can read Meet
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [decided-tangent-point-is-the-radical-foot]
---



Found by PATHS's sweep for
`an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`,
the profile pair pass's instance of the same reading.

## The defect

`crates/topo/src/validate.rs`, the circle × circle arm of
`segments_meet` (check 9's arm 5): where the carriers are decided
tangent, the candidates are `placed.foot()` and the chord ends either
side of it, and `meet_at` reads each through `window`, which asks
`containment::arc_trim` of a point it documents as on the carrier.
On a decided internal tangency the radical foot stands about `g·r/d`
off both circles (`d` the centre distance, `g` the internal
clearance), so near-concentric carriers put it far off them: 0.029·r
at `d = 17ε`, `g = 0.5ε`, at every ε. `arc_trim_margin`'s chordal
defects move by up to that offset when the point leaves the carrier
radially.

Measured on the margin formula alone (`r = 1`, the point 0.029 inside
or outside the circle on the tangency direction, the arc's end 0.01
rad short of it): an arc whose apex stands 2.5 rad from the tangency
reads the inside point at +0.0063 (in), and one whose apex stands 0.3
rad from it reads the outside point at +0.0116 (in), against −0.013
and −0.011 on the carrier. With `g > 0` the foot falls inside both
circles and with `g < 0` outside them, and the clamped half-chord puts
the chord ends on the foot when nested, so for nested carriers every
candidate is the foot. Two arcs that each stop a centimetre short of
the tangency can then both read in, and check 9 reports a meeting that
does not exist. Not reproduced through a body.

It is a false refusal, not a silent acceptance, as far as this reading
goes. The foot can also read OUT of an arc that holds the tangency: by
the same formula, up to a hold of about 1.7·δ (δ = |g|·r/d the foot's
offset) inside the circles and 0.8·δ outside. That arc's end then
stands within about `d·θ²/2 + |g| ≤ 1.5·g²/d + |g|` of the other
carrier, below Kε wherever this arm runs (`d` above about Kε/2), and
either the other arc holds it or the other arc ends between it and the
tangency, inside the first arc. Arms 1–3 run first and read that end
as a vertex on the other loop, at Zero or escalated.

## Remedy

The profile pair pass's (`crates/profile/src/seg.rs` `arc_arc`): read
both trims at the midpoint of the circles' nearest points on the line
of centres, `|g|/2` off each, rather than at the foot. Arm 5 already
asks the chord ends at a tangency, so a chord deeper than the band,
which the pair pass reads as two crossings (`carrier_circles_chord`),
is read here too, and the foot is the one candidate that needs moving.
