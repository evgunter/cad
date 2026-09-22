---
id: a-widened-derived-placement-normalises-a-straddling-newell-sum
kind: issue
title: A boss on a widened derived frame refuses on the symbolic lane at clause 1: newell normalises a cross-sum whose enclosure contains zero, so a margin the tier proves zero is never asked
status: open
opened: 2026-09-14
priority: P0
cost: H
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
`work/decide/interval-self-dot-straddles-before-rule-a`'s mechanism,
which is about `dot(v, v)` as a product of independent copies (that row
is closed: DECIDE-1's census, #3001, found no such product at
`Interval` on any certification path a measured document takes). The defect is
one step further on: the DIVISION by a length whose enclosure contains
zero. The true `normal_sum` has a definite direction at every parameter
point of the box (the boss's side plane is a planar rectangle for every
`h ∈ [0.95, 1.05]`); the enclosure straddles zero only by DEPENDENCY
WIDENING of the translate-to-origin cross-sum over a placement that
reaches the loop's points through the derived frame's own newell and
normalisation.

## The cascade, and the five callers

`newell_plane` is called **five times** in the kernel — three in
`sweep::extrude` (the near cap, the far cap, and each side plane:
`extrude.rs:596`, `:694`, `:1102`) and twice in `sweep::loft` (the
bottom and top caps: `loft.rs:378`, `:511`) — so a document that stacks
an extrude on a derived frame walks the door repeatedly, each time over
points the previous walk helped place. Line numbers ride along; the
callers are named.

Measured on the height document at `5e-2`, with `sign_within` at a band
of `1e-300` (so nothing is decided by tolerance) at every call, in
order: **eight calls per replay, and exactly one straddles in the
DECISION channel.** The cube's six (near cap, four side planes, far
cap) and the BOSS's cap all DECIDE every component of `normal_sum` —
but that is `Decide for Sym` answering through the FORM (`h − h` is
the zero form), not the value: their x/y VALUE enclosures already
straddle (the cube's far cap `N.x, N.y ∈ [-0.8, 0.8]`, the boss's cap
`±1.33`, `±1.36`, against a true 0), and `normalize()`,
`orthonormal_basis` and the point placement compute with the value.
The boss's first SIDE plane is where the sign is undecided in both
channels: `sum.y` is `[-2.0507, 0.8977]` where its true value is
`−0.5` (twice the wall's area). The authored control — the same
widened origin with exact axes — walks the same translate-to-origin
step at its side planes and stays off zero (`[-0.9, -0.1]`).

The seed is one level up, in the SAME translate-to-origin step, and it
is visible in the offsets the cross-sum is built from. At that eighth
call the first corner's `(p − centroid).z` is

```
[-0.19954, 0.44840]        (true value: -0.125)
```

— a 0.648-wide enclosure over a parameter half-width of `0.05`, while
the same offset at the cube's own caps is decided. The boss's side loop
mixes corners placed through the derived frame (origin carries `h`)
with corners at the boss's own far cap (`h + 0.25`), and their `h`
occurrences do not cancel against a centroid built from all four. That
is `work/sym/real-margin-dependency-widening`'s mechanism — the `h − h`
a translate-to-origin step performs at a cap — arriving here amplified
by the derived frame's RE-DERIVATION of a placement the parameter
already passed through.

So candidate 1 below tightens only the last of these, which is where
this document's refusal is; a formulation that cancels the shared
translation before the cross products would help wherever the offsets,
not the loop, carry the widening, and that is every one of the five
callers.

**The two readings, reconciled** (the review's delta round, by
execution). The review read a three-site CASCADE — the cube's cap
seeds it (`h − h` widened to `±0.8` in `N.x, N.y`), the boss's cap
amplifies it on already-widened corners, the boss's side straddles;
the lane's probe read ONE straddling site. Both are right about what
they measure: the lane's probe asks the decision channel, where the
first seven calls are decided by their zero FORMS; the review reads
the value channel, where the seed's site is the cube's cap. So the
seed is the cube's cap and candidate 1 must apply at the caps too —
which "every one of the five callers" already says.

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

## A fifth site, 50× narrower (SYM-5 PR-2's review R2, 2026-09-14)

The widths above are this row's cheapest reproduction, not its floor.
R2's STACKED document reaches the same clause-1 `Invalid` at
`half = 1e-3` — fifty times narrower than the `5e-2` the two documents
above refuse at. It is the `Guided` lift and the BOSS that refuse: with
the symbolic tier's rule E on, both cubes are carried and the boss's
side plane is the one margin left, at
`newell_plane_residual … margin is invalid`. Under `Pinned` the same
document certifies whole. The construction:

- an authored `Datum::Frame { origin: 0, u: (1,0,0), v: (0,1,t) }` with
  `t = 0.25 ± 1e-3` (a Scalar document parameter);
- a unit square profile on it, extruded 1.0;
- a `Datum::FaceFrame` on that cube's END cap;
- a unit square on THAT, extruded 1.0, and a `FaceFrame` on its end cap;
- a half-size square on the second derived frame, extruded 0.25.

So the chain's depth moves the width at which the cross-sum's
enclosure reaches zero, and two derived rungs are enough at a parameter
half-width a document would actually carry.
`editor-core/tests/m10_derived_frame_tilted_interval`'s
`sym5_the_reach_on_documents_the_unit_did_not_build` builds it
(`tiltV stacked-2`).

## Added 2026-09-15 (SCALAR's `unit-vector-witness-in-geom-core` sweep)

The same `normal_sum.normalize()` (`crates/geom-brep/src/newell.rs`,
`newell_plane`) is why `Vec3::orthonormal_basis` keeps a bare door:
`geom_core::UnitVec3` now carries "unit, by a decided length" across
function boundaries and `UnitVec3::orthonormal_basis` is the witness
door, but `newell_plane` decides no length before it normalizes — the
residual decide comes after, on a different quantity — so it holds
nothing to mint with, and that unit did not add a decision the site
does not make. Whatever this row's fix does to the cross-sum, the
normalize that follows it is the place a `UnitVec3::new(sum,
"newell_plane_normal", band)` belongs (the band is in hand); the day
it lands, this site and `step-import`'s `recognize` (exch's
`recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`)
are the only two callers of the bare door, and it retires with them.

