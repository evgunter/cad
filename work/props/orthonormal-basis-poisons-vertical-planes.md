---
id: orthonormal-basis-poisons-vertical-planes
kind: issue
title: orthonormal_basis manufactures a poisoned (and for n.x!=0, unbounded) chart frame for every vertical plane under Interval
status: open
opened: 2026-08-29
github: 1157
refs: [1116, 1143, 1146]
---

## From GitHub issue 1157

Opened 2026-08-29; 1 comment.

## What

`Vec3::orthonormal_basis` (`crates/geom-core/src/linalg/vec.rs:342-349`, Duff 2017 branchless) manufactures a **poisoned** (`Trv`) chart frame for **every vertical plane** under the `Interval` scalar — and for `n.x != 0` an **unbounded** one.

```rust
let s = T::one().copysign(self.z);      // :343
let a = -T::one() / (s + self.z);       // :344   <-- here
```

Duff's trick is that `s` carries the sign of `n.z`, so the true `s + n.z = 1 + |n.z| >= 1`, never zero. **At `Interval` that correlation is lost.** `Interval::copysign` (`interval.rs:356-376`) is strict on both sides (`lo > 0` / `hi < 0`) and must return the two-sided hull at a zero-containing sign, so for `n.z = [0,0]`:

- `s = copysign(1, [0,0]) = [-1, 1]`, dec `Def`
- `s + n.z = [-1, 1]` — **contains zero**
- `a = -1 / [-1,1] = [-inf, +inf]`, dec **`Trv`** — the first non-real

`Trv` flows into `u_ref` via `b1 = (1 + (s*n.x^2)*a, s*b, -(s*n.x))`. Measured decoration signature `[Trv, Trv, Def]` — the `z` component escapes only because it never touches `a`.

**Two severities, and the second is worse:**
- `n = (0, +/-1, 0)`: values stay exact (`0 * entire = 0` in set-based arithmetic) but the decoration is `Trv` — a *certification* failure.
- `n = (1, 0, 0)`: `u_ref.x = [-inf, +inf]` — the chart frame is **unbounded**, not merely uncertified.

The second case means this is not only a decoration-hygiene problem.

## The doc identifies the regime and dismisses it

`vec.rs:317-320`:

> *"It does **not** narrow when `n.z` straddles zero — `s` is then `[-1, 1]` ... and that case is academic anyway, since `a = -1/(s + n.z)` is unbounded there."*

It is not academic. The word **"straddles" hides the point enclosure `[0,0]`**, which *contains* zero without straddling it, and `copysign`'s strict test lands it in the two-sided branch. That is the most common plane orientation in an axis-aligned model. The derivation's claim that the sign flip "keeps `s + n.z` away from zero" is true in R and false under naive interval evaluation.

## Blast radius

`newell_plane` (`geom-brep/src/newell.rs:156`) calls it, and `extrude`'s side-wall builder (`sweep/src/extrude.rs:1067`) calls that. **On the die blank, 4 of 6 planar supports carry a poisoned `u_ref`.** Every vertical plane built through this path is affected.

Why it went unseen: a poisoned `u_ref` only refuses once something asks for a **chart image** on that surface. The `Resolved::Scaffold` meter (`certify.rs:1673-1686`) computes `p.distance(mc.eval(s))` and never touches the surface, so the pre-collapse kernel never asked. The PCURVE fillet-strut conversion is simply the first construction in this repo to ask.

## Chain to the observed refusal

`orthonormal_basis` -> `newell_plane` -> `Surface::Plane { u_ref: Trv }` -> `chart_pcurve`'s plane arm dots against `u_ref`/`v_ref` so the whole `Pcurve::Harmonic` is `Trv` -> `q` `Trv` -> `surface.eval` `Trv` -> `p.distance(sp) = [0, 5e-324]` dec `Trv` -> `sign_within` -> `is_certified()` false (threshold `Def`) -> `MarginDiag::Invalid`.

