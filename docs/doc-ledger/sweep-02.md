# Sweep 2 — 2026-08-20: the second smell scan is folded into the first

**`docs/SMELL-SCAN-2-2026-08.md`** — deleted, and its entire content
carried into **`docs/SMELL-SCAN-2026-08.md`**: findings S59–S116 with
their tiers, its §A as **§A2**, its §B as **§B2**, and its process
observations as **C18–C25** inside §C.

Recover the file as it stood at the merge with
`git show <this sweep's SHA>^:docs/SMELL-SCAN-2-2026-08.md`, or read it
in place — nothing was dropped, and the only edits were the ones the
merge itself required.

**Why it existed and why it does not.** It was written as a separate
file so that a scan landing mid-wave would not collide with the fix
tracks editing the first document. That collision was real. Separation
was the wrong fix for it: the two documents share **one ID space** by
design, so a reader holding an `S`-number could not tell which file to
open, and — the concrete cost — **both files stated the same wrong
number about §C**. The second scan said its process observations
continued *"at C15"*; the first scan's forward pointer said the same;
§C already ran to **C17**. Neither author could see it from inside
their own file. One register, one ID space.

**What the merge changed, exhaustively:**

- the second scan's `# Tier N` headings gained a `Second scan · ` prefix,
  so their anchors no longer collide with the first scan's;
- its `# §A` and `# §B` became `## §A2` and `## §B2`, being about that
  scan's findings rather than this document's;
- its `## C15`–`## C22` became `## C18`–`## C25`, and the sentence
  claiming §C ran to C14 was replaced with one that says what happened;
- its `## Contents` list was dropped into this document's own Contents.

Findings, verdicts (all still blank), citations and prose are otherwise
byte-for-byte as merged.
