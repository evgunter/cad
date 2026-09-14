---
id: a-widened-derived-placement-normalises-a-straddling-newell-sum
kind: issue
title: A boss on a widened derived frame refuses on the symbolic lane at clause 1: newell normalises a cross-sum whose enclosure contains zero, so a margin the tier proves zero is never asked
status: open
opened: 2026-09-14
---


## What

Found by SYM-5's phase-1 measurement (the unit that was to make a
stored unit vector cheap to carry), by execution, on `origin/main` at
`d0d430fc7`.

A profile on a DERIVED frame (`Datum::FaceFrame`) over a body whose
height is a widened parameter, extruded, refuses certification on
`Sym<Interval>` at a half-width of `5e-2` — at every ε row (default,
`1e-6`, `1e-12`), under both `ProfileLift`s. Its authored-frame twin
certifies. The refusal is ONE decision of 29 on
`newell_plane_residual`, and its diagnostic is **`Invalid`** — a
domain violation, clause 1 of the E12 theorem — not a band straddle
and not a symbolic freeze.

`geom_brep::newell::newell_plane` builds the side plane's normal as
`normal_sum.normalize()`, i.e. `normal_sum / normal_sum.norm()`. For
the boss's first side plane over that box, measured at the refusal
site with `sign_within`:

| quantity | enclosure / sign |
| --- | --- |
| `normal_sum.x` | `Zero` |
| `normal_sum.y` | `[-2.0507, 0.8977]` — **straddles zero** |
| `normal_sum.z` | `Zero` |
| `normal_sum.norm_squared()` | `[0, 4.3782]` |
| `normal_sum.norm()` | `[0, 2.0924]` |
| every component of `normal_sum.normalize()` | `Invalid` |

`Vec3::norm_squared` already squares component-wise through `powi(2)`,
so the lower bound is an exact `0` and not a spurious negative — this
is NOT
`work/sym/interval-self-dot-straddles-before-rule-a`'s mechanism, which
is about `dot(v, v)` as a product of independent copies. The defect is
one step further on: the DIVISION by a length whose enclosure contains
zero. The true `normal_sum` has a definite direction at every parameter
point of the box (the boss's side plane is a planar rectangle for every
`h ∈ [0.95, 1.05]`); the enclosure straddles zero only by DEPENDENCY
WIDENING of the translate-to-origin cross-sum over a placement that
reaches the loop's points through the derived frame's own newell and
normalisation.

## Why it matters beyond the one document

The symbolic tier PROVES this margin. Under `SymRules::shipped()` the
residual's early normal form is the ZERO form — rendered as `0` by the
shape report — and 28 of the 29 `newell_plane_residual` decisions on
the same document are `Theorem`. Clause 1 is what refuses: the value
channel has no certified value on the whole box, so the identity test
is never asked. **No rule of the atom algebra can reach this**, which
is why SYM-5 stopped after its measurement: the tier is not what stands
between a derived frame and its authored twin here.

The same reading says the freeze population is not the cause either.
With the exact constant fold alone (`SymRules { const_fold: true,
..none() }`, the early walk off) the document freezes NOTHING and still
refuses at `5e-2` with this margin.

## Reproduction

`crates/editor-core/tests/m10_derived_frame_interval.rs`:

- `m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width`
  pins the measured state, including this refusal by name;
- `m10_the_derived_frames_refusal_is_not_a_freeze` pins that A0 alone
  freezes nothing and still refuses here;
- `m10_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`
  is DOCM-1 R1's parity row, ported unchanged and `#[ignore]`d: it goes
  green when this row is answered, and is the acceptance test for it.

## What a fix would have to do

One of, in the order of how much they claim:

1. **Tighten the cross-sum.** `newell_plane`'s translate-to-origin sum
   re-evaluates each point against a centroid built from the same
   points; on a widened placement each occurrence of the parameter
   widens independently. A formulation that cancels the shared
   translation before the cross products would keep the enclosure off
   zero. This is the cheapest and the narrowest.
2. **Certify the length before dividing.** `Vec3::normalize` has no
   door that asks whether the norm is certified positive over the box;
   the two witnesses beside it (`is_finite_length`,
   `is_underflowed_length`) ask about overflow and underflow, not about
   an enclosure that contains zero. A normalisation that refuses TYPED
   there would at least name the cause at the site instead of handing a
   `Trv` frame downstream (the same posture
   `orthonormal-basis-poisons-vertical-planes` asks for).
3. **Do not re-derive what the door already certified.** The boss's
   side planes are built by newell over points the derived frame
   already placed; the frame's own axes are certified unit vectors. A
   door that carries the placement's certified frame into the side
   plane would not normalise a widened cross-sum at all. This is a
   DOCM/derived-frame-door conversation, not a linalg one.

## Home

PROPS — `crates/geom-core/src/*` (the linalg interval-honesty lane) and
`crates/editor-core/tests/m10*` are in this program's paths, and the
row is an interval-honesty finding about `Vec3::normalize`'s contract
on a widened box. `crates/geom-brep/src/newell.rs` is in no program's
paths; the row is filed here because its mechanism and both candidate
fixes 1 and 2 are this program's ground. Filed by SYM at SYM-5's
phase-1 report (2026-09-14).
