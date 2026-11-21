#!/usr/bin/env python3
"""Mutants against the frozen head (C9), switched at run time so one build serves all of them:
each mutated expression reads `geom_brep::review_mutant("name")` (env REVIEW_MUTANT).
  mutants.py apply <worktree>          rewrite the sources (all mutants dormant)
  mutants.py run <worktree> [prefix..] run each mutant: the crate's lib tests, reds listed
(CARGO_TARGET_DIR must be set for `run`.)"""
import subprocess, sys, re, os
EXT = 'crates/geom-brep/src/extent.rs'
CEQ = 'crates/geom-brep/src/carrier_eq.rs'
CYL = 'crates/geom-brep/src/pair_section/cylinders.rs'
ARM = 'crates/geom-brep/src/curvature_arm.rs'
CUR = 'crates/geom-brep/src/props/curved.rs'
SIB = 'crates/step-import/src/build.rs'
SIR = 'crates/step-import/src/reach.rs'
def m(n): return f'crate::review_mutant("{n}")'
def g(n): return f'geom_brep::review_mutant("{n}")'
# (file, original text, replacement text) -- each replacement holds one or more switched mutants.
EDITS = [
 (EXT, 'Lever::of_consumed_extent(self.reach_from(p).max(self.reach_from(q)))',
  f'Lever::of_consumed_extent(if {m("a1-either-old-number")} {{ self.reach_from(p) + (p - q).norm() }} else if {m("a2-either-first-only")} {{ self.reach_from(p) }} else if {m("a3-either-lesser")} {{ self.reach_from(p).min(self.reach_from(q)) }} else {{ self.reach_from(p).max(self.reach_from(q)) }})'),
 (EXT, 'let foot = self.foot_on(Line { origin: o, dir: a });\n        let c = a.dot(n);',
  f'let foot = if {m("b1-ruled-pivot-stored-origin")} {{ o }} else {{ self.foot_on(Line {{ origin: o, dir: a }}) }};\n        let c = a.dot(n);'),
 (EXT, 'let hinge = foot - n * gap;', f'let hinge = if {m("r6-ruled-hinge-at-foot")} {{ foot }} else {{ foot - n * gap }};'),
 (EXT, 'self.hinge_lever(foot, -(gap * c)).length()\n                + self.turn_lever(hinge, (n, a), c, cos).length(),',
  f'self.hinge_lever(foot, if {m("b2-ruled-shift-dropped")} {{ T::zero() }} else if {m("r5-ruled-shift-sign")} {{ gap * c }} else {{ -(gap * c) }}).length()\n                + if {m("r4-ruled-turn-dropped")} {{ T::zero() }} else {{ self.turn_lever(hinge, (n, a), c, cos).length() }},'),
 (EXT, 'let (lo, hi) = self.range_along(axis.origin, axis.dir);\n        Lever::of_consumed_extent(lo.abs().max(hi.abs()) / half_angle.sin_cos().1)',
  f'let (lo, hi) = self.range_along(if {m("c1-cone-window-from-world-origin")} {{ Point3::origin() }} else {{ axis.origin }}, axis.dir);\n        Lever::of_consumed_extent(if {m("c2-cone-near-end")} {{ lo.abs().min(hi.abs()) / half_angle.sin_cos().1 }} else if {m("c3-cone-cosine-dropped")} {{ lo.abs().max(hi.abs()) }} else if {m("r3-cone-sine-for-cosine")} {{ lo.abs().max(hi.abs()) / half_angle.sin_cos().0 }} else {{ lo.abs().max(hi.abs()) / half_angle.sin_cos().1 }})'),
 (EXT, 'let gap = w.dot(normal);\n        let foot = f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2));',
  f'let gap = if {m("r7-perp-gap-sign")} {{ -w.dot(normal) }} else {{ w.dot(normal) }};\n        let foot = if {m("d1-perp-foot-at-pivot")} {{ f1 }} else {{ f1 + a1 * (w.cross(a2).dot(cross) / sin.powi(2)) }};'),
 (EXT, 'let across = (r1 + r2) / sin;\n        let along = r1.min(r2) + gap.abs() * half;',
  f'let across = if {m("r8-crossing-across-one-radius")} {{ r1 / sin }} else {{ (r1 + r2) / sin }};\n        let along = if {m("d2-crossing-larger-radius")} {{ r1.max(r2) + gap.abs() * half }} else if {m("r9-crossing-half-gap-dropped")} {{ r1.min(r2) }} else {{ r1.min(r2) + gap.abs() * half }};'),
 (EXT, 'Lever::of_consumed_extent((hi - lo).norm())\n}',
  f'Lever::of_consumed_extent(if {m("e1-cover-diameter-ball-radius")} {{ ExtentBall::of_cover(cover).radius() }} else if {m("r10-cover-diameter-longest-side")} {{ let d = hi - lo; d.x.abs().max(d.y.abs()).max(d.z.abs()) }} else {{ (hi - lo).norm() }})\n}}'),
 (ARM, 'lane.and_then(|lane| lane.chart_arm(chart, p, reach.length()).ok())\n            .map_or_else(poison, T::from_computed),',
  f'lane.and_then(|lane| if {m("r13-chart-arm-refusal-is-reach")} {{ Some(lane.chart_arm(chart, p, reach.length()).unwrap_or(1.0e9)) }} else {{ lane.chart_arm(chart, if {m("r11-chart-arm-ignores-p")} {{ Point3::origin() }} else {{ p }}, if {m("r12-chart-arm-doubles-reach")} {{ reach.length() * T::from_f64(2.0) }} else {{ reach.length() }}).ok() }})\n            .map_or_else(|| if {m("f1-chart-arm-laneless-is-reach")} {{ reach.length() }} else {{ poison() }}, T::from_computed),'),
 (CYL, 'stands((p, T::zero()), region, band)', f'stands((p, if {m("g1-point-given-radius")} {{ T::one() }} else {{ T::zero() }}), region, band)'),
 (CYL, 'if positive(region.radius() - (apart + radius)) {\n        RegionPlacement::Inside\n    } else if positive(apart - (region.radius() + radius)) {\n        RegionPlacement::Outside',
  f'let (inside, outside) = if {m("g2-definite-arms-exchanged")} {{ (RegionPlacement::Outside, RegionPlacement::Inside) }} else {{ (RegionPlacement::Inside, RegionPlacement::Outside) }};\n    if positive(region.radius() - (apart + radius)) {{\n        inside\n    }} else if positive(apart - (region.radius() + radius)) {{\n        outside'),
 (CYL, 'if point_placement(origin + dir * t, region, band) == RegionPlacement::Outside {\n        return None;\n    }',
  f'if !{m("r16-region-span-never-outside")} && point_placement(origin + dir * t, region, band) == RegionPlacement::Outside {{\n        return None;\n    }}'),
 (CYL, 'RegionPlacement::Outside | RegionPlacement::Straddles => {\n            point_placement(p, pose.region, band)\n        }',
  f'RegionPlacement::Outside | RegionPlacement::Straddles => {{\n            if {m("r15-holds-always-straddles")} {{ RegionPlacement::Straddles }} else {{ point_placement(p, pose.region, band) }}\n        }}'),
 (CEQ, 'reach.lever_from_either(pivot, first.foot(pivot))', f'reach.lever_from_either(pivot, if {m("h1-cyl-pivot-alone")} {{ pivot }} else {{ first.foot(pivot) }})'),
 (CEQ, 'return (*c1, *c2, reach.lever_from_either(p1, p2));', f'return (*c1, *c2, reach.lever_from_either(p1, if {m("r1-cone-arm-first-apex-only")} {{ p1 }} else {{ p2 }}));'),
 (CEQ, 'let arm = reach.lever_from_either(p1, p2);', f'let arm = reach.lever_from_either(if {m("r2-cone-reading-second-apex-only")} {{ p2 }} else {{ p1 }}, p2);'),
 (CUR, 'RimArms::uniform(cone_arm(&rims, sin_a), crate::uniform_curvature_arm(cone))',
  f'RimArms::uniform(cone_arm(&rims, sin_a), if {m("r17-cone-boundary-arm-session-box")} {{ Lever::session_box() }} else {{ crate::uniform_curvature_arm(cone) }})'),
 (SIB, 'let extent = geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1).max(faces.lever());',
  f'let extent = if {g("i1-step-edge-extent-alone")} {{ geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1) }} else {{ geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1).max(faces.lever()) }};'),
 (SIB, 'crate::reach::Reach::of(solid, [plus, minus])', f'crate::reach::Reach::of(solid, if {g("i2-step-one-face-for-both")} {{ [plus, plus] }} else {{ [plus, minus] }})'),
 (SIR, 'geom_brep::cover_diameter(&self.0)', f'if {g("r14-step-reach-lever-is-ball-radius")} {{ geom_brep::ExtentBall::of_cover(&self.0).lever() }} else {{ geom_brep::cover_diameter(&self.0) }}'),
 ('crates/geom-brep/src/lib.rs', 'pub use conic_bend::{', '/// REVIEW: the run-time mutant switch (env `REVIEW_MUTANT`).\n#[must_use]\npub fn review_mutant(name: &str) -> bool {\n    std::env::var("REVIEW_MUTANT").is_ok_and(|v| v == name)\n}\npub use conic_bend::{'),
]
CRATE = {'i1': 'step-import', 'i2': 'step-import', 'r14': 'step-import'}
cmd, wt = sys.argv[1], sys.argv[2]
names = []
for _, _, new in EDITS: names += re.findall(r'review_mutant\("([^"]+)"\)', new)
if cmd == 'apply':
    for path, old, new in EDITS:
        full = os.path.join(wt, path); src = open(full).read()
        assert src.count(old) == 1, (path, old[:50], src.count(old))
        open(full, 'w').write(src.replace(old, new))
    print('applied', len(names), 'mutants')
else:
    want = sys.argv[3:]
    for name in names:
        if want and not any(name.startswith(w) for w in want): continue
        crate = CRATE.get(name.split('-')[0], 'geom-brep')
        r = subprocess.run(['cargo', 'nextest', 'run', '--profile', 'ci', '-p', crate, '--lib', '--no-fail-fast', '--status-level', 'fail', '--final-status-level', 'none'],
                           cwd=wt, capture_output=True, text=True, env={**os.environ, 'REVIEW_MUTANT': name})
        out = r.stdout + r.stderr
        if 'could not compile' in out: print(name, 'DID NOT COMPILE', flush=True); print(out[-3000:]); break
        fails = sorted(set(re.findall(r'^\s+(?:FAIL|SIGABRT|TIMEOUT)\s+\[[^\]]*\]\s+\(\s*\d+/\d+\)\s+(\S+ \S+)', out, re.M)))
        summ = re.findall(r'Summary.*', out)
        print(f'{name}: {"RED" if fails else "SURVIVES"} ({len(fails)}) {summ[-1] if summ else out[-300:]}', flush=True)
        for f in fails[:10]: print('    ', f, flush=True)
        if len(fails) > 10: print(f'     ... and {len(fails)-10} more', flush=True)
