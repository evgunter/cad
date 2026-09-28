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
