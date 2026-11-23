---
id: a-proven-zero-margin-still-offers-to-tighten
kind: issue
title: A margin the symbolic tier proved zero still offers 'tighten the tolerance below m/K': tightens_below ignores MarginDiag::theorem()
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: encl/proven-zero-no-tighten
pr: 4840
closed: 2026-10-12
---


(Filed by the ENCL orchestrator from a designer's report on `no-tighten-offer-quotes-a-value-below-the-runs-resolution`, 2026-10-12.)

When the symbolic tier proves a margin identically zero, `sym.rs`'s discharge path returns `Sign::Zero` with the numeric reading kept and tagged `proven(theorem)` (`margin.proven(how.theorem())`). `MarginDiag::tightens_below` (geom-core `predicate.rs`) never reads `theorem()`, so `sized_recourse` offers a tolerance for a margin the proof says no tolerance decides. Example: a proven-zero margin with a nonzero numeric value (2.7e-27, or even an in-band 5e-9). That offer is false, not merely unresolvable, and it breaks D4 ¶1 clause (i)'s "only where … a smaller ε decides it".

Repair: `tightens_below` returns `None` wherever `theorem()` is `Some`, so the ending falls to the lever plus the decision's `otherwise`/`at_zero` note. Pin it: a Sym Zero with an in-band numeric reading, on a `SizedPass::Positive` decision, must not offer to tighten. Independent of the [ev] question on an ε floor (the sibling row).
