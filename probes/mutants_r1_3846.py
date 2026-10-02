import subprocess,sys
p='crates/topo/src/boolean/reduce.rs'
orig=open(p).read()
M={
'M1_hold_disabled':[("""                    CurvedEvent::Interior => {
                        held.push(""","""                    CurvedEvent::Interior => {
                        return Err(BooleanError::CurvedPierceUnsupported { operand: x_is, face, edge: edge_key, band });
                        #[allow(unreachable_code)]
                        held.push(""")],
'M2_settle_accepts_all':[("""    for h in held {
        let (x, y)""","""    for h in held.into_iter().take(0) {
        let (x, y)""")],
'M2b_settle_ignores_verdict':[("""CurvedEvent::Interior | CurvedEvent::Pierce { .. } => return Err(h.refusal),""","""CurvedEvent::Interior | CurvedEvent::Pierce { .. } => {}""")],
'M3_leading_only':[("""            if v == h.end {
                reached = true;
                break;
            }""","""            if v == h.end || true {
                reached = true;
                break;
            }""")],
'M4a_line_wall_not_held':[("""SpanVerdict::Unsettled if covered => Ok(CurvedEvent::Interior),""","""SpanVerdict::Unsettled if covered && false => Ok(CurvedEvent::Interior),""")],
'M4b_torus_arc_not_held':[("""SpanVerdict::Unsettled if covered && s1 == Sign::Positive => {""","""SpanVerdict::Unsettled if covered && s1 == Sign::Positive && false => {""")],
'M5_circle_not_held':[("""if ends == [None, None] && !inside.clear() {""","""if ends == [None, None] && !inside.clear() && false {""")],
}
names=sys.argv[1:] or list(M)
for n in names:
    s=orig
    for a,b in M[n]:
        assert s.count(a)==1,(n,a)
        s=s.replace(a,b)
    open(p,'w').write(s)
    r=subprocess.run(r"cargo nextest run -p topo -p sweep --no-fail-fast 2>&1 | grep -E '^\s+FAIL|Summary' | sort -u",shell=True,capture_output=True,text=True,env=dict(__import__('os').environ,CARGO_INCREMENTAL='0'))
    print('=== ',n); print(r.stdout[-3000:]); sys.stdout.flush()
open(p,'w').write(orig)
