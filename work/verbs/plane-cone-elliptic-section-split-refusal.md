---
id: plane-cone-elliptic-section-split-refusal
kind: issue
title: topo::split refuses every cone face - the elliptic plane×cone section is a closed form R1 rules out by decision, not by difficulty
status: open
opened: 2026-08-31
github: 1339
refs: [908, VERBS-CONE, VERBS-C5ARMS]
---

## From GitHub issue 1339

opened 2026-08-31, 0 comments.

`topo::split` refuses every body carrying a cone face. The cut a modeller
actually wants there — a tilted plane through a cone frustum, section an
ellipse — is a closed form, and the only thing standing in front of it is
a routing note that rules the whole conic trio out *by decision* rather
than by difficulty.

Met while curating the demo montage: the teapot's spout is a cone frustum,
and cutting its tip at an angle (what a real spout has) refuses.

## What happens today

`splitting::classify::gate_operand` walks **every** face of the operand and
admits `Plane` and `Cylinder` only; `Cone`/`Sphere`/`Torus`/`Nurbs` refuse
`SplitReduceError::CurvedBooleanUnsupported`. The gate is whole-body, so a
part with one cone face cannot be split anywhere, even by a plane that
misses the cone.

Underneath, `geom_brep::intersect::plane_cone_section` builds only the
exact degenerates (spec §3.3, R1): apex-through (two generator lines /
tangent line / apex point) and the axis-normal `Circle`. Generic tilt
returns `SectionError::RoutesToGeneralRung` saying

> generic tilt routes to the general rung PERMANENTLY — the conic trio is
> outside the closed-form inventory by decision, and only an arm that adds
> parabola/hyperbola moves it.

That is the sentence this issue proposes to revise, in one member only.

## The elliptic member is already half-derived in the function

With unit axis `a`, half-angle `α`, unit plane normal `n`, write
`c = a·n`, `s = ‖a×n‖`. The apex lane already computes the discriminant

```
D = sin α·s − cos α·|c|
```

and reads its sign as `ApexLinePair` / `ApexTangentLine` / `ApexPoint`.
**That is the conic-type discriminant**, and the apex lane is just its
degenerate column: `D < 0` ⇔ ellipse, `D = 0` ⇔ parabola, `D > 0` ⇔
hyperbola (degenerating, apex-on-plane, to a point, a tangent line and a
line pair respectively). The off-apex lane can read the same margin
through the same `decide` call it already makes, and take the `Negative`
branch instead of refusing it.

For `δ = (apex − q)·n` (the signed apex-to-plane distance the function
already binds as `apex_gap`), the elliptic section has semi-axes

```
b = |δ|·sin α / sqrt(c² − sin²α)          (semi-minor)
A = |δ|·sin α·cos α / (c² − sin²α)        (semi-major)
```

with `A ≥ b` iff `c² ≤ 1`, and both collapsing to the existing
`rim_r = |h|·tan α` at `c = 1` — i.e. the current `AxisNormalCircle` is
this formula's own boundary case, which is the check that it is the right
formula. Centre and `u_ref` fall out of the same Dandelin construction.
Residual is zero by construction (D4 ¶2), as on the cylinder arm.

Parabola and hyperbola stay refused, and would refuse **better** than
today: naming which conic they are rather than "generic tilt".

## What the split lane needs beyond the section

Not audited in depth — listing what I found so the scope is not
under-guessed:

- `splitting::classify::gate_operand` — admit `Cone`.
- `splitting::neighborhood::sector_face` / `sector_face::resolve` —
  `SectorCarrier` is `Plane | Cylinder | Sphere`; a cone sector arm is
  needed for ON-vertex classification.
- `geom_brep::pcurve` — has `ellipse_pcurve_on_plane` and
  `ellipse_pcurve_on_cylinder`; an `ellipse_pcurve_on_cone` is missing.
- `join`/`finish` already carry `Ellipse` carriers (the cylinder arm's
  work), so those look untouched.

## What R1 actually decided (checked in the history)

The routing note reads as if the ellipse were ruled out with the other
two. It was not — R1 is about **parabola and hyperbola only**, and its
grounds do not reach the ellipse.

`docs/M5-PLAN.md` at its introducing commit (`c7b1cae1`), R1 verbatim:

> **R1 — plane×cone staging trigger (OQ1's rider).** OQ1 decided
> (b)-staged-via-(a): `Ellipse` ships with plane×cylinder booleans; the
> parabola/hyperbola decision "rides on whether plane×cone acceptance
> shapes make M5." *Resolution: no plane×cone acceptance shape joins the
> M5 corpus, so the trio does NOT land in M5.* […] Grounds: the M5
> acceptance shapes (R5) are cylinder/sphere/torus/NURBS-dominated;
> adding two unbounded conic variants for a configuration nothing
> exercises would be speculative enum growth — exactly what
> (b)-staged-via-(a) was designed to avoid paying early.

Three things follow:

1. **"Two unbounded conic variants"** names parabola and hyperbola. The
   ellipse is not a variant to add: `Curve3::Ellipse` ships in the *same
   PR* (M5 PR 5) and is what the plane×cylinder arm already mints. The
   anti-speculative-enum-growth argument has no purchase on it.
2. **The grounds are "a configuration nothing exercises."** That premise
   was true of the M5 corpus and is not true now: the demo tour's teapot
   spout is a cone frustum, and the montage curation wants its tip cut.
   R1 was a *staging* decision keyed to whether anything needed it.
3. `M5-PR5-SPEC.md` (`be933982`) says the same in its own words —
   "Parabola/hyperbola do NOT land (R1). No speculative fields, no
   reserved discriminants."

`docs/DRAFT-DESIGN.md` already records this narrowing, per Ev's #908
note: *"R1's 'permanent' refusal bars only the exact conic special cases
(parabola/hyperbola never join the analytic curve inventory); a generic
plane×cone section as a fitted NURBS curve is fine."* That correction
concerns the fitted route; this issue is the stronger claim beside it —
the **elliptic** member is exact, closed-form, and needs no inventory
growth at all.

## Scope

Elliptic (and the circular boundary already shipped) only. Parabolic and
hyperbolic sections keep their typed refusal — and would refuse better
than today, naming which conic they are rather than "generic tilt".

Not a design revision, on the reading above: R1's ratified content is
untouched. What it does need is the `intersect.rs` routing note and the
C5 table row rewritten to say what R1 says, since today they overclaim.

## Home

VERBS: the change lands in `crates/geom-brep/src/intersect.rs` (VERBS territory) and is exactly the charter's curved-boolean breadth — the C5 section arms and the cone operand lanes (`VERBS-C5ARMS`, `VERBS-CONE`).
