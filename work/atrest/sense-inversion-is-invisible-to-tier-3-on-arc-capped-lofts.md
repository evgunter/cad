---
id: sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts
kind: issue
title: inverting every face's sense on an arc-capped loft leaves tier 3 green with an unchanged positive enclosure
status: dispatched
opened: 2026-09-12
priority: P0
cost: H
parent: ATREST-2
---


## The finding

Found by a PERF-6 reviewer while probing the +V check, and OUTSIDE
that unit's fence: it is a fact about what `sense` means on an
arc-capped loft, not about when check 7 stops.

`Body::flipped_face_sense_for_tests` applied to EVERY face of the
three-station `arc_section` loft (the fixture
`crates/sweep/tests/reporting_door_bit_digest.rs` calls `arc_prism`)
leaves tier 3 GREEN, and leaves the body's volume enclosure unchanged
and positive. The same whole-body inversion applied to
`square_prism` or to `step-export`'s `loft_prism` refuses
`LoopRoleInverted`.

So on a body whose caps are bounded by arcs, the stored sense bit can
be inverted on every face at once with no tier-3 consequence and no
change to the quantity check 7 reads. Either the bit means something
those bodies' faces do not carry, or it means something tier 3 does
not check on them.

## Why it is not a check-7 bug

The +V invariant reads a volume ENCLOSURE, and the enclosure did not
move: the quadrature lane's Green form is winding-derived end to end
(the signed UV area IS `s_f·|Ω|` through the stored loop traversal),
which `geom-brep`'s `props::quad` module docs state as a deliberate
property — no sense bit enters it. Flipping the bit therefore cannot
change the flux, and check 7 is reading the same body it was.

What is unexplained is the OTHER half: why the same inversion reaches
`LoopRoleInverted` on the polygonal lofts and not here, and what a
consumer is entitled to conclude from a sense bit that no at-rest
check on these bodies reads.

## Measured (2026-09-20)

Three answers, each pinned by a row in
`crates/sweep/tests/m5_s10_face_sense.rs`. The bodies are
`sweep::loft_body` output, which `topo` cannot reach, so the rows sit
in the S10 sense-acceptance suite rather than in `tier3_tests`.

**None of the three candidate causes above is it.** The rim roles are
stored, not derived; the arc cap's outer/ring role IS computed; and
nothing gates the arc body out earlier — `validate_geometric` on the
honest arc loft is `Ok(())`, all nine checks run. The cause is
narrower and entirely mechanical, and it is a CONJUNCTION of two
classes `work/verdict/m6-sense-gate-recorded-residuals.md` already
records (its residuals 3 and 4), reached here by a body that carries
both at once.

### 1. The raiser, and what goes the other way

`LoopRoleInverted` has exactly one at-rest producer: tier 3's **check
6, planar arm** in `topo::validate`, the Newell arm that falsifies a
loop's stored winding against `plane_outward_normal(face, normal)` —
the one place the planar bit is read as a claim. Its gate is
`all_lines`: the arm skips any loop whose certified carriers are not
all `geom::Curve3::Line`.

Measured, whole-body inversion of each loft:

- **square loft** — two planar caps, each a four-line loop, four
  spline-chart walls. Refusal set: exactly two `LoopRoleInverted`,
  one per cap, and nothing else. **The four walls contribute no
  refusal.**
- **arc loft** (`arc_section`, one bulged vertex) — two planar caps,
  each a loop of three lines and one `Circle`, four spline-chart
  walls. `all_lines` is false on both caps, so the arm skips them;
  the walls are skipped as before. Refusal set: **empty**.

So the discriminant is the cap alone, and the predicate that goes the
other way is `all_lines` on the cap's stored carriers. The arc cap is
not special as a *cap* — it is special as a loop with a conic carrier,
which is residual 4's class; the walls are silent on **both** bodies,
which is residual 3's.

### 2. Does any at-rest check read `Face::sense` on this body?

