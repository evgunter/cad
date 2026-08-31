# S-BOOL log — boolean reach and containment

Narrative record; the plan is `docs/S-BOOL-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-BOOL. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-31)

Opened on Evan's direction (in-chat: "you can also take S-BOOL if
that's not claimed yet" — verified unclaimed against docs, branches
and open PRs), by the S-MESH orchestrator in the same opening PR. The
plan is a DRAFT design conversation for its **Rulings sought**
section; BOOL-1 is dispatchable pre-ratification as a charter-named
defect fix whose reproduction is already pinned `#[ignore]`d on main
(recorded here as a unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `bool/`** — unit branches
  `bool/<unit>-<slug>`, orchestrator branch `bool/orchestrator` (the
  opening PR rides the S-MESH session branch; see that log).
- **A/B ordinal band: S-BOOL = 1100–1199**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit as
  S-MESH's 1200–1299 (same orchestrator; renumbered twice after
  the GAUTH and SEAT collisions — see `docs/S-MESH-LOG.md`; the
  1100–1199 band was fixed on main by the ordinal-1100 claim at
  BOOL-1's review dispatch). Implementer
  blocks are named `BOOL-B1, BOOL-B2, …` (`BOOL-<n>` are unit names).
- **Container and process facts are S-MESH's** (`docs/S-MESH-LOG.md`,
  Opening state): one remote container, one lane budget shared by the
  two programs, dispatches interleaved. Away-channel tag `(S-BOOL
  orchestrator)`.

**Sweep at opening**: all seven charter issues open with zero
comments; the VERBS fence is confirmed from VERBS' own plan ("S-BOOL's
honest remainder … was never VERBS'"). #1152's reproduction is
committed `#[ignore]`d with un-ignore instructions at the site;
#1011's two red-on-landing pins are torus-shaped and flip with the
torus arm (VERBS-authored file — coordinated flip); #542 sits on
Track R fence ground (seam recorded in both plans); #433's proposal
rides PR #576's body and is retrieved before the conversation opens.
Track Q is current in §D (16 rows, re-derived 2026-08-31; D285/D286
left with CERT-2); the S112 member-(e) pointer to the landed D282 is
deleted in the opening PR. Carve-outs: D283 (Evan's), S83/D36 (wait
on P-2/#1177), H11's third door (N's ground, filed not edited).