**The residual is exact zero to within one subnormal** — matching the f64 arm's `0e0`. Only the decoration refuses.

Raising site note: the payload says `check: ChartResidual`, which is `CertCheck`, so the site is `certify.rs:1742-1753` (the `Resolved::Chart` arm), **not** `pcurve_cache::schedule_residuals` — instrumenting the latter yields zero trace lines. The predicate name misleads.

## Reproduction

```
cargo run -p sweep --features interval --example poison_hunt_onb
```
Four steps: the ONB replayed for `n=(0,0,-1)` [all `Com`] against `n=(0,-1,0)` and `n=(1,0,0)` [`a = [-inf,inf] Trv`]; `newell_plane` inheriting it; the exactly-zero residual classifying `Ok(Zero)` on a cap and `Err(Invalid)` on a side wall; and the real `extrude` blank showing 4 of 6 supports poisoned.

## Fix direction (not taken — `geom-core`'s owner should choose)

The poison is **manufactured by the formula**, not by the geometry: the true quantity `1 + |n.z|` is bounded away from zero, and the `Trv` comes from evaluating a correlated expression naively. Candidates:

1. Compute `s + n.z` as `1 + |n.z|` directly, restoring the correlation the branchless form destroys.
2. Branch on the decided sign of `n.z` where it is decidable, falling back only when genuinely straddling.

Either keeps the f64 path bit-identical. Note this is decidable **without** settling #1143's contract question (poison-absorbs vs widens): whatever that answers, this input should never have produced a non-real.

## Provenance

Isolated during PCURVE P-1b's review chain. Not `fillet` (#1116 was re-scoped off exactly this misattribution), not `pcurve_cache`. Evidence that #1143's caseload contains at least one plain dependency-problem bug rather than a contract question.

## Comments

**2026-08-29** — comment:

(PCURVE orchestrator) — **This is now a ratified-contract violation, not just a bug.** Raising the note here because it changes what "fix later" costs.

`docs/DUAL-DESIGN.md` **DL6** (merged in PR #1146, Ev's sign-off 2026-08-29) states the contract this defect breaks:

> in a certified lane, `Invalid`/NaI is a legal outcome **only when the inputs pose no real question**; pipelines take the **widening path over an absorbing one wherever both exist**; refusals distinguish "too wide at this ε" from "non-real entered, naming the minting site".

Measured against that, `orthonormal_basis` fails all three clauses on the same input:

1. **The inputs pose a real question.** `n = (0,±1,0)` is a unit normal and the true `s + n.z = 1 + |n.z| = 1`. Nothing is ill-posed; the non-real is manufactured by evaluating a correlated expression naively.
2. **An absorbing path was taken where a widening one exists.** `-1/[-1,1]` absorbs to `Trv`. Restoring the correlation — computing `1 + |n.z|` directly — is exactly the widening-path-that-exists this clause requires.
3. **The refusal does not name the minting site.** It surfaces four layers downstream as `ChartResidual` on `pcurve_map_residual`, which is why isolating it needed a dedicated investigation and why #1116 spent four corrections blaming the wrong subsystem.

Two consequences worth stating:

- **The fix is decidable without waiting on #1143's audit.** DL6 already answers "absorb or widen" for this case; there is no design question left here, only the change.
- **The blast radius is every vertical plane** under `Interval`, which is the most common orientation in an axis-aligned model, and for `n.x != 0` the frame is *unbounded* rather than merely uncertified.

Also noting for whoever picks this up: M10 has since found a **third member of the class at plain f64 on the default lane** (a decidedly-coincident boss/plate pair whose diagnostic claims ill-posedness). So this is not an interval-only phenomenon, and a fix here should not assume the pattern is confined to the certified scalar.

Still not mine to fix — `geom-core` linalg, workspace-wide. Reproduction and two fix candidates are in the issue body.

## Home

`crates/geom-core/src/*` is in S-CERT's `paths:` territory, and interval-mode honesty (chart frames, certified enclosures) is its charter.