No. There are four readers in the at-rest battery and all four are
gated shut:

| reader | how it reads the bit | gate | why the arc loft misses it |
|---|---|---|---|
| check 6, planar arm | `plane_outward_normal` | `all_lines` | every planar face carries a `Circle` |
| check 6, curved arm | `(side == Sign::Positive) != face.sense` | skips `Plane` and `Surface::spline_chart()` | every face is one or the other |
| tier 2, C7 material arm | `sense_plus` / `sense_minus` on a definitely-smooth edge | `nurbs_adjacent` short-circuits to `ContactMark::Unmarked` | every edge has a spline-chart face |
| check 7 | `props::curved_face(surface, &outer, face.sense, band)` | reached only by a face with an iso boundary and no certified quad lane (the rimless sphere band) | no loft face reaches it; the reporting door gives the **same reading** under the inversion — bit-identical where it computes, the same typed refusal at the tight ε where these rational walls honestly run out of budget |

The `nurbs_adjacent` entry is the one not previously written down. It
is a deliberate, documented exemption BY KIND (implicit-form gradients
are poison on a spline chart, so `classify_dihedral` cannot run), and
its short-circuit precedes the material arm — so on a body every edge
of which touches a spline chart, which is every loft, the arm's
`Face::sense` read is unreachable too. That is an answer, not a new
defect: with no derived contact class there is no second encoding to
compare the bit against.

### 3. Is an inverted body reachable through the PUBLIC API?

**Yes — this is a real gap, not a test-door artefact.**
`topo::Body::set_face_sense` is `pub`, is not `#[doc(hidden)]`, and is
the door the constructors themselves use to attach the honest bit. The
pinned row builds the inverted arc loft with that door alone, from an
integration test outside `topo`, and `validate_geometric` returns
`Ok(())` on the result, at every ε row, with the enclosure unmoved
(and positive wherever the fixed schedule computes it).
The same public door on the square loft IS refused, which is the
control: the door writes the bit, so the silence is the checks'.

A second public route reaches the same state from outside the process:
`step-import`'s adoption phase copies `advanced_face.same_sense`
verbatim, and its pre-adoption inversion guard covers **cylinder and
cone wall faces only** — an imported solid whose arc-bounded planar
caps or NURBS walls are stated inverted is adopted and certifies.
Filed on EXCH's slate as
`step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard`.

### The rows, and how each goes red

In `crates/sweep/tests/m5_s10_face_sense.rs`:

- `only_the_line_bounded_cap_refuses_a_whole_body_sense_inversion` —
  answer 1. Red when check 6's planar arm widens past line carriers
  (`work/curved/verbs-1031b-assigner-checker-divergence.md`'s open
  question), when the curved arm stops exempting spline charts, or
  when a bulged loft segment stops minting an arc carrier.
- `every_sense_reading_gate_shuts_on_the_arc_loft` — answer 2. Red
  when any of the four gates opens: a loft that mints an analytic
  cylinder for a circular-arc segment breaks both the
  `Plane`-or-spline assertion and the per-edge nurbs-adjacency one; a
  quadrature that folds the bit into a loft face's flux breaks the
  same-reading enclosure.
- `the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts` —
  answer 3. Red when `set_face_sense` stops being public (a
  compile break), when a gate catches the public-door inversion on the
  arc loft, or when one stops catching it on the square control.

Both gap-pinning assertions are deliberately monotone against the
kernel improving: they fail the day the gap closes, and the repair is
to re-cut the row, never to loosen it. Verified red by perturbation —
swapping the bulged section for an unbulged one makes rows 1 and 3
fail at exactly the assertion that carries the answer.

Prior art, not duplicated: `m5_s11_concave_sense.rs`'s
`loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below` already
pins a SINGLE lofted wall's flip being invisible. What is new here is
the whole-body inversion, the cap half, the gate enumeration and the
public-door reachability.
