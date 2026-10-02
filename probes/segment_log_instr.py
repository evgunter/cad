import sys
root=sys.argv[1]
f=root+'/crates/topo/src/boolean/rest.rs'
s=open(f).read()
helper='''
pub(crate) fn r1_log(msg: &str) {
    if let Ok(dir) = std::env::var("R1DIFF") {
        use std::io::Write;
        let name = std::thread::current().name().unwrap_or("?").to_string();
        let path = format!("{dir}/{}", std::process::id());
        if let Ok(mut fh) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(fh, "{name}\\t{msg}");
        }
    }
}
'''
s=s+helper
s=s.replace("    Ok(Some(segments))\n}","    r1_log(&format!(\"SEG {:?}\", segments.iter().map(|s| (s.a_u, s.a_v, s.b_u, s.b_v)).collect::<Vec<_>>()));\n    Ok(Some(segments))\n}",1)
assert 'SEG' in s
open(f,'w').write(s)
f=root+'/crates/topo/src/boolean/ops.rs'
s=open(f).read()
old="return match super::rest::try_rest_union(red, a, b, decls, band, tol)? {"
new="""let rr = super::rest::try_rest_union(red, a, b, decls, band, tol);
                super::rest::r1_log(&match &rr {
                    Ok(Some(BooleanResult::Body(bb))) => {
                        let mut h: u64 = 0xcbf29ce484222325;
                        for (_k, p) in bb.body.points.iter() {
                            for c in format!("{:?}", p).bytes() {
                                h = (h ^ u64::from(c)).wrapping_mul(0x100000001b3);
                            }
                        }
                        format!("REST body faces={} edges={} pts={:016x}", bb.body.faces().count(), bb.body.edges().count(), h)
                    }
                    Ok(Some(_)) => "REST other".to_string(),
                    Ok(None) => "REST none".to_string(),
                    Err(e) => format!("REST err {e:?}"),
                });
                return match rr? {"""
assert old in s
s=s.replace(old,new)
open(f,'w').write(s)
