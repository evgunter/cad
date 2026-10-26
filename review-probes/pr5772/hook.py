import sys,re
p=sys.argv[1]
s=open(p).read()
old="""            wire_combine(
                &crate::verbs::boolean::boolean(),
                (op, node, members),
                id,
                (unprojected, vals),
                doc,
                env.boolean_sweep,
                tol,
            )
        }"""
new="""            let r = wire_combine(
                &crate::verbs::boolean::boolean(),
                (op, node, members),
                id,
                (unprojected, vals),
                doc,
                env.boolean_sweep,
                tol,
            );
            nary_capture(&r);
            r
        }"""
assert s.count(old)==1, s.count(old)
s=s.replace(old,new)
s+='''
/// Review capture hook (not for merging).
fn nary_capture<T: Decide>(r: &OpResult<T>) {
    use std::hash::{Hash, Hasher};
    use std::io::Write;
    let Ok(dir) = std::env::var("NARY_CAPTURE") else {
        return;
    };
    let test = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .unwrap_or_default()
        .replace("::", "__");
    let h = |s: String| {
        let mut x = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut x);
        format!("{:016x}", x.finish())
    };
    let line = match r {
        Ok(o) => format!(
            "OK body={} names={} contacts={} rows={} cited={} carried={} parts={}",
            h(format!("{:?}", o.payload)),
            h(format!("{:?}", o.outputs)),
            h(format!("{:?}", o.contacts)),
            h(format!("{:?}", o.coincidences)),
            h(format!("{:?}", o.cited_inputs)),
            h(format!("{:?}", o.carried)),
            o.parts
        ),
        Err(e) => format!("ERR {}", h(format!("{e:?}"))),
    };
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{test}.txt"))
        .expect("capture file");
    writeln!(f, "{line}").expect("capture write");
}
'''
open(p,'w').write(s)
