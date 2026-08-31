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
