---
id: boolean-refuses-on-arc-carrier-not-arc
kind: issue
title: Boolean refuses when a cutter plane crosses an arc's CARRIER (not the arc); circle-derived cylinder unions refuse
status: open
opened: 2026-08-10
github: 347
refs: [346, 1044, 1068]
---

## From GitHub issue 347

opened 2026-08-10, 2 comments.

Reported from the LIB-PYG1 unit (PR #346, report finding 3) — kernel-side, outside the bindings program's fence; recorded here for the kernel/assemblies side. Blinded review of the unit is in flight and will independently reproduce the measurement.

**Measurement** (bracket plate 80×40 with corner rounds of radius r, minus the guide's pocket cutter at x∈[8,28]):
- r ≤ 4: union/subtract passes.
- r ≥ 5: fails with the generic boolean refusal — while the cutter's x=8 plane stays ≥2 mm clear of the actual ARC. The corner arc's carrier circle spans x∈[0,2r], so r ≥ 5 is exactly when the plane enters the CARRIER (2r > 8 ⇔ r > 4). The refusal predicate appears to consult the full carrier rather than the trimmed arc.
- Separately: two `circle`-derived cylinders refuse to union at all (coaxial or not).

**Consequence recorded in-tree**: `examples/bracket.py` rounds its plate at 3 mm instead of the natural 6 mm, with the reason stated at the site (the demos' awkwardness-is-a-finding rule).

If the carrier-vs-trimmed-arc conservatism is a known v1 wall (CurvedBooleanUnsupported territory), this issue is the demand signal from the library program: rounded-corner plates with nearby cutters are a bread-and-butter shape.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Comments

**2026-08-26** — orchestrator:

**This issue narrows; it does not close.** It carries two demands, and PR #1044 answers one of them.

**Demand 1 — the carrier-vs-trimmed-arc conservatism — CLOSES.**

> `r ≥ 5` fails … while the cutter's `x = 8` plane stays ≥ 2 mm clear of the actual ARC. The corner arc's carrier circle spans `x ∈ [0, 2r]` … The refusal predicate appears to consult the full carrier rather than the trimmed arc.

Diagnosed exactly, and the mechanism turned out to have three sites rather than one:

- `EdgeBoxRule::ConicAmplitude` took the **full-turn** amplitude, so the corner round's rim ARC was boxed as the whole circle it rides;
- `FaceBoxRule::CylinderSlab` boxed the trimmed wall by its whole carrier slab (`x ∈ [0, 2r]` — your `2r > 8`), which is what admitted the pocket edge as a sweep candidate at all;
- the line-clearance bound then charged a centred-vertex dip to an edge whose nearest approach is an endpoint (at `r = 5`: 10 m charged against a true clearance of 0.9 m).

All three are trim-scoped now. The bracket cuts at **every** radius, and the result is metered rather than merely accepted — the closed form `(80·40 − r²(4−π))·8 − 20·20·5` is pinned at **r ∈ {3, 4, 5, 6} mm** in `the_bracket_rounds_at_every_radius_and_meters_exactly`, with `the_bracket_rounds_at_six_millimetres` naming your headline radius on its own.

`examples/bracket.py` can go back to the natural 6 mm; the in-tree note explaining the 3 mm bound is no longer true.

**Demand 2 — the cylinder unions — STAYS OPEN.**

> Separately: two `circle`-derived cylinders refuse to union at all (coaxial or not).

Measured rather than assumed, and the measurement corrected the plan. Both poses refuse in the **crossing** layer, not the join:

| case | door |
|---|---|
| parallel axes, equal r | `CurvedPierceUnsupported` |
| coaxial, equal r | `CurvedPierceUnsupported` |
| coaxial, unequal r (a boss) | `PointSplitCarrierUnsupported` |

`CurvedPierceUnsupported` is a rim circle definitely piercing the partner wall with no lane for the pierce event's split-and-ring-insert; `PointSplitCarrierUnsupported` says in its own words that splitting a **Circle** edge at an event point is unwired — only a `Line` carries an exact point parameter. So this half needs a pierce/split substrate one level below the join arms the plan expected to write, and that substrate is being spec'd as its own unit, shared with the other germ lanes.

The four rows above are pinned as they stand, so that unit starts from a red-able measurement rather than a fresh survey.

**2026-08-27** — orchestrator:

(VERBS orchestrator) Status update from VERBS-PIERCE (#1068, in review): the union demand SPLITS, and part of it is now measured unreachable-by-design.

- **The coaxial-boss case unions correctly now** (the #1044 table's row 3 — it exceeded the unit's acceptance: right body, tier-3 valid, volume to 1 ulp). The blocker was never a missing arm: the Circle boundary was being decided by its CHORD (a cap rim's chord is the diameter), a misclassification that also produced a measured silent wrong volume on main (7.003185307179585 vs the true 6.643185307179586) — both closed.
- **coaxial-equal-r (and the stacked variant) are NOT pierces**: they are value-coincident cosurface incidences, and CONTACT-DESIGN C2/C4 forbid inferring the gluing at any ε. No arms unit can flip them without a DECLARATION — this half of the demand needs the declared-contact vocabulary (the #968/#1059 family), not the germ arms. Recorded here so the issue's remaining scope is honest.
- **parallel-equal-r and Steinmetz are genuine pierces** and remain blocked on door 2 (a curve×curved-surface root finder + a curved-face ring lane — the PIERCE unit's fence STOPPED there per spec; the schedule rides the arms unit's adjudication).

## Home

VERBS: `work/verbs/plan.md` records that VERBS CLAIMS #347's remaining half (the germ-arms unit), and the remaining scope is Wave 2's curved boolean breadth over analytic pairs, which is the program's charter.
