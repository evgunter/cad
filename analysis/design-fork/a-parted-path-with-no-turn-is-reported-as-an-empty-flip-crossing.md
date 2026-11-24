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

## The measurement contradicts the reconciled recommendation (11:05 UTC); PR 7700 on hold

M13 (`probe/M13-REPORT.md` on `boxfinish/probe-build-record` at `447298e0b`): the record exists and is scalar-independent (0 differences in 10,872 builds of equal standing), and it separates no known flip: every one of today's 661 `FlipCrossing` leaves holds the witness's record, 660 of them a turned choice, the planted-flip fixture among them. The test all three set for the record ("leaves either side of known flips differ in it") fails. A comment and a hold in the title went on PR 7700 at 11:06; Ev had not answered.

## Round 2 (dispatched 12:08 UTC)

All three shown M13's report and `probe/briefs/fork-parted-path/round-2.md` (what was found; five questions: is certifying every known flip the right outcome or the sign of a wrong record; what a certificate claims and to whom; what `FlipCrossing` is for when no decided leaf builds another record; the fallback; what is measured or built first).

Round-2 replies (all three in by 12:11 UTC; raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/<label>-round-2.md` at `9a154f43411`):
- **All three withdrew the cells gate** on the measurement, and all three now hold that a leaf across a turned structural choice is not the witness's part.
- **A moved** (back to its first gate, changed): choices and standings against the `f64` witness; checks the leaf's own; a turned choice a named flip; a parting refused at once under its own class, never bisected (partings are present at zero width, 0 at a pruning query).
- **B moved** to the same gate as A, in the same terms ("a part of the nominal's form"); a standing change is a flip only when the failure is proven definite.
- **C moved** to its round-0 gate (the whole record, a turned check a flip, a parting bisected) with a new reference: the leaf is compared with the nominal's own replay at the box scalar over a zero-width box, which must build what the `f64` witness builds (the cell record's one job), else the drive refuses whole. Its fallback is A's and B's gate.
- Positions moved and did not meet: A and B agree; C differs on the reference, on checks and on a parting.

## Round 3 (dispatched 12:13 UTC)

All three shown the other two's round-2 reports and `round-3.md` (the table of where they stand; the reference; checks; a parting; whether they now agree).

Round-3 replies (all three in by 12:14 UTC; raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/<label>-round-3.md` at `8814e87e01b`): **the positions crossed.**
- **A moved to C's round-2 lead** (the same-lane reference, the whole record, a parting bisected), amended: the reference is tied to the `f64` witness by its LOG (decided, the same choices, no turn, no parting), since a tie by cells would pass the lanes' contradiction at one point.
- **B moved to C's round-2 lead** likewise, amended: the tie also refuses a choice turned between reference and witness (a parting between lanes tolerated); no `f64` fallback reference.
- **C moved to A's and B's round-2 gate** (the `f64` witness, choices and standings, a terminal named `Parted`), withdrawing its reference on two objections (the tie is blind to what a choice decides; an undecided reference) and withdrawing the comparison of checks (a side reader's ray decision is a check whose other sign is not the part's shape).

## Round 4 (dispatched 12:17 UTC)

All three shown the other two's round-3 reports and `round-4.md`: the table, the arguments each side must answer, and a request to state each recommendation as a function of a measurement now running (M13b: the reference on every driven document; the 2,505 decided leaves re-sorted against it by the whole record and by choices; the six lane pairs labelled; turned checks), so that the numbers decide it.

