# Design fork: an IsoLine states its ends twice (BOXFINISH)

Protocol: ddd201f411 (`docs/DESIGN-FORK-PROTOCOL.md`). Item: `work/boxfinish/an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends.md` (P1, design).

## Draws (2026-11-15, 02:10 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 58 | below 86: THREE designers (one Fable, two Opus) |
| labels | 117 | not 255; 117 mod 3 = 0: Fable is A; the two Opus designers are B and C |

Mapping: **A = Fable 5.2, B = Opus 5.5 (first Opus dispatched), C = Opus 5.5 (second Opus dispatched)**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

Dispatched concurrently as three cloud sessions with the same statement of the problem and the same pointer to `docs/prompts/designer.md`.

## Sessions (dispatched 02:09 UTC, one turn, concurrently)

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWU5GkLjNmyBmhNGAtf |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWX43ubJbGeKbqQSmCe |
| C | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXQagXVTSE1tSzNevK |

The statement of the problem (identical for all three) is the orchestrator's brief of 2026-11-15; it names no candidate solution. The item file's "The question" paragraph carries one of the orchestrator's guesses (that the certificate's need "may be a discrete fact and not two points"); the brief tells the designers to treat that paragraph as a hypothesis.

Token counts for a three-designer fork: read from each session's `result` events when the reports are in (first report, and total).
