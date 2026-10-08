import subprocess, sys, os
P = sys.argv[1]
CJ = P + '/crates/topo/src/chord_join.rs'
JN = P + '/crates/topo/src/boolean/join.rs'
RU = P + '/crates/topo/src/splitting/rules.rs'
M = {
 'chord_join_span_alone': (CJ, 'let sec = agreed_section(read(&reach), read(&round), band).map_err(table)?;', 'let sec = agreed_section(read(&reach), read(&reach), band).map_err(table)?;'),
 'germ_span_alone': (JN, 'crate::chord_join::agreed_section(read(across), read(round), band)', 'crate::chord_join::agreed_section(read(across), read(across), band)'),
 'agreed_serves_span_where_round_escalates': (CJ, '        (Err(e), _) | (Ok(_), Err(e)) => Err(e),', '        (Err(e), _) => Err(e),\n        (Ok(s), Err(_)) => Ok(s),'),
 'agreed_any_served_class_agrees': (CJ, '(Ok(s), Ok(r)) if core::mem::discriminant(&s) == core::mem::discriminant(&r) => Ok(s),', '(Ok(s), Ok(_r)) => Ok(s),'),
 'cylinder_gate_plane_only': (RU, '        geom::Surface::Plane { .. } | geom::Surface::Cylinder { .. }\n    );', '        geom::Surface::Plane { .. }\n    );'),
}
which = sys.argv[2:] or list(M)
filt = "test(chord_join::tests::) | test(frame_dispatch_tests::) | test(splitting::rules::tests::)"
env = dict(os.environ, CARGO_TARGET_DIR='/home/user/r2p-target')
for name in which:
    F, a, b = M[name]
    orig = open(F).read()
    assert orig.count(a) == 1, name
    open(F, 'w').write(orig.replace(a, b))
    try:
        r = subprocess.run(['cargo', 'nextest', 'run', '-p', 'topo', '--lib', '--no-fail-fast', '-E', filt], cwd=P, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        fails = sorted(set(l.strip().split(')')[-1].strip() for l in out.splitlines() if l.strip().startswith('FAIL [')))
        summ = [l.strip() for l in out.splitlines() if 'Summary' in l or l.startswith('error')][:2]
        print(name, 'KILLED' if r.returncode else 'SURVIVED', summ, fails, flush=True)
    finally:
        open(F, 'w').write(orig)
