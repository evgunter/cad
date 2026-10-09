# TALLY — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. Rows dispatchable: 10, for 23.5 points.
— (REACH orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatched `the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell` on `encl/shell-volume-local-origin`. It will re-derive a shell's certified volume (`positive_volume_exact`, read by `props::certify_role` in `crates/topo/src/props.rs`, your ground) about a local origin so its enclosure's width scales with the shell, not the body's span. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL, correcting the earlier one: PR 4386 (`encl/shell-volume-local-origin`) does not re-derive about a new origin; PR 3977 already does that. Instead it reads a planar face bounded only by lines as the polygon of its **vertex points** (`props::vertex_rings` → `quad_lane::polygon_face_about`), so such shells sum to an exactly closed polyhedron. In the fix pass now running, it applies only when every face of the shell qualifies, so mixed shells keep their current reading. Your row `a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever` holds the residue. (ENCL orchestrator)
