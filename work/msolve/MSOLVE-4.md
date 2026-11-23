---
id: MSOLVE-4
kind: unit
title: A mate's memo key carries the solve's answer
status: closed
opened: 2026-09-05
refs: [mate-memo-key-does-not-carry-the-solve]
branch: msolve/4-mate-memo-key
closed: 2026-09-06
pr: 1960
---


## Spec

`docs/MSOLVE-4-SPEC.md`. Answers `mate-memo-key-does-not-carry-the-solve`
(parked on this unit). Waits on MSOLVE-1 only because both touch the
mate arm of the content key; dispatches the moment 1929 merges.

## Closed (2026-09-06, PR 1960)

Landed: the content key's mate arm feeds the solve's answer (role tag
and fault presence), read once off `op_env.poses` beside the instance's
placement as one `SolveAnswer` and fed per arm; key format v6;
`crates/viewer/src/tree.rs`'s blame corroboration retired, CHROME's
attribution rows passing on the kernel's answer with one row's premise
rewritten as that row's own comment instructed; the reviews' rows
adopted (the finding's shape through the session doors, two faults in
succession, per-mate reuse); the resolver stub hoisted into the shared
test fixture. Verified on the tree: the memo has one reuse site and it
matches only `Ok`, so a fault's content need not feed the key. Reviews:
correctness MERGEABLE on CHROME's bench through the viewer's doors,
keys measured bit-identical on the corpus before the bump; style found
no MAJOR. Filed from the reviews (`work/issues/`): two tag functions
for `ContactClass` in the content key; the eleven resolver-stub copies.
