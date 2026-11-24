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

## First reports (recorded as delivered, before any reconciliation; all three in by 09:25 UTC, raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/` at `e14ae2f4cb5`)

| | A | B | C |
|---|---|---|---|
| Answer | The certificate compares CHOICES; a leaf's checks are its own (each definite and passing over the leaf). A turned choice is `FlipCrossing`, named, non-empty by type. A parting at a pruning query bisects. Any other parting is a kernel defect and fails loud (bisects until every steering branch is logged or marked). Kernel parity by a zero-width replay instrument. | The certificate compares the BUILD (per node: standing and names), not the decision log; the log explains. Built otherwise: `FlipCrossing` naming the first node that does, with the turned decision as cause where there is one. Nothing lies between. Kernel parity by an instrument and a `debug_assert`. Fallback if name tables are not a complete record: choices-only gate with a parted leaf unsettled. | Type a decided-different leaf as Turned or Parted. Turned (same question, opposite definite answers; a turned CHECK counts, role carried): `FlipCrossing`, named. Parted: unsettled, bisect on the parting node's reads, `ReadsNoVariable` or `Budget` at the floor; never `FlipCrossing`. The whole record stays the gate. Corpus parity test at the nominal. |
| One line of argument | Comparing checks with the witness tests the kernel's lanes against each other, not the document; predicate flips are the only places topology changes. | The gate compares a record of how the leaf decided when the claim certified is what it built; the log can prove "same" and never "different". | Equal vectors prove the same build and the converse is false; each terminal refusal rests on a proof that splitting cannot help, and a parting has none. |
| Confidence | likely | likely (name tables' completeness: unsure) | likely (bisect over terminal: pending a measurement) |
| Rejected the framing | Yes: the three states mean different things. | Yes: the question is one level too low (the gate compares the wrong thing). | Partly: E6 is silent on the parted case, it does not decide it. |
| Proposed changing ratified text | Yes: E6's certified bullet and no-flips bullet. | Yes: E6's leaf-protocol conditions (first two bullets). | Yes: the first sentence of E6's no-flips bullet. |

Agreement at first report: `FlipCrossing` non-empty by type; `is_empty` and `FlipUnfounded` go; a parted leaf is never a `FlipCrossing`; kernel self-disagreement is an instrument's question. **Split, three ways, on what the certificate compares** (choices; the build; the whole record), and so on a turned check and on check sequences that differ.

Served models at first report: not yet read from the sessions.

## Round 1 (dispatched 09:32 UTC)

All three shown the other two's first reports and `probe/briefs/fork-parted-path/round-1.md` (the table of where they stand, four questions, three facts they contradict each other on to check against the tree).

Round-1 replies (all three in by 09:35 UTC; raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/<label>-round-1.md` at `be616078d88`):
- **A moved to B's gate** (the build): a lane choice is logged as a choice (`closed_form_shown`), so a gate on the log refuses a leaf whose cells are the witness's at any strictness; withdrew the fail-loud arm for a parting away from a pruning query. Found the check that passes with either sign it had not found (`edge_wedge_over_as`, `Role::Check`).
- **B did not move**; its three facts (a crease's `DIHEDRAL_SIGN` at tier 3; `walk_pairs` handing pairs to `pair_verdict`; `pn_axis_normal` through `closed_form_shown`) strengthen it. Named the structural record (standing, cell names, bounding cells, glued coincidences) if name tables alone are not complete.
- **C moved to B's gate**, withdrew the whole-record gate ("the stand-in kept after showing it is one") and its side-reader objection (`side_read` is recorded once as a choice), and added that the build be compared as standing plus named cells plus incidence.
- All three name the same fallback (the gate on choices and standings; a parting bisects; nothing in the drive panics) and the same first measurement (the build's record is complete and scalar-independent).

**Reconciled after one round: agreed.** Put to Ev as `[ev]` PR 7700 (`boxfinish/ev-leaf-certifies-on-the-build`) at 09:44 UTC with E6's two bullets as the diff and the three round-1 `For Ev` sections verbatim, labelled A, B, C. Row 7700 (three-designer table, the recommendation half) is on that branch. Tokens per label (first report / total) and the mapping are filled in once Ev decides.
