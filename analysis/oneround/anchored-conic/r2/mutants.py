#!/usr/bin/env python3
"""Reviewer r2's mutants for PR 7203 (C9). Run in a worktree of the frozen
head: each mutant is one textual replacement, the listed test filter is run
with cargo nextest, and the file is restored. usage: mutants.py [names...]"""
import subprocess, sys, os
C = "crates/geom/src/curves/conic.rs"
GEOM = ["-p", "geom"]
BREP = ["-p", "geom-brep", "-E",
        "test(anchored) | test(running) | test(far_centred) | test(resolution) | test(conic) | test(cone_section)"]
BOTH = None
M = {
 # the author's eight
 "A1_anchor_moved_by_quarter_seam": (C, "Born::Anchored => at.with_origin(self.at.origin()),\n            },\n            born: self.born,",
        "Born::Anchored => at.with_origin(self.at.origin() + (at.u_ref() * self.semi_v - self.at.u_ref() * self.semi_u)),\n            },\n            born: self.born,", GEOM),
 "A2_quarter_phase_dropped": (C, "phase: (turn.select_le_zero(c0, s0), turn.select_le_zero(s0, -c0)),", "phase: (c0, s0),", GEOM),
 "A3_half_phase_dropped": (C, "phase: (back.select_le_zero(c0, -c0), back.select_le_zero(s0, -s0)),", "phase: (c0, s0),", GEOM),
 "A4_reversal_keeps_sine": (C, "phase: (self.phase.0, -self.phase.1),", "phase: (self.phase.0, self.phase.1),", GEOM),
 "A5_square_uncharged": (C, "let sq = transcendental(sh).square();", "let t0 = transcendental(sh); let sq = Rounded { value: sh.powi(2), error: T::from_f64(2.0) * sh.abs() * t0.error };", BOTH),
 "A7_sines_uncharged": (C, "fn transcendental<T: Real>(x: T) -> Rounded<T> {\n    Rounded {\n        value: x,\n        error: T::from_f64(2.0 * geom_core::UNIT_ROUNDOFF) * x.abs(),",
        "fn transcendental<T: Real>(x: T) -> Rounded<T> {\n    Rounded {\n        value: x,\n        error: T::zero() * x.abs(),", BOTH),
 "A6_key_anchor_mark_removed": ("crates/mesh/src/memo.rs", "if datum == geom::CurveDatum::Anchor {", "if false && datum == geom::CurveDatum::Anchor {", ["-p", "mesh", "-E", "test(curve_key) | test(key)"]),
 "A8_cone_vertex_as_centre_less_semi_axis": ("crates/geom-brep/src/intersect.rs",
        "let vertex = apex - n * apex_gap + u_major * tilt.near_vertex(delta_sigma);",
        "let vertex = { let far = apex - n * apex_gap + u_major * tilt.near_vertex(delta_sigma); let toward0 = u_major * (T::zero() - delta_sigma.unit_sign()); (far - toward0 * major) + toward0 * major };", BREP),
 # mine
 "R1_anchored_ellipse_evaluates_from_derived_centre": (C,
        "Some(anchor) => {\n                let (ku, kv) = offsets(theta, self.phase);\n                let (u, v) = (self.at.u_ref(), self.at.v_ref());\n                anchor + (u * (self.semi_u * ku) + v * (self.semi_v * kv))",
        "Some(_anchor) => {\n                let ((s, c), v_ref) = self.basis(theta);\n                Conic::center(self) + (self.at.u_ref() * (self.semi_u * c) + v_ref * (self.semi_v * s))", BOTH),
 "R2_phase_pair_swapped_in_evaluator": (C, "let (ku, kv) = offsets(theta, self.phase);\n                let (u, v) = (self.at.u_ref(), self.at.v_ref());\n                anchor +",
        "let (ku, kv) = offsets(theta, (self.phase.1, self.phase.0));\n                let (u, v) = (self.at.u_ref(), self.at.v_ref());\n                anchor +", GEOM),
 "R3_same_content_ignores_form_ellipse": (C, "self.born == other.born\n            && same(&self.at, &other.at)\n            && (self.born == Born::Centred", "same(&self.at, &other.at)\n            && (self.born == Born::Centred", BOTH),
 "R4_same_content_ignores_form_circle": (C, "self.born == other.born && same(&self.at, &other.at)", "same(&self.at, &other.at)", BOTH),
 "R5_same_content_ignores_phase": (C, "|| (same_scalar(self.phase.0, other.phase.0)\n                    && same_scalar(self.phase.1, other.phase.1)))", "|| true)", BOTH),
 "R6_map_scalar_drops_phase": (C, "phase: (f(self.phase.0), f(self.phase.1)),", "phase: (U::one(), U::zero()),", BOTH),
 "R7_placed_drops_phase": (C, "at: map(&self.at),\n            born: self.born,\n            phase: self.phase,", "at: map(&self.at),\n            born: self.born,\n            phase: (T::one(), T::zero()),", BOTH),
 "R8_data_omits_phase": (C, "Born::Anchored => Some(Vec3::new(self.phase.0, self.phase.1, T::zero())),", "Born::Anchored => None,", ["-p", "geom", "-p", "mesh", "-p", "step-import"]),
 "R9_azimuth_ignores_phase_sine": (C, "w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0),", "w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0 * T::zero()),", BOTH),
 "R10_azimuth_ignores_phase_cosine": (C, "self.semi_u * c0 + w.dot(self.at.u_ref()),", "self.semi_u + w.dot(self.at.u_ref()),", BOTH),
 "R11_centre_ignores_phase_sine": (C, "- self.at.u_ref() * (self.semi_u * c0)\n                        - self.at.v_ref() * (self.semi_v * s0),", "- self.at.u_ref() * (self.semi_u * c0),", BOTH),
 "R12_centred_at_anchored_sets_origin": (C, "pub fn centred_at(&self, center: Point3<T>) -> Self {\n        self.placed(|at| match self.born {\n            Born::Centred => at.with_origin(center),\n            Born::Anchored => at.translated(center - Conic::center(self)),\n        })\n    }\n\n    /// The principal frame at the centre ([`Conic::frame`]).\n    #[must_use]\n    pub fn frame(&self) -> Frame<T> {\n        Conic::frame(self)\n    }\n\n    /// `(semi_u, semi_v)` in stored order.",
        "pub fn centred_at(&self, center: Point3<T>) -> Self {\n        self.placed(|at| at.with_origin(center))\n    }\n\n    /// The principal frame at the centre ([`Conic::frame`]).\n    #[must_use]\n    pub fn frame(&self) -> Frame<T> {\n        Conic::frame(self)\n    }\n\n    /// `(semi_u, semi_v)` in stored order.", BOTH),
 "R13_circle_azimuth_anchored_from_origin": (C, "Azimuth::of(w.dot(self.at.v_ref()), self.radius + w.dot(self.at.u_ref()))", "Azimuth::of(w.dot(self.at.v_ref()), w.dot(self.at.u_ref()))", BOTH),
 "R14_sig_anchor_mark_removed": ("crates/step-import/src/build.rs", "core::iter::once(tag | (u64::from(anchored) << 8))", "core::iter::once(tag | (u64::from(anchored && false) << 8))", ["-p", "step-import"]),
 "R15_offsets_first_ignores_phase": (C, "(-(T::from_f64(2.0) * sh.powi(2) - (T::one() - c0)), s - s0)", "(-(T::from_f64(2.0) * sh.powi(2)), s - s0)", GEOM),
 "R16_running_offsets_second_exact": (C, "transcendental(s) - Rounded::exact(s0),", "Rounded::exact(s) - Rounded::exact(s0),", BOTH),
 "R17_circle_map_scalar_drops_form": (C, "Circle {\n            at: frame(&self.at),\n            born: self.born,", "Circle {\n            at: frame(&self.at),\n            born: Born::Centred,", BOTH),
 "R18_zero_point_is_anchor": (C, "Some(_) => self.point_at(T::zero()),", "Some(p) => p,", BOTH),
 "R19_mint_toward_flipped_sign_kept": ("crates/geom-brep/src/intersect.rs", "let e = Curve3::ellipse_anchored(vertex, n, toward, (major, minor), band)?;", "let e = { let k = Curve3::ellipse_anchored(vertex, n, toward, (major, minor), band)?; let f = k.frame().unwrap(); Curve3::ellipse(f.origin(), f.axis(), major, minor, f.u_ref(), band)? };", BREP),
}
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/tgt-mut")
names = sys.argv[1:] or list(M)
for name in names:
    path, old, new, scope = M[name]
    src = open(path).read()
    if src.count(old) != 1:
        print(f"MUTANT {name}: PATTERN COUNT {src.count(old)}", flush=True); continue
    open(path, "w").write(src.replace(old, new))
    scopes = [GEOM, BREP] if scope is None else [scope]
    res = []
    try:
        for sc in scopes:
            r = subprocess.run(["cargo", "nextest", "run", "--profile", "ci", "--no-fail-fast", *sc],
                               env=env, capture_output=True, text=True)
            out = r.stdout + r.stderr
            if "error: could not compile" in out or "error[E" in out:
                res.append((sc[1], "COMPILE-ERROR", [l for l in out.splitlines() if l.startswith("error")][:3])); continue
            fails = sorted({l.split("]")[-1].strip() for l in out.splitlines() if l.lstrip().startswith(("FAIL", "SIGABRT", "TIMEOUT"))})
            summ = [l.strip() for l in out.splitlines() if "tests run:" in l]
            res.append((sc[1], "RED" if fails else "GREEN(survives)", len(fails), fails[:12], summ[-1:] ))
    finally:
        open(path, "w").write(src)
    print(f"MUTANT {name}: {res}", flush=True)
