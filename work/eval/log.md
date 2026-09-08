# EVAL log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/eval/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2), on the ground
SEAT's sweep (DOC-LEDGER sweep 8, the same day) left in no program's
`paths`. Six `work/issues/` items re-homed here by `git mv` with a
`## Re-homed` record each — five open and `two-verb-seats-do-not-compose`
set to `deferred` with its ratification cited — and four code-quality
Track V rows claimed (`D360`, `D367`, `D368`,
`emit-blend-restates-the-kernels-own-arguments`). Track V's other
rows (`D364`, `G4` on `crates/profile`, `S190` on `assembly.rs`) sit
on S-BOOL's and DOCM's globs and stay in `work/code-quality/`;
`S190`'s trigger (#855) has fired and the row reads as verify-and-close
for DOCM.

The paths are enumerated, not globbed, for the reason TOPO's are. No
branch exists yet; the first dispatch is unit 1.

## Readiness pass (2026-09-08, remote session)

Read in full before this entry: `work/README.md`, the plan, every item,
`docs/prompts/implementer-discipline.md`, `docs/prompts/reviewer-style-lane.md`,
`docs/REVIEW-STYLE-DISPATCH.md`, the orchestration and lane memories.
**Verdict: the program is ready to dispatch.** `work.py lint` is clean,
the band is claimed, no `eval/` branch exists on origin, no open PR
touches EVAL's `paths`, and every site the five E units cite is live on
main at `063bf1dc8`. What the pass found, so the first lane does not
re-derive it:

- **Citations moved since filing.** `wire.rs:1132` (the `Pinned` lift)
  is `:1226`; `wire.rs:784` (`pinned_plane`) is `:878`; `D368`'s
  "hand-lifted `Vec3` at `anchor.rs:238`" is now the `v` closure inside
  `map_affine` (`:252`), and its `from_f64` instance IS `embed_affine`
  — so `D368` closes by construction when unit 1 retires
  `embed_affine`. `embed_profile` (`anchor.rs:281`) is a third caller
  of the walk the item did not list; it goes with unit 1. The rest of
  `embed_profile` (a per-vertex `Point2` lift, `:291`) is `D385`'s
  shape on `crates/profile`'s door and is NOT unit 1.
- **Unit 1 spec written**: `docs/EVAL-1-SPEC.md`, branch
  `eval/1-affine-lift`; both items set to `spec`. The `try_map`
  question goes to PROPS by note, with a `parked` row filed in the unit
  that writes the note.
- **Unit 2 sites**: sentinels at `eval/mod.rs:2950`/`:3069`, the
  census at `:4422`; tags written outside the sentinels today are
  `40`, `41` (twice, `:3122` and `:4232`), `42`, `43`, `44`, `45`.
- **Fence gap, DOCM's to close**: `names/merged.rs` (DOCM-8, PR 2073,
  merged after this program opened) is in neither EVAL's enumerated
  `paths` nor DOCM's. DOCM's by authorship; announced there rather than
  drawn here.
- **Remote-session posture.** No tmux, no `cad-work`, no local
  monitors: lanes are subagents sharing this checkout (the both-ways
  hazard in `memories/agent-lane-operations.md` applies — lanes commit
  and push before the orchestrator moves the shared ref), hosted CI is
  the gate, `[ev]` PRs get a PR subscription. The orchestrator's
  state-sync rides this session's designated branch
  (`claude/work-eval-readiness-au40qs`) in place of `eval/orchestrator`.
