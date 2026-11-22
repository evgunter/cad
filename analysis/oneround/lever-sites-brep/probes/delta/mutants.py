#!/usr/bin/env python3
"""Delta review: mutants re-planted on the frozen head c65b9292, switched at run time
(`geom_brep::review_mutant(name)`, env REVIEW_MUTANT) so one build serves all.
  mutants.py apply <worktree>
  mutants.py run <worktree> [prefix..]   (CARGO_TARGET_DIR set; runs each crate's whole fast set, lib + integration)"""
import subprocess, sys, re, os
EXT='crates/geom-brep/src/extent.rs'; CEQ='crates/geom-brep/src/carrier_eq.rs'; CYL='crates/geom-brep/src/pair_section/cylinders.rs'
ARM='crates/geom-brep/src/curvature_arm.rs'; SIB='crates/step-import/src/build.rs'; SIR='crates/step-import/src/reach.rs'
def m(n): return f'crate::review_mutant("{n}")'
def g(n): return f'geom_brep::review_mutant("{n}")'
EDITS=[
 (EXT,'Lever::of_consumed_extent(self.reach_from(p).max(self.reach_from(q)))',
  f'Lever::of_consumed_extent(if {m("a1-either-old-number")} {{ self.reach_from(p) + (p - q).norm() }} else if {m("a2-either-first-only")} {{ self.reach_from(p) }} else if {m("a3-either-lesser")} {{ self.reach_from(p).min(self.reach_from(q)) }} else {{ self.reach_from(p).max(self.reach_from(q)) }})'),
 (EXT,'let o = self.foot_on(Line { origin: o, dir: a });\n        let c = a.dot(n);',
  f'let o = if {m("b1-ruled-pivot-stored-origin")} {{ o }} else {{ self.foot_on(Line {{ origin: o, dir: a }}) }};\n        let c = a.dot(n);'),
 (EXT,'let hinge = o - n * gap_signed;', f'let hinge = if {m("r6-ruled-hinge-at-foot")} {{ o }} else {{ o - n * gap_signed }};'),
 (EXT,'self.hinge_lever(o, -(gap_signed * c)).length()\n                + self.turn_lever(hinge, (n, a), c, cos).length(),',
  f'self.hinge_lever(o, if {m("b2-ruled-shift-dropped")} {{ T::zero() }} else if {m("r5-ruled-shift-sign")} {{ gap_signed * c }} else {{ -(gap_signed * c) }}).length()\n                + if {m("r4-ruled-turn-dropped")} {{ T::zero() }} else {{ self.turn_lever(hinge, (n, a), c, cos).length() }},'),
 (EXT,'let window = self.range_along(axis.origin, axis.dir);',
  f'let window = self.range_along(if {m("c1-cone-window-from-world-origin")} {{ Point3::origin() }} else {{ axis.origin }}, axis.dir);'),
 (EXT,'lever: Lever::of_consumed_extent(lo.abs().max(hi.abs()) / sin_cos.1),',
  f'lever: Lever::of_consumed_extent(if {m("c2-cone-near-end")} {{ lo.abs().min(hi.abs()) / sin_cos.1 }} else if {m("c3-cone-cosine-dropped")} {{ lo.abs().max(hi.abs()) }} else {{ lo.abs().max(hi.abs()) / sin_cos.1 }}),'),
 (EXT,'let foot = f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2));',
  f'let foot = if {m("d1-perp-foot-at-pivot")} {{ f1 }} else {{ f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2)) }};'),
 (EXT,'let along = r1.min(r2) + gap.abs() * half;', f'let along = if {m("d2-crossing-larger-radius")} {{ r1.max(r2) + gap.abs() * half }} else {{ r1.min(r2) + gap.abs() * half }};'),
 (EXT,'    let (lo, hi) = cover.box_in(&BoxAxes::World);\n    Lever::of_consumed_extent((hi - lo).norm())',
  f'    let (lo, hi) = cover.box_in(&BoxAxes::World);\n    Lever::of_consumed_extent(if {m("e1-cover-diameter-ball-radius")} {{ ExtentBall::of_cover(cover).radius() }} else {{ (hi - lo).norm() }})'),
 (EXT,'        widest.max(hi - lo)\n', f'        if {m("x2-edges-one-width")} {{ hi - lo }} else {{ widest.max(hi - lo) }}\n'),
 (EXT,'    Lever::of_consumed_extent(own.max(width))', f'    Lever::of_consumed_extent(if {m("x1-edges-own-only")} {{ own }} else if {m("x3-edges-width-times-sqrt3")} {{ own.max(width * T::from_f64(1.7320508)) }} else {{ own.max(width) }})'),
 (ARM,'lane.and_then(|lane| lane.chart_arm(chart, p, reach.length()).ok())\n            .map_or_else(poison, T::from_computed),',
  f'lane.and_then(|lane| if {m("r13-chart-arm-refusal-is-reach")} {{ Some(lane.chart_arm(chart, p, reach.length()).unwrap_or(1.0e9)) }} else {{ lane.chart_arm(chart, if {m("r11-chart-arm-ignores-p")} {{ Point3::origin() }} else {{ p }}, if {m("r12-chart-arm-doubles-reach")} {{ reach.length() * T::from_f64(2.0) }} else {{ reach.length() }}).ok() }})\n            .map_or_else(|| if {m("f1-chart-arm-laneless-is-reach")} {{ reach.length() }} else {{ poison() }}, T::from_computed),'),
 (CYL,'            radius: T::zero(),', f'            radius: if {m("g1-point-given-radius")} {{ T::one() }} else {{ T::zero() }},'),
 (CEQ,'reach.lever_from_either(pivot, first.foot(pivot))', f'reach.lever_from_either(pivot, if {m("h1-cyl-pivot-alone")} {{ pivot }} else {{ first.foot(pivot) }})'),
 (CEQ,'return (*c1, *c2, reach.lever_from_either(p1, p2));', f'return (*c1, *c2, reach.lever_from_either(p1, if {m("r1-cone-arm-first-apex-only")} {{ p1 }} else {{ p2 }}));'),
 (SIB,'    let read = |band| read_section(spec, (sa, sb), region, extent.below(), band);\n    // An escalation at the file\'s band is the file\'s ambiguity: its\n    // statements do not decide the reading at their own precision.\n    let at_file',
  f'    let read = |band| read_section(spec, (sa, sb), region, if {g("s1-analytic-section-reads-above")} {{ extent.above() }} else {{ extent.below() }}, band);\n    let at_file'),
 (SIB,'    let read = |band| read_section(spec, (sa, sb), region, extent.below(), band);\n    // An escalation at the file\'s band is the file\'s ambiguity: its\n    // statements do not decide the reading at their own precision.\n    match read',
  f'    let read = |band| read_section(spec, (sa, sb), region, if {g("s2-spline-section-reads-above")} {{ extent.above() }} else {{ extent.below() }}, band);\n    match read'),
 (SIB,'normals.0.cross(normals.1).norm() * extent.below().length();', f'normals.0.cross(normals.1).norm() * if {g("s3-planes-gap-reads-above")} {{ extent.above().length() }} else {{ extent.below().length() }};'),
 (SIB,'geom_brep::coaxial_tangent_circle(sa, sb, extent.above(), band)', f'geom_brep::coaxial_tangent_circle(sa, sb, if {g("s4-coaxial-reads-below")} {{ extent.below() }} else {{ extent.above() }}, band)'),
 (SIB,'        below: from_apexes(run.below()),\n        above: from_apexes(run.above()),',
  f'        below: if {g("s5-below-without-apexes")} {{ run.below() }} else {{ from_apexes(run.below()) }},\n        above: if {g("s6-above-without-apexes")} {{ run.above() }} else {{ from_apexes(run.above()) }},'),
 (SIB,'            id: edge,\n            what: "an edge whose printed carrier has no region over its interval",', f'            id: if {g("s7-no-region-names-the-asker")} {{ id }} else {{ edge }},\n            what: "an edge whose printed carrier has no region over its interval",'),
 (SIR,'    Ok(geom_brep::LeverBounds::Between { below, above })', f'    if {g("s8-below-is-above")} {{ return Ok(geom_brep::LeverBounds::Exact(above)); }}\n    if {g("s9-above-is-below")} {{ return Ok(geom_brep::LeverBounds::Exact(below)); }}\n    Ok(geom_brep::LeverBounds::Between {{ below, above }})'),
 (SIR,'        boundary(solid, faces)?\n', f'        boundary(solid, if {g("s10-below-over-one-face")} {{ &faces[..1] }} else {{ faces }})?\n'),
 ('crates/geom-brep/src/lib.rs','pub use conic_bend::{','/// REVIEW: the run-time mutant switch (env `REVIEW_MUTANT`).\n#[must_use]\npub fn review_mutant(name: &str) -> bool {\n    std::env::var("REVIEW_MUTANT").is_ok_and(|v| v == name)\n}\npub use conic_bend::{'),
]
cmd,wt=sys.argv[1],sys.argv[2]
names=[]
for _,_,new in EDITS: names+=re.findall(r'review_mutant\("([^"]+)"\)',new)
if cmd=='apply':
    for path,old,new in EDITS:
        full=os.path.join(wt,path); src=open(full).read()
        assert src.count(old)==1,(path,old[:60],src.count(old))
        open(full,'w').write(src.replace(old,new))
    print('applied',len(names),'mutants')
else:
    want=sys.argv[3:]
    for name in names:
        if want and not any(name.startswith(w) for w in want): continue
        crate='step-import' if name.startswith('s') else 'geom-brep'
        r=subprocess.run(['cargo','nextest','run','--profile','ci','-p',crate,'--no-fail-fast','--status-level','fail','--final-status-level','none'],cwd=wt,capture_output=True,text=True,env={**os.environ,'REVIEW_MUTANT':name})
        out=r.stdout+r.stderr
        if 'could not compile' in out: print(name,'DID NOT COMPILE',flush=True); print(out[-3000:]); break
        fails=sorted(set(re.findall(r'^\s+(?:FAIL|SIGABRT|TIMEOUT)\s+\[[^\]]*\]\s+\(\s*\d+/\d+\)\s+(\S+ \S+)',out,re.M)))
        summ=re.findall(r'Summary.*',out)
        print(f'{name}: {"RED" if fails else "SURVIVES"} ({len(fails)}) {summ[-1] if summ else out[-300:]}',flush=True)
        for f in fails[:8]: print('    ',f,flush=True)
        if len(fails)>8: print(f'     ... and {len(fails)-8} more',flush=True)
