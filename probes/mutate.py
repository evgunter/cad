import sys
m=sys.argv[1]; p='crates/topo/src/boolean/ops.rs'
s=open(sys.argv[2]).read()
def rep(a,b,count=1):
    global s
    assert s.count(a)>=1, (m,a)
    s=s.replace(a,b,count)
if m=='M1':
    rep('walk: T::from_f64(band.escalate()) * settled','walk: T::from_f64(0.0 * band.escalate()) * settled')
    rep('exact: Interval::from_f64(band.escalate()) * settled_exact','exact: Interval::from_f64(0.0 * band.escalate()) * settled_exact')
elif m=='M2':
    rep('walk: T::from_f64(band.escalate()) * settled','walk: T::from_f64(100.0 * band.escalate()) * settled')
    rep('exact: Interval::from_f64(band.escalate()) * settled_exact','exact: Interval::from_f64(100.0 * band.escalate()) * settled_exact')
elif m=='M3':
    rep('            let (mut margin, mut area) = (allowance.exact, Interval::zero());','            return Err(implausible());\n            #[allow(unreachable_code)]\n            let (mut margin, mut area) = (allowance.exact, Interval::zero());')
elif m=='M4':
    rep('''    let escalated = |diag| BooleanError::Escalated {
        decision: BooleanDecision::VolumeBackstop,
        diag,
    };
    loop {
        let p = got.props();''','''    if true { return Ok(()); }
    let escalated = |diag| BooleanError::Escalated {
        decision: BooleanDecision::VolumeBackstop,
        diag,
    };
    loop {
        let p = got.props();''')
elif m=='M5':
    rep('            if ba && bb {\n                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);','            if false && ba && bb {\n                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);')
open(p,'w').write(s)
