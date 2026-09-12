---
id: shell
kind: program
title: SHELL — shell, offset and transform
status: open
opened: 2026-09-03
area: kernel
prefix: shell/
tag: (SHELL orchestrator)
ab_band: 2300-2399
paths: [crates/topo/src/shell.rs, crates/topo/src/replace_face.rs, crates/topo/src/transform.rs, crates/topo/src/offset_together.rs, crates/topo/src/offset_axial.rs, crates/geom-brep/src/offset.rs, crates/geom-brep/src/offset_meters.rs, crates/sweep/tests/verbs_shell*.rs, crates/editor-core/src/clearance.rs]
keep_out: [geom-brep/src/offset_fit.rs is PROPS' (S-CERT's successor) — the Approx transform's certifier is filed as a row there, editor-core/src/clearance.rs is shared ground with M10 until SHELL-3 lands (the file is M10's; SHELL-3 is the joint unit that moves its body-level half into topo behind interval — ruled B at #1737) and with PROPS while its sign-hull unit retires the planar re-chart in it, LIB-G17 (Node::Shell) consumes the ShellNaming record and the shell VerbRecord arm in crates/verbs must agree with it, the boolean germ lanes and topo/src/boolean/* are S-BOOL's and CURVED's, tier3-approx-regrid-per-face-cost stays PERF's and parks on an Approx-heavy fixture this program produces]
---

VERBS' Wave-3 leftovers in `topo/{shell,replace_face,transform,
offset_together,offset_axial}.rs` and `geom-brep/offset*`, plus the
E7 clearance engine's first kernel consumer: the `ShellNaming` birth
channel (landed, SHELL-1), the `Approx` transform door (landed,
SHELL-2), the clearance engine's move into `topo` and the curved
wall-clearance gate at the shell door (SHELL-3, SHELL-4 — ruled B at
#1737), the ruled shell-of-hollow-body semantics, the cone nappe
home, and the shell/offset follow-ups. Class H with two E openers.
Opened ahead of VERBS' exit on Ev's direction (2026-09-04); VERBS
closed 2026-09-04 and the shell files are this program's territory.
Charter and unit order: `work/shell/plan.md`; narrative in
`work/shell/log.md`.
