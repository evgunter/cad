import sys
p = sys.argv[1]
s = open(p).read()
anchor = "    let reach = selection_reach(body, faces, from)?;\n    let crossings = match ray_crossings(body, geos, from, d, reach, band, AIMED_ROWS) {"
assert s.count(anchor) == 1
dump = '''    let reach = selection_reach(body, faces, from)?;
    // Review 6174 probe: an env-gated dump of each aimed ray; prints only.
    #[cfg(any(test, feature = "test-support"))]
    if std::env::var_os("REVIEW_RAY_DUMP").is_some() {
        match ray_crossings(body, geos, from, d, reach, band, AIMED_ROWS) {
            Ok(c) => {
                let all: Vec<String> = c
                    .all()
                    .into_iter()
                    .map(|(sp, h, s)| {
                        format!(
                            "[{:?},{:?}] at {:?} n{} {:?}/{:?} sideless {} mark {}",
                            sp.0, sp.1, h.at, h.count, h.face, h.other, s, h.mark.is_some()
                        )
                    })
                    .collect();
                eprintln!("RAYDUMP face {face:?} aim {aim:?} from {from:?} d {d:?} reach {reach:?}: {all:?}");
            }
            Err(e) => eprintln!("RAYDUMP face {face:?} aim {aim:?} from {from:?} d {d:?}: fault {e:?}"),
        }
    }
    let crossings = match ray_crossings(body, geos, from, d, reach, band, AIMED_ROWS) {'''
s = s.replace(anchor, dump)
open(p, 'w').write(s)
