---
id: ciw-rows-and-ci-local-prose-rotted-by-the-c1-c3-restore
kind: issue
title: The C1-C3 restore left three CIW premises stale: a guard count, a nightly tabulation, and ci-local's dispatch comment
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator at the merge of PR 2434,
which restored TCOST-C1/C2/C3 to the pull-request gate and deleted two of
their three nightly counterparts. These three sites are CIW's —
`local-scripts/*` by territory, and two open CIW items that
one-file-one-item forbids another program editing.

**Two other CIW premises are NOT in this row because the same PR
un-broke them**: `work/ciw/facade-guards-defer-to-rustdoc-json` and
`work/ciw/nightly-demotions-have-never-run`'s **rustdoc** row both build
on `rustdoc (gate, every root)` existing, and it does — an earlier
revision of that PR deleted it, a review established the deletion lost
the only full-strength `viewer --all-features` doc pass, and the job was
kept. Worth stating so nobody re-files them.

## The three

**1. `work/ciw/python-suite-zero-test-guard-three-copies` counts three
copies; there are now two.** The item's title, its body and its argument
all turn on the number. PR 2434 removed the nightly re-take that carried
the third. The finding survives — two hand-kept copies of a guard still
drift — but every figure in it is off by one, including in the title,
which is the part that shows on the board.

**2. `work/ciw/nightly-demotions-have-never-run` tabulates jobs that no
longer exist.** Its subject is whether the 2026-09 demotions ever
executed unattended. Two of the rows it tabulates —
`python suite (ungated re-take)` and `release-corruption` — are deleted;
`rustdoc (gate, every root)` remains and its row is unaffected. So the
item is not wrong, it is **partly about a population that is gone**, and
a reader cannot tell which of its rows still have a subject.

**3. `local-scripts/ci-local.sh`'s dispatch comment above the python row
is false in both halves.** It reads *"The hosted half is seed-gated and
the nightly re-takes it"*. The hosted half is no longer seed-gated (it
runs on every code-tier run, gated only on `run_build`) and there is no
nightly re-take to do the re-taking. Found by the fix-pass lane, in the
file it was already editing, and left deliberately rather than fixed
across the fence.

## Why site 3 is the one to do first

It is live prose in an executable file that a contributor reads to
understand what their local gate does and does not cover. The two item
sites are tracker rows a reader will reconcile against the tree anyway;
the `ci-local.sh` comment is the kind that gets believed.

It also sits in the same file as
`work/ciw/ci-local-topo-release-guard-cannot-pass` — a separate row,
filed the same day, about `topo_release()` greping for a row that cannot
exist under the repo's own `[profile.release] debug-assertions = true`.
Whoever opens `ci-local.sh` for one should read for the other; the two
are independent defects that happen to share a file.

## The class, and it is the one PR 2434 named itself

That PR's own sweep note says it best: its greps searched for **names** —
job names, flag spellings, `billed minute`, `CI-MINUTES-2026-08` — and
*"a comment arguing a demotion without naming its job, or prose that
merely assumes the nightly runs one of these rows, is invisible to every
pattern above."* All three sites here are that class, and two of the
three were found by a human reading rather than by any pattern. No
census exists for prose that assumes a CI arrangement without naming the
job that implements it, and none of the structural gates
(`check-ci-mirror-parity.py`, the marker check, the roster gate,
`doc-gate.sh --selftest`) reads prose at all.

## Related

`work/ciw/critical-path-citations-name-a-job-that-is-not-the-pole` and
`work/ciw/billed-minute-arguments-survive-across-ci-yml` are the same
shape from the same week; `work/tcost/nightly-demotions-c1-c3-were-bought-with-billed-minutes`
(closed at PR 2434) is the re-cost that caused this rot.
