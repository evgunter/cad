---
id: the-pose-basin-reads-the-second-carriers-reach-as-an-arc-where-only-a-chord-bound-is-proved
kind: issue
title: The pose's basin reads the second carrier's reach as an arc where only a chord bound is proved
status: open
opened: 2026-11-19
priority: P1
cost: M
---


Filed by ONEHOME's fork 275 step 3c (PR 7137), from its full review. The
gap is in main's derivation, which predates 3c. The priority is
soundness: a decided "no meeting" could rest on the unproved step.

**The premise.** The pose's window bound, "The gap's lower bound" in
`crates/geom-brep/src/curve_pair/pose.rs:306`, reads (P1)
(`pose.rs:314`) as "every pair of the window lies within `span` of
`(p, q)` along each carrier", with `span = region + travel`
(`Polish::closest`, `pose.rs:1010`). The derivation measures `m` by arc
length from the polished pair, and the basin's step "by (P1)
`m ≤ span`" (`pose.rs:351`) reads it as an arc on both carriers. (P2)
(`pose.rs:328`) reads `K` over the arcs that bound covers.

**What is proved.**
- On the first carrier, `ℓ` is metres along it from the crossing: `e/|d̂·n̂|` in `line_conic`, and `speed_hi · arc` in `transverse`. That is an arc bound.
- On the second carrier, a pair within the escalation `e` has its point within `ℓ + e` of the crossing, in metres. That is only a chord bound. Nothing turns it into an arc along the second carrier from `q`. A conic's arc can exceed its chord, and the polish's start on the second carrier is the crossing's foot, which need not be its nearest point (a thin ellipse's foot lands on the other branch: the meet oracle's draw 1721, seed `0x601115d67dc12df3`, effort 20, pinned at `meet_rows::a_crossing_whose_foot_on_a_thin_ellipse_is_far_is_read_by_the_ball_about_it`).

The walk's window is its pieces, a parameter run on each carrier, and
is an arc bound on both. This is the pose's alone.

**The shape of a probe** (the full review's chord-versus-arc geometry): a small circle as the second carrier, radius `r = 10⁻⁴` m, its arc `(π − 0.3, π + 0.3)`; a segment crossing its plane at `(−r + d, 0, 0)` with slope `e/ℓ`, so it passes `e·d/ℓ` over `(−r, 0, 0)`.
- At `ℓ = 1.3r` and `d = 1.15r` the crossing's foot is `(r, 0, 0)`, `2.84` rad of arc from the arc but `1.14r` in metres.
- 3c's first off-span guard read the parameter distance and decided no meeting there, inside the zero band. The walk-based guard (`Span::clear_by`, the ball of `ℓ + e` about the crossing) does not.

**Tried on 3c's head, not broken.** A probe of 100,000 draws (three
seeds), each read in both operand orders:
- the second carrier is a circle or an ellipse, semi-axis `10⁻⁴` to `1`, ratio `1` to `10⁻⁴`;
- a segment passes within `0.9` of the zero band of a random point of it, at a tilt to the plane down to `10⁻⁵` rad, so `ℓ` runs long;
- the spans are random about the meeting.

No pair read a decided "no meeting". 72% met and 28% refused.

The argument for why it holds where the basin is proved: `2K·span < sin φ ≤ 1` puts `span` under `1/(2K)`, and over such an arc a conic's arc exceeds its chord by about 1% at most. What is left unproved is the polished pair's `q` lying far from the crossing (a far foot). The meeting's second point is then near the crossing but outside `span` of `q`.

**The planned fix (step 3d, ahead of step 4).** A decided one-piece
premise on the second carrier: the second span's arc within the ball of
`ℓ + e` about the crossing is one piece, with curvature × chord small
enough that no other branch of the carrier enters the ball. That turns
the chord into an arc no longer than `(π/2)·chord`, and the basin then
reads `m ≤ max(arc_a, (π/2)·chord_b) + travel`, measured from `q`,
with `q`'s own distance from the crossing in it.
