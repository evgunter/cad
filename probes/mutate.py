import sys
p='/home/user/rev4023-mut/crates/geom-core/src/linalg/lsq.rs'
s=open(p).read()
m=sys.argv[1]
reps={
'M1':("""                    let l = if at(dropped, dropped) < 0.0 {
                        -0.0
                    } else {
                        0.0
                    };""","""                    let l = 0.0f64;"""),
'M2':("poison = poison.or(Some((l, y)));","poison = Some((l, y));"),
'M3':("                        poison = Some(v);","                        poison = poison.or(Some(v));"),
'M4':("""                let mut s = x[i][c];
                for q in (i + 1)..end {
                    s -= at(i, q) * x[q][c];
                }
                if let Some(v) = poison {
                    s -= 0.0 * v;
                } else if flips {
                    s -= -0.0;
                }""","""                let mut s = x[i][c];
                if let Some(v) = poison {
                    s -= 0.0 * v;
                } else if flips {
                    s -= -0.0;
                }
                for q in (i + 1)..end {
                    s -= at(i, q) * x[q][c];
                }"""),
'M5':("""                if let Some((l, y)) = poison {
                    s -= l * y;
                } else if flips {""","""                if let Some((_l, _y)) = poison {
                } else if flips {"""),
}
a,b=reps[m]
assert s.count(a)==1,m
open(p,'w').write(s.replace(a,b))
