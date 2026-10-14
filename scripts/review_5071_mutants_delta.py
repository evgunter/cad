import sys
m=sys.argv[1]
P='crates/topo/src/pcurves.rs'; G='crates/geom-brep/src/pcurve_cache.rs'
files={}
def rep(p,a,b):
    s=files.setdefault(p,open(p).read())
    assert a in s,(m,a)
    files[p]=s.replace(a,b,1)
if m=='M0': rep(P,"BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),","BranchMiss::OnMark => Err(BranchMiss::OnMark),")
elif m=='M1': rep(P,"Err(cause) => NearRead::InBand(cause),","Err(_) => NearRead::Off,")
elif m=='M2': rep(P,"                deferred.get_or_insert(cause);\n","                return Err(cause);\n")
elif m=='M3a': rep(P,"arm.meter(gap - T::from_f64(f64::from(m)) * step)","arm.meter((gap - T::from_f64(f64::from(m)) * step) * T::from_f64(0.5))")
elif m=='M3b': rep(P,"arm.meter(gap - T::from_f64(f64::from(m)) * step)","arm.meter((gap - T::from_f64(f64::from(m)) * step) * T::from_f64(2.0))")
elif m=='M4': rep(P,"""            NearRead::InBand(cause) => match miss {
                BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),
                undecided => Err(undecided),
            },""","""            NearRead::InBand(cause) => Err(BranchMiss::Undecided(cause)),""")
elif m=='M5': rep(P,"                deferred.get_or_insert(cause);\n","                deferred = Some(cause);\n")
elif m=='MB': rep(P,".filter(|m| !orbit.offsets().contains(m));",".filter(|_| false);")
elif m=='MB1': rep(P,".filter(|m| !orbit.offsets().contains(m));",".filter(|m| !orbit.offsets().contains(m)).take(1);")
elif m=='MB2':
    rep(G,"Sign::Zero => return Err(stop(BranchMiss::OnMark, [k, k + 1])),","Sign::Zero => return Err(stop(BranchMiss::OnMark, [k - 1, k])),")
elif m=='MB3':
    rep(G,"Sign::Zero => return Err(stop(BranchMiss::OnMark, [k - 1, k])),","Sign::Zero => return Err(stop(BranchMiss::OnMark, [k, k + 1])),")
for p,s in files.items(): open(p,'w').write(s)
