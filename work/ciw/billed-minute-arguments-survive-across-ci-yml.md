---
id: billed-minute-arguments-survive-across-ci-yml
kind: issue
title: ci.yml still argues live configuration decisions in billed minutes, including one that decides the shard matrix
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator, out of the style review of
S-TCOST PR 2434. `.github/workflows/*` is CIW's by `work/ciw/program.md`,
and `work/ciw/f3-recosting-on-a-public-repo` is the row that established
the premise, so this lands here rather than travelling.

## The premise, already ratified and not re-opened here

`evgunter/cad` went public on **2026-09-03**; standard-runner minutes are
free. `work/ciw/plan.md` §The 2026-09-04 re-read rules that
`docs/CI-MINUTES-2026-08.md`'s opening premise is dead and **no figure
from it may be quoted forward**.

## What PR 2434 did, and the gap it left

That PR restored three checks whose demotions had been bought in billed
minutes, and rewrote the three demotion notes the item named. **It swept
the three comments and left the class in the file it was editing** —
`docs/prompts/reviewer-style-lane.md`'s standing trap, *"when the diff is
itself a fix for a structural finding, check whether the fix mints a
fresh instance of the defect it closes"*, landing again.

This row is that class. It is not a criticism of the PR's scope: the item
named three comments and the PR did three. It is the sweep nobody has
run.

## Two sites read directly, because they are not the same case

**Site 1 — a residue, and straightforwardly stale.** `ci.yml`, the
doc-gate caching step's F6 write-up:

> `scripts/doc-gate.sh` 67 s -> 33 s / whole job 136 s -> 99 s, **i.e. 3
> billed -> 2 billed**

A pre-public reading, in a currency that no longer exists, citing F6 in
the dead ledger. Nothing decides on it today; it is text that will be
quoted forward by the next reader who needs a number. **Straightforward
rewrite or deletion.**

**Site 2 — a LIVE DECISION, and the interesting one.** `ci.yml`, the
block deciding *not* to merge the two test shards:

> From the figures in `docs/CI-MINUTES-2026-08.md` (F2) ... so 2 billed
> minutes each; merged they are ~139 s, which is 3. That is ONE billed
> minute saved for ~66 s of added latency on a leg that sits directly
> behind a build job on the critical path.

**Read in full, this block's conclusion survives its own dead premise,
and that must not be lost in the fix.** It leads with billed minutes, but
what it actually weighs them against is *"~66 s of added latency ... on
the critical path"* — and latency is the currency that still decides.
Strip the minutes and the argument reads: merging the shards adds ~66 s
to a critical-path leg and buys nothing. That is stronger now than when
it was written, because the thing on the other side of the trade went to
zero.

So the fix here is **not** to re-decide the knob. It is to restate the
argument in the currency that decides it, and to stop the block citing a
ledger no figure may be quoted from. A reader who notices the dead
premise and re-opens the decision would be re-deciding it correctly and
wastefully.

## The rest of the class

The style review named further candidate sites in `ci.yml` and did not
read them all: sites near the k-lint siting note, the ruff and python
clippy siting notes, and several others it listed by line. **Those line
numbers are not reproduced here — they rot, and the review read only
two of them.** The sweep is the work: `git grep -n "billed" .github/`
plus `git grep -n "CI-MINUTES-2026-08" .` and a read of each hit.

**Per hit, the disposition is one of three**, and they are genuinely
different:

1. **Residue** — a figure nothing decides on. Rewrite or delete.
2. **A live decision whose conclusion survives** (site 2). Restate the
   argument in wall clock; do not re-open the decision.
3. **A live decision whose conclusion does NOT survive.** Then the knob
   is genuinely open and gets its own measured PR — and under S-TCOST's
   rule, which CIW's own `keep_out` cedes to (*"CI build knobs
   (profile/cache/sharding) stay S-TCOST's rule — measured in-unit or
   not at all"*), that measurement is the deliverable and not the
   argument.

Sorting each hit into those three is most of the work, and getting it
wrong in the direction of (3) is how a correct configuration gets
churned.

## Two things this row does not touch

- **`docs/CI-MINUTES-2026-08.md` itself.** PR 2434 annotated three of its
  headings rather than editing them. The style review's separate finding
  — that an *unannotated* section still carries a live-tense roster of
  nightly jobs summing to "an ordinary night ~15" — is about that
  document, not about `ci.yml`, and is a different row.
- **The shard COUNT.** `work/tcost/nextest-shard-count-needs-remeasure`
  is S-TCOST's, live, and being measured now. Site 2 is about merging the
  two shards into one, which is a different knob. The S-TCOST lane has
  been told site 2 exists so its own verdict does not land beside a
  contradiction.

## What the review's pattern could not match

It searched for `billed`, `CI-MINUTES-2026-08` and job names. A comment
arguing from minutes without the word "billed", or citing the ledger's
figures without its filename, matches none of it — and the review says
so. `ci.yml` is 5 265 lines of which roughly 76 % are comments, so the
population is large and only partially seen.
