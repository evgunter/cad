---
name: cad-working-style
description: How Ev works on the CAD project — discuss/ratify in DESIGN.md before implementing
metadata: 
  node_type: memory
  type: feedback
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Design decisions are discussed in chat, refined through Ev's pushback,
then ratified into docs/DESIGN.md and committed — keep the doc synced
with every settled point, and for genuinely-open questions propose with
a firm recommendation and honest counterarguments rather than deciding
unilaterally. Prefer principled/structural solutions over escape hatches
— Ev deliberately omits fallback variants as a design challenge.
Fail-loud over limp-along, always. See [[ev-profile]].

**Doc prose discipline (Ev).** Living docs state the PRESENT design
only — no history narration (that lives in the M-logs, PR descriptions
and git), and no references to things not planned. When retiring a
mechanism, a one-line pointer at the log/PR suffices; cut the story.

**Writing memories (Ev, 2026-08-18).** `MEMORY.md` is read at the
start of every session and its pointers are followed as relevance
dictates — so an index line is paid for every session, and a file is
paid for whenever anyone follows its pointer, which for the
operational ones is most of them. Cost scales with read frequency,
and the justification has to clear two tests:

- **Is it unnecessary?** If deleting it would cost little, delete it.
  That covers the obvious, the repetitive, and anything already
  enforced by CI or by a script — but also any guard against a CHEAP
  failure. Nothing here is deployed; main going red and getting
  caught on the next PR is barely an issue, and checking harder up
  front usually costs more than letting the gate catch it.
- **Is it harmful?** If following the rule is worse than having no
  rule, delete it. Mostly this is the previous case with a price tag:
  ceremony that buys less than it costs, or a shaky heuristic wired
  to a destructive action.

Then, for what survives:

- **Be as concise as the content allows.** A rule is one imperative
  line. The incident that earned it is not part of the rule.
- **Git history exists.** Do not carry mentions of things tried and
  reverted, retired, or superseded — whoever needs that finds it in
  the M-logs, the PR descriptions, or `git log`. "So it is not
  re-derived from first principles" is not a reason to keep it in a
  file everyone reads every day.
- **No live counters.** Ordinals, schema versions, block/slot state,
  in-flight status: point at the one authoritative place instead. A
  second copy is stale the moment it is written, and it is the copy
  people read first because it is shorter.
- **No specific measurements.** A memory states a durable rule; the
  reading that argued for it is not the rule. If the number was
  evidence — a timing, a size, a share — keep the rule and drop it;
  git history holds the argument. If something currently decides
  against it, it is live data: point at the register or the constant
  that re-takes it, the way the index's `K telemetry state` line
  does.

**Code comments (Ev, 2026-08-21).** The same tests apply, with a
lower bar: keep a comment only if something would go wrong without it.
The obligation a caller must uphold, why a match is exhaustive, why an
API is private, why a panic path is absent, what a refusal means, a
hazard invisible from the code — those stay. The incident that produced
a rule, a dated timing, a count of call sites, an argument with a
position nobody holds any more: git history has them. A style pass that
finds a smell and NARRATES it in place has not fixed it.