Round-4 replies (all three in by 12:19 UTC; raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/<label>-round-4.md`): **the positions crossed back, and narrowed.**
- **A moved back** to the `f64` witness, choices and standings, a terminal named `Parted` (C's round-3 state "as it stands"): its own log tie made the reference redundant on choices, and checks mix shape (`dihedral_sign`) with an instrument's route. Adds: a turn on a leaf that holds the nominal is a kernel defect; anything that is the part's shape is recorded once as a choice where it is decided.
- **B moved back** likewise, adding the same site repair (a crease's convexity read in the finished-body check is answered once as a choice, the `side_read` pattern).
- **C moved** to A's round-3 state (the same-lane reference tied by its choices, the whole record, a parting bisected) with B's no-second-reference rule: with the choices tied, the reference is the choices gate plus comparable checks, and "a wrong certificate breaks the promise; a wrong refusal costs mass". Corrected its own example (`side_at_infinity_order` is a magnitude and cannot turn between passing signs; the class survives on the rays' other rows, unobserved).
- **All three now state the same core** (choices and standings equal to the `f64` witness's; a turned choice a named non-empty flip; cells neither gate nor tie; a parting never a flip and named where it ends; a standing change a flip only when proven definite; the lanes' contradiction at one point a kernel defect; "the nominal's form", not "topology") **and each gave its recommendation as a function of M13b.** What is left between them: whether checks are compared too (which needs the same-lane replay, and refuses a drive whole where the box lane cannot decide or tie the nominal) or shape readings taken in a check scope are recorded as choices at their sites; and with it, a parting bisected or terminal.

No round 5 until M13b lands: each report's stated function is applied to its numbers first.

## Round 5 (dispatched 13:08 UTC to all three; statement `probe/briefs/fork-parted-path/round-5.md` at `35e33022c47`)

M13b landed at 13:05 (`probe/M13b-REPORT.md` on `boxfinish/probe-build-record` at `3cd35c15fb0`). Each stated function, applied by the orchestrator as a hypothesis the designers may correct:
- C's (adopt the same-lane reference if zero or a handful of certifying drives would be refused whole): the count is ZERO, so C's function says adopt.
- A's and B's (keep a same-lane reference only if a lane pair is a box-only decision that steers what is built): no pair is labelled so (four are one lane logging work the other does not run; one is unlabelled; `pcurve_chart_placement` is none of the three labels), so their functions say the `f64` witness on choices and standings.
- The functions do not meet, while **the two gates return the same class for every one of the 2,307 decided leaves measured**, and both certify the plate's leaf that today's gate refuses.

Shown to all three: the whole of M13b and M13's addendum; each other's round-4 replies; the orchestrator's reading above; **a correction** (the orchestrator had told all three that the extrude's undecided length is decided outside the funnel; the code reads it through `decide`, and which failing arm the 27,667 leaves are is being measured); and one new fact put as a question: of the 660 named flips, 440 have the witness's form by every reading the instrument takes and differ only in coordinates the enclosures carry.

Asked: the recommendation now; the strongest case the measurement leaves for the other answer; whether a turned choice on a leaf with the witness's form is this question or another; and, if they still differ, the split in two sentences Ev can choose between.

Round-5 replies (all three in by 13:10 UTC; raw on `boxfinish/briefs` `probe/briefs/fork-parted-path/<label>-round-5.md` at `e820b3ea3f9`): **all three moved; two now hold one comparison.**
- **A moved** from choices-only to **E6's gate as ratified**: the leaf's whole record, checks included, against the `f64` witness; no second run; a parting bisects and is named at the floor; the two lanes logging alike becomes a kernel rule held by a parity test over the driven witnesses. What moved it: the reference equals the witness wherever anything certifies (342 of 347), so a reference checks nothing once the lanes agree; and it took back "checks out" on the others' false-certificate argument (a crease only a check reads).
- **B moved** (departing from its own stated function, and saying so) to the **same-lane reference**, whole record, tied to the witness by its choices else the drive refuses whole; a parting bisects. Its reason: with the reference's cost measured at nothing, what decides is the direction of the unseen error, and leaving checks out can certify wrongly.
- **C moved** off the reference (its round-4 rule tested whether the reference costs certificates, not whether it buys any; it buys none) to **E6's comparison exactly**, with a parting refused at once and named. C adds a second question it calls Ev's: certify on the FORM a leaf builds, read off the finished body, since a finished-body check turned on all 220 crease flips and on none of the other 440.
- All three: the 440 form-preserving flips are a question of which decisions are choices at their sites (rows per predicate), not of the gate.

## Round 6 (dispatched 13:15 UTC to all three; statement `probe/briefs/fork-parted-path/round-6.md` at `e820b3ea3f9`)

Shown: each other's round-5 replies; a table of the three positions; and two corrections from the measurement lane (M13b section 7 at `efceb821b`): the extrude's 27,667 leaves are a `Zero` decided in the funnel inside the zero band, a definite failure with no certified mass below (so nothing was decided outside the funnel, and the orchestrator's earlier statement was wrong); and `extrusion_normal_component`'s 122 flips DO move the body across its sketch plane, through a frame neither the cell record nor any check holds, so the flips are 318 coordinates-only, 220 a crease, 122 a body on the other side of its plane with no cell or check differing. Asked: one comparison or the split; a parting bisected or refused at once; whether "same form" stands given the 122; and one paragraph of the E6 text each would have Ev ratify.
