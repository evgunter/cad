---
id: main-red-the-plates-ledger-reads-four-more-calls-since-the-least-turn-reads-no-exactness
kind: issue
title: main red (slow set): the plate's Sym walk ledger reads four more calls on Plain/Assertion and Door/Decision since PR 7434 (the least turn reads no exactness off the nominal)
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by `oneround/box-region` (PR 7225) while running its re-pinned slow rows at three ε after merging main.

**What is red.** `editor-core m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` is in the slow set. On main `2130aa45` it reads, against main's pin:
- `Plain/Assertion`: calls 1047 → 1051, forms 12224 → 12370;
- `Early/Assertion`: calls 1047 → 1051, forms 12674 → 12820;
- `Door/Decision`: calls 550 → 554, forms 12366 → 12512;
- `Plain/Decision`: calls and forms unmoved, its digest `2e82fe8f…` → `31fb2516…`;
- `Early/Decision`: unmoved.

It reads the same at ε = 1e-9, 1e-6 and 1e-12.

**Bisected** first-parent over main from `a3d5ce89` (the pin reads) to `2130aa45`: the first bad commit is `ccabcfb7`, the merge of PR 7434 (`doctail/the-least-turn-reads-no-exactness-off-the-nominal`).

**What is owed.** Say which of the PR's reads are the four new calls on each walk. For the forms, say whether they are the price of the least turn read as asked or a read the plate need not make. Then re-pin. PR 7225 re-pins the row at its own reading: this move plus its own +12 forms on three lines, both recorded on `work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`. So main reads green once PR 7225 merges, and this row is the cause's account, not a red that stays.

**The pin is gone (2026-11-23).** PR 7501's merge `43bbb727` (`ciw/census-digest-goldens`) retired `PLATE_LEDGER` and its row. The plate is now held by the growth guard (`the_plates_largest_form_is_under_the_growth_guard`), so the red this row records no longer exists on main. What PR 7434 owes for the four calls (which reads they are) stands as a question, not a red.
