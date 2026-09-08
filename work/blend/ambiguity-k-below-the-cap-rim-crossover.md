---
id: ambiguity-k-below-the-cap-rim-crossover
kind: unit
title: Tol accepts K below the cap-rim crossover K* = 1.272, where two doors behave differently and one refuses
status: review
opened: 2026-09-05
branch: blend/k-no-floor
pr: 2149
---

## Finding (FILLET-H6's lane, PR 1891 — recorded, deliberately not fixed there)

`Tol` accepts **any** `K > 1` from `CAD_AMBIGUITY_K`
(`crates/geom-core/src/tolerance.rs`, the predicate is `v > 1.0`), and no CI
row varies it: the whole matrix runs at the default K = 10. FILLET-H6 measured
what happens below `K* ≈ 1.272` and the answer is that two of `extrude`'s
doors stop agreeing about the same body.

**The crossover.** `extrude`'s two direction gates admit an extrusion vector
whose in-plane component is at most ε against a normal component of at least
`K·ε`, so an admitted `w` parts from the sketch normal by up to `1/K` and the
cap–wall angle obeys `sin θ ≥ K/√(K² + 1)` (the wall normal is `chord × w`).
The `Smooth` outcome needs `sin θ · arm ≤ ε` against an arm the profile door
already put at `arm ≥ K·ε`, i.e. `sin θ ≤ 1/K`. The two close exactly when
`K⁴ > K² + 1` — `K* = √((1 + √5)/2) ≈ 1.272`. Above it no admitted extrusion
can produce a smooth cap rim; below it, ordinary ones do.

**The two doors, measured end to end at K = 1.1** (`fillet_h6_cap_rim`'s
re-exec row, and `review_fillet_h6_r1_probes` / `review_fillet_h6_r2_probes`):

- the **direction gates** admit `Vector(ε, 0, K·ε)` — both are satisfied at
  their own thresholds, neither is in band;
- the **profile door** admits `rect(2, 1.002·K·ε)` — the short chord is
  definite by exactly the band the rim's arm gate reads;
- the **rim upgrade** then classifies all four short cap rims `Smooth`. Before
  this PR it minted chart images and `extrude` returned `Ok`, and
  `validate_geometric` refused that body with four
  `SliverDihedral { material_wedge_side }`. It now refuses at the door
  (`ExtrudeError::SmoothCapRim`), which is the honest local fix but does not
  answer the question below.

**The question.** Should `Tol` carry a K floor at all — and if so, is it this
crossover, or something a kernel-wide argument sets? That is kernel policy, not
one verb's, and it wants Ev: a floor changes what every predicate in the
workspace admits, and the crossover above is only the first place a
below-`K*` run was observed to split two doors' verdicts. Related, and
deliberately separate: nothing in CI exercises a non-default K, so a floor (or
its absence) is currently unmeasured everywhere except here.

Cited: `crates/sweep/src/extrude.rs` (`upgrade_rim`'s arm and
`ExtrudeError::SmoothCapRim`), `crates/geom-core/src/tolerance.rs`,
`crates/sweep/tests/fillet_h6_cap_rim.rs`,
`crates/sweep/tests/review_fillet_h6_r2_probes.rs`.

## Home

`work/fillet/` — found on FILLET-H6's lane. The decision is Ev's (kernel
tolerance policy), so it is an issue awaiting a design call, not a unit.


## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). Its `## Home` named `work/fillet/`, which closed on 2026-09-06 (DOC-LEDGER sweep 7); BLEND is FILLET's successor and carries this as its `[ev]` question on kernel tolerance policy (`crates/geom-core/src/tolerance.rs` is PROPS' file; the ruling is Ev's and no lane resolves it by implementing).

## Question for Ev (BLEND, 2026-09-07)

**Should `Tol` refuse K below some floor, and if so which one?** Today
`Tolerance::init` and the `CAD_AMBIGUITY_K` self-initialization accept
any finite `K > 1` (`crates/geom-core/src/tolerance.rs`, `v > 1.0`);
D4 ratifies exactly that: *"K is a policy dial — refusal rate and f64
noise headroom — not a correctness parameter: soundness rests on
escalate-never-guess, D4 ¶2 certification and interval replay, for any
K > 1."* The finding above is the first measured place where a run
below `K* = √φ ≈ 1.272` (the root of `K⁴ = K² + 1`) makes two of
`extrude`'s doors admit what its rim classifier then reads as a smooth
cap rim; FILLET-H6 closed it the D4 way — the door refuses typed
(`ExtrudeError::SmoothCapRim`) and no body the at-rest gate would
reject is minted — and the run stayed sound at every K tried.

Three answers, with a recommendation.

