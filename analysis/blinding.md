# Design fork: face-pair-contact-pairing-has-four-spellings

Dispatched 2026-11-12 by the ONEDOOR orchestrator. Protocol at dispatch: ddd201f411f.

- Arm byte 17 (< 86): three designers, two Opus and one Fable.
- Label byte 26 (26 mod 3 = 2): Fable is C; the two Opus designers are A and B, in dispatch order.

Deviations: none at dispatch.

Sessions, dispatched concurrently at 16:03 UTC with the same brief:
- A: session_018u3gVG66x6uzfDSsm9z6zJ (Opus 5.5, configured `claude-opus-5-5`)
- B: session_018u3gVGephd8xdKuiGLFdBT (Opus 5.5, configured `claude-opus-5-5`)
- C: session_018u3gVGoc3BLFjDDGPBuPNZ (Fable 5.2, configured `claude-fable-5-2`)

**Aborted at 16:04 UTC, before any report: an orchestrator error.** The fork had already been weighed and converged on 2026-11-12 by two designers (`work/onedoor/face-pair-contact-pairing-has-four-spellings.md`, "Designed"), and ruled a unit, not an `[ev]` fork, because nothing ratified changes. That design was on `onedoor/orchestrator`, not main. The orchestrator read the row at `origin/main` after a context compaction, and the summary it was resuming from had dropped the design. All three sessions were interrupted and archived after about 90 s. Cost as the harness reports it: A $0.66, B $0.68, C $1.23. No report was delivered and no row is recorded: there is no fork.
