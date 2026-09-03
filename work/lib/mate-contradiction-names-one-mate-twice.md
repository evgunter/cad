---
id: mate-contradiction-names-one-mate-twice
kind: issue
title: Mate-solve contradiction diagnostics: mates 6 and 6 cannot both hold, and a levered angle printed in metres
status: open
opened: 2026-09-01
github: 1462
refs: [1461]
---

## From GitHub issue 1462

opened 2026-09-01, 0 comments.

Found by the `story_assembly` integration lane (same repro as issue 1461: a FrameCoincidence mate committed with a nonzero clocking rider, refused by `mate_clocking_redundant` at the next evaluation). Two message defects in the refusal's rendering:

1. **The contradiction names one mate twice.** The self-contradictory-rider arm reports `held == added`, so the sentence reads "mates 6 and 6 cannot both hold" — to a user that reads like an indexing bug, and it hides the actual shape (one mate contradicting *itself* via its rider), which the arm knows.

2. **The clash magnitude is a levered angle labeled metres.** The disagreement prints as `1.5707963267948966 m` — θ·arm dressed as a length. Whatever the internal currency, a roll disagreement surfaced to a user wants either the angle with its own unit or an honest "levered by the contact arm" phrasing; a raw π/2 wearing metres is the D6 display discipline stopping one formatter short.

Both are presentation-only — the refusal itself is correctly typed and correctly fires.

(story-suites orchestrator)

## Home

`work/lib/` — S-MATE's `keep_out` puts the refusal-display prose with LIB, and this is entirely presentation of a refusal's rendered message (the D6 display discipline), not the solve itself.
