import sys
p='crates/topo/src/pcurves.rs'
s=open(p).read()
m=sys.argv[1]
def rep(a,b):
    global s
    assert a in s, (m,a)
    s=s.replace(a,b,1)
if m=='M0':  # in-band arm answers OnMark again
    rep("BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),","BranchMiss::OnMark => Err(BranchMiss::OnMark),")
elif m=='M1':  # InBand mapped to Off inside the read
    rep("                in_band.get_or_insert(cause);\n","                let _ = cause;\n")
elif m=='M2':  # first InBand returns before a later Closes
    rep("                in_band.get_or_insert(cause);\n","                return NearRead::InBand(cause);\n")
elif m=='M3a':  # band test off by 2 (looser: half the margin)
    rep('decide("pcurve_loop_pole_gap", arm.meter(off), band)','decide("pcurve_loop_pole_gap", arm.meter(off * T::from_f64(0.5)), band)')
elif m=='M3b':  # band test off by 2 (stricter: twice the margin)
    rep('decide("pcurve_loop_pole_gap", arm.meter(off), band)','decide("pcurve_loop_pole_gap", arm.meter(off * T::from_f64(2.0)), band)')
elif m=='M4':  # narrowing removed: undecided mark takes the read's cause
    rep("""                NearRead::InBand(cause) => match miss {
                    BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),
                    undecided => Err(undecided),
                },""","""                NearRead::InBand(cause) => Err(BranchMiss::Undecided(cause)),""")
elif m=='M5':  # last InBand cause instead of first
    rep("                in_band.get_or_insert(cause);\n","                in_band = Some(cause);\n")
open(p,'w').write(s)
