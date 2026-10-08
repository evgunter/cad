---
id: inline-merges-variables-by-equal-value
kind: issue
title: Inline merges a part's variable into a same-named host variable when their definitions are bit-equal: identity inferred from equal values, which D10 rules out
status: closed
branch: intent/inline-carries-variables
pr: 4243
opened: 2026-10-03
closed: 2026-10-07
priority: P0
cost: M
refs: [variables-replace-the-parameter-table]
---


`refactor::inline` (`crates/editor-core/src/refactor.rs`) merges a
part's parameter into the host's parameter of the same name when the
two declarations are `bit_eq`, and refuses `VarNameConflict` otherwise.
Under D10 and VARIABLES-DESIGN VR1–VR2 identity is minted and a name is
only text: two variables whose values agree are two variables, and a
name collision is a naming question, not an identity one. The
variables unit (`docs/INTENT-VARS-1-SPEC.md`, ruling Q4) keeps today's
merge so that unit changes one thing at a time; this row decides what
inline does instead (carry the part's variables as new ids and resolve
a name collision by refusing or by leaving the carried one unnamed) and
builds it. Found by the spec draft for that unit.

## Closed

Inline carries every part variable under an id the host mints and
refuses `VarNameConflict` on a name the host holds, equal values or
not. FORK-6 (`work/intent/log.md`) settled the split half: split moves
a variable with its readers, so `inline(split(d))` is `d` up to minted
ids.
