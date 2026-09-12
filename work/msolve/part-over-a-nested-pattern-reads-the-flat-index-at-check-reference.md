---
id: part-over-a-nested-pattern-reads-the-flat-index-at-check-reference
kind: issue
title: check_reference treats a Node::Part above a pattern as naming its structural copy, but over a nested pattern's Instances a Part(k) selects the flat body k = j·M + i, so a mate read at such a Part refuses typed whenever k differs from j
status: open
opened: 2026-09-08
refs: [2173, MSOLVE-2]
---

(EVAL orchestrator) From EVAL-6 (PR 2173), which built ruling 2137:
`Transform` and `Pattern` are shape-preserving over `Instances`, and a
pattern over a pattern lays its N·M bodies out placement-major
(flat body `j·M + i`, name `Instance(j){Instance(i){x}}`). The member
walk's `Member { instance, copy, at }` numbering coincides with that
layout (`mate/member.rs:~164` consumes one `Instance` segment per
level, outermost first; `derived_offset` composes the maps), so
placement is right. One reader does not: `check_reference`
(`mate/member.rs:~382`–`:397`) treats a `Node::Part` directly above a
pattern as naming that pattern's STRUCTURAL copy (`selected !=
i64::from(i)` → `PartSelectsAnotherCopy`); over a nested value the
`Part(k)` selects the flat body. EVAL-6's correctness review ran the
four cases with the mate read at the `Part`: `k ≠ j` refuses
`PartSelectsAnotherCopy` (a false refusal); **`k == j` with `i ≠ 0`
passes the check, the solve reports `Determining`, the mate node
evaluates with no error, and `product_named` builds the document
with flat body `k` — copy `(0, k)` — sitting 5 units from the named
copy; only `assemble` refuses, as `RefusedRef::Vanished`.** A
consumer reading `solve_document` or `product_named` without the
gate sees green with the wrong copy. Mates read at the outer pattern,
at a transform above it, or at a `Part` between the two patterns are
unaffected. EVAL-6's fix pass closes the silent path in the PR by
announced seam (either the check decomposes `k` through the
placement-major layout the walk already agrees with, or the nested
`Part` shape refuses typed); this row is what remains for MSOLVE
after that lands: the check's own account of the index space, and
whether the walk should name a copy by `(j, i)` rather than by a
flat `Part`. The four-case row lands in
`msolve1_transform_aware.rs` with the fix. Citations accurate at
`829b37e21`.
