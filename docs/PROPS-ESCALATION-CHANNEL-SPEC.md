# PROPS escalation-channel — the three families the channel misses

**Binding at dispatch** (PROPS program; items
`work/props/escalation-channel-misses-op-minted-indeterminates.md` and
`work/props/indeterminate-error-arms-sweep.md` — read both in full;
difficulty logged at spec: **H / STRUCTURAL**, dual review). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/escalation-channel`, cut from `main`.

## What this is

PR #1969 gave `k_stats::Bracket` an escalation channel beside its
verdicts, and that unit's own dual found the channel's hole: it records
the `Indeterminate`s the FUNNEL produces and nothing else, while three
families reach a consumer without ever passing through the log. The
first item names all three and carries the reviewer's executed
instance — `enters_material` with a collapsed lever arm hands the
caller an `Indeterminate` while the frame holds a DEFINITE verdict and
an empty escalation list. Eight shipped sites, each listed with its
file and line, each minting `MarginDiag::Invalid` after a definite
sign. `crates/geom-brep/tests/kstats_escalation_channel.rs` exists and
**goes red when this lands**, which is the red-first row handed to you
rather than one you must invent.

The second item is the deletion sweep the k-stats spec named as a
different unit: roughly forty error variants across five crates that
carry a funnel escalation out of an op, which matching on becomes
unnecessary once the log carries what they carry.

## The order, which is the whole design of this unit

**The channel first, the sweep second, and the sweep is scoped by what
the channel then carries.** The second item says so itself: the two
arms in `classify_replay` stay load-bearing until the first lands,
because the log does not carry op-minted escalations or the mate
solve's. So:

1. Close the channel's hole for the eight sites. The item's own
   framing is the one to keep: a site that asks the funnel, gets a
   definite sign, and then mints its own `Indeterminate` is making a
   decision the log cannot see. Decide whether that is recorded by the
   minting site or made impossible, and argue the choice — a channel
   that requires every future site to remember to record is a channel
   with the same hole one commit later.
2. Then the sweep, over the roughly forty variants, classified: which
   still carry information a consumer needs — the op's own context,
   its Display text, its recourse sentence, its site — and which are
   pure wrappers a consumer could read off the log instead. **Classify
   all of them; retire only what the measurement says is a pure
   wrapper**, and file the rest as measured with the reason each stays.

## Rulings

- **Do not touch `classify_replay`'s two arms** until the channel
  carries the op-minted family; the second item says they are
  load-bearing until then and it is right.
- **The mate solve's escalations are named but not this unit's.** The
  first item lists them as a third family; measure whether closing the
  first family closes them too, and if not, file rather than widen.
- **A retired variant is a public API change.** Every one you retire
  gets its consumers re-spelled in the same PR and its owner's ground
  named under a `## Seams crossed` heading — five crates are in the
  sweep's range and most of them are not PROPS'.

## Posture

- ε posture: none — this unit records decisions, it does not make new
  ones. Say so.
- Recorded-verdict populations WILL move: the channel gains entries by
  construction. Every census digest and golden that reads the log
  re-baselines with its digits and its reason (discipline §3), and the
  count of new entries per body is part of the receipt.
- **This machine has a contended build mutex** — read
  `memories/agent-lane-operations.md` §Build concurrency, wrap every
  heavy cargo call in `local-scripts/with-build-slot.sh`, pass no `-j`,
  and let hosted CI be the record.
- Review: dual, in the experiment.
- **Landing: both items get `pr:` and `status: review`. DO NOT MERGE.**
  No `Co-Authored-By`, no `CI-Config:` trailer, no empty commits.

## Acceptance

The eight sites' escalations reach the log, with the handed red-first
row green and the choice between recording and making-impossible
argued; the forty-odd variants classified with each retirement's
consumers re-spelled and each survivor's reason stated; every moved
population re-baselined with digits; hosted CI green.
