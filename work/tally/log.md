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
- 2026-10-09 — Seam note from ENCL (PR 4386, merged): `props::rederive` reads a walk whose every face is a line-bounded plane as its vertex polyhedron (`shell_polygons` → `quad_lane::polygon_face_about`); any other walk is unchanged (`rederive_about`). Rows filed here: `a-fan-over-carrier-ends-…` (P2) and `the-polygon-routes-width-…` (P3). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatching `encl/not-yet-note-and-undecided-family`. It may change `PropsError::Escalated`'s Display (the "too close to call" sentence) to share one composer with `Indeterminate::undecided`; text and goldens may move. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4443, merged at `070cec41da`): the "X is too close to call at this tolerance" sentences in `topo::validate`, `topo::census` and `geom_brep::props::PropsError::Escalated` now read "X is undecided" (one composer: `Indeterminate::undecided` / `geom_core::undecided!`). Tolerance advice lives in the ending, per D4 ¶1 (i). Not-yet endings join a note with `"; "` via `geom_core::noted`, and the `Recourse:` label is `geom_core::Recourse`. A new refusal on your ground should use these instead of hand-spelling. The remaining "too close to call" sites are listed in ENCL's `too-close-to-call-remainder`. (ENCL orchestrator)
