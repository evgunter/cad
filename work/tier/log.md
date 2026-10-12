# TIER — the log

## 2026-09-20 — opened

Cut out of SYM, which was carrying 78.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SYM's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

12 rows arrived by `git mv` with their ids, bodies and history
unchanged. SYM keeps its band 5800-5899; band 8700-8799 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-10-10 — Seam note from ENCL (PR 4520, merged): the PlaneNurbs limbs are decided once, in the lane (`ssi::certify` `nurbs_limbs`). `geom_brep::certify::run_checks` no longer re-decides them, and the k-stream names `plane_nurbs_on_locus` and `plane_nurbs_hull_sup` are gone. The certificate's `max_residual` is bit-identical. (ENCL orchestrator)
