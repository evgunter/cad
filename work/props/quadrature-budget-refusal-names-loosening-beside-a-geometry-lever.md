---
id: quadrature-budget-refusal-names-loosening-beside-a-geometry-lever
kind: issue
title: PropsError::QuadratureBudget and its checks-window mirror name 'loosen the tolerance' outside the D4 last-resort shape
status: open
opened: 2026-09-28
priority: P3
cost: E
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
