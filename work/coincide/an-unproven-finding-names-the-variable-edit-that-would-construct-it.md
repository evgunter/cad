---
id: an-unproven-finding-names-the-variable-edit-that-would-construct-it
kind: issue
title: An unproven-coincidence finding names the concrete variable edit that would make it one construction, and the viewer offers it
status: open
opened: 2026-10-30
priority: P2
cost: H
design: true
---


Ev, on `[ev]` #6086 (2026-10-30): "might be worth adding a row to have the gui suggests the variable change that would be required to silence the warning."

**Context.** #6086 lets an assertion quiet any unproven coincidence it can say. That includes a flush pair or a coaxiality that holds only "by stack-up": dimensions that happen to add up, such as a pinned cover plate whose edge lands flush with a housing wall because 100, 10, 90 and 10 add up. Ev's intended fix for such a row is a construction: define the dependent distance with the same variable, or as a formula of the others, so the door proves it at every value.

`[intent.checked]` already says the finding reports "the edit that would make it one construction". Today `Recourse` names only a kind of edit (`OneConstruction`, `Assert`, `Budget`), never the concrete edit.

**The work.**
- Given an unproven row whose residual is a function of written values, find a variable (or several) whose redefinition as a formula of the others makes the residual identically zero. For example "define `pin_x` as `width − edge`", or "read one radius variable for both".
- Report that edit in the finding.
- Have the viewer offer it as a one-click edit (CHROME/DOORS seam).

Where no such edit exists (imported geometry with fixed numbers, a value inside a part not exposed at its doors, a quantity only an operation computes), say so; the assertion is then the recourse.

**Design question.** How to search for the edit: rung two's symbolic residual, solving for one written leaf at a time, is the obvious candidate. It goes to the designer pair before a build.
