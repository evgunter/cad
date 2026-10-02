"""Re-apply each mutant, run topo's ci-profile suite plus the slow
ellipse fuzz rows, record killed/survived, restore. Usage: python3 mutants.py [names...]"""
import subprocess, sys, os
ER = "crates/topo/src/boolean/ellipse_roots.rs"
CR = "crates/topo/src/boolean/circle_roots.rs"
RD = "crates/topo/src/boolean/reduce.rs"
M = {
 "subdiv_speed_hi->lo": (ER, "speed_hi: conic.speed_hi(),", "speed_hi: conic.speed_lo(),"),
 "ladder_speed_lo->hi": (ER, "speed_lo: conic.speed_lo(),", "speed_lo: conic.speed_hi(),"),
 "first_harmonic_speed_hi->lo": (ER, "&first,\n            conic.speed_hi(),", "&first,\n            conic.speed_lo(),"),
 "first_harmonic_charge_dropped": (ER, "noise: noise + second,", "noise: noise,"),
 "ladder_lever_lo->hi": (ER, "lever: two * conic.speed_lo(),", "lever: two * conic.speed_hi(),"),
 "at_end_speed_local->lo": (RD, "|c| c.speed_at(t))", "|c| c.speed_lo())"),
 "subdiv_monotone_noise_dropped": (CR, "speed_hi * (least_slope - noise) / most_bend", "speed_hi * least_slope / most_bend"),
 "subdiv_clear_noise_dropped": (CR, "(value(m).abs() - fall - noise) / f_per_metre", "(value(m).abs() - fall) / f_per_metre"),
 "subdiv_fourth_bound_zero": (CR, "let fourth_bound = a1 + T::from_f64(16.0) * a2;", "let fourth_bound = T::zero() * (a1 + a2);"),
 "subdiv_root_side_check_dropped": (CR, "Ok(Sign::Zero) => roots.push(root),\n                _ => return Ok(CircleRoots::Uncertain),", "_ => roots.push(root),"),
 "subdiv_odd_count_check_dropped": (CR, "if roots.len() > 4 || roots.len() % 2 == 1 {", "if roots.len() > 4 {"),
}
names = sys.argv[1:] or list(M)
env = dict(os.environ, CARGO_INCREMENTAL="0")
for name in names:
    path, old, new = M[name]
    src = open(path).read()
    assert src.count(old) == 1, (name, src.count(old))
    open(path, "w").write(src.replace(old, new))
    try:
        r1 = subprocess.run(["cargo", "nextest", "run", "--profile", "ci", "-p", "topo", "-E", "not test(fuzz_3805)", "--no-fail-fast"], capture_output=True, text=True, env=env)
        r2 = subprocess.run(["cargo", "nextest", "run", "-p", "topo", "--ignore-default-filter", "-E", "test(ellipse_roots::fuzz_rows)", "--no-fail-fast"], capture_output=True, text=True, env=env)
        out = r1.stdout + r1.stderr + r2.stdout + r2.stderr
        fails = sorted({l.split("]")[-1].strip() for l in out.splitlines() if l.strip().startswith("FAIL [")})
        built = "error[" not in out
        print(f"{name}: {'KILLED' if fails else ('BUILD-ERROR' if not built else 'SURVIVED')} {fails[:4]}", flush=True)
    finally:
        open(path, "w").write(src)
