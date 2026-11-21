"""Apply the reviewer's decision logger to crates/geom-core/src/k_stats.rs (same edit on base and head).
Usage: python3 klog.patch.py <worktree>.  Inert unless KLOG_DIR is set."""
import sys
p = sys.argv[1] + '/crates/geom-core/src/k_stats.rs'
s = open(p).read()
anchor = "    CURRENT.with(|c| c.set(prev));\n"
assert s.count(anchor) == 1
s = s.replace(anchor, anchor + "    klog::tally::<T>(name, &outcome);\n")
s += r'''
#[allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, missing_docs, unsafe_code)]
mod klog {
    use super::{Decided, Indeterminate};
    use std::collections::BTreeMap;
    use std::hash::{Hash, Hasher};
    use std::sync::{Mutex, OnceLock};
    type Key = (&'static str, &'static str, &'static str);
    static TALLY: Mutex<BTreeMap<Key, (u64, u64)>> = Mutex::new(BTreeMap::new());
    static DIR: OnceLock<Option<String>> = OnceLock::new();
    unsafe extern "C" {
        fn atexit(cb: extern "C" fn()) -> i32;
    }
    extern "C" fn dump() {
        let Some(Some(dir)) = DIR.get() else { return };
        let Ok(t) = TALLY.lock() else { return };
        if t.is_empty() {
            return;
        }
        let args: Vec<String> = std::env::args().collect();
        let exe = std::path::Path::new(&args[0]).file_name().unwrap().to_string_lossy().to_string();
        let exe = match exe.rfind('-') {
            Some(i) => exe[..i].to_string(),
            None => exe,
        };
        let test = args.iter().position(|a| a == "--exact").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "ALL".into());
        let name: String = format!("{exe}__{test}").chars().map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '.' }).collect();
        let mut out = String::new();
        for ((ty, pred, o), (n, h)) in t.iter() {
            out.push_str(&format!("{ty}\t{pred}\t{o}\t{n}\t{h:016x}\n"));
        }
        let _ = std::fs::write(format!("{dir}/{name}.tsv"), out);
    }
    pub(super) fn tally<T>(name: &'static str, outcome: &Result<Decided, Indeterminate>) {
        let dir = DIR.get_or_init(|| {
            let d = std::env::var("KLOG_DIR").ok();
            if d.is_some() {
                unsafe { atexit(dump) };
            }
            d
        });
        if dir.is_none() {
            return;
        }
        let (o, m) = match outcome {
            Ok(d) => (
                match d.sign {
                    super::Sign::Positive => "pos",
                    super::Sign::Negative => "neg",
                    super::Sign::Zero => "zero",
                },
                format!("{:?}", d.margin),
            ),
            Err(e) => ("indet", format!("{:?}", e.margin)),
        };
        let mut h = std::collections::hash_map::DefaultHasher::new();
        m.hash(&mut h);
        let mut t = TALLY.lock().unwrap();
        let e = t.entry((std::any::type_name::<T>(), name, o)).or_insert((0, 0));
        e.0 += 1;
        e.1 = e.1.wrapping_add(h.finish());
    }
}
'''
open(p, 'w').write(s)
# The workspace forbids unsafe; the logger's atexit needs it (instrumented trees only).
c = sys.argv[1] + '/Cargo.toml'
t = open(c).read()
assert t.count('unsafe_code = "forbid"') == 1
open(c, 'w').write(t.replace('unsafe_code = "forbid"', 'unsafe_code = "allow"'))
