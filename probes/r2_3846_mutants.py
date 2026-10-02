import sys
p='/home/user/cad/crates/topo/src/boolean/reduce.rs'
s=open(p).read()
m=sys.argv[1]
def rep(a,b):
    global s
    assert s.count(a)==1, (m, a, s.count(a))
    s=s.replace(a,b)
if m=='M1_hold_disabled':
    rep("""                        });
                        continue;
                    }
                    CurvedEvent::None | CurvedEvent::Recorded => continue,""","""                        });
                        return Err(held.pop().unwrap().refusal);
                    }
                    CurvedEvent::None | CurvedEvent::Recorded => continue,""")
elif m=='M2_settle_accepts_all':
    rep("""    for h in held {
        let (x, y): (&Body<T>, &mut Body<T>) = match h.x_is {""","""    if !held.is_empty() { return Ok(()); }
    for h in held {
        let (x, y): (&Body<T>, &mut Body<T>) = match h.x_is {""")
elif m=='M3_leading_only':
    rep("""            if v == h.end {
                reached = true;
                break;
            }""","""            if true {
                reached = true;
                break;
            }""")
elif m=='M4_line_not_held':
    rep("""                        SpanVerdict::Unsettled if covered => Ok(CurvedEvent::Interior),
""","")
elif m=='M5_circle_not_held':
    rep("""                    if ends == [None, None] && !inside.clear() {
                        return Ok(CurvedEvent::Interior);""","""                    if false {
                        return Ok(CurvedEvent::Interior);""")
elif m=='M6_torus_not_held':
    rep("""                SpanVerdict::Unsettled if covered && s1 == Sign::Positive => {
                    Ok(CurvedEvent::Interior)
                }""","")
elif m=='M7_settle_ignores_interior':
    rep("""                CurvedEvent::None | CurvedEvent::Recorded => {}
                CurvedEvent::Interior | CurvedEvent::Pierce { .. } => return Err(h.refusal),""","""                CurvedEvent::None | CurvedEvent::Recorded | CurvedEvent::Interior => {}
                CurvedEvent::Pierce { .. } => return Err(h.refusal),""")
open(p,'w').write(s)
