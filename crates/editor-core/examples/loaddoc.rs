//! Review probe: load each argument's document on a 1 MiB thread.
#![allow(clippy::all, missing_docs)]
fn main() {
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).unwrap();
        let r = std::thread::Builder::new()
            .stack_size(1 << 20)
            .spawn(move || {
                let t = std::time::Instant::now();
                let r = editor_core::load(&text, geom_core::Tol::witness());
                let s = match r {
                    Ok(l) => format!("OK nodes={}", l.doc.len()),
                    Err(e) => format!("ERR {e:?}"),
                };
                format!("{s} in {:?}", t.elapsed())
            })
            .unwrap()
            .join();
        match r {
            Ok(s) => println!("{path}: {:.300}", s),
            Err(_) => println!("{path}: PANIC"),
        }
    }
}
