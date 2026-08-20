---
name: cad-working-style
description: How Evan works on the CAD project — discuss/ratify in DESIGN.md before implementing
metadata: 
  node_type: memory
  type: feedback
  originSessionId: 11974b46-1641-48d9-9802-fdf44dcb6927
---

Design decisions are discussed in chat, refined through Evan's pushback,
then ratified into docs/DESIGN.md and committed — keep the doc synced with
every settled point, and present proposals for genuinely-open questions
rather than deciding unilaterally.

**Why:** the doc is deliberately the durable contract across
sessions/agents; conversation is where refinement happens. Evan's pushback
has consistently improved the design (intensional edge geometry,
dropping the decision log, import-as-adoption, prefer-intrinsic rule all
originated from or were sharpened by their objections).

**How to apply:** for open questions, propose with a firm recommendation
and honest counterarguments; expect and welcome refinement. Prefer
principled/structural solutions over escape hatches — Evan deliberately
omits fallback variants (e.g. no `Explicit` edge geometry) as a design
challenge. Fail-loud over limp-along, always. During implementation,
treat the deferred Q1 items as design discussions in the first PRs, not
things to batch-implement silently. See [[evan-profile]].

**Doc prose discipline (Evan, 2026-07-28/29, #124 round 11 +
#132)**: living docs state the PRESENT design only — no history
narration (that lives in the M-logs, PR descriptions, and git),
and no references to things not planned ("the docs don't need to
reference things that we don't plan on doing unless it's a change
from before"). When retiring a mechanism, a one-line pointer at
the log/PR suffices; cut the story.

**Writing memories (Evan, 2026-08-18).** `MEMORY.md` is read at the
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
