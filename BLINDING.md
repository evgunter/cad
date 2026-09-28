# Design fork: how a sweep's declared cusp contacts reach the at-rest gates

Fork: `work/gather/product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares.md`
Protocol: bb10a4cd (`docs/DESIGN-FORK-PROTOCOL.md`)

- urandom byte: 201 (odd) -> **A = Fable, B = Opus**.
- Deviation: the byte was drawn at 2026-09-28 about 05:00 UTC, a few minutes AFTER both
  designers were dispatched concurrently and BEFORE either report was delivered. The
  dispatch labels in the orchestrator's session ("Designer A" = Opus) are not the
  protocol's labels; this file's mapping is the one of record.
- Blinding is weak in this session: Ev can see the orchestrator's cloud-session
  transcript, where each dispatch names its model.

## Further designers (round 4, 2026-09-28 ~06:50 UTC)

Dispatched fresh on Ev's comments (none of the four homes feels right; weigh reopening #131).
- **C = Opus, D = Fable.** No byte drawn: labels were assigned at dispatch, and the orchestrator's
  session transcript (visible to Ev) names each dispatch's model, so blinding does not hold here.
- Both received the same problem statement, with the earlier options as context only and #131
  stated as open for revision.
