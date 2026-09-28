---
id: a-shell-role-is-decided-by-two-spellings
kind: issue
title: a shell's Outer/Void role is decided by two spellings: check 7's plus_v_decide (tier 3, check 10) and props' chk_shell_volume_sign (classify_shells, the SHELL verbs)
status: open
opened: 2026-09-27
priority: P4
cost: E
refs: [tier-3-does-not-check-shell-roles-per-solid]
---



From ATREST-7's review (PR #3301, Q1). The decision "`Outer` if the
volume bracket's low end is definitely positive, `Void` if its high end
is definitely negative" had three spellings on the branch; ATREST-7
removed its own (`shell_winding_role`) and check 10 now decides a role
with check 7's `plus_v_decide` and its predicates `positive_volume` /
`positive_volume_enclosure` (`validate.rs`, `shell_role`). One remains
beside it: `props::classify_shells_via` decides the same thing under
`chk_shell_volume_sign`, at the REPORTING level, and refuses typed on a
zero or straddling bracket rather than answering "undecided" — which
is what `classify_shells`/`classify_shells_of` and, through them, the
SHELL verbs' `OperandOuterShells` gate (`shell.rs`, the per-solid role
read) and the census consume.

The two differ in contract (sign-level walk that may stop early vs a
reporting read that refuses), so they are not interchangeable today;
what could be shared is the enclosure-to-role reading itself, one
function in `props` both call. Not done in ATREST-7 because the
`chk_shell_volume_sign` rows are pinned by name in several suites
(`shell_census_is_thread_count_invariant`, `dsc_checks`, the shell8
probes) and moving them is SHELL/CENSUS ground.
