# CURVED log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/curved/plan.md`. A/B band 2200–2299
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CURVED section is the
charter this plan restates. Opens at VERBS' exit. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `census-at-rest-two-boolean-lane-premises` from `work/mate/`
- `overlap-lane-boundary-crossing-cuts` from `work/mate/`
- `dev1-cylinder-sphere-circle-locus-arm` from `work/mate/`
- `m9-3-semantic-residues` from `work/mate/`
- `torus-declared-rest-lane-banked` from `work/mate/`
- `cylindrical-rest-pair-hits-planar-merge` from `work/mate/`
- `signed-penetration-depth` from `work/m10/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for dispatch (2026-09-04)

New orchestrator (this session runs CURVED and TRIM). VERBS' walk was
ratified at #1793 and its directory swept (closure commit c1e7ea195);
S-MATE was swept into MSOLVE. Both named opening conditions hold.
Actions in the opening PR, on Ev's in-chat answers (sequencing is the
orchestrator's; design forks are Ev's):

- Adopted all fourteen VERBS re-homes from `work/issues/` by `git mv`
  (ids unchanged): `VERBS-C5ARMS`, `VERBS-CONE`,
  `arc-aware-point-in-loop`, `boolean-refuses-on-arc-carrier-not-arc`,
  `c5-plane-torus-cone-cylinder-arms`,
  `circle-residual-harmonics-needs-torus-arm`,
  `declared-cusps-second-order-wedge-arm`, `pierce-ring-has-no-join-arm`,
  `pinch-carrying-machinery-valence-4` (parked on SEAT-6),
  `plane-cone-elliptic-section-split-refusal`,
  `plane-nurbs-ssi-misblames-control-net`,
  `ssi-lever-arm-min-fold-hides-poison`,
  `torus-operand-boxes-span-whole-ring`,
  `verbs-1031b-assigner-checker-divergence` (kept here: the merge-door
  lane owns the same arc-winding machinery).
  `sphere-flux-arm-refuses-partial-bands` went to PROPS (a props unit).
- The S-BOOL fence written on both sides (`work/bool/program.md`
  keep_out, this program's paths/keep_out); the shared `boolean/*` and
  `splitting/*` globs stay claimed by both, the prose fence decides.
- Decision (unilateral, sequencing): first dispatch is `VERBS-C5ARMS`
  PR-2 (cone×cylinder; specced, small, the one executable VERBS
  remainder), then `cylindrical-rest-pair-hits-planar-merge` (the
  honest typed skip) and `torus-operand-boxes-span-whole-ring`.
- Decision (Ev, in-chat): the RIMCAP torus half / spiric carrier
  design conversation opens NOW as an `[ev]` PR, in parallel with the
  first units, joined with #1377's pinch machinery as one doc.
- Decision (Ev, in-chat): S-BOOL's ceded items (operand reach, the
  containment doors, the graft/boolean-declarations singles) are
  negotiated over the away channel for an early handover rather than
  waiting for S-BOOL's exit.
- Orchestrator branch `curved/orchestrator` (this program's state-sync
  for both programs rides here; TRIM's own log names it too). The
  away channel is armed on `curved/,trim/` with `@ trim` as an extra
  address; repo-wide NEW ISSUE/PR lines are narrowed to summons
  (`CAD_CHANNEL_NEW_EVENTS=summons`, added to the script this PR — Ev's
  in-chat ask: PR noise off, "@ curved" mentions must reach me).

## First dispatches and the design conversation (2026-09-04, later)

- **Block CURVED-B1** drawn branch-side (`curved/b1-block`): byte 53 ⇒
  fable at slot 2. Slot 0 = VERBS-C5ARMS PR-2 (Opus), dispatched on
  `curved/c5arms-2`; slot 1 REORDERED to CURVED-TORUS PR-1 (Opus) on
  `curved/torus-box` because its spec ratified first (#1874); slot 2 =
  the merge-door unit (Fable), spec lane still measuring.
- **`docs/CURVED-TORUS-SPEC.md` ratified (#1874)** with three binding
  refutations: the torus implicit is the linearized form (no harmonic
  triple; PR-2 encloses by subdivision with a certified `f2`); a
  boundary-tight box does NOT retire lily wall 1 (a concentric coplanar
  disc lies in every AABB of the larger circle) — PR-1's acceptance is
  a RE-AIM and #1488 is re-scoped accordingly; MATE-7a's "one function
  away" was measured on the coincident full-torus pair, not the lily.
  `Torus` on `boolean_arm_exists` is this program's third torus unit,
  filed as `torus-operand-gate-admission`. Rulings in the spec's §9.
- **`[ev]` #1858 opened**: `docs/CURVED-SPIRIC-DESIGN.md` (ruling item
  `spiric-carrier-ruling`). Measured: the shell rim is always the
  two-oval spiric regime (the double point is unreachable from the
  shell door); the pinch family's curves are ellipses already carried
  — RIMCAP's "one design doc" premise refuted; the general rung is
  blocked for tori by one function (the C9 ring's missing `sqrt`).
  Ev's first answer: a special-case kind like `Ellipse` (Q1=(b),
  Q2=(b1)); asked why (c2) reads as rejected — replied that (c2) is
  the general rung and stays the route for pairs without a closed
  form, and asked whether the ring `sqrt` (Q3(ii)) should open now
  (reading (B)) or after the variant ships (reading (A), recommended).
  Sign-off on Q3(i)/Q4/Q5/Q6 requested by 👍 (watchlist entry).
- **Handover request to S-BOOL** posted on #1835 (`@ s-bool`); no
  reply yet; nothing dispatches on those files until it comes.
- **Operations.** Six Fable/Opus lanes ran concurrently on an 8-core,
  9 GB box shared with another orchestrator (load 25–40): cold builds
  serialized on the machine mutex with waits of 1–2.5 h; two spec
  lanes could not take their one measurement and pre-registered it as
  the implementer's opening act instead. Three Fable spec lanes were
  killed by a transient Fable 429 and resumed by SendMessage without
  loss. The pre-push rustfmt hook outlived the SSH connection under
  this load ("Connection to github.com closed by remote host") and one
  ratification commit was silently not pushed — #1865 merged without
  its rulings, carried by #1876. Docs-only pushes from the orchestrator
  now use `--no-verify`; implementer briefs say what to do. GraphQL on
  the shared GitHub account is periodically exhausted; PR create/merge
  go through the REST API.

## VERBS-C5ARMS PR-2 merged (2026-09-05) — the first CURVED unit

PR #1864, ordinal 2200, sample #143; block CURVED-B1 slot 0 concluded.
The dual (R1 Opus, R2 Fable) both MERGEABLE-AFTER-FIXES; adjudication
on the PR (comment 5551363172); fifteen union items all taken. The
substantive design change from review: admission on the STATION
`|R·cot α|` against `extent` rather than an angular guard (R1's
measured ~101 ε off-surface mint at α = 1e-10 / extent = 100). The
pair is EXCLUDED from the A/B tally under 3(e) — R2 was killed twice
by the account's usage limit and resumed. Class findings filed:
`teapot-walls-have-no-suite-row`,
`c5-gate-admits-every-pose-of-an-implemented-pair`; log-only: the
predicate-dimension audit's counts go stale with every new name and
nothing re-takes them; three coaxiality policies now coexist in
`intersect.rs` (declared-and-verified, measured-and-refused,
measured-and-admitted). Operations: the `CI-Config` commit-trailer
path was deleted on main (`eeb912512`, 2026-09-04) — briefs must stop
mentioning it; the k-lint draw is retired and all five rows run.
