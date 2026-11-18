# IMPORTGATE model-compare draws, 2026-11-17

Protocol: e1c428efbb (corrected 2026-11-17; first written as d327ffb52, a shallow-clone graft — e1c428efbb was the last commit on main touching the protocol before these draws; 083ca253dc landed at 18:25Z, after them). One /dev/urandom byte per draw: even = Opus, odd = Fable. Letters and tiers logged in `work/importgate/log.md` on `claude/importgate-orchestrator-booa6e` (9fc7a0de4) before the draws.

| unit | arm | byte | model | drawn | logged at |
|---|---|---|---|---|---|
| importgate/parse-depth: step-parser-recurses-once-per-nested-list-and-a-deep-file-kills-the-process (letter S, tier single FULL) | implementer | 168 | Opus | 2026-11-17T18:00:05Z | 9fc7a0de4 |
| importgate/knot-place: the-c0-knot-split-refuses-what-it-cannot-place (letter M, tier single FULL) | implementer | 90 | Opus | 2026-11-17T18:00:05Z | 9fc7a0de4 |
| importgate/interp-layer: import-interpretation-layer-reads-the-file-at-its-band (letter H, tier dual) | implementer | 246 | Opus | 2026-11-17T18:12:15Z | 526d3d477 |
| importgate/parse-depth: PR 7069 head 4602a0e7d7 (letter S, tier single FULL) | reviewer | 34 | Opus | 2026-11-17T18:34:00Z | protocol 083ca253dc |
| importgate/knot-place: PR 7084 head 69933c8708 (letter M, tier single FULL) | reviewer | 95 | Fable | 2026-11-17T20:12:30Z | protocol 083ca253dc |
| importgate/interp-layer: PR 7091 frozen head 91b46ffaca (letter H, tier dual, concurrent) | slot order | 95 | R1 = Fable, R2 = Opus | 2026-11-17T21:58:32Z | protocol 083ca253dc |
| importgate/interp-layer: PR 7091 | blind-coding map | 192 | even → A = R1, B = R2 | 2026-11-17T21:58:32Z | |

**Fable outage, 2026-11-18 (rule 12).** The Fable reviewer of PR 7091's confirmation re-review stopped on the usage limit (resets 2026-11-22 14:00 UTC); Ev said proceed on Opus (PR 7091, 2026-11-18T07:17Z). PR 7091 (`importgate/interp-layer`) records **no row**: its Fable re-review half re-runs on Opus during the outage. No draws are taken for `importgate/build` or `importgate/singularity-meet`, dispatched during the outage; neither records a row. Rows already recorded (PR 7069, PR 7084) stand.
