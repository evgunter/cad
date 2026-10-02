# Claim-6 instrumentation of join.rs find_match / loose_partners (apply after
# segment_log_instr.py, which provides rest::r1_log). Logs, per match, the
# chosen candidate's swept angle phi (germ_separation / radius), the minimum
# phi over all facing candidates, and disagree = chord-nearest != arc-nearest.
f='crates/topo/src/boolean/join.rs'
s=open(f).read()

old_fm="""                    let frame = germ_section_frame(red, &rga, band)?;
                    if !germs_face_each_other(frame, &rga, &ega, p_c, p_e, band)? {
                        continue;
                    }"""
add_fm="""
                    let r1phi = frame.map(|f| { let s = germ_separation(f, &rga, p_c, p_e); let d = p_c - f.0; let u = d - f.1 * f.1.dot(d); s / u.norm() });
                    let r1take = match &best { None => true, Some((bd, _)) => decide("bool_join_nearest", Margin::of(dist - *bd), band).ok() == Some(Sign::Negative) };
                    if r1take { r1best = r1phi; r1bp = format!("{:?}->{:?} d1={:?} d2={:?} frame={:?}", p_c, p_e, rga.dir, ega.dir, frame); }
                    if let Some(ph) = r1phi { r1all.push(format!("[{ph:?} {:?}->{:?} chord={:?}]", p_c, p_e, dist)); r1min = Some(match r1min { None => ph, Some(m) => if decide("r1", Margin::of(ph - m), band).ok() == Some(Sign::Negative) { ph } else { m } }); }"""
s=s.replace(old_fm, old_fm+add_fm,1)
s=s.replace("    let mut best: Option<(T, Match)> = None;","    let mut best: Option<(T, Match)> = None;\n    let mut r1best: Option<T> = None; let mut r1min: Option<T> = None; let mut r1bp = String::new(); let mut r1all: Vec<String> = Vec::new();",1)
s=s.replace("    Ok(best.map(|(_, m)| m))\n}","""    if let Some(phi) = r1best { super::rest::r1_log(&format!("FINDMATCH {r1bp} ALL {r1all:?} phi={phi:?} minphi={r1min:?} disagree={}", r1min.map(|m| decide("r1", Margin::of(phi - m), band).ok() == Some(Sign::Positive)) == Some(true))); }
    Ok(best.map(|(_, m)| m))
}""",1)
old_lp="""            let frame = germ_section_frame(red, &g, band)?;
            if !germs_face_each_other(frame, &g, &g2, p, p2, band)? {
                continue;
            }"""
add_lp="""
            let r1phi = frame.map(|f| { let s = germ_separation(f, &g, p, p2); let d = p - f.0; let u = d - f.1 * f.1.dot(d); s / u.norm() });
            let r1take = match &best { None => true, Some((bd, _)) => decide("bool_join_nearest", Margin::of(dist - *bd), band).ok() == Some(Sign::Negative) };
            if r1take { r1best = r1phi; }
            if let Some(ph) = r1phi { r1min = Some(match r1min { None => ph, Some(m) => if decide("r1", Margin::of(ph - m), band).ok() == Some(Sign::Negative) { ph } else { m } }); }"""
s=s.replace(old_lp, old_lp+add_lp,1)
s=s.replace("""        let mut best: Option<(T, (usize, usize))> = None;
        for &(j, t) in &loose {""","""        let mut best: Option<(T, (usize, usize))> = None;
        let mut r1best: Option<T> = None; let mut r1min: Option<T> = None;
        for &(j, t) in &loose {""",1)
s=s.replace("""        let partner = best.map(|(_, jt)| jt);""","""        if let Some(phi) = r1best { super::rest::r1_log(&format!("LOOSE phi={phi:?} minphi={r1min:?} disagree={}", r1min.map(|m| decide("r1", Margin::of(phi - m), band).ok() == Some(Sign::Positive)) == Some(true))); }
        let partner = best.map(|(_, jt)| jt);""",1)
open(f,'w').write(s)
