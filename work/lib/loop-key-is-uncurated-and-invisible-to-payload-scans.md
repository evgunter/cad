---
id: loop-key-is-uncurated-and-invisible-to-payload-scans
kind: issue
title: LoopKey is curated out of step with its sibling keys, and no payload scan can see it
status: open
opened: 2026-09-03
refs: [LIB-CUR4]
---


Two findings that share one cause, both from LIB-CUR4's struct-payload
sweep.

**1. `LoopKey` is curated out of step with its siblings.** The prelude
carries `VertexKey`, `EdgeKey` and `FaceKey` (group 4, through the
`topo` re-export) and does not carry `LoopKey`. But
`ValidationError::RingMeetsOuter` — a prelude-carried refusal — has the
shape `{ face: FaceKey, ring: LoopKey, contact: RingContact }`
(`crates/topo/src/validate.rs:842`), and `RingContactEscalated` beside
it carries `ring: LoopKey` too. So the arm names three key types, two
of which a prelude consumer can spell and one of which it cannot.

This is weaker than the carriage LIB-CUR4 made — a key is bound and
passed on, not branched on, so matchability in the CUR3 sense survives
— but it is an inconsistency in the SAME group of the prelude rather
than a rung below it, and it should be settled deliberately: either
`LoopKey` joins its three siblings, or the reason it does not join them
is written where the other three are listed.

**2. The blind spot that hid it, which is a fact about every payload
scan this program has run.** `LoopKey` is minted by
`slotmap::new_key_type!` (`crates/topo/src/entity.rs:147`), a macro. A
source-level index of `pub struct` / `pub enum` declarations cannot see
it — CUR3's enum-indexed scan could not, and LIB-CUR4's struct-payload
extension could not either, which is why this was found by reading the
arm rather than by the tool. CUR3 named four blind spots (a)-(d) and
LIB-CUR4 closed (a) one rung; this is a fifth:

  (e) types minted by macro are invisible to a declaration-level index,
      so every `slotmap` key in a payload position is unscannable.

A second methodology note from the same sweep, worth recording with it:
a FLAT name index silently takes the wrong definition when two crates
declare the same name. `viewer::blend::BlendError`
(`crates/viewer/src/blend.rs:170`) shadowed
`sweep::blend::BlendError` on the first run and hid the entire fillet
quartet LIB-CUR4 went on to carry. Any future scan must be
crate-aware and restricted to the façade-reachable crate set.
