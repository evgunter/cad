# Design fork: how a construction asks whether a value is an identity (BOXFINISH)

Protocol: 747202177c (`docs/DESIGN-FORK-PROTOCOL.md`). Item: `work/boxfinish/the-axis-mint-asks-for-proof-only-where-the-value-reads-zero.md` (P1, design), with DECIDE's general form `work/decide/a-construction-can-ask-whether-a-value-is-an-identity-only-by-taking-a-sign-reading.md`.

## Draws (2026-11-15, 15:46 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 79 | below 86: THREE designers (one Fable, two Opus) |
| labels | 129 | 129 mod 3 = 0: Fable is A; the two Opus designers are B and C |

Mapping: **A = Fable 5.2, B = Opus 5.5 (first Opus dispatched), C = Opus 5.5 (second Opus dispatched)**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

Dispatched concurrently as three cloud sessions with the same statement of the problem and the same pointer to `docs/prompts/designer.md`.

## Sessions (dispatched 15:48 UTC, one turn, concurrently)

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWVGw17S1owoZDQWTKY |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXrRgtjC4aFXveqsMT |
| C | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXsPyvjRQb3Qf4JNAm |

The statement of the problem (identical for all three) is the orchestrator's brief of 2026-11-15, 15:48 UTC; it names no candidate solution. Two exposures to a candidate, both named to the designers as hypotheses: the two item files carry one implementer's sketch (a non-deciding query on `geom_core::Decide`), and the brief lists what the answer "may change" (the verbs, validation, the scalar traits, the symbolic tier, the doors, ratified text), which names possible homes.

Token counts for a three-designer fork: read from each session's `result` events when the reports are in (first report, and total).