## Added 2026-09-21 (SYM-10's Phase 1): the site moves to the boss's SIDE plane on the sign-hull construction

On `main` merged with `props/sign-hull`, the height document at
`5e-2` (`m10_derived_frame_interval`'s `boss_on_widened_box`) refuses
on the symbolic lane at the BOSS's side plane, loop 0 segment 0:
`newell_plane_residual … margin is invalid` — under the shipped set at
both lifts and under `none` — where Duff's construction refused at the
boss's cap. Same class, same document, a different face of the same
boss: the widened cap's newell normal is the enclosure the frame is
built from either way, and which of the boss's newell sums straddles
first is the construction's. The plain `Interval` lane on this
document refuses earlier still, at the CUBE's `carrier_endpoint_end`
`[0, 0.21]` (node 2), so the boss is never reached there. Measured by
`sym10_phase1_the_derived_frame_rows_refusal_rendered` and
`sym5_phase1_the_newell_refusal_at_5e_2` on `sym/10-decision-door`;
the consequence for the row that pins the `none` rung is
`work/decide/the-derived-frame-refusal-rows-none-rung-pins-the-retired-construction`.

## The wall at 5e-2 moved past this class (DECIDE-3, 2026-09-21)

On the TILTED derived boss at `half = 5e-2` under `Guided` this row's
clause-1 `newell_plane_residual … margin is invalid` is no longer what
refuses. With rule G and the decision read shipped the tier settles
the frame, the cap plane goes through, and the one refusal left is
`interval_span_forward`, `[0, 1.0254777289518563e1]` against
`escalate = 1e-8` — the stored interval's own span over a box that
wide, not a sampled check and not a tier question.
`m10_derived_frame_tilted_interval::m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`
pins that refusal now, with the same "THIS PINS A DEFECT" framing.

The HEIGHT document's `5e-2` rung (`m10_derived_frame_interval`) still
refuses on this row's margin, so the class is not answered — what
moved is which document reaches it first. Filed by DECIDE-3's lane.
