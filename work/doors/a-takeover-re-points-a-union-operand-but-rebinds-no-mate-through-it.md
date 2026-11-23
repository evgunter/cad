---
id: a-takeover-re-points-a-union-operand-but-rebinds-no-mate-through-it
kind: issue
title: A takeover re-points a ⊔ operand reading the taken body but rebinds nothing read through that ⊔: a mate on its face goes stale
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Found by the PR 6331 correctness review (probe `probe_mate_through_an_operand`). A `⊔` in a body's space reads the body `B` as an operand, a world copy of that `⊔` is drawn, and a small block's copy is rested on the copy's top (a face of `B`, carried through the `⊔` as `From { B, top }`). A fillet of `B` or a split of `B` re-points the `⊔`'s operand to the result. No rebind is written, and the small block's placement fails `Vanished` on the top. A copy of `B` itself, in the same position, is re-seated.

Why: `Takeover::repoint` (`crates/viewer/src/session.rs`) re-points a `⊔` operand (and the product slot) by `DocEdit::SetParam` alone. Only the copies go through the kernel's `Recording::take_over` (`crates/editor-core/src/take_over.rs`), which rebinds what reads a copy. Once its operand is re-pointed, the `⊔` carries `From { result, top' }` where it carried `From { B, top }`, so every name read through it, in it or in a copy of it, needs the same rebind a copy's reader gets.

This is not `a-takeover-leaves-payload-names-and-appearance-read-in-the-copy-stale` (that row is about payload names and appearance keys read in a re-pointed copy; this one is about selections read through a re-pointed `⊔` operand, which `Rebind` already reaches).

**Fix when dispatched:** route the operand re-point through the takeover door's rebinding as well. The successor of `top` is read off the `⊔`'s new table by the operand's new read, and the rebind is sited at every selection that reads the `⊔` or a copy down its chain. Then rows: the probe's fillet and split cases stay seated.
