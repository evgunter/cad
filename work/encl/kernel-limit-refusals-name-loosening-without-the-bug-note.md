---
id: kernel-limit-refusals-name-loosening-without-the-bug-note
kind: issue
title: kernel-limit refusals say 'loosen the tolerance' without the D4 last-resort note that this may be a kernel bug
status: dispatched
opened: 2026-09-28
priority: P3
cost: E
---


## What

The D4 ¶1 ruling (Ev, `[ev]` PR 3352, 2026-09-28) keeps "loosen the tolerance" only as a last resort: in a refusal that would otherwise name no recourse at all, i.e. a kernel approximation limit. In that case it also says the refusal may indicate a kernel bug worth reporting. Today's sites say "loosen" with no such note.

**Offset fit (ENCL).** The `OffsetFitError` Display arms name it at:
- `crates/geom-brep/src/offset_fit.rs:756`, `:781`, `:803` and `:814` (stall, budget, best-bound);
- `crates/topo/src/validate.rs:2358`, `:2365` and `:2395`, where the checks window mirrors them (`classify_offset_fit` DRIFT).

**Mass properties (PROPS).** `crates/geom-brep/src/props/mod.rs:588` ("loosen the tolerance or simplify the trim").

For each site, first check that it really has no other recourse:
- "split the face" and "simplify the trim" are geometry levers, so those sites are not last-resort sites. There the loosening clause should likely go.
- Where loosening is the only recourse left, add the bug note, in one shared constant beside `KERNEL_DEFECT_ENDING` in `geom_core::predicate`.

The PROPS site is PROPS's call. This row carries the ENCL sites and notes that one.
