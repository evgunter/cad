# SHOW — the demo refresh (plan)

Opened 2026-10-02 on Ev's request in chat ("we're overdue for some
demos"), from a three-way audit of the tour against main at
`6cacd42dd`: which fudges dodge gaps that have since closed, which
planned stops are unblocked, and what has landed with no demo. The
audit's findings are the slate below; the full tour ran clean on that
head, so every wall probe still pinned is still refusing.

## Charter

Retire the tour's fudges whose kernel gaps are closed, build the
stops that were waiting on the kernel, and show the last month's
landings. **Fold before adding** (Ev, in chat, 2026-10-02: "combine
some of them with each other or existing montage items to keep the
number of things in the montage down (and make each one more
impressive)"): a capability joins the existing cell where it makes
that part better, and the sheet gains exactly one cell — the snowman.

Not in scope, and why:
- The partly-unblocked items (the GUI lane's measures-and-clearance
  cell; the chain's self-intersection check) — their certified halves
  are still blocked and both already have rows on their owners
  (`author/the-gui-has-no-clearance-consumer`,
  `sym/the-chain-demo-detects-no-self-intersection`). Orchestrator's
  call, logged.
- Every live wall probe still refusing (lily 1/2/7/8/12, klein 3/4/6,
  teapot 2/3, torusvessel 1, impeller's round hub). Their kernel rows
  are on their owners; this audit filed the one that had none
  (`cleave/boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule`).
- Torus operands under ∖/∩ (cube through the donut): every answered
  case is disjoint or nested, so it is not yet a picture.

## The slate, in dispatch order

Waves are three lanes wide (the box has 4 cores), and two units that
edit one scene file never run at once.

1. **Wave 1**
   - `snowman-cell` (M) — the one new cell.
   - `klein-scene-should-adopt-the-one-body-loop-sweep` (M), with its
     rider `klein-bottle-loop-sweeps-from-a-world-axis-placement-not-the-paths-normal-plane`.
   - `heatsink-placedunion-base-union-unfinished` (M) — flush declared
     fins plus the joined-rim fillet.
2. **Wave 2**
   - `projectbox-section-cuts-through-bores` (M).
   - `tiltedcut-engraved-face` (M).
   - `lofts-correspondence-twist` (E) and `teapot-lid-unbored` (E),
     one lane, two PRs.
3. **Wave 3**
   - `letterforms-flush-declared` (M).
   - `lily-lanceolate-blade-sections` (M).
   - `rocker-keyhole-crease-fillets` (M).
4. **Wave 4**
   - `bench-on-a-gauge` (H).
   - `gallery-writes-every-document-scene` (E) — after the heat sink,
     whose gallery row moves.
   - `long-turn-helix-has-no-demo` (M) — after the projectbox unit, into
     the same cell if it improves it.
   - `split-node-chords-by-name-has-no-demo` (M) — last.

## Every unit

- **Demo doctrine** (`memories/demo-purpose.md`; the crate doc of
  `demos/tour/src/main.rs`; `docs/prompts/implementer-discipline.md`
  §3): the natural spelling through the public doors. A refusal met on
  the way is a library finding — pin it as a live wall probe
  (`walls::wall`), file it on the program owning the refusing door
  with its payload, and never author around it silently.
- **Oracle**: the kernel test that proved the capability names the
  closed form; the scene asserts the same, through its own build.
- **Gates**: hosted CI is the record. The tour's suite and clippy run
  on the PR when the diff touches `demos/tour`; renders do not. A unit
  that moves frames dispatches `render.yml` on its branch (the GitHub
  MCP `actions_run_trigger`, or `local-scripts/render-hosted.sh` where
  `gh` exists), pulls the committed cells, and LOOKS at them before
  the PR is marked ready; the PR body says what moved and why.
- **README**: each unit rewrites its own stops-table row in
  `demos/README.md` and nothing else there, so concurrent lanes merge
  cleanly.
- **Review**: single Opus review, style lane plus the unit's claims
  (the oracle's derivation; that nothing is authored around a gap; the
  narration matches what the scene builds), with an end-to-end run of
  the tour. `bench-on-a-gauge` gets a full review (new API surface,
  two programs' ground). `lofts-correspondence-twist` and
  `gallery-writes-every-document-scene` merge on the orchestrator's
  read. The tier is logged at dispatch.

## Exit criteria

Every row closed, or closed-with-reason; `demos/README.md`'s stops
table and the module docs it summarises describe what each scene
builds; the montage carries the snowman and no other new cell.
