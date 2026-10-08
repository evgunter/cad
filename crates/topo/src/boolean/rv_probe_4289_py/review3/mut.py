import subprocess, sys, os, re, shutil
F='/home/user/r3/crates/topo/src/boolean/sectors.rs'
orig=open(F).read()
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/r3-mut-target')
M={
 'Xsign': [("let x = d * p.dot(n).abs() + p * d.dot(n).abs();","let x = d * p.dot(n).abs() - p * d.dot(n).abs();")],
 'Xnoabs': [("let x = d * p.dot(n).abs() + p * d.dot(n).abs();","let x = d * p.dot(n) + p * d.dot(n);")],
 'Lreach': [("(x, l_d.min(located))","(x, l_d)")],
 'Larm0': [("(x, l_d.min(located))","(x, arm)")],
 'ElseS': [("                        Ok(true) => continue 'reference,","                        Ok(true) => continue,")],
 'EscS': [("""                        Err(e) => {
                            escalation.get_or_insert(e);
                            continue 'reference;
                        }
                    }
                }
            }""","""                        Err(_) => continue,
                    }
                }
            }""")],
 'NoRound': [("let rounding = T::from_f64(8.0 * f64::EPSILON);","let rounding = T::from_f64(0.0);")],
 'Aon': [("                    let Some((x, lever)) = crossing(d, reach, p, s0.arm, s.normal.vec()) else {","                    if side == SideCode::On { continue; }\n                    let Some((x, lever)) = crossing(d, reach, p, s0.arm, s.normal.vec()) else {")],
 'BndLever': [("        if decide(name, Margin::levered(x, lever), band).map_err(escalate)? == Sign::Negative {","        if decide(name, Margin::levered(x, lever.min(s.arm)), band).map_err(escalate)? == Sign::Negative {")],
}
for name in sys.argv[1:]:
    s=orig
    for a,b in M[name]:
        assert s.count(a)==1, (name,a); s=s.replace(a,b)
    open(F,'w').write(s)
    try:
        p=subprocess.run(['cargo','test','--release','-p','topo','--lib','--no-run'],cwd='/home/user/r3',env=env,capture_output=True,text=True)
        m=re.findall(r'(/home/user/r3-mut-target/release/deps/topo-[0-9a-f]+)', p.stdout+p.stderr)
        if not m: print(name,'BUILD FAILED', (p.stderr)[-800:], flush=True); continue
        shutil.copy(m[-1], f'/home/user/bin-{name}'); print(name,'built',flush=True)
    finally:
        open(F,'w').write(orig)
