---
id: cone-apex-cap-refuses-degenerateface
kind: issue
title: props: a cone face bounded by one rim with the apex interior refuses DegenerateFace; its missing extreme is the apex, and the guard against its unbounded complement needs a sense bit fn cone does not take
status: review
opened: 2026-09-15
priority: P0
cost: H
branch: props/curved-residues
pr: 2924
---


Filed by the PROPS sphere-pole-side unit (`docs/PROPS-SPHERE-POLE-SIDE-SPEC.md`,
the cone-apex deliverable), which served the sphere's rim-only polar cap
(issue 1250) and measured this sibling rather than serving it.

## The input, and the measurement

A cone face bounded by **one rim circle and nothing else**, the apex
interior to it — issue 1250's shape on the other singular chart. The
parse pushes one level (the rim's signed slant `v₀`), `min_max` gives
`lo == hi`, and `require_extent` refuses `DegenerateFace`. Executed on
the 45° cone about `+Z` with the rim at signed slant `1`, both
traversals:

```
apex cap, rim u 0→2π    REFUSE DegenerateFace
apex cap, rim u 2π→0    REFUSE DegenerateFace
```

(`crates/geom-brep/tests/props_sphere_pole_side.rs`, the row named
`the_cone_apex_cap_still_refuses_degenerate_face` — kept as this issue's
executed record.) `props_cone_nappe` does not catch it first: the extent
refusal comes before the nappe classify.

## The shape the fold would take, and the one thing it does not carry

**The apex needs no σ.** The sphere's missing extreme is one of TWO
poles and σ picks between them, which is the whole content of
`sphere_rim_only_pole_level`. The cone's is the apex, level `0`, and
there is no second candidate — so the fold itself is the three lines
the spec asked about: push `T::zero()` when a generator-free cone
boundary's levels collapse, and the closed form
`sin α·Δu·|v_hi² − v_lo²|/2` measures the apex cap. (Corrected here on
the R2 review lane's reading, PR 2741: the first version of this filing
said σ was needed for the fold. It is not.)

**What is not free is the guard against the UNBOUNDED complement.**
The same rim traversed the other way bounds the rest of the nappe,
which runs to infinity and is no finite face of any solid — and with
the apex pushed unconditionally it would measure the apex cap's area
through the public `curved_face` door. That is the exact defect the
sphere arm's first landing shipped and the dual review caught
(`props_rim_only_closed`, PR 2741): an admitted input answered wrongly,
which the D2 addendum has no row for
(`d2-addendum-has-no-row-for-an-admitted-input-answered-wrongly`).

Deciding it needs the material side, i.e. σ, i.e. `Face::sense` — and
`fn cone` takes no sense bit, deliberately: a cone's flux needs no
material side at all, since generators run through the apex, so
`(p − apex)·n_chart = 0` and the anchored term vanishes. Inside a BODY
the inverted traversal is still caught, at tier 3's check 6, through
`boundary_material_sign`'s cone arm and `linear_rim_side`; at the
public door it is not caught at all. So the serving decision is
exactly: is the closed form allowed to answer a face whose orientation
only a body-level gate checks? The sphere arm answered "no" and paid
one extra decide for it, and this row should be taken the same way —
with `props_rim_only_closed`'s sibling on the cone (`Δu = τ` for an
apex cap) as part of the same change.

## Not the cylinder's shape

A cylinder's rim-only face is **genuinely extent-less** and needs no
lane: the surface is unbounded along its axis in both directions, so
one rim circle bounds no finite face whichever way it is traversed and
there is no missing extreme for a traversal to name. Pinned as the
negative result in `a_cylinder_rim_only_face_is_extent_less`
(same suite). The cone is bounded on the apex side only, which is what
makes it this row rather than that one.

## Classification

