# CERT-1 — the sphere polar acceptance defects (#723 + #893)

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The two issues are the primary specification — this document
sequences them and fixes the acceptance surface; where it and an
issue disagree on a measured fact, re-measure and say so.

## Scope

`crates/geom-brep/src/props/curved.rs` (the sphere arms and the rim
lever), `props/mod.rs`'s module docs where they describe the sphere
premise, `docs/predicate-dimension-audit.md`'s affected row(s), new
fixtures and rows. Nothing else moves: no quadrature-engine
consolidation (that is Track R's C3, deliberately gated behind this
unit), no `mesh::walk` changes, no `vec.rs` (PCURVE P-2 is in
flight there), no schema or public-API change.

## The two mechanisms (both accepting-direction, both sphere-only)

1. **#723 — extent from endpoints.** `curved.rs`'s `sphere`
   (`:1102`) takes `(lo, hi)` from `min_max` over meridian
   **endpoint** latitudes (`sphere_boundary`'s meridian arm pushes
   only `e.p0()`/`e.p1()`); a pole-crossing arc reaches ±1 in its
   interior unseen. Confirmed on the rim-bearing arm (−47% through
   the STEP door at tier-3 green) and on the **rimless two-band
   arm** (−29% from a hemisphere split at ±π/4 — issue comment 4).
   The torus arm is immune because its extent comes from the
   anchor meridian's **stored span**; #723's option (2) is to carry
   that derivation to the sphere, and it is the option this unit
   takes (the rimless arm has no rim, so only (2) covers it).
2. **#893 — the lever collapses toward the poles.**
   `sphere_boundary`'s rim arm mints `RimLevel::Unit(sin_v,
   T::zero())` (`:1199`, likewise `as_level` at `:1226`), so the
   predicate's margin is `R·|Δ sin v|` — axial separation, which
   vanishes as `cos v̄ → 0` — and two genuinely distinct near-polar
   rims decide `Zero`, passing a non-rectangular domain. The
   candidate to probe first: the second channel is already there —
   cylinder/cone mint both channels (`:214`) and the chord helper
   (`:493`) consumes both — so carrying `cos_v` in the sphere's
   rim level makes the chord measure direction separation
   (~geodesic, `R·Δv`-scaled) instead of its axial shadow. If that
   candidate fails (bit-stability of existing definite rows, or a
   semantic reason `Unit`'s second channel cannot mean this here),
   an explicit refusal in the collapsing regime is the fallback —
   classified per the D2 addendum (valid input the lane could
   serve = row 2), never a silent widening.

## Order of work — rows first, red first

1. The failing rows, before any fix, each red for the issue's
   stated reason:
   - a pole-crossing meridian arc (rim-bearing arm) whose accepted
     volume is wrong vs. closed form;
   - the rimless hemisphere split at ±π/4 (same set of points as
     the pole-split loop, which measures exactly `2πR²`);
   - a near-polar interior rim pair that currently decides `Zero`
     and passes (#893's ask 1 — the row no suite has).
2. The fix(es), then the rows go green with the **correct** values
   asserted (closed forms, not regression captures).
3. The STEP half-cap twins from #723's reproduction, re-derived
   from the issue text (the original artifacts died with their
   machine) and committed as fixtures: the split half-cap imports
   and certifies the **exact** volume (3.518158565e-7 m³ against
   closed form); the behaviour of the no-split twin is pinned as
   whatever is now true and honest (its old `DegenerateFace`
   refusal may legitimately change under the span-derived extent —
   decide from the geometry, and say which way it went and why).
4. The audit-table row(s) corrected: `docs/predicate-dimension-audit.md`
   currently marks the sphere rim row `OK` while its own prose
   records the accepting-direction collapse (#893 ask 3). Per that
   document's citation discipline, cite by target name, not line
   range.

## Sweep obligation (assume it is a class)

Every `min_max`-over-levels consumer in `curved.rs` (`:161`,
`:172`, `:185`, `:775`, `:916` at survey) gets a one-line
disposition in the PR body: why its kind's meridians/levels cannot
hide an interior extremum (cylinder/cone: monotone lines; cone
apex: `props_cone_nappe`), or what was done. State what the sweep
pattern could not match.

## Acceptance

- The three red-first rows green with closed-form values; the
  half-cap fixture certifies exactly; no existing definite row
  flips (any margin that legitimately moves is re-derived with the
  argument in the PR body, per no-baseline-is-a-target).
- ε-three-outcome honesty on every new row; the props lanes are
  scalar-generic, so the interval lane matters — say in the PR
  whether the gate's draw covered it and run
  `cargo test -p geom-brep` locally at default ε plus the interval
  feature for the touched suites (that is the standard brief; the
  hosted gate proves the rest).
- The PR body answers C-m's three recorded questions (from #723's
  first comment): which engine is authoritative after the fix,
  whether the convergence-block change implies the same change in
  the other copies, whether `QUAD2_AREA_PIECES` was load-bearing.
- No `Co-Authored-By` trailer in lane commits (blinding overrides
  the harness convention; if one lands in a pushed commit, note it
  in the PR body and carry on — never rewrite history).
- Issue-closing keyword hygiene in the PR body: this repo has
  twice closed issue 723 by *describing* it. Break the token
  adjacency everywhere (write "issue 723", never a closing keyword
  followed by the reference).

## Refusal classification

Any refusal this unit mints or changes is classified against the
D2 addendum in the PR body (`DESIGN.md:1118` at last derivation —
re-derive the anchor): a pole-crossing arc and a near-polar rim
pair are valid inputs the lane could serve, so a refusal there is
row 2, and the PR says what the honest serving alternative would
cost.

## Out of scope, stated so it is not rediscovered

The `closing_column` debug_assert in `mesh/src/walk.rs` (#723's
"Related" section) — mesh ground, another stream's; the
quad-engine consolidation (C3/C-m); #883 and anything behind
`H-R16`; S82's smell-scan verdict line (this unit's record answers
it, Ev reads it at plan ratification).
