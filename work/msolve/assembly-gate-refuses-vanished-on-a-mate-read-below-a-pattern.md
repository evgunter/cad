---
id: assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern
kind: issue
title: The assembly gate refuses Reference Vanished for a mate read at the transform below a pattern, though the solve placed it correctly
status: open
opened: 2026-09-05
---


Found by MSOLVE-1's correctness review (PR 1929, NOTE-4), outside the
unit's fence.

Document: `P(T(top))`, a pattern over a transform over an instance,
with a mate read AT the transform (`at = T`, name `top/…`). The walk
admits it, the solve is green (`Determining`), the product gathers
(`base`, `P`, the mate) and copy 0 — which IS `T(top)` — is seated. The
assembly gate (`crates/editor-core/src/assembly.rs`, the mint's
`resolve_face` over the product's name table) then refuses
`Reference { side: B, why: Vanished }`: the bare name `top/…` has no
row in the product's table, because only the pattern is a root and its
rows are `Instance(i)`-qualified. So a correct placement is reported as
a vanished reference — the diagnostic mis-describes what happened.
What the gate should read for a mate whose operand is consumed by a
pattern (copy 0's row? the member's row through the walk?) is the
question; the honest interim is a refusal that says the operand is
not a product root rather than that the name vanished.
