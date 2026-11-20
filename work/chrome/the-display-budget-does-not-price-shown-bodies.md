---
id: the-display-budget-does-not-price-shown-bodies
kind: issue
title: the display budget prices the gathered product, not the bodies the author's toggles show
status: open
opened: 2026-11-01
priority: P3
cost: M
---



Found in the STYLE review of PR 6270 (`the-view-pane-toggles-spaces-and-bodies`).

The δ the budget fits (`evalseam::FitService`, handed
`DocSession::fit_request`) prices the landing's gathered product body.
The picture draws more: every drawn body (`world::Picture::of`), and since
PR 6270 every body the author's toggles show
(`world::Picture::showing`, `DisplayState::shown`). A show of a large
operand adds triangles that the budget never counted, so the δ chosen
"for the triangle budget" (the View pane's note) no longer bounds what
is drawn.

**What it should do.** Price the picture the index builds: the
landing's bodies the key's `Shown` draws, or say in the View pane's
note that the budget prices the product alone.

Done when: a row shows an operand much larger than the product and
either the fitted δ moves or the note says what it priced.

Moved from SPACES to CHROME on 2026-11-20, by `git mv`, when SPACES merged into CHROME: SPACES held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CHROME carries.
