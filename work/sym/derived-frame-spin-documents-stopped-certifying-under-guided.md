---
id: derived-frame-spin-documents-stopped-certifying-under-guided
kind: issue
title: The in-plane spin and half-spin derived-frame documents certified under Guided at SYM-5 and refuse on main at the wall join, unattributed: the rows are evidence-only, so nothing went red
status: open
opened: 2026-10-13
priority: P1
cost: M
---


## What

SYM-5's PR-2 fix pass (2026-09-14) measured two derived-frame
documents whose frame turns in its own plane. Each is a `FaceFrame` on
the cap of a cube extruded from an authored frame, with a boss on that
frame (`m10_derived_frame_tilted_interval`'s
`sym5_the_reach_on_documents_the_unit_did_not_build`, half `1e-3`).
With rule E on, under `ProfileLift::Guided`:

| document | 2026-09-14 (SYM-5's table) | `main` at `621e036e19` (2026-10-13) |
| --- | --- | --- |
| in-plane spin, `u=(1,t,0)`, `v=(−t,1,0)` | certifies | **1 refusal**, frozen 756, symbolic 693 |
| half spin, `u=(1,t,0)`, `v=(0,1,0)` | certifies, frozen 1467 | **1 refusal**, frozen 756, symbolic 692 |

Both now refuse at the boss's extrude: "the wall join at loop 0
vertex 0 is neither a definite corner nor definitely smooth". The
enclosures are `[0, 1.359e3]` for the spin and `[0, 4.150e1]` for the
half spin. Under `Pinned` both still certify, and with rule E off both
refuse once, as they did on 2026-09-14.

**Nothing went red**, because the row that measures them is
`#[ignore]`d evidence and asserts nothing. The gating rows in that
file (`m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`,
`m10_the_tilt_u_derived_boss_certifies_once_the_read_settles_its_frame`,
`m10_the_start_cap_and_flip_z_certify_as_the_end_cap_does_and_rule_f_is_inert`)
pass on `main`, and none of them builds a spin document.

## Not yet known

- **Which change moved them.** It is unattributed: the frozen count
  fell from 1467 to 756, so a change to the tier, the extrude's wall
  join, or the fixture is possible. Candidates landed between
  2026-09-14 and 2026-10-13, among them:
  - DECIDE-3's definite quadratic;
  - DECIDE-10's atom key;
  - INTENT stage 3's fixture rewrite.
- **Whether the old reading was right.** Whether these documents
  certifying under `Guided` was sound at the time is also not
  re-checked.

## What answers it

1. Bisect the spin row (`CAD_SYM5_CASES="spin derived"`, release)
   between SYM-5's merge and `main`.
2. Then either fix the cause, or show that the old certification was
   unsound and say so here.
3. Either way, the spin and half-spin documents get a GATING row, so
   the next change that moves them goes red.

Found by SYM's re-take of `derived-frame-placement-freezes-on-the-symbolic-lane`
on 2026-10-13 (that item's last section has the full table).
