# Design fork: two-curve-curve-meeting-finders

Dispatched 2026-10-13 (~20:55Z) by the FUSE orchestrator. Blinding byte 216 (parity 0): Opus is A, Fable is B.
Protocol at dispatch: f0807b14e4

**Slip, disclosed.** The byte was not drawn at dispatch. It was drawn after the three reconciliation rounds and before the `[ev]` PR opened.
- In the round messages the orchestrator labelled the designers ad hoc, the reverse of this mapping (Fable "A", Opus "B").
- Those labels appeared only in orchestrator↔designer messages, never in anything Ev reads.
- The PR and the log row use the byte's mapping above.
