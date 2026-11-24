# Design fork: what a leaf whose replay differs from its witness other than by a turned sign is (BOXFINISH)

Protocol: ddd201f411f (`docs/DESIGN-FORK-PROTOCOL.md`, the last commit on `origin/main` touching it at dispatch). Item: `work/boxfinish/a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing.md` (P1, M, design), on `boxfinish/orchestrator` at dispatch. `docs/ERROR-DESIGN.md` E6 decides the case today, so this fork is expected to go to Ev.

## Draws (2026-11-24, 09:14 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 72 | below 86: THREE designers (one Fable, two Opus) |
| labels | 116 | not 255; 116 mod 3 = 2: Fable is C; the two Opus designers are A and B in dispatch order |

Mapping: **A = Opus 5.5 (first dispatched), B = Opus 5.5 (second dispatched), C = Fable 5.2**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

The statement of the problem (identical for all three) is `probe/briefs/designer-parted-path.md` on `boxfinish/briefs` at `416eb1b3c5d`; it names no candidate solution. Exposures to a candidate, as hypotheses: the item says the question is whether a parted path is "refused mass, or a leaf not proven that a narrower box may re-align"; the program's log entry of 2026-11-24 records the orchestrator's withdrawn ruling (a typed not-proven outcome that bisects).

## Sessions (dispatched 09:16 UTC, concurrently, one statement of the problem; checkout `boxfinish/orchestrator` at `cbc9453b153`)

| Label | Model (configured) | Session |
|---|---|---|
| A | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXNnnWWx7RxrwELkDi |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXaM4QLJXkeQhLCfJ6 |
| C | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWXLcZJhHKTUmqHHSMo |
