---
id: bare-rounding-doors-are-read-in-column-zero-only
kind: issue
title: bare-rounding reads running.rs's pub items in column 0 only, so an attributed one-line or macro-made door is missed silently
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6640
---


Found by the delta review of PR 6620 (`onecopy/gate-blind-spots`, 170697bd). All of these are NOTE-grade; that PR merged with them open.

1. **Door reading.** `doors()` reads only `pub` items that start in column 0. Two shapes are missed silently, with no UNRECOGNISED:
   - a one-line attributed `#[inline] pub fn x`;
   - a `pub fn` emitted by a macro.

   This is the same class the PR closed: a hand-written assumption about where a door may sit. Read items with a real item parser, or red on any `pub` token that the parser cannot classify.
2. **Crate alias scope.** A crate alias binds file-wide, not per block. With `mod n { use other as gc; gc::rounding_charge(..) }`, the call is counted when another module of the same file aliased `geom_core` as `gc`. This over-counts, so it reds; it is latent.
3. **`running` module.** `modules` always contains `running`, so `crate::running::dot(..)` in any crate is counted. Latent; it reds.
4. **Indirect crate alias.** `use pncad::geom_core as gc; gc::rounding_charge(x)` is not counted, because `lead(segs) == ["geom_core"]` only accepts a direct alias. The header says "a crate alias of it" with no qualifier.
5. **Header and printf.**
   - The new "counts are compared with the tree…" paragraph sits inside the WHAT REDS list, which orphans `REASON`.
   - The self-test summary has a stray ",,".
   - The README entry wording ("a formula or a stored form owed a re-posing") does not cover the `oracle`, `exact-arith` and `fixture` rows.