D2 addendum **row 2**: reachable by input, valid, lane unbuilt — the
refusal is typed. **There is no recourse lane**, and a first version of
this filing said there was: `topo::props`' per-face dispatch routes
structurally on the carrier KIND, so only an `Ellipse`/`Nurbs`-trimmed
boundary or a spline chart enters `quad(…)`, and a circle-bounded cone
face refused by `curved_face` is refused, final. What row 2 costs here
is the whole answer, not a `pad > 0` enclosure. (R2 review lane, PR
2741; the same false sentence stood in issue 1250's imported text and
in `require_iso_rectangle`'s docs, and is corrected in both.)

## Served (PR 2924, branch `props/curved-residues`)

**The fold.** `cone_apex_level` pushes level `0` when a
generator-free cone boundary's levels collapse, inside `cone_boundary`
so that all three doors read the same extent. It needs no σ, exactly
as the corrected filing said. It DOES need the sibling of the sphere's
unanimity: σ is `d_u_sign` under the face's one sense bit
(`rim_interior_side`), so unanimity of σ is unanimity of `d_u_sign`
and the cone can require it without the bit — which is what keeps the
true zero-extent patch (rims at one level traversed opposite ways) at
`DegenerateFace`, pinned in
`cone_rims_at_one_level_with_opposite_traversals_stay_degenerate`.
A generator-bearing zero-extent face states its own `v`-domain and is
not folded either (`a_generator_bearing_zero_extent_cone_face_stays_degenerate`).

**The closed form, both traversals, both nappes.**
`sin α·Δu·|v_hi² − v_lo²|/2` with `lo = 0`, i.e. `π·v²·sin α` over a
whole turn. Executed on the 45° cone about `+Z`, apex at the origin,
`v ∈ {SL, 2·SL, −SL}` with `SL = 0.010` m, both traversals: area
within `1e-12` relative of the closed form and flux exactly `0.0`
(apex at the origin ⇒ the anchored term, the only term a cone's flux
has, vanishes identically). Whole cap at `v = SL`:
`2.221441469079183e-4` m².

**The closure guard, red-first.** `props_rim_only_closed` — the
sphere's guard generalised to take the kind's own azimuthal arm
(`RimArms::azimuth`; the cone's is its first rim's radius) — with the
guard removed, the four shapes the sphere unit's row uses answer an
area through the public `curved_face`:

```
half a rim only (du = pi)                    ACCEPT area=1.110720734539592e-4 (whole cap 2.221441469079183e-4, ratio 0.5000)
quarter rim only                             ACCEPT area=5.553603672697958e-5 (whole cap 2.221441469079183e-4, ratio 0.2500)
the same full rim stated twice               ACCEPT area=4.442882938158366e-4 (whole cap 2.221441469079183e-4, ratio 2.0000)
full rim + an extra half arc, same direction ACCEPT area=3.332162203618774e-4 (whole cap 2.221441469079183e-4, ratio 1.5000)
```

With it, all four refuse by that name:

```
half a rim only (du = pi)                    REFUSE props_rim_only_closed
quarter rim only                             REFUSE props_rim_only_closed
the same full rim stated twice               REFUSE props_rim_only_closed
full rim + an extra half arc, same direction REFUSE props_rim_only_closed
```

**The inverted traversal, verified rather than inherited.** The
filing said the complement is caught inside a body at tier 3's check
6; that is now true because the fold is in the shared parse, so
`boundary_material_sign`'s cone arm has an extent to read a side
against and answers `Encoded` where the sphere's rim-only cap answers
`Unencoded`. Executed: `apex cap +u: Ok(Encoded(Negative))`,
`-u: Ok(Encoded(Positive))` — definite and opposite. On a body built
through the Euler doors (one rim row as two half arcs, giving the cap
and the rest of the nappe on one chart), `validate_geometric` raises
`CurvedSenseInverted` naming exactly the face whose encoded side
contradicts its stored `Face::sense`, and no other
(`topo::all cone_apex_cap_body::the_unbounded_complement_is_caught_by_check_6`).

**ε posture: nothing moved.** The fold reuses `require_extent`'s own
cone comparand (the bare slant difference) under the sphere fold's
name `props_rim_only_extent`, and the guard reuses
`props_rim_only_closed`'s comparand at the kind's own arm. No new
predicate name, no new comparand, no new lever; `docs/predicate-
dimension-audit.md`'s two rows are widened to name the cone arm.

## Fix pass — the guard decided a SUM, not a COVER (PR 2924, R2 review)

**A live defect on `main`, not only on this branch.**
`require_rim_only_closed` compared `du_of_rims`' per-group SUM of spans
to a turn, so any multiset of same-direction arcs totalling `τ` passed
it. The function has two call sites — this unit's cone fold and the
sphere's `sphere_rim_only_pole_level`, shipped in PR 2741 — so the
shape reached both. Red-first, with the sum alone deciding:

```
cone   the same HALF rim stated twice (du = tau)    ACCEPT area=2.221441469079183e-4 ratio=1.0000
cone   two overlapping arcs, spans summing to tau   ACCEPT area=2.221441469079183e-4 ratio=1.0000
sphere the same HALF rim stated twice               ACCEPT area=0.00032708658071349983 (the whole cap)
```

That is the sphere unit's own MAJOR — an area answered for a rim that
does not close — reached past the guard written to close it.

**The fix decides a tiling.** `props_rim_only_join` requires, for each
edge in the loop's own order, that its traversal END is the next one's
traversal START, cyclically: a point deviation in metres, through the
funnel, at `require_rim_incidence`'s dimension. Sum and chain together
ARE a cover — each arc runs the same way in `u` with non-negative span,
so a chained arc `k` covers `[S_{k−1}, S_k]` and `S_n = τ` tiles the
circle exactly once. Neither half suffices: three arcs of `2τ/3` chain
into a closed cycle and double-cover (the sum refuses them); a half rim
twice totals a turn and covers half (the chain refuses it). The verdict
is invariant under cyclic rotation of the loop — asserted for every
anchor of every tiling — so it is a fact about the face, not about the
flattening.

**Pinned as a property, not a list.** For `n ∈ 1..=6` the tiling
`[k·τ/n, (k+1)·τ/n]` measures the cap at every anchor, and four
families derived from those same arcs refuse: one short, one repeated,
`n` copies of one arc (sum exactly `τ`), and the tiling with two
adjacent arcs exchanged (sum exactly `τ`). The last two are the ones a
sum cannot see.

**The guard moved into the fold, and the flag is gone.** Two of
`cone_boundary`'s three callers bound the `folded_apex` flag and
discarded it, so a half rim only answered `Ok(())` at the shape door
and a definite `Encoded` side at the gate for a boundary the flux lane
refused — an unpaired derivation, the shape that raises
`CurvedSenseInverted` over an honest `NotIsoRectangle`. A fold now
refuses a rim that does not close rather than folding and leaving the
check to whoever asked, so there is no flag to forget. Executed: door,
gate and flux all refuse `props_rim_only_closed` for a half rim and
`props_rim_only_join` for the same half rim twice
(`a_boundary_that_does_not_close_refuses_at_every_door`).

**Coverage the fixtures did not have.** Every committed cone row sat at
apex = origin, `α = 45°`, axis `+Z` — where `sin α = cos α`, so a
sin/cos swap is invisible, and where the anchored term is identically
zero, so no flux assertion can fail. The R2 probe's 54-row general-cone
battery is adopted (`the_apex_cap_measures_on_a_general_cone`): three
half-angles, two apexes, three axes, both nappes and both traversals,
asserting the area closed form, the anchored flux `apex·n̂·πr²` and that
the two traversals anchor OPPOSITE flux.
