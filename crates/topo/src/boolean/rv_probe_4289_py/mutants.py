import subprocess, sys, os, shutil
W='/home/user/rv-wt'; F=W+'/crates/topo/src/boolean/sectors.rs'
orig=open(F).read()
M={
 'P1 one piece only': ('            held ^= crosses;','            let _ = crosses;'),
 'P2 met inverted': ('        Some(SideCode::Out) => (true, true),\n        Some(SideCode::In) => (false, true),','        Some(SideCode::Out) => (false, true),\n        Some(SideCode::In) => (true, true),'),
 'P3 bound sides swapped': ('[(s.start, side), (s.end, code)]','[(s.start, code), (s.end, side)]'),
 'M1 crossing counted at the p end': ('            if side == SideCode::On || side == code {\n                continue;\n            }','            if side == SideCode::On {\n                held ^= true;\n                continue;\n            }\n            if side == code {\n                continue;\n            }'),
 'M2 p in-band reading not passed over': ('                Err(e) => {\n                    escalation.get_or_insert(e);\n                    continue \'reference;\n                }\n            };\n            if side','                Err(_) => continue,\n            };\n            if side'),
 'M3 unpointed read as pointed': ('        _ => (true, false),','        _ => (true, true),'),
 'M4 parity reset per face': ('            held ^= crosses;','            held = (base == SideCode::In) ^ crosses;'),
 'M5 arc through a vertex read as no crossing': ('                    Ok(None) => continue \'reference,','                    Ok(None) => crosses = false,'),
}
sel=sys.argv[1:] or list(M)
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/rv-wt-target')
for name in sel:
    a,b=M[name]
    assert orig.count(a)==1, (name, orig.count(a))
    open(F,'w').write(orig.replace(a,b))
    res=[]
    for args in (['--lib','boolean::sectors','boolean::vtxfac'], ['--test','all','a_vertex_read_by_two_sector_passes','a_vertex_read_again_classes_every_edge'], ['--lib']):
        p=subprocess.run(['cargo','nextest','run','-p','topo','--no-fail-fast']+args,cwd=W,env=env,capture_output=True,text=True)
        out=p.stdout+p.stderr
        fails=[l.strip().split()[-1] for l in out.splitlines() if l.strip().startswith('FAIL [')]
        if 'error[' in out and 'could not compile' in out: fails=['COMPILE ERROR']
        res.append(sorted(set(fails)))
        if args[0]=='--test' and fails: break
    print(name, '->', 'KILLED' if any(res) else 'SURVIVED', res, flush=True)
open(F,'w').write(orig)