**1. No floor; D4 stands; the crossover is the verb's fact, stated at
its refusal (RECOMMENDED).** `K > 1` is the only condition the band's
semantics need (an empty ambiguity interval is what `K ≤ 1` breaks).
The crossover is not a property of `Tol`: it is the composition of two
of `extrude`'s gates — a tilt admitted at `1/K` against a lever the
profile door admits at `K·ε` — and a different pair of doors composes
to a different algebraic number; a floor derived from this pair
protects nothing else. What D4 promises at small K is refusal, not
building, and H6 delivered it. What is owed instead is
**measurement**, because "for any K > 1" has never been exercised in
CI (every row runs K = 10): one nightly row at a small K (K = 1.1, the
value H6 measured at) over the Band-4 corpus and the tour, asserting
the H6 invariant kernel-wide — every door either builds a body the
at-rest gate accepts or refuses typed; no door mints what the gate
rejects. That row is the instrument for the class this finding is one
instance of, and it goes red on the next split without anyone deriving
its crossover by hand. Counter-argument, stated honestly: a small-K
row costs a nightly job's compute for a configuration no user has
asked for, and it may surface several door/gate splits at once, each
its own unit; that is the point of running it, but it is also a bill.

**2. A floor at `K* = √φ`, refused typed at `Tolerance::init`.** Closes
the one observed split at the cheapest possible site and documents the
crossover in the tolerance module, where a reader of `K` will find it.
Against it: the number is one verb's, so writing it into the kernel's
tolerance policy states a general rule from one instance — the shape
this project's own memory calls a fudged invariant — and it silently
promises that K ≥ K* is enough, which nothing measures; the next split
(a three-decision composition would sit at a higher root) arrives with
the same surprise and a floor that says otherwise.

**3. A kernel-wide floor from an argument, not from this instance.**
The honest general statement is that no finite K makes every
composition of admitted verdicts definite: an n-fold composition of a
`1/K` tilt against a `K·ε` lever closes at ever higher roots, so a
floor would have to pick a composition depth to guarantee. That is a
real design decision (what does "definite" promise about the
*composition* of two definite verdicts?) and it is larger than this
item. If Ev wants it asked, it is a DESIGN.md D4 revision and gets its
own conversation; this item should not smuggle it in.

**On the answer.** (1): the item closes on the recommendation, with
one sentence at `Tolerance.k`'s doc pointing at `SmoothCapRim` as the
witness that small K refuses rather than builds, and the nightly row
filed as its own item (owner: the nightly is CIW's ground, the
corpus walk is `crates/sweep/tests/`' — BLEND files it and announces).
(2): a one-line change at `validate` plus a `K*` constant with its
derivation, BLEND's to land. (3): no change here; a D4 conversation
opens.

## Ruling (Ev, PR 2119, 2026-09-08)

*"i don't think this deserves a special refusal case. it seems like
nothing bad happens if we just remove the special case and leave no
constraint on K."* — after the geometry was spelled out (the dihedral
classifier meters `sin θ · chord` over the rim edge's own extent, and
below `K* = √φ` a chord the profile door admits times a tilt the
direction gates admit lands under ε) and what happens without the
refusal (the at-rest gate refuses the same body as
`SliverDihedral { material_wedge_side }`, because a smooth rim has no
material side; nothing silent).

**Executed as this unit:** `Tol` keeps `K > 1` and gains no floor;
`ExtrudeError::SmoothCapRim` and its arm in `upgrade_rim` are removed —
the smooth verdict stores the conventional description exactly as the
strut arm's zero-side case does, and the body reaches the at-rest gate,
which refuses it there. `Tolerance.k`'s doc carries one sentence: any
`K > 1`; below `√φ` extrude admits slivers the dihedral lever cannot
resolve, and they refuse at rest. FILLET-H6's rows
(`crates/sweep/tests/fillet_h6_cap_rim.rs` and the two H6 probe
suites) flip from asserting the door refusal to asserting the build
and the at-rest refusal at K = 1.1; every `K*` sentence in
`extrude.rs`/`lib.rs` goes with the arm (the five-homes issue unit 2's
fix pass files closes on this unit). The nightly small-K row proposed
under option 1 is NOT filed: the ruling reads small K as acceptable
as-is, with tier 3 as the instrument. E-shaped: single style review.

## Landed (BLEND unit K, PR 2149, 2026-09-08)

`Tol` keeps `K > 1` and gains no floor. `ExtrudeError::SmoothCapRim` and the
refusal arm in `upgrade_rim` are gone: a definitely-smooth cap rim keeps the
conventional description by the predicate — an image at rest in the WALL's
chart, through `topo::Body::describe_at_rest`, the door the strut join's
under-determined case already uses. The wall rather than the cap because the
wall is swept from the rim's own carrier (a line leg's quad has the two rim
chords as opposite edges; an arc leg's cylinder is the carrier's own registered
rim identity), so containment holds by construction, and certification meters
the named chart only.

The body then reaches the at-rest gate, which refuses it there: at K = 1.1
`extrude(rect(2, 1.002·K·ε), Vector(ε, 0, K·ε))` returns `Ok` with four smooth
short rims and `validate_geometric` gives four
`SliverDihedral { material_wedge_side }` — a smooth cap–wall pair has no
material side. `fillet_h6_cap_rim`'s re-exec row asserts all of it, including
the chart the four rims rest in. `Tolerance.k`'s doc carries the ruling in one
sentence.

Every `K*` sentence in `extrude.rs` and `lib.rs` went with the arm, closing
`extrude-cap-rim-argument-and-k-star-have-five-homes` on the same PR; the
constant is now spelled once in `crates/`, at the row that measures both sides
of it. Every extrude and revolve fixture at K = 10 is bit-identical between
base and head across all eight `bitdump` files. The nightly small-K row is not
filed, per the ruling.
