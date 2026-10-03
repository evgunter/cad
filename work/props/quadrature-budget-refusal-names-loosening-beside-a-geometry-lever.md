---
id: quadrature-budget-refusal-names-loosening-beside-a-geometry-lever
kind: issue
title: PropsError::QuadratureBudget and its checks-window mirror name 'loosen the tolerance' outside the D4 last-resort shape
status: review
opened: 2026-09-28
priority: P3
cost: E
branch: props/recourse-grammar
---


## What

D4 ¶1 (i) (Ev, `[ev]` PR 3352, 2026-09-28): no refusal advises loosening
ε, except one that would otherwise name no recourse at all (a kernel
approximation limit). That one names loosening as a last resort and says
the refusal may indicate a kernel bug worth reporting. The ENCL sites
now do this through `geom_core::KERNEL_LIMIT_LAST_RESORT`
(`kernel_limit_last_resort!` for a `&'static str`).

Two PROPS sites are outside that shape, and they disagree with each
other:

- `crates/geom-brep/src/props/mod.rs`, `PropsError`'s `Display`,
  `QuadratureBudget` arm: "... loosen the tolerance or simplify the
  trim". It names a geometry lever (simplify the trim) AND loosening.
- `crates/topo/src/validate.rs`, `classify_mass_props`,
  `P::QuadratureBudget`: "Recourse: loosen the tolerance". The checks
  window names loosening alone, with no bug note.

## The judgement this needs (PROPS's call)

Is "simplify the trim" a real lever for a spent quadrature budget?

- **If it is**, the loosening clause goes from both sites, and the
  mirror names the trim.
- **If it is not**, both sites name loosening to the target the payload
  gives, followed by `KERNEL_LIMIT_LAST_RESORT`.

D4's own text lists "a quadrature budget spent" among its examples of a
refusal that names no other recourse, which leans to the second
reading. The ENCL lane left both sites alone because the call is about
PROPS's levers.

The `QuadratureBudget` arm is also still in the pre-concision voice (a
stage prefix, 62 literal words), which is
`props-refusal-prose-outgrows-the-viewer`; the two can land together.

The ENCL row that found it is `work/encl/kernel-limit-refusals-name-loosening-without-the-bug-note.md`.

## Resolved (props/recourse-grammar) — the judgement, and its ground

**"Simplify the trim" is NOT a lever this refusal has.** The second
reading, and it is now the only one at both sites:

1. Nothing in the payload is about the trim. `QuadratureBudget` carries
   `width_len`, `target_len` and `rounds` — an enclosure width against a
   target — so the refusal cannot tell whether the trim is why.
2. The refusal is raised on the EXACT arm (`rounds: 0`, no composite
   round at all) and on the last round's proven lower bound, where no
   trim complexity is implicated.
3. The target is `QUAD_TARGET_LEN_FACTOR·ε`, linear in ε, which the
   variant's own doc already calls "the ε knob".
4. D4 ¶1 (i) names a quadrature budget spent as its own example of a
   refusal that would otherwise name no recourse at all — which is what
   licenses loosening here, and only here.

**So both sites name loosening TO THE TARGET THE PAYLOAD GIVES**, then
`geom_core::KERNEL_LIMIT_LAST_RESORT`. Because the target is linear in
ε, the width the enclosure reached names the tolerance whose target that
width would meet: `width_len / QUAD_TARGET_LEN_FACTOR`. One home,
`geom_brep::props::quadrature_budget_recourse`, which the variant's
`Display` and `topo::validate`'s checks-window mirror both compose, so
the two cannot disagree again.

The in-band twin is `PropsCheck::Converged`, which ends in
`Unsized::LastResort` — `KERNEL_LIMIT_RECOURSE`, the no-value form,
since an escalation of the meter carries no width to size the loosening
to.
