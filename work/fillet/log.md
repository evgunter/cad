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

Ev 👍'd [#1733](https://github.com/evgunter/cad/pull/1733) (option 1:
`NoCornerSideCandidate` stays as a stated defensive arm); executed on
that PR — the doc comment states the invariant, the item is closed.
The H4 spec is `docs/FILLET-H4-SPEC.md`; block FILLET-B1's pre-draw
fields and draw are recorded branch-side on `fillet/b1-block` (slot 0
= H4, slots 1–2 bank for H5, H6). H4 dispatches when a heavy lane
frees (E1 or E2).

Rulings landed (2026-09-04): [#1735](https://github.com/evgunter/cad/pull/1735)
approved — the rim selector is `rim_of(body, edge)` in `topo::query`;
the item is a unit, spec `docs/FILLET-RIM-SPEC.md`, the seam announced
in `work/seat/log.md`, and it takes block FILLET-B1's slot 1 ahead of
H5 (record branch-side). [#1734](https://github.com/evgunter/cad/pull/1734):
Ev asked whether reporting the whole list would be worse; answered
"not worse, it costs shape", approved — every refusing crossing is
reported with its corner point, nearest-to-the-anchors first; the item
is a unit (spec to follow). [#1736](https://github.com/evgunter/cad/pull/1736):
Ev asked whether the transverse cut-off would be contradicted by the
later run-out design and whether the open-chain door's plane–plane
restriction is being extended; answered (different situation from the
mid-curve stop, a rename at most; H7 is the only widening) — awaiting
the 👍. H4 dispatched on `fillet/h4-concave-closed-rim`.

[#1736](https://github.com/evgunter/cad/pull/1736) ruled (Ev: "ok
sounds good"): H7 builds the transverse cut-off at perpendicular caps;
the item is a unit, spec to follow after H4/RIM/H5. All four opening
rulings are now answered; only the cut-off's tag name remains to be
ratified, inside H7's spec. This session is subscribed to its own
open `[ev]` PRs so comments wake it.

**H4 stopped at Phase 1 and was re-scoped, same lane (2026-09-04).** The
lane measured, gate off, that every concave closed rim reaches its door
and the surgery correctly refuses to cut a seam at a foot beyond the
rim: the curved arms and `plane_sphere_blend` fold the supports' sense
bits and never the chain's convexity, so a concave rest is the convex
one mirrored through the rim (waist spine `0.5 − r√2` vs the void-side
`0.5 + r√2`). With the fold applied and the surgery untouched, the
waist, the lily mouth and a `cube ∪ ball` boss (which the boolean
builds and which routes to the LADDER) all carve tier-3 clean at their
Pappus volumes, pad 0. The spec's stop clause routed the arms out; the
orchestrator's re-scope routes them back in — same territory, the
ratified rolling-ball convention, BLEND-4's plane–plane fold the
precedent — as `docs/FILLET-H4-SPEC.md` §"Re-scope at Phase 1". L /
NUMERIC unchanged, re-logged branch-side. PR
[#1752](https://github.com/evgunter/cad/pull/1752) is the unit's PR; the
finding is filed as `concave-rim-arms-rest-ball-on-material-side`,
which the unit closes.

**E1 reviewed (2026-09-04): MERGEABLE-AFTER-FIXES**, single style review
on frozen `acb85399` of PR [#1743](https://github.com/evgunter/cad/pull/1743).
C1/C2/C3/C5 held under executed differentials and mutants; C4 missed one
stale sentence. The review's real yield is a sibling class: the shared
gate reads `lo() > 0` unmetered while the band's zero is 1e-9, so a
positive size under ε still reaches the false-fact refusal at both doors
(fillet: a false headroom sentence; chamfer: `DependentNormals` about an
orthonormal corner) — the blend's is the one unmetered spelling of three
(shell and tube meter positivity against the band). Adjudicated: the fix
pass (implementer-inherited) adopts the reviewer's rows `--no-ff`
(order pin, one-refusal row, the `Interval` bracket rows that make the
interval claim real — the flipped probe is `f64`), fixes the prose and
the duplicated rationale, and files the sub-ε class as its own issue
with a characterization row rather than widening E1; the order change
(size gate before repeated-edge) is disclosed as a change and stands.
The inline-Display-advice class the review surfaced (`NonpositiveSize`,
`RepeatedEdge` map to `Recourse::None` yet advise) is routed to E2's
inventory. E3 dispatched on `fillet/e3-margin-payloads`.
