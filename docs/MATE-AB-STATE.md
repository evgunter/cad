# S-MATE A/B block state (branch-side; not for reviewer eyes)

Block MATE-B1 (v6 triple {opus, opus, fable}) drawn 2026-08-31:
/dev/urandom byte **108** (< 252, accepted), 108 mod 3 = **0** ⇒
fable in slot 1. Triple: slot 1 FABLE · slot 2 OPUS · slot 3 OPUS.

- Slot 1 → MATE-1 (issue 945; difficulty pre-logged M/L in the
  plan's opening commit, recorded numeric M at dispatch).
- Slots 2–3 unconsumed.

MATE-1 dispatch consumed slot 1 (FABLE) — 2026-08-31, spec
recorded numeric M. Dual drawn at review dispatch: byte 180,
parity 0 ⇒ R1 OPUS + R2 FABLE, concurrent on frozen 1e5cf098
(the claim is on main). Slots 2–3 unconsumed.

MATE-2 dispatch consumes slot 2 (OPUS) — 2026-08-31, difficulty M
(pre-logged in the plan). Spec docs/MATE-2-SPEC.md on main
(PR #1413). Slot 3 (OPUS) unconsumed.

MATE-3 dispatch consumes slot 3 (OPUS) — 2026-08-31, difficulty L
(pre-logged in the plan). Spec docs/MATE-3-SPEC.md on main
(PR #1414). BLOCK MATE-B1 FULLY CONSUMED.

Block MATE-B2 (v6 triple {opus, opus, fable}) drawn 2026-08-31:
/dev/urandom byte **71** (< 252, accepted), 71 mod 3 = **2** ⇒
fable in slot 3. Triple: slot 1 OPUS · slot 2 OPUS · slot 3 FABLE.

MATE-6 dispatch consumes B2 slot 1 (OPUS) — 2026-08-31, difficulty
M (pre-logged in the plan). Spec docs/MATE-6-SPEC.md on main
(PR #1414). B2 slots 2–3 unconsumed.

MATE-2 dual METHOD NOTE (recorded 2026-08-31, BEFORE R1 dispatch):
SEQUENTIAL same-head on frozen c27ecb5a, per the v6 contention
posture — two sibling implementer lanes (MATE-3, MATE-6) hold the
machine's stated 3-lane budget, so R1 runs now and R2 dispatches
from the stored brief VERBATIM at R1's delivery. Briefs symmetric,
stored at docs/MATE-2-REVIEW-BRIEF.md on this branch pre-R1; only
the R-label and probe branch name substitute. Byte 212 parity 0 ⇒
R1 OPUS + R2 FABLE (claimed on main, PR #1419).

MATE-6 dual METHOD NOTE (recorded 2026-08-31, BEFORE R1 dispatch):
SEQUENTIAL same-head on frozen 65fcc134 — the three-lane budget is
held by the MATE-3 implementer, the MATE-2 fix pass, and MATE-6 R1;
R2 dispatches from the stored brief VERBATIM at R1's delivery.
Brief stored at docs/MATE-6-REVIEW-BRIEF.md on this branch pre-R1;
only the R-label and probe branch substitute. Byte 8 parity 0 ⇒
R1 OPUS + R2 FABLE (claimed on main, PR #1424).

MATE-4a dispatch consumes B2 slot 2 (OPUS) — 2026-08-31, difficulty
M (pre-logged in the plan's MATE-4 entry, impl half). Spec
docs/MATE-4A-SPEC.md on main (PR #1430). B2 slot 3 (FABLE)
unconsumed.

MATE-5 dispatch consumes B2 slot 3 (FABLE) — 2026-09-01, difficulty
L (pre-logged in the plan). Spec docs/MATE-5-SPEC.md on main
(PR #1438). BLOCK MATE-B2 FULLY CONSUMED.

Block MATE-B3 (v6 triple {opus, opus, fable}) drawn 2026-09-01:
/dev/urandom byte **173** (< 252, accepted), 173 mod 3 = **2** ⇒
fable in slot 3. Triple: slot 1 OPUS · slot 2 OPUS · slot 3 FABLE.

MATE-8 dispatch consumes B3 slot 1 (OPUS) — 2026-09-01, difficulty
M (pre-logged at spec). Spec docs/MATE-8-SPEC.md on main
(PR #1470). B3 slots 2–3 unconsumed.

MATE-7a dispatch consumes B3 slot 2 (OPUS) — 2026-09-01, difficulty
L (pre-logged at spec). Spec docs/MATE-7A-SPEC.md on main
(PR #1471). B3 slot 3 (FABLE) unconsumed.

MATE-8 dual DRAW (2026-09-01, recorded at dispatch): CONCURRENT on
frozen 11947309b — the three-lane budget holds (MATE-7a implementer
+ R1 + R2). Byte 50 parity 0 ⇒ R1 OPUS + R2 FABLE (claimed on main
as ordinal 1306, PR #1476). Briefs symmetric, stored at
docs/MATE-8-REVIEW-BRIEF.md on this branch pre-dispatch; only the
R-label and probe branch substitute ({R}=R1 ⇒ mate/8r1-probes,
{R}=R2 ⇒ mate/8r2-probes; {OTHER} = the sibling's label+branch).

MATE-7a dual DRAW (2026-09-01, recorded at dispatch): CONCURRENT on
frozen 530eb8f59 — the three-lane budget holds (MATE-8 fix pass +
R1 + R2). Byte 41 parity 1 ⇒ R1 FABLE + R2 OPUS (claimed on main
as ordinal 1307, PR #1482). Briefs symmetric, stored at
docs/MATE-7A-REVIEW-BRIEF.md on this branch pre-dispatch; only the
R-label and probe branch substitute ({R}=R1 ⇒ mate/7a-r1-probes,
{R}=R2 ⇒ mate/7a-r2-probes; {OTHER} = the sibling's label+branch).
