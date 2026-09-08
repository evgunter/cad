# SHELL — shell, offset and transform (plan)

**STATUS: OPEN (2026-09-03); DISPATCHING (2026-09-04); second
orchestrator session opened 2026-09-08.** Opened 2026-09-03 from
`docs/WORK-TRACKS-2026-09.md` (SHELL section), which is this program's
charter until this plan supersedes it. Live state is
`work/shell/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`shell/`** — unit branches
`shell/<unit>-<slug>`. Each orchestrator session's designated
branch plays the `shell/orchestrator` role (the opening session's was
`claude/shell-orchestrator-track-qxa7vk`; the second session's is
`claude/work-shell-readiness-y31rxk`); the branch-side A/B block
record lives on the session's branch and reaches main when the block
concludes or at the next session's opening. Away-channel tag
`(SHELL orchestrator)`. A/B ordinal band **SHELL = 2300–2399**,
claimed in `docs/MODEL-AB-LOG.md`'s banding entry.

## Charter

Finish the shell and offset verbs to the semantics Ev ruled and give
them the naming record and the clearance gate their consumers wait
on. Every item here is VERBS' "open shell residue this wave has not
scheduled" plus the M10-5 clearance engine's first kernel consumer.

## Opening condition — superseded, then met

The charter said "dispatches at VERBS' exit". Ev directed the program
to open ahead of that (in-chat, 2026-09-04); VERBS' seven shell items
moved here by header edit and `git mv`. VERBS closed later the same
day (exit walk ratified at #1793, tracker retired at #1799), so
`crates/topo/src/offset_axial.rs` — held back while VERBS-RIMCAP PR-1
(#1674) rewrote it — is this program's territory from 2026-09-08.

## Territory

`crates/topo/src/{shell,replace_face,transform,offset_together,offset_axial}.rs`,
`crates/geom-brep/src/{offset,offset_meters}.rs`,
`crates/sweep/tests/verbs_shell*.rs`, `crates/editor-core/src/clearance.rs`
(shared with M10 until SHELL-3 lands, and with PROPS' sign-hull unit
while it is in flight). Not this program's: `offset_fit.rs` (PROPS),
`crates/verbs` (the closed SEAT's vocabulary; its shell `VerbRecord`
arm must agree with `ShellNaming`), `editor-core`'s recipe doors
(LIB).

## Unit order

1. **SHELL-1** `shell-needs-shellnaming-birth-channel` — LANDED
   (PR #1756, 2026-09-04). LIB-G17 unparked.
2. **SHELL-2** `transform-rigid-refuses-approx-face` — LANDED
   (PR #1758, 2026-09-04); the plan's old item 4, pulled forward.
3. **SHELL-5** `shell-of-hollow-body-thicken-every-boundary` — LANDED
   (PR #2159, 2026-09-08): one thin solid per operand shell, the
   planar clearance gate grown by `t`, `RimNaming::side`. Block
   SHELL-B1 concluded.
4. **SHELL-6** `mint-offset-ignores-cone-mirror-nappe` — LANDED
   (PR #2178, 2026-09-08): `face_nappe`/`group_nappe`, one home in the
   offset lane; the live ε-scale sign defect closed. Block SHELL-B2
   slot 0. The winding-predicate rename (`shell-offset-three-followups`
   item 2) is NOT in it: three owners' files and a K-lint population
   — it stays on that item until announced on TOPO's and S-BOOL's
   boards.
4b. **SHELL-7** `axial-door-refuses-a-one-surface-seam-corner` —
   LANDED (PR #2200, 2026-09-08): the one-surface corner and every
   same-surface latitude seam; the full-period torus shells. Block
   SHELL-B2 slot 1.
4c. **SHELL-8** `shell-open-on-a-multi-solid-body` — LANDED (PR
   #2207, 2026-09-08): the verb applied per solid on a multi-solid
   body, designations on any solid; `insert_voids` (S-BOOL seam) and
   `classify_shells_of` (PROPS seam). Block SHELL-B2 slot 2, the
   block's last; SHELL-B2 concluded. Follow-ups filed:
   `shell-doors-still-walk-the-whole-body` (lane) and
   `shelled-result-does-not-name-the-wall-it-built` (orchestrator).
4d. **SHELL-9** — LANDED (PR #2223, 2026-09-08): `shell` runs the
   closing pcurve mint; the sphere half of
   `void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam`
   closed. Block SHELL-B3 slot 0. Filed by the lane:
   `shell-launders-a-stale-operand-row` (the class is kernel-wide —
   TOPO's `producer-closing-mint-is-a-convention-with-thirteen-copies`). The drum half is TOPO's
   (`work/topo/revert-does-not-mirror-plane-chart-images`); the
   cone-hyperbola refusal is parked on
   `offset-lane-has-no-conic-carrier` (Ev's fork).
5. RULED B (Ev, #1737, 2026-09-04): **SHELL-3** — the clearance
   engine's body-level half moves into `topo` behind `interval`
   (joint with M10; no behaviour change, the M10-5/6 suites are the
   differential; draft spec `docs/SHELL-3-SPEC.md`) — then **SHELL-4**
   — `shell` runs E7's self-intersection question on the cavity clone
   at certifying scalars and refuses typed;
   `shell-curved-wall-clearance-window` closes on SHELL-4's refusing
   row, `shell-curved-clearance-consumer` when both land. **SHELL-3
   dispatches after PROPS' sign-hull unit merges** (it edits ~130
   lines of `clearance.rs`; a move and an in-file edit do not run
   concurrently), and asks M10's orchestrator for the co-review at
   dispatch.
6. `shell-offset-three-followups` item 1 — the props inventory's
   curved-face-with-ring reading (a PROPS seam; announced first).
7. `clearance-refusal-names-one-face-twice-across-bodies` — rides
   SHELL-3 or SHELL-4, whichever touches the raise sites; the
   `Display` half is owed either way.
8. `no-approx-faced-body-is-both-movable-and-valid` — the OFF-C rows
   marked decorative can be strengthened on the loft now that
   `transform_rigid` maps described NURBS carriers; the cache walls
   are MESH's and EXCH's (filed there) and this row closes when the
   rows are strengthened.
9. `tier3-approx-regrid-per-face-cost` (PERF's) stays parked on an
   `Approx`-heavy fixture this program produces.

## Adjacent, not taken

- `work/seat/shell-doors-take-tolerance-beside-tol` — SEAT's; SHELL-1's
  opening measurement supplies the caller census that item asks for.
- `LIB-G17` (Node::Shell) — LIB's; unparks at SHELL-1's merge.

## Exit shape

The verbs' README states the ruled semantics and every item above has
landed; the walk convention applies.
