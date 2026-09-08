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
`Part(k)` selects the flat body, so a mate read at such a `Part`
refuses typed whenever `k ≠ j` — a false refusal, never a wrong
placement. Mates read at the outer pattern, at a transform above it,
or at a `Part` between the two patterns are unaffected. The fix is
one index-space translation at the check (or reading the flat index
through the same layout the walk already agrees with). Citations
accurate at `829b37e21`.
