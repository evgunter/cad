import sys, re
root = sys.argv[1]
p = root + "/crates/editor-core/src/eval/parts.rs"
s = open(p).read()
old_hit = """        if let Some(hit) = entries.get(&key) {
            return hit.clone();
        }"""
assert old_hit in s
s = s.replace(old_hit, """        if let Some(hit) = entries.get(&key) {
            let v = hit.clone();
            self.probe_log(doc_ref, &v);
            return v;
        }""")
old_tail = """        entries.insert(key, value.clone());
        value
    }"""
assert s.count(old_tail) == 1
s = s.replace(old_tail, """        entries.insert(key, value.clone());
        self.probe_log(doc_ref, &value);
        value
    }

    fn probe_log(&self, doc_ref: &DocRef, value: &Result<PartValue<T>, PartFault>) {
        let Ok(path) = std::env::var("DEPTH_REV_LOG") else { return };
        use std::hash::{Hash, Hasher};
        use std::io::Write;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        let (cls, text) = match value {
            Ok(v) => ("ok".to_string(), format!("OK {:?} {:?} {:?} {:?} {:?} {:?} {:?}", v.body, v.names, v.contacts, v.minted, v.unminted, v.carried, v.carried_unminted)),
            Err(f) => {
                let kind = super::NodeErrorKind::Part { doc_ref: *doc_ref, fault: f.clone() };
                let lines: Vec<String> = kind.carried_chain().map(|l| l.line()).collect();
                let c = format!("{f:?}");
                (format!("err:{}:levels={}", c.split(|ch: char| !ch.is_alphanumeric()).next().unwrap_or(""), lines.len()), format!("ERR {f} || {} || {c}", lines.join(" | ")))
            }
        };
        text.hash(&mut h);
        let chain: Vec<String> = self.chain.iter().map(|r| format!("{r}")).collect();
        let line = format!("GET {} [{}] {} {:016x} {}\\n", self.chain.len(), chain.join(">"), doc_ref, h.finish(), cls);
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = f.write_all(line.as_bytes());
        }
    }""")
open(p, "w").write(s)

p = root + "/crates/editor-core/src/eval/mod.rs"
s = open(p).read()
m = re.search(r"(pub fn evaluate<T>\(\n.*?\n\{\n)(    evaluate_at_descent\(doc, prior, cancel, opts, &\[\],[^\n]*\n)(\})", s, re.S)
assert m, "evaluate"
call = m.group(2).strip().rstrip(";")
new = m.group(1) + f"""    let ev = {call};
    if let Ok(path) = std::env::var("DEPTH_REV_LOG") {{
        use std::hash::{{Hash, Hasher}};
        use std::io::Write;
        let mut h = std::collections::hash_map::DefaultHasher::new();
        format!("{{:?}} {{:?}} {{:?}}", ev.nodes, ev.order, ev.outcome).hash(&mut h);
        let line = format!("EVAL {{}} part_evaluations={{}} nodes={{:016x}}\\n", ev.document, ev.part_evaluations, h.finish());
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {{
            let _ = f.write_all(line.as_bytes());
        }}
    }}
    ev
""" + m.group(3)
s = s[:m.start()] + new + s[m.end():]
open(p, "w").write(s)
print("ok")
