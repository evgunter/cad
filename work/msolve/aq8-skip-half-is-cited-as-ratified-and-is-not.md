---
id: aq8-skip-half-is-cited-as-ratified-and-is-not
kind: issue
title: AQ8's (b)-SKIP half is cited as ratified across the tree and lives only in a test comment and a commit message
status: closed
opened: 2026-09-04
closed: 2026-09-05
---


Found by PR 1749's review (FIX). Filed here rather than on a program's
slate: the citation is spread across crates and the ratified doc is
shared ground.

## The gap

`docs/ASSEMBLY.md:225-228` ratifies **one half** of AQ8: the weld, and
`TornCluster` when a cut would tear a cluster.

The **other half** — that a mate which is not an A12 edge contributes
nothing to a split's interface crossings, the "(b) SKIP" ruling, with
its ratification condition about trusted-at-rest records — is cited
around the tree as though ratified, and exists only in:

- a test-file comment, `crates/editor-core/tests/asm_r2b_assembly.rs:742`
- a commit message, `8aca95b53` (2026-08-17)

Neither is a ratified home. `CLAUDE.md` puts design for finished work in
a README beside the code, listed in DESIGN.md's companion table; a
comment in a test file is not that, and a commit message is not
reachable by anyone who does not already know to look for it.

## Why it is worth fixing rather than tolerating

The half that IS ratified and the half that is NOT are load-bearing for
different arguments, and the unratified one is doing real work: PR 1749
turns on it (a gate matching a mate head's spelling rather than the
member vocabulary would mint an interface crossing for a mate that never
solved, which "(b) SKIP" is what forbids). That PR expanded the citation
into three new places before the review caught it and had them
re-pointed at where the rule actually lives.

So the citation count is growing against a rule with no ratified text,
which is how a convention becomes load-bearing without ever having been
agreed. The reviewer's framing: a second publication of an unratified
claim is what turns one lane's reading into the tree's assumption.

## Two ways to close it

1. **Ratify it** — the SKIP half joins the weld half in the assembly
   design's ratified home, with the ratification condition stated. If
   the rule is what several units already depend on, this is the
   honest end state.
2. **Demote the citations** — every site citing it says it is a
   convention with a named origin rather than a ratified clause.

(1) is almost certainly right, but it is a design ratification and not
this filing's to make.

## Home

`work/issues/` — the sites span `editor-core` and the doc is shared;
S-MATE's charter covers the assembly design and would be the natural
claimant.

## Closed (MSOLVE orchestrator, 2026-09-05)

The SKIP half was already ratified: Ev's 👍 on the addendum comment of
PR 592 (2026-08-17, "👍 on THIS comment ratifies option (b)") is the
ratification, and the ASM orchestrator's reply on that thread records
it. What was missing was the doc home. Carried into
`crates/editor-core/ASSEMBLY.md`'s AQ8 clause with its condition
(a record for a mate that never solved is trusted-at-rest state), and
the citations that said "not in `ASSEMBLY.md`" re-pointed: the two
test headers here, the two in `mate/solve.rs` and `refactor.rs` on
MSOLVE-1's branch, which rewrites both sites.
