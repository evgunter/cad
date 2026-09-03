---
id: containment-examination-is-extent-box-coarse
kind: issue
title: Containment examination is extent-box coarse — no non-convex-container assembly can certify after PR 737
status: open
opened: 2026-08-20
github: 750
refs: [637, 737, BOOL-4]
---

## From GitHub issue 750

Opened 2026-08-20; 0 comments.

Filed out of Track C's H14 unit (#737), which is a style-scope unit and correctly cannot carry this. Ev, 2026-08-20: *"the cost sounds like it's real kernel work and not style."*

## What #737 changed, and why

`census.rs` arm 2 skipped the cross-solid containment examination for any solid pair carrying contact records. That was a **live wrong answer**, not a near-miss: a 1 m cube wholly inside a 4 m cube, flush at `z = 0`, with four bottom corners declared v-on-f and **every record geometrically true**, returned `Ok(())` at `793a7ae`. The same body undeclared returns `CensusUndecidable`. **Declaring a true contact switched the containment examination off.**

That the records were true is established by the kernel rather than by inspection: `Ok(())` means `confirm_declarations` pushed no `StaleContactDeclaration`, so all four residuals are zero and all four `contain == In`, with every earlier tier passed.

The skip is deleted. A solid pair carrying records is now examined exactly like one carrying none.

## The cost this issue is about

Deleting the skip makes the verdict independent of declarations — the intended property — but it removes the **only** route by which a non-convex-container assembly could pass the certifying door.

Reproduced adversarially, with no placeholder geometry anywhere (plain `Plane` surfaces), through `validate_pseudomanifold` past tier 3, with genuine all-positive containment:

- Container: `common::prism_z` over the L profile `(0,0), (3,0), (3,1), (1,1), (1,3), (0,3)`, `z ∈ [0,1]`
- Part: cube `x ∈ [1,2]`, `y ∈ [1.2,2]`, `z ∈ [0.2,0.8]`, resting flat on the inner wall `x = 1`, four v-on-f records all confirming
- The part is **wholly outside the bracket's material**

| | declared | undeclared |
|---|---|---|
| base `793a7ae` | `Ok(())` | 9 errors incl. containment refusal |
| after #737 | `Err([CensusUndecidable{Solid, Solid, "one instance's extent box inside another's"}])` — that one error | same 9 |

**This is not a false refusal.** `CensusUndecidable` claims undecidability, and the arm genuinely cannot separate this input from the embedded-cube case above. The refusal is honest. But after #737, **no L-bracket, blind-bore, pocket or cavity assembly can pass the certifying door by any declaration.**

## The actual cause, and why the existing remedy does not fit

**The extent-box test is too coarse.** It compares axis-aligned extent boxes, and a part sitting in a concavity has its box inside the container's box while sharing no material.

`ASSEMBLY-DESIGN.md:199` scopes C6's recorded gate-skips to **interference fits** — deliberately overlapping shells. **The L-bracket has no overlap at all.** A gate-skip would suppress the refusal rather than fix it, and suppressing it re-creates exactly the class #737 removed: a declaration that turns a check off.

So the fix is a containment test that can separate "inside the extent box, outside the material" from "inside the material" — kernel geometry, not a cleanup.

One falsification already done, worth recording so it is not retried: the obvious record-free narrowing that reuses data already in `Geo` — deriving a separating plane from the container's own face planes — **is unsound exactly on non-convex containers.** Extend the L's `x = 1` plane and points at `x > 1, y < 1` are outward of it and inside the material.

## Related, from the same review

`declared.faces` has no cross-solid test, so a record naming two faces of the **same** solid would back events within it. Not claimed as a defect; noted here because it is adjacent and was found while establishing the above.

## Provenance

- The defect #737 fixed: S49's class, third instance. S49 fixed arm 1's planar×planar skip; #637 left two residues; #737 fixed those and found a third in arm 1.
- The reproduction and the measurements above are from #737's adversarial review lane, 2026-08-20.
- #737's own body overstated one quantifier — *"**Ordinary** declared assemblies clear on the arm's own margin"* — which the L-bracket case refutes. That is corrected in #737 itself.

## Home

S-BOOL: the containment door is the program's charter ("operand gates and containment doors that refuse or mis-admit legal inputs") and the fix is already scheduled there as the unit `BOOL-4`, which records the handoff to S-MATE at landing.
