---
id: inherited-red-is-not-attributed-to-its-merge
kind: unit
title: A red inherited from main is not attributed to the merge that caused it - the diagnosis is re-derived by every lane that trips over it
status: open
opened: 2026-09-07
---


Opened 2026-09-07 out of Ev's F3 ruling
(`f3-recosting-on-a-public-repo`), which declines the post-merge run on
the grounds that nobody would read it. That is right about detection and
it leaves the thing the recorded instances actually cost.

## What the two instances cost, and it was not detection

`main` was non-compiling twice on 2026-09-04. **Both were found**, and
found quickly, because a red `main` reddens the next PR's merge ref —
that is F3's stated compensating control and it works. What it does not
do is say **whose** break it is.

Measured, from `merge-order-semantic-break-reaches-main` and
`tree-wide-guards-outside-the-change-closure`: **42 red runs on 20
branches, four innocent branches**, 34 m 25 s of non-compiling `main`,
and two agents diagnosing the same one-line break in the same hour
because neither could see the other doing it. Every lane that tripped
over it paid the same diagnosis independently, and the diagnosis is not
hard — it is "is this red mine, or did I inherit it" — it is just
invisible from inside a single PR.

## The convention this automates already exists

Ev, in chat 2026-08-31: a red **inherited** from `main` — reproduced on
`main`'s own tree, not the PR's — does not block the merge, but it must
be annotated on the PR with its issue, and the lane that caused it owes
the fix. That is written into `memories/agent-lane-operations.md` as a
rule a human follows by hand, at the moment they are least equipped to:
mid-diagnosis, on someone else's break.

## The shape, and what it is not

**Not a gate.** It changes no verdict, blocks no merge and adds nothing
to a green run. On a run that has **already failed**, it asks whether the
same failure reproduces on `main`'s own tree, and if it does, it says so
and names the merge that introduced it.

**Not the post-merge run under another name.** It detects nothing the
tree does not already detect — the red is already there, on the PR, in
front of someone. It answers a question that red does not answer.

**Not a watcher.** It lands where a person is already looking, which is
the whole of Ev's objection to the push gate, and is why this survives
that ruling rather than being closed by it.

## What is unmeasured, and must be before it lands

- **The cost.** One job on an already-red run, but "reproduce on `main`'s
  tree" is a build, and its size is exactly what the F3 measurement says
  is expensive. Whether it can be scoped to the failing row rather than
  the workspace is the number that decides the shape.
- **The attribution.** Naming *which* merge introduced a break is a
  bisect in the general case. The cheap version — "the failure is on
  `main`'s tip too, here is the tip" — is most of the value at a fraction
  of the cost, and may be all of it.
- **Whether it fires often enough to be worth having.** Two instances in
  one day is two samples, not a rate. The population is derivable from
  the run history the F3 measurement already walked.

No lane should design this before those three are taken. The unit is
opened with the numbers named rather than dispatched.
