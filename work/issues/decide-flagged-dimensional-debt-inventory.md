---
id: decide-flagged-dimensional-debt-inventory
kind: issue
title: Dimensional-debt inventory — the 11+1 decide_flagged sites (F2/F6/F7/F10/F13/F14/F15 families)
status: open
opened: 2026-08-06
github: 214
refs: [213, 501, 502, S13]
---

## From GitHub issue 214

opened 2026-08-06, 1 comment.

Ev's ask on #213: the `decide_flagged` debt lane needs concrete scheduling. The inventory IS the grep — `grep -rn decide_flagged crates/` lists every site with its ledger row id as a compile-time argument; docs/predicate-dimension-audit.md holds each row's dimensional analysis and deferral disposition. Families: **F2** ×4 (validate.rs sense/loop checks), **F6** ×3 (chart_arms residue), **F7** ×1 (fitted-lane NURBS span), **F10** ×1 loop covering 7 checks (transform.rs), **F13** ×1 (cone-nappe fallback cosine — N5 branch-selection family), **F14** ×1 (revolve angle-vs-τ radians), **F15** ×1 (revolve_axis_dir_in_plane unit-dot sine — the #213 review's catch, with its scale-blindness probe as the standing pin). Each retires by giving its family the honest metering (per-kind lever or restatement), shrinking the grep to zero; sequencing: opportunistic riders on units already touching each family (the F13 fix rides the next cone-chart unit, F14/F15 the next editor-core wire unit), with a standing rule that NO new decide_flagged site ships without a ledger row. The count is asserted in the census suite, so silent growth fails a test.

## Comments

**2026-08-15** — orchestrator:

(M8 orchestrator) F6 and F7 are RETIRED by PR #502 (M8-F67, merged 2026-08-15): the F7 bare rate and both census F6 interval_forward sites now gate one metred quantity (knot-domain length × certified speed lower bound, collapsed-arm at every lane), the cone azimuth headroom takes its real lever, and the decide_flagged census asserts **8** (was 12): F2 ×4, F10, F13, F14, F15. The K stream took zero outcome changes (1.82M-row diff, reviewer-reproduced) — no baseline re-cut. The excluded topo::pcurves arm residue is #501. The audit doc's rows were truth-passed in the same unit (incl. the formerly-false trim_containment FLAG).

## Home

`work/issues/`: the remaining families span `validate.rs`, `transform.rs`, the cone charts and the editor-core wire, so no single open program's territory or charter owns the inventory; the code-quality register cites it from `S13` but does not carry it.
