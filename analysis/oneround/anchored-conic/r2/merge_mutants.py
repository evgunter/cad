#!/usr/bin/env python3
"""Reviewer r2, delta read of PR 7203 at b303a32a6: the first review's
survivors re-planted on the reworked conic.rs, each run in the crate that
owns the arm. Run in a worktree of the frozen head."""
import subprocess, sys, os
C = "crates/geom/src/curves/conic.rs"
GEOM = ["-p", "geom"]
M = {
 "D1_same_content_ignores_form": (C, "(Held::Centre, Held::Arc { .. }) | (Held::Arc { .. }, Held::Centre) => false,", "(Held::Centre, Held::Arc { .. }) | (Held::Arc { .. }, Held::Centre) => true,", [GEOM]),
 "D2_same_content_ignores_phase": (C, "same_scalar(p.0, q.0) && same_scalar(p.1, q.1)", "{ let _ = (&same_scalar, p, q); true }", [GEOM]),
 "D3_data_omits_phase": (C, "Some(Vec3::new(c0, s0, T::zero())),", "{ let _ = (c0, s0); None },", [GEOM, ["-p", "mesh"], ["-p", "step-import"]]),
 "D4_map_scalar_drops_phase": (C, "held: self.held.with_phase(|(c0, s0)| (f(c0), f(s0))),", "held: self.held.with_phase(|_| (U::one(), U::zero())),", [GEOM]),
 "D5_azimuth_ignores_phase_sine": (C, "w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0),", "w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0 * T::zero()),", [GEOM]),
 "D6_azimuth_ignores_phase_cosine": (C, "self.semi_u * c0 + w.dot(self.at.u_ref()),", "{ let _ = c0; self.semi_u + w.dot(self.at.u_ref()) },", [GEOM]),
 "D7_mapped_drops_phase": (C, "            at: map(&self.at),\n            ..*self\n", "            at: map(&self.at),\n            held: self.held.with_phase(|_| (T::one(), T::zero())),\n            ..*self\n", [GEOM]),
 "D8_memo_key_mark_removed": ("crates/mesh/src/memo.rs", "if datum == geom::CurveDatum::Anchor {", "if false && datum == geom::CurveDatum::Anchor {", [["-p", "mesh"]]),
 "D9_square_uncharged": (C, "let sq = transcendental(sh).square();", "let t0 = transcendental(sh); let sq = Rounded { value: sh.powi(2), error: T::from_f64(2.0) * sh.abs() * t0.error };", [GEOM]),
 "D10_sin_theta_uncharged": (C, "transcendental(s) - Rounded::exact(s0),", "Rounded::exact(s) - Rounded::exact(s0),", [GEOM]),
 "D11_walk_names_anchor_center": (C, "super::CurveDatum::Anchor,\n                Some(", "super::CurveDatum::Center,\n                Some(", [GEOM, ["-p", "mesh"], ["-p", "step-import"]]),
 "D12_reversal_keeps_sine": (C, "held: self.held.with_phase(|(c0, s0)| (c0, -s0)),", "held: self.held.with_phase(|(c0, s0)| (c0, s0)),", [GEOM]),
 "D13_quarter_phase_dropped": (C, ".with_phase(|(c0, s0)| (turn.select_le_zero(c0, s0), turn.select_le_zero(s0, -c0))),", ".with_phase(|(c0, s0)| (c0, s0)),", [GEOM]),
 "D14_half_phase_dropped": (C, "(back.select_le_zero(c0, -c0), back.select_le_zero(s0, -s0))", "(c0, s0)", [GEOM]),
 "D15_evaluator_reads_derived_centre": (C, "let (ku, kv) = offsets(theta, phase);\n                let (u, v) = (self.at.u_ref(), self.at.v_ref());\n                self.at.origin() + (u * (self.semi_u * ku) + v * (self.semi_v * kv))",
        "let _ = phase; let ((s, c), v_ref) = self.basis(theta);\n                Conic::center(self) + (self.at.u_ref() * (self.semi_u * c) + v_ref * (self.semi_v * s))", [GEOM]),
 "D16_sig_reads_one_scalar_fewer": ("crates/step-import/src/build.rs", "core::iter::once(tag)\n                .chain(\n                    data.into_iter()", "core::iter::once(tag)\n                .chain(\n                    data.into_iter().take(5)", [["-p", "step-import"]]),
 "D17_difference_charge_dropped": (C, "let first = doubled - (Rounded::exact(T::one()) - Rounded::exact(c0));", "let first = { let d = Rounded::exact(T::one()) - Rounded::exact(c0); Rounded { value: doubled.value - d.value, error: doubled.error + d.error } };", [GEOM]),
 "D18_centred_at_arc_sets_origin": (C, "Held::Arc { .. } => at.translated(center - Conic::center(self)),", "Held::Arc { .. } => at.with_origin(center),", [GEOM]),
}
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/tgt-mut", CARGO_INCREMENTAL="0")
for name in (sys.argv[1:] or list(M)):
    path, old, new, scopes = M[name]
    src = open(path).read()
    if src.count(old) != 1:
        print(f"MUTANT {name}: PATTERN COUNT {src.count(old)}", flush=True); continue
    open(path, "w").write(src.replace(old, new))
    res = []
    try:
        for sc in scopes:
            r = subprocess.run(["cargo", "nextest", "run", "--profile", "ci", "--no-fail-fast", *sc], env=env, capture_output=True, text=True)
            out = r.stdout + r.stderr
            if "could not compile" in out:
                res.append((sc[1], "COMPILE-ERROR", [l for l in out.splitlines() if l.startswith("error")][:3])); continue
            fails = sorted({l.split(")")[-1].strip() for l in out.splitlines() if l.lstrip().startswith(("FAIL", "SIGABRT", "TIMEOUT"))})
            summ = [l.strip() for l in out.splitlines() if "tests run:" in l]
            res.append((sc[1], "RED" if fails else "GREEN(survives)", len(fails), fails[:6], summ[-1:]))
    finally:
        open(path, "w").write(src)
    print(f"MUTANT {name}: {res}", flush=True)
print("ALLDONE", flush=True)
