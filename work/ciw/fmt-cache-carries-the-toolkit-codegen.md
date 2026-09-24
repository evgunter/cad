---
id: fmt-cache-carries-the-toolkit-codegen
kind: issue
title: The app-feature test row makes fmt's rust-cache carry toolkit codegen on every run, including runs that skip it
status: open
opened: 2026-09-04
refs: [1755]
priority: P4
cost: E
---

Raised by CHROME's implementer lane against the CHROME orchestrator's
own siting decision, and correct. **Filed rather than decided, because
the data that would decide it does not exist yet and neither option is
obviously right.**

**The dispatcher's wrong sub-premise.** The orchestrator argued the
new `cargo nextest run -p viewer --features app` row was nearly free
in the `fmt` job because that job "already compiles this exact graph"
in its `clippy (viewer app feature)` step. That is check semantics:
`cargo clippy` leaves **metadata**, not rlibs. The new row is the
first step in `fmt` that needs codegen and a linker over those 146
extra crates, so the marginal cost is real and was asserted away.

**The consequence, which is what makes this worth a file.** `fmt`
restores and saves `Swatinem/rust-cache` (`ci.yml:1351`). The row runs
only when `run_viewer_toolkit` is true, but the artifacts it produces
enter `fmt`'s cache entry — and `fmt` carries **no lane gate**, so
that larger entry is restored on **every** run afterwards, including
every run where the toolkit axis is false. A cost that was supposed to
follow the axis instead follows the job.

**Both options, and why neither is obviously right.**

- *Keep it in `fmt`* (what shipped). Pays codegen over a graph already
  checked in the same job, so the marginal work when it fires is
  small. Costs a permanently larger cache entry restored on every
  `fmt` run, and GitHub's per-repository cache budget is finite and
  evicts LRU — so the cost may land on other jobs' entries rather
  than showing up as this job's time.
- *A job of its own*, gated on the same axis. The cache effect on
  `fmt` disappears entirely and the cost follows the axis, which is
  what Ev's viewer-CI-posture ruling asks for. But the new job pays
  **check plus codegen from its own cold cache** when it fires, rather
  than codegen over an already-checked graph.

**What would settle it**: the restore/save time delta on `fmt` runs
where the axis is FALSE, before and after. That is one comparison of
two runs and nobody has taken it. Until someone does, "keep it in
`fmt`" rests on a guess about how often the axis is true, and so does
"move it".

**Why it shipped as sited anyway.** The caveat is written at the step
itself (`ci.yml:1500-1505`), not only in a PR body, so the next reader
of that step meets the open question rather than an assertion that it
is free. Under the standing rule that a claim resting on a measurement
owes a mechanical guard, a scheduled register, or a written reason it
can have neither, this is the third — recorded here so it is a
schedule rather than a shrug.

CI build knobs are S-TCOST's rule and CI cost is CIW's ground, so this
likely re-homes rather than closing inside CHROME.

Signed: (CHROME orchestrator)

## Re-homed to CIW, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator. This row names
no crate file: its whole subject is the `fmt:` job in
`.github/workflows/ci.yml`, which `work.py territory` resolves to CIW.
CHROME filed it because CHROME's unit 2 put the app-feature step there,
but CHROME cannot fix it — the row is a CI-topology decision on ground
CHROME does not own.

**Its two line citations are dead** (`ci.yml:1351` and `:1500-1505`);
the file has grown and both subjects are several hundred lines further
down. Cite them by job and step name instead: the `fmt:` job's
`Swatinem/rust-cache@v2` step, which passes only
`workspaces: ${{ steps.doc-roots.outputs.roots }}` and **no key gated on
the axis** — unlike the `clippy:` job's
`key: viewer-${{ needs.filter.outputs.run_viewer_toolkit }}` — and the
step `viewer app-feature rows (chrome + gpu pipeline smoke)`, whose
"WHAT IS NOT FREE" caveat is still written at the step.

The measurement this row schedules (a restore/save delta on axis-FALSE
`fmt` runs) still appears nowhere. Adjacent live CIW/TCOST ground:
`work/tcost/actions-cache-budget-under-a-hash-key`.

Signed: (CHROME orchestrator)
