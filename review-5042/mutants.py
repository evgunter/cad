import sys,re
p='/home/user/cad/crates/topo/src/pcurves.rs'
s=open(p).read()
m=sys.argv[1]
def rep(a,b):
    global s
    assert s.count(a)==1,(m,a[:60],s.count(a)); s=s.replace(a,b)
if m=='M1':
    rep("""            let at_foot = derive_chart_foot(carrier.eval(t0), surface, half_edge)?
                .map(|foot| stated(computed_foot::<T>(foot).x));
            iso_schedule(what, at_foot.into_iter().collect(), prior)""",
"""            let at_foot = derive_chart_foot(carrier.eval(t0), surface, half_edge)?
                .map(|foot| stated(computed_foot::<T>(foot).x));
            let _ = prior;
            match probe_pick(what, &carrier, (t0, span), surface, band, at_foot) {
                Ok(Some(c)) => Ok(c),
                Ok(None) => Err(refuse("mutant M1")),
                Err(cause) => Err(PcurveMintError::Escalated { half_edge, cause }),
            }""")
elif m=='M2':
    rep("""                [cu0, cu1].map(stated),""","""                [cu0, cu1].map(stated).into_iter().filter(|_| false),""")
elif m=='M3':
    rep("stated.into_iter().chain(columns).chain(rows),","columns.chain(rows).chain(stated),")
elif m=='M4':
    rep("""            Ok(None) => prior,
            Err(cause) => prior.or(Some(cause)),""","""            Ok(None) => { let _ = prior; None }
            Err(cause) => Some(cause),""")
elif m=='M5':
    rep("""            iso_schedule(what, at_foot.into_iter().collect(), prior)""","""            if let Some(cause) = prior { return Err(PcurveMintError::Escalated { half_edge, cause }); }
            iso_schedule(what, at_foot.into_iter().collect(), None)""")
elif m=='M6':
    # boundary columns only probe, then at-foot not offered (only schedule)
    rep("""            iso_schedule(what, at_foot.into_iter().collect(), prior)""","""            let _ = at_foot; iso_schedule(what, Vec::new(), prior)""")
open(p,'w').write(s)
