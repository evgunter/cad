---
id: shell-nesting-witness-refuses-on-a-point-scoped-reading
kind: issue
title: validate::witness_insides refuses check 10 on any witness's refusal, where a refusal about that one point could be retried at the next vertex
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Unmeasured: no fixture is known to reach it. Found by the sweep of
CLEAVE's `cleave/ray-walk` branch, which made the boolean's shell
witness ladder (`boolean::shell_witness::complex_side`) try the next
witness on every refusal about one point
(`PointInSolidError::inconclusive`, which now also holds the confined
limits: `PartialConeFace`, `EdgeCarrierUnsupported`,
`WallOutlineUnsupported`).

`validate::witness_insides` (`crates/topo/src/validate.rs`), check 10's
witness loop and the result sort's, tries each vertex of a shell
against every other shell. `OnBoundary` moves on to the next vertex
(`continue 'witness`), but any refusal returns `Insides::Refused(e)` at
once, an in-band reading about that one vertex included. Under tier 3's
premise (shells do not cross) every point of a shell answers the same,
so a refusal about one point says nothing another vertex could not
settle.

The likely fix is the shell witness ladder's: on
`e.inconclusive()` move on, keep the first, and refuse it only if no
vertex reads. Whether check 10 means to be stricter than the boolean
here is the question to settle first; nothing in the function's docs
says it does.
