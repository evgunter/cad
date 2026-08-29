# S-QA log — gates that lie

Narrative record; the plan is `docs/S-QA-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-QA. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction (in-chat: "can you orchestrate its program
S-QA", pointing at the merged work-streams cut), by a fresh
orchestrator on a remote container. The plan is a DRAFT design
conversation for its **Rulings sought** section; QA-1 and QA-2 are
dispatchable pre-ratification as charter-named defect fixes whose
shapes are the issues' own recommendations (recorded below as a
unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `qa/`** — unit branches
  `qa/<unit>-<slug>`, orchestrator branch `qa/orchestrator` (the
  harness-designated session branch
  `claude/program-sqa-orchestration-sllggq` carries the opening PR
  and is otherwise unused, per the S-CERT precedent).
- **A/B ordinal band: S-QA = 800–899**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same commit, per
  that entry's rule (S-CERT holds 700–799; 800 was the next free
  band at claim time). Implementer blocks are named `QA-B1, QA-B2, …`
  (`QA-<n>` are unit names).
- **This session runs in a remote container** (the S-CERT/M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-QA
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk
  ~29 G free is the binding constraint: lanes are worktrees sharing
  one object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent
  lane targets, review targets reclaimed at report time. The clone
  arrived SHALLOW; unshallowed with a blob filter at opening.

**Sweep at opening** (beyond the charter, what the slate is grounded
in): the charter's "red now" is already paid down to the class —
#1102's instance fixed at #1108 (2026-08-27), #1190 filed and closed
same-day (2026-08-29), three innocent-PR detonations in three days
total; #1051's ask LANDED 2026-08-28 (request-a-point:
workflow_dispatch + `CI-Config:` trailer + `CONFIG_SOURCE`), so that
unit is verify-and-close; #746's mechanism largely closed by the
K/P/W session's C15 (#1187, merged 2026-08-29), residues on Track K;
#1038's 146-row instance handed to VERBS with an audit, the
gate-side class fix remaining; the probe-suite-census selftest
flaked red over green content on 2026-08-29 (broken-pipe producers —
J-fence, ours); main's default-lane clippy red is LIB's #1225;
the K/P/W session is live on tracks K/P/W and its
bounds-allowlist rows are contested by live branches — untouched
here.

**Unilateral decisions at opening** (per the orchestration memory's
log rule):

1. QA-1 and QA-2 dispatch pre-ratification. Ground: every item is
   charter-named; the fix shapes are the issues' own recommendations
   (#888's exit-status check; #1128's option 1+3, whose cost
   argument — a green run does identical work — is in the issue;
   #1122's option 3, the say-it-pinned half only, with option 2 held
   for Q2's ruling; #1051 is verification of a landed feature). The
   one Evan-flavored piece in QA-2 (LANE=both on pin) is excluded
   from the dispatch until ruled.
2. The rulings split: Q1 and Q2 amend what a PR run gates — that has
   been Evan's call at every precedent (sampling, k-lint sampling,
   F3, the declined scheduled run) — so both wait even though a
   no-spend reading exists; Q3 changes what a tour-scene PR owes at
   merge, which is other programs' workflow. Everything else in the
   slate is faithful elaboration of ratified ground and proceeds.
3. #1102 closed at opening on #1108's record (instance fixed, main
   green since 2026-08-27; the class is this program's charter and
   is tracked by QA-2/QA-3, not by an open instance issue).
4. The opening PR rides the harness session branch rather than
   `qa/orchestrator`, to respect the harness branch designation for
   this session's own pushes; unit lanes use `qa/` per the cut.
