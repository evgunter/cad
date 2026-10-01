---
id: approx-capped-box-fixture-runs-the-mint-and-pins-the-minted-body
kind: issue
title: box_with_approx_cap skips mint_pcurves, so the suite pins the un-minted body's refusals and never the meshable Approx-capped box
status: open
opened: 2026-09-22
priority: P3
cost: E
refs: [tessellate-refuses-approx-face-without-caches]
---


Filed by the TESS orchestrator from the Approx-face survey
(`tess/approx-face-survey`, `d423d1b46`), 2026-09-22.

`crates/sweep/tests/common/approx.rs::box_with_approx_cap` builds the
`Approx`-capped box through the public doors and deliberately does not
call `topo::mint_pcurves`. Since EXCH's PR 1798 (the LINE-carrier limb
of the seam class, 2026-09-17) the mint succeeds on that body and it
then meshes (6 patches), weighs (volume 4.000…) and passes tier-3
check 7, at six (d, ε) rows — measured by the survey's probe
(`crates/sweep/tests/tess_approx_survey_probe.rs` on the branch). The
suite's only rows on this fixture (`verbs_offc_consumer::the_walls_a_
placed_approx_capped_part_still_meets`) pin the UN-minted body's
refusals and would keep passing whatever the mint did. Wanted: the
helper runs the mint (or a sibling does), and a row asserts the minted
body meshes, weighs and validates — the only in-tree `Approx` body
built through the public doors that exercises the LINE limb. The
survey's probe is the row's starting point.
