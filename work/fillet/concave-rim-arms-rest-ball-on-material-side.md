---
id: concave-rim-arms-rest-ball-on-material-side
kind: issue
title: fillet: the coaxial and plane-sphere arms rest the ball on the MATERIAL side of a concave chain — the closed-rim surgery carves the concave band unchanged once the arm is right
status: closed
opened: 2026-09-04
refs: [concave-closed-rim-has-no-band, 1244]
pr: 1752
closed: 2026-09-04
---

Filed from FILLET-H4's Phase 1 (`docs/FILLET-H4-SPEC.md`), by its stop
clause: the concave pair's `EdgeBlend` is itself wrong, so the defect
is in the arms and outside `concave-closed-rim-has-no-band`'s statement.
Every number below was taken with the convexity arm of
`resolve_rim`'s loop deleted locally (uncommitted), on the merge base
`4f2dc1cf`.

## The shape

Every curved-support arm folds each support's **stored sense bit** into
the side the ball rests on, and never the chain's **convexity
verdict**:

- `crates/sweep/src/blend/battery.rs:959` and `:974` — `curved_arm`
  passes `senses.0`/`senses.1` (documented at `:833` as "the two
  supports' stored sense bits") straight into `Meridian::trace` and
  `Ruling::trace`;
- `crates/sweep/src/blend/arms.rs:677` and `:809` — `trace` turns that
  bit into `side = ±1`, which `SupportTrace::contact` (`:498`) and
  `sheet_center` (`:549`) read as "the material is on the `−normal·side`
  side; rest the ball there";
- `crates/sweep/src/blend/arms.rs:381`, `:383`, `:394` —
  `plane_sphere_blend` puts the spine `radius` BELOW the plane
  (`h = depth + radius`, `spine_center = sphere_c − n·h`) and picks the
  offset sphere `R ∓ r` from the sphere's sense alone; its callers at
  `battery.rs:869`/`:878` hand it `senses.1`/`senses.0`.

The plane–plane arm (`arms.rs:279`, `signed = ±radius` at `:287`) and
the corner ball (`:909`, `:920`) DO fold `convex` — that is BLEND-4's
work, and it is why the open-chain concave band carves. The two
plane–plane spellings are the precedent; the curved arms never got the
fold. On a concave chain every curved arm therefore returns the
convex-side rest MIRRORED through the rim: the ball inside the
material, the torus tangent to the supports' EXTENSIONS beyond the rim,
and both trim circles on the carriers but outside both faces — which is
the stop clause's own sentence ("a trimline circle not on its support").

## Measured

Three concave closed rims, `fillet_edges` with the gate off. Each
reaches its rim door and refuses at `surgery.rs:2516`
(`seam_split_param`: "a trimline does not cross its support's seam
meridian inside its span") — which is the surgery CORRECTLY declining
to cut a seam at a foot that lies beyond the rim.

| fixture | door | arm | torus spine (arm) | void-side spine (hand) | trim circles vs faces |
|---|---|---|---|---|---|
| waist of `(0,0)→(1,0)→(0.5,0.5)→(1,1)→(0,1)` revolved, r = 0.05 | ANNULUS (2 arcs) | `ConeConeTorus` | `0.4292893218813453` = `0.5 − r√2` | `0.5707106781186547` = `0.5 + r√2` | both radius `0.4646` at `y = 0.4646` (upper cone, face spans `y ≥ 0.5`) and `y = 0.5354` (lower cone, face spans `y ≤ 0.5`): on each cone's extension past the rim |
| boss `cube(1) ∪ ball(R 0.09, c z = 0.96)`, rim radius `0.0806`, r = 0.02 | LADDER (ring of the top plane, two half-caps) | `PlaneSphereTorus` | centre `z = 0.98` (below the plane, inside the slab), `s = 0.06708` | centre `z = 1.02`, `s = 0.09220` | plane trim radius `0.067 < 0.0806`: inside the boss's footprint, where the plane face has its hole; sphere trim at `z = 0.9857`, below the plane |
| lily lantern mouth (`demos/tour/tests/blend1_r1_wall6_probes.rs`'s numbers), rim `0.25298` at `y = 0.80780`, r = 0.02 | ANNULUS (2 arcs) | `SphereConeTorus` | `s = 0.24016 < 0.25298` | `s = 0.26584` | cone trim at `y = 0.80663` (cone face spans `y ≥ 0.80780`), sphere trim at `y = 0.80876` (sphere face spans `y ≤ 0.80780`) |

The waist row's arm value agrees with the hand MIRROR to the last
printed digit, which is what makes this a sign and not a numerics
question.

## The constructive half (the premise of the gate is false)

Folding convexity into the arms locally — `curved_arm` handing
`Meridian::trace`/`Ruling::trace` the side `sense == convex`, and
`plane_sphere_blend` taking `convex` (`signed = ±radius`, `h = depth +
signed`, offset `R − r` when `sphere_sense == convex` else `R + r`,
plane trim at `spine_center + n·signed`) — and running the SAME three
requests through the UNCHANGED surgery:

| fixture | carves | `validate_geometric` | census | `ΔV` | `volume_pad` |
|---|---|---|---|---|---|
| waist | one annulus band | clean | `(8,14,8) → (10,17,9)`, the convex twins' own delta | `+1.7387214704556e-3`; Pappus fill `2π[x_v r²(1 − π/4) + √2 r³(5/6 − π/4)]` = `1.7387214704551e-3`, diff `5.2e-16` | `0.0` |
| boss | one ladder band | clean | `(11,16,8) → (13,19,9)` | `+1.0518273598104e-5` — the pip's own `−1.0518273598326e-5` with its sign flipped | `0.0` |
| lily mouth | one annulus band | clean | `(10,18,10) → (12,21,11)` | `+1.1125648e-7` | `0.0` |

The convex twins (waist base and top, the pip rim) were unmoved by the
fold, as the fold is the identity when `convex` holds (`sense == true`
is `sense`; `signed = radius`; the same offset branch) — stated by
construction here, NOT measured bit-for-bit (no dump differential was
run for this experiment).

So the sentence at `surgery.rs:858` — "a concave chain adds material,
which no closed-rim carve builds" — is a gate and not a fact: both
closed-rim carves build the material-adding band with no change to
their walks. What the gate is actually protecting callers from is the
arm above it, so **it must stay until the arms fold**; deleting it
alone turns a typed refusal into `seam_split_param`'s span refusal on
every concave closed rim.

## Reach (the shape sweep)

Grep for the shape — an arm side-read that does not see `convex` —
over `crates/sweep/src/blend/` (`trace(s[ab], senses.[01])`,
`sphere_convex`, `signed`):

- `battery.rs:959` `Meridian` sides — NOT folded (this issue);
- `battery.rs:974` `Ruling` sides — NOT folded, same fix, but no
  fixture can reach a concave ruled band today (ruled pairs refuse at
  the open-chain admission door);
- `battery.rs:869`/`:878` `plane_sphere_blend` — NOT folded (this
  issue);
- `battery.rs:860` `plane_plane_blend(.., convex)` — folded;
- `arms.rs:920` `corner_ball` — folded;
- `chamfer_strip` — no ball, no side (its normal is a positive
  combination of the outward normals).

Blind spot: the sweep keys on the arm constructors' PARAMETERS; a side
decided by arithmetic on stored data with no named side/sense/convex
token would not match it.

## What would close it

The arm fold above (two families, one predicate-free sign each,
`convex` read off the chain's stored verdict — S10/S11), with the gate
in `resolve_rim` retired in the SAME change and FILLET-H4's rows
(`docs/FILLET-H4-SPEC.md` §Phase 2) landed on top of it; the spec's
"Out of scope" clause on the arms is what routes that re-scope to the
orchestrator. Artefacts of the measurement: the probe and the
uncommitted diff are in the H4 lane's private directory
(`/home/user/cad-work/fillet-h4/`), not in the tree.
