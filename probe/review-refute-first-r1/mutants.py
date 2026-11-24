import subprocess, sys, os, re
ROOT='/home/user/cad'
P=ROOT+'/crates/geom-core/src/sym/point.rs'
S=ROOT+'/crates/geom-core/src/sym.rs'
OUT=os.path.expanduser('~/.local/share/cad-work/review-refute-first/mut')
M={
 'M9_cert_dropped':(P,"    out.filter(|x| x.is_certified())\n","    out\n"),
 'M10_memo_kept':(S,"                sess.points.clear();\n","                let _ = &sess.points;\n"),
 'M10b_memo_kept_on_move':(S,"sess.params.insert(indet_param(symbol.0), (lo, hi)) != Some((lo, hi))","sess.params.insert(indet_param(symbol.0), (lo, hi)).is_none()"),
 'Mtie_when_gt_at_zero':(P,"        Some(d) if Bounds::hi(d) <= 0.0 => &[1],\n        Some(d) if Bounds::lo(d) > 0.0 => &[2],","        Some(d) if Bounds::hi(d) < 0.0 => &[1],\n        Some(d) if Bounds::lo(d) >= 0.0 => &[2],"),
 'Matan2_swapped':(P,"Some(Real::atan2(kid(0)?, kid(1)?))","Some(Real::atan2(kid(1)?, kid(0)?))"),
 'Mdual_reads':(S,"impl<T> PointReading for crate::Dual<T> {\n    fn refutes(_: SymId) -> bool {\n        false","impl<T> PointReading for crate::Dual<T> {\n    fn refutes(n: SymId) -> bool {\n        point::refutes(n)"),
 'Mcopysign_swapped':(P,"Some(Real::copysign(kid(0)?, kid(1)?))","Some(Real::copysign(kid(1)?, kid(0)?))"),
 'Mpowi_abs_exponent':(P,"Real::powi(kid(0)?, n.payload as u32 as i32)","Real::powi(kid(0)?, (n.payload as u32 as i32).abs())"),
 'Mfloor_identity':(P,"SymOp::Floor => Some(Real::floor(kid(0)?))","SymOp::Floor => Some(kid(0)?)"),
 'Mhull_read_as_max':(P,"SymOp::Opaque | SymOp::Hull | SymOp::Free => None,","SymOp::Opaque | SymOp::Free => None,\n            SymOp::Hull => Some(Real::max(kid(0)?, kid(1)?)),"),
}
env=dict(os.environ, CARGO_INCREMENTAL='0', CARGO_TARGET_DIR='/home/user/review-refute-target', CARGO_PROFILE_DEV_OPT_LEVEL='1', CARGO_PROFILE_TEST_OPT_LEVEL='1', CARGO_PROFILE_DEV_DEBUG='line-tables-only', CARGO_PROFILE_TEST_DEBUG='line-tables-only')
for name in sys.argv[1:]:
    f,a,b=M[name]
    src=open(f).read()
    assert src.count(a)==1,(name,src.count(a))
    open(f,'w').write(src.replace(a,b))
    try:
        r=subprocess.run(['cargo','nextest','run','--profile','ci','-p','geom-core','--no-fail-fast','-E','not test(review_the_battery)'],cwd=ROOT,env=env,capture_output=True,text=True)
        log=r.stdout+r.stderr
        open(f'{OUT}/{name}.fast.log','w').write(log)
        fails=sorted(set(re.findall(r'^\s+(?:FAIL|SIGABRT|TIMEOUT)\s+\[.*?\]\s+(?:\(.*?\)\s+)?(\S+ \S+)',log,re.M)))
        summ=[l for l in log.splitlines() if 'Summary' in l or l.startswith('error')]
        r2=subprocess.run(['cargo','nextest','run','--profile','ci','-p','geom-core','--test','all','-E','test(review_the_battery)','--run-ignored','all','--no-capture'],cwd=ROOT,env=env,capture_output=True,text=True)
        rows=[l for l in (r2.stdout+r2.stderr).splitlines() if l.startswith('ROW')]
        open(f'{OUT}/{name}.rows.txt','w').write('\n'.join(rows)+'\n')
        base=open(os.path.expanduser('~/.local/share/cad-work/review-refute-first/b.txt')).read().splitlines()
        lost=[x for x,y in zip(base,rows) if x!=y]
        print(f'== {name}: {summ} fails={fails} battery_rows={len(rows)} differing_from_base={len(lost)}',flush=True)
        for l in lost[:3]: print('   ',l,flush=True)
    finally:
        open(f,'w').write(src)
