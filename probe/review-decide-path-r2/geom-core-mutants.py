import subprocess, sys, os
R='/home/user/cad'
C='crates/geom-core/src/sym/contradiction.rs'; S='crates/geom-core/src/sym.rs'
M = {
 'M1_exact_false': (C, "let exact = matches!(sess.witness, Some(crate::Witness::Exact));", "let exact = false;", "sym_contradiction_instrument_rows::an_algebraic_zero"),
 'M2_algebra_first_attempt_only': (S, "    for (attempt, rules, bits) in sess.retry.attempts(sess.rules) {\n        // **The GROWTH GUARD**", "    if path == Path::Algebra { return None; }\n    for (attempt, rules, bits) in sess.retry.attempts(sess.rules) {\n        // **The GROWTH GUARD**", "sym_contradiction_instrument_rows::a_retrys_zero_under"),
 'M3_no_receipt_restore': (C, "        sess.counts = counts;\n", "        let _ = counts;\n", "sym_contradiction_instrument_rows::a_retrys_zero_under"),
 'M4_no_enclosure_check': (S, "(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),", "true || reaches_free(self.node),", "enclosure_excludes_zero_panics"),
 'M5_no_free_read': (S, "(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),", "(lo <= 0.0 && 0.0 <= hi),", "a_free_identity_zero_is_checked"),
 'M5b_no_free_read_brep': (S, "(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),", "(lo <= 0.0 && 0.0 <= hi),", "a_floor_clear_of_zero"),
 'M6_keep_free': (C, "report.contradictions.retain(|c| !reaches_free(c.node));", "report.contradictions.retain(|_| true);", "a_free_identity_zero_is_checked"),
 'M7_no_door': (C, "None if door && !sess.registry.is_empty() => {", "None if false && door && !sess.registry.is_empty() => {", "registration"),
 'M8_walk_absent': (C, "            if !sess.nodes.contains_key(&d.node) {", "            if false && !sess.nodes.contains_key(&d.node) {", "outside_the_session"),
 'M9_algebra_none': (C, "match ladder(sess, d.node, Path::Algebra) {", "match None::<(Rung, u8)> {", "rounded_contradiction"),
 'M10_check_attempt0_only': (S, "(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),", "(lo <= 0.0 && 0.0 <= hi) || attempt > 0 || reaches_free(self.node),", "enclosure_excludes_zero_panics"),
}
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/review-decide-target', CARGO_INCREMENTAL='0', CARGO_PROFILE_DEV_OPT_LEVEL='1', CARGO_PROFILE_TEST_OPT_LEVEL='1', CARGO_PROFILE_DEV_DEBUG='line-tables-only', CARGO_PROFILE_TEST_DEBUG='line-tables-only')
for name in sys.argv[1:]:
    f, old, new, filt = M[name]
    p=os.path.join(R,f); s=open(p).read(); assert s.count(old)==1, name
    open(p,'w').write(s.replace(old,new))
    try:
        r=subprocess.run(['cargo','nextest','run','-p','geom-core']+(['-p','geom-brep'] if name.startswith('M5') else [])+['--no-fail-fast','-E',f'test(/{filt}/)'],cwd=R,env=env,capture_output=True,text=True)
        out=[l for l in (r.stdout+r.stderr).splitlines() if ('PASS [' in l or 'FAIL [' in l or 'Summary' in l or 'error' in l.lower()[:8])]
        print('=====',name, 'exit', r.returncode); print('\n'.join(out[-14:])); sys.stdout.flush()
    finally:
        open(p,'w').write(s)
