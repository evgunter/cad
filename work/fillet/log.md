# FILLET log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/fillet/plan.md`. A/B band 2000–2099
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose FILLET section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `fillet-nonpositive-radius-false-fact-refusal` from `work/issues/`
- `recourse-sentences-owe-followability-pin` from `work/issues/`
- `bare-f64-margin-payload-family` from `work/issues/`
- `concave-closed-rim-has-no-band` from `work/issues/`
- `repaired-pole-rim-serves-no-closed-door` from `work/issues/`
- `extrude-cap-rim-smooth-arm-noop` from `work/issues/`
- `fillet-ruled-spine-arms-no-surgery` from `work/issues/`
- `nocornersidecandidate-has-no-producer` from `work/issues/`
- `fillet-refusal-describes-unbracketed-crossing` from `work/issues/`
- `no-public-rim-arc-selector` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestration opens (2026-09-04)

Picked up on Ev's direction (in-chat, 2026-09-04: "pick up program
`fillet` as the orchestrator"). Single-orchestrator remote box, no
away-channel, no `gh`: GitHub goes through the MCP tools, lanes are
Agent-tool worktrees with private `CARGO_TARGET_DIR`s seeded from one
warm build, at most two heavy lanes at once (four cores). The
orchestrator branch is the session's designated
`claude/fillet-orchestrator-cc9l8o` rather than `fillet/orchestrator`;
unit branches keep the plan's `fillet/` prefix.

**Design work up front — the assessment Ev asked for.** None blocks
the openers or the first three H units: E1–E3 have their shapes
written in their item files, H4 is the closed-rim analogue of the
open-chain convexity fold BLEND-3/BLEND-4 already landed, H5 is a
widening of one door the item itself names, H6 is a reachability
argument. What IS design: the four D rulings (by construction — each
goes out as an `[ev]` PR now, so Ev's answers arrive while the E/H
units run), and H7, whose chain terminations are the run-out question
ARMS3 A3-3 names and OQ6 reserves for Ev — it gets its own `[ev]` PR
early rather than waiting to be reached.

Decisions taken unilaterally at opening:

- **The E openers run outside the A/B experiment**: the charter Ev
  ratified gives them a single style review, v6 is a dual-per-row
  protocol, and S-TCOST's precedent is that non-dual units record no
  row. They draw no ordinal and no block slot; block FILLET-B1 opens
  with H4.
- **Track T's stale park cleared**: `D322`–`D324` were parked on
  #1360, which merged 2026-08-31; they are `open` again and land as
  riders on the first unit that opens `blend/surgery.rs` or
  `blend/naming.rs` (H4 for `D322`/`D325`; `D323`/`D324` together, the
  naming pair, on whichever lane touches `naming.rs` first).
- **Dispatch order**: E1 and E2 in parallel now; E3 after E1 merges
  (both edit `blend/mod.rs`'s error surface). Item-header state
  (`kind`, `status`, `branch`, `pr`) rides each unit's own PR per
  `work/README.md`; this branch carries only the log and the park
  clears.
- **Lane commits carry no model trailer and no model name**, the
  standing lane rule, experiment or not; orchestrator commits keep the
  harness trailer.

The four `[ev]` PRs are open (2026-09-04), one per ruling, each a
question section appended to its item with a firm recommendation:
[#1733](https://github.com/evgunter/cad/pull/1733) `NoCornerSideCandidate`
(keep as a stated defensive arm),
[#1734](https://github.com/evgunter/cad/pull/1734) the arc-carrier refusal
attribution (carry the corner point; report the crossing nearest the
anchors), [#1735](https://github.com/evgunter/cad/pull/1735) the rim
selector (`rim_of(body, edge)` in `topo::query`, SEAT's seam),
[#1736](https://github.com/evgunter/cad/pull/1736) H7's terminations (the
transverse cut-off at perpendicular caps). E1 and E2 dispatched on
`fillet/e1-nonpositive-radius` and `fillet/e2-recourse-followability`.
