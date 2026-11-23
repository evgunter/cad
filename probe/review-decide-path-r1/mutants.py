import subprocess, sys, os
WT='/home/user/review-decide-mutwt'
SYM=WT+'/crates/geom-core/src/sym.rs'; CON=WT+'/crates/geom-core/src/sym/contradiction.rs'
M=[
 ('M1_exact_false',CON,'let exact = matches!(sess.witness, Some(crate::Witness::Exact));','let exact = false;'),
 ('M2_algebra_first_attempt_only',SYM,'    for (attempt, rules, bits) in sess.retry.attempts(sess.rules) {','    if path == Path::Algebra { return None; }\n    for (attempt, rules, bits) in sess.retry.attempts(sess.rules) {'),
 ('M3_no_receipt_restore',CON,'        sess.counts = counts;\n','        let _ = counts;\n'),
 ('M4_no_enclosure_check',SYM,'(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),','true || (lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),'),
 ('M5_check_ignores_free',SYM,'(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),','(lo <= 0.0 && 0.0 <= hi),'),
 ('M6_instrument_keeps_free',CON,'    report.contradictions.retain(|c| !reaches_free(c.node));','    let _ = reaches_free;'),
 ('M7_never_asks_door',CON,'None if door && !sess.registry.is_empty() => {','None if false && door && !sess.registry.is_empty() => {'),
 ('M8_walks_absent',CON,'if !sess.nodes.contains_key(&d.node) {','if false && !sess.nodes.contains_key(&d.node) {'),
 ('M9_algebra_none',CON,'.or_insert_with(|| match ladder(sess, d.node, Path::Algebra) {','.or_insert_with(|| match ladder(sess, d.node, Path::Algebra).filter(|_| false) {'),
 ('R1_door_first_attempt_only',CON,'ladder(sess, d.node, Path::Decision).map(|at| (Event::Registration, at))','ladder(sess, d.node, Path::Decision).filter(|at| at.1 == 0).map(|at| (Event::Registration, at))'),
 ('R3_check_skips_door_rung',SYM,'(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),','(lo <= 0.0 && 0.0 <= hi) || matches!(rung, Rung::Door) || reaches_free(self.node),'),
 ('R4_check_skips_early_rung',SYM,'(lo <= 0.0 && 0.0 <= hi) || reaches_free(self.node),','(lo <= 0.0 && 0.0 <= hi) || matches!(rung, Rung::Early) || reaches_free(self.node),'),
 ('R6_definite_sign_still_walks',SYM,'            count_decision(false);\n            report::record(&numeric, false, None, self.value.enclosure_probe());\n            return numeric;','            let _ = discharge_in(self.node, Path::Decision);\n            count_decision(false);\n            report::record(&numeric, false, None, self.value.enclosure_probe());\n            return numeric;'),
]
env=dict(os.environ,CARGO_INCREMENTAL='0',CARGO_TARGET_DIR='/home/user/review-decide-mut-target',CARGO_PROFILE_DEV_OPT_LEVEL='1',CARGO_PROFILE_TEST_OPT_LEVEL='1',CARGO_PROFILE_DEV_DEBUG='line-tables-only',CARGO_PROFILE_TEST_DEBUG='line-tables-only')
FILT="test(/sym_contradiction_instrument_rows|sym11_witness_kind|m10_9_r1_sym_probes|review_decide_path_r1|the_walks_and_the_ring|sym_deferred|sym::tests/)"
only=sys.argv[1:]
for name,f,a,b in [('BASELINE',None,None,None)]+M:
    if only and name not in only: continue
    subprocess.run(['git','-C',WT,'checkout','-q','--','.'],check=True)
    if f:
        s=open(f).read()
        if s.count(a)!=1: print(name,'ANCHOR COUNT',s.count(a),flush=True); continue
        open(f,'w').write(s.replace(a,b))
    r=subprocess.run(['cargo','nextest','run','-j','2','-p','geom-core','--ignore-default-filter','--no-fail-fast','-E',FILT],cwd=WT,env=env,capture_output=True,text=True)
    out=r.stdout+r.stderr
    import re
    out=re.sub(r'\x1b\[[0-9;]*m','',out)
    fails=sorted(set(re.findall(r'^\s+FAIL \[.*?\] \(.*?\) (\S+ \S+)',out,re.M)))
    summ=[l.strip() for l in out.splitlines() if 'Summary' in l or l.startswith('error')]
    print('==',name,'|',summ[-1:] ,flush=True)
    for x in fails: print('   RED',x,flush=True)
    if 'error' in ''.join(summ): print(out[-1500:],flush=True)
subprocess.run(['git','-C',WT,'checkout','-q','--','.'],check=True)
print('ALLDONE',flush=True)
