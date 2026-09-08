---
id: dead-code-quality-row-pointers-after-the-tracker-cut
kind: issue
title: D103 and D107 are cited at work/code-quality/ paths that no longer resolve; the rows moved in the 2026-09-06 cut
status: open
opened: 2026-09-08
---


## What

Six files cite `D103` or `D107` at `work/code-quality/` paths that do
not resolve. Both rows moved in the tracker-wide cut of 2026-09-06
(`docs/WORK-TRACKS-2026-09.md`), which re-homed code-quality's rows to
the programs it opened; the citations did not follow.

**Where they are now** (verified, 2026-09-08): `work/gates/D103.md`
and `work/topo/D107.md`. `work/code-quality/` still exists and still
holds other rows, so the directory being present is not evidence a
path in it resolves — which is presumably why these survived.

## The sites

`git grep -c "work/code-quality/D103\.md"` — five files, eight
occurrences:

- `crates/viewer/README.md:764` — the only one in shipped code
- `work/gates/anchored-exact-text-skip-has-three-homes.md:107`
- `work/view/boundary-rule-has-no-mechanical-check.md:100`
- `work/view/log.md:2326`
- `work/view/pick-and-parts-name-the-session-driver.md:69, 110, 151, 184`

`git grep -c "work/code-quality/D107\.md"` — one file, two
occurrences:

- `work/code-quality/d107-release-profile-job-lives-in-nightly.md:10, 49`

That last one is the sharpest: a row **inside `work/code-quality/`**
cites a sibling that is no longer its sibling.

`work/view/log.md:2326` is a dated log entry and its pointer may be
exempt under the usual reading — a log records what was true when it
was written. The other five are present-tense citations and are not.

## Why this is filed rather than fixed

Found by METER unit 8's dead-pointer sweep (PR 2177), which swept the
class rather than the instance: 21 distinct `work/code-quality/*.md`
citations across the tree, **4 dead at 7 sites**. Two of the four were
METER's own and were fixed in that PR. These six sites are outside
METER's fence — `crates/viewer/`, `work/gates/`, `work/view/`, and
`work/code-quality/` itself — so METER reported rather than filed
them, and the orchestrator is routing them here rather than to one
program because no single program owns all six.

The sweep's stated blind spots, so the next reader knows what this
list does not cover: a citation naming a row id without its path, and
a citation in a non-text artefact.

## Why it is filed HERE

`work/meta/`, not `work/issues/`, and not on any one owner's slate.
The precedent is this program's own
`stale-track-t-citations-in-fillet-and-cert` — the same shape, a
stale-citation finding spanning two other programs' slates, held here
and routed rather than fixed. META's `keep_out` is explicit that *"a
stale citation in another program's slate is routed to its owner and
never fixed across the fence"*, and names that row as the standing
instance. This is the second.

The orchestrator filed this into `work/issues/` first and moved it.
That was wrong on `work/README.md`'s own terms: `issues/` is for a
finding whose owner is undecided or disputed, not for one whose owner
is plural, and *"an unsorted pile of related items there is what the
sweep exists to prevent."*

## Why it matters

`work/README.md`'s standing rule is that a census has one executable
home and every other site points at it, **because a pointer cannot go
stale**. A pointer that does not resolve is the counterexample, and
six of them sit in the tracker that states the rule. The cure is
cheap; the reason to record it is that a tracker-wide re-home is
exactly the event that produces this class, and the next one will
produce it again unless the re-home sweeps its own inbound citations.
