"""R2 review mutants for PR #4135: each is (name, file, old, new); the
driver applies one, runs the rows and probes, records, and restores."""
import subprocess, sys, os, shutil
ROOT = sys.argv[1]
OUT = sys.argv[2]
R = "crates/topo/src/boolean/reduce.rs"
C = "crates/topo/src/boolean/conic_quadric/mod.rs"
I = "crates/geom-brep/src/implicit.rs"
M = [
 ("M1-convexity-restored", R,
  "geom::Surface::Torus { .. } | geom::Surface::Cone { .. }\n                ) =>",
  "geom::Surface::Torus { .. }\n                ) =>"),
 ("M2-no-floor-reread", C,
  "Ok(Sign::Positive | Sign::Negative) | Err(_) => CircleRoots::Uncertain,",
  "Ok(Sign::Positive | Sign::Negative) | Err(_) => CircleRoots::OnSurface,"),
 ("M3a-swap-in-quadric-harmonics", I,
  "        c2: (aa - bb) * half / per,\n        s2: a * b * up.dot(vp) / per,\n        terms: (d.norm() + conic.speed_hi()).powi(2) + r.powi(2),",
  "        s2: (aa - bb) * half / per,\n        c2: a * b * up.dot(vp) / per,\n        terms: (d.norm() + conic.speed_hi()).powi(2) + r.powi(2),"),
 ("M3b-swap-in-first-harmonic-arm", C,
  "        cos_part: h.c1,\n        sin_part: h.s1,\n        lo_noise: noise,\n        hi_noise: noise,\n        phase_noise: T::zero(),\n    })",
  "        cos_part: h.s1,\n        sin_part: h.c1,\n        lo_noise: noise,\n        hi_noise: noise,\n        phase_noise: T::zero(),\n    })"),
 ("M4-no-conic-cone-route", R,
  "                | geom::Surface::Cylinder { .. }\n                | geom::Surface::Cone { .. },\n            ) => super::conic_quadric",
  "                | geom::Surface::Cylinder { .. },\n            ) => super::conic_quadric"),
 ("M5-no-far-nappe-telloff", R,
  ".map(|nappe| (apex, axis, half_angle, nappe)),",
  ".map(|nappe| (apex, axis, half_angle, nappe)).filter(|_| false),"),
 ("M6-no-conic-apex-rung", C,
  "Ok(Sign::Zero | Sign::Negative) | Err(_) => return Ok(CircleRoots::AtApex),",
  "Ok(Sign::Zero | Sign::Negative) | Err(_) => {}"),
 ("M7-no-conic-slack-meter", C,
  "            Some(&meter),\n",
  "            { let _ = &meter; None },\n"),
 ("M8-cone-floor-one", I,
  "floor: sin_a.min(cos_a) * near / per,",
  "floor: { let _ = (sin_a, cos_a, near); T::one() },"),
 ("M9-no-line-slack-rung", R,
  "            Ok(Sign::Zero | Sign::Negative) => {}\n            Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain),\n        }\n    }\n    let mut thetas",
  "            Ok(Sign::Zero | Sign::Negative) => {}\n            Ok(Sign::Positive) | Err(_) => {}\n        }\n    }\n    let mut thetas"),
 ("M10-no-line-apex-rung", R,
  "        Sign::Zero | Sign::Negative => return Ok(CircleRoots::AtApex),",
  "        Sign::Zero | Sign::Negative => {}"),
 ("M11-line-depth-unscaled", R,
  "Margin::of(disc / (a2.abs() * reach)),",
  "Margin::of({ let _ = reach; disc / a2.abs() }),"),
]
only = sys.argv[3:] 
for name, f, old, new in M:
    if only and name not in only:
        continue
    p = os.path.join(ROOT, f)
    src = open(p).read()
    assert src.count(old) == 1, (name, src.count(old))
    shutil.copy(p, p + ".orig")
    open(p, "w").write(src.replace(old, new))
    env = dict(os.environ, CARGO_INCREMENTAL="0")
    log = os.path.join(OUT, name + ".log")
    with open(log, "w") as lf:
        r1 = subprocess.run(["cargo", "nextest", "run", "-p", "topo", "-p", "sweep", "-p", "geom-brep", "--all-features",
            "--no-fail-fast", "--success-output", "immediate", "--failure-output", "immediate",
            "-E", "test(/cone_rows|line_cone_rows|probe_r2_conic|probe_r2_line|reach_cone_root_lane|probe_r2_e2e|wall_root_tests|conic_quadric/)"],
            cwd=ROOT, stdout=lf, stderr=subprocess.STDOUT, env=env, timeout=3000)
    shutil.move(p + ".orig", p)
    os.utime(p, None)
    print(name, "exit", r1.returncode, flush=True)
