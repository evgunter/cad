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
things to batch-implement silently. See [[evan-profile]],
[[cad-project-state]].

**Doc prose discipline (Evan, 2026-07-28/29, #124 round 11 +
#132)**: living docs state the PRESENT design only — no history
narration (that lives in the M-logs, PR descriptions, and git),
and no references to things not planned ("the docs don't need to
reference things that we don't plan on doing unless it's a change
from before"). When retiring a mechanism, a one-line pointer at
the log/PR suffices; cut the story.
