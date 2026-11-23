# PCTAIL leaves the tracker — 2026-11-02

PCTAIL — the P3 tail of pcurve certification: its refusal vocabulary,
its guards, and the coverage it claims. Opened 2026-10-01 by PCERT's
priority-seam cut, on PCERT's file `crates/geom-brep/src/pcurve_cache.rs`.

On 2026-11-02 the track budget changed to count only dispatchable P0, P1
and unbanded rows (`work/README.md`, Track size), and PCTAIL fell to 6
points. It merged back into PCERT the same day, while PCERT's
orchestrator was live (Ev, in chat: "dumping pctail into pcert is ok.
(adding rows to an existing orchestrator is safe, but taking them away
may not be, and merging two tracks which both have live orchestrators
certainly is not)"). PCERT carries `budget: 36`, the merged load.

Its 14 item files moved to `work/pcert/` with their ids and bodies
unchanged, each with a dated note. Its charter and `keep_out` are folded
into `work/pcert/program.md`, its plan is appended to `work/pcert/plan.md`
as a separate section, PCERT's log carries a note to its orchestrator,
and prose citations of `work/pctail/` outside logs now point at
`work/pcert/`. Its plan set no `## Exit criteria`, so it closes without
an exit walk.

Recover its program, plan and log at c335a694:

    git show c335a694:work/pctail/<FILE>
