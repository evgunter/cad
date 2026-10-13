import subprocess,sys,os,re
BIN="/home/user/tgt-head/release/deps/all-ddea738157d6fa05"
def run(eps,theta,phi,corner):
    env=dict(os.environ,CAD_TOLERANCE_EPS=repr(eps),PROBE_THETA=repr(theta),PROBE_OPENINGS=str(phi))
    if corner: env["PROBE_CORNER"]="1"
    out=subprocess.run([BIN,"--ignored","--exact","corner_names_glue_review_probes::probe_corner_names_glue","--nocapture"],env=env,capture_output=True,text=True).stderr
    res={}
    for l in out.splitlines():
        if not l.startswith("CNG"): continue
        t=l.split(); pose=t[3]; mm=re.search(r" (AuB|BuA|AnB|BnA|A-B|B-A) (.*)$",l); op=mm.group(1); rest=mm.group(2)
        res[(pose,op)]=rest
    return res
eps0=float(sys.argv[1]); theta=float(sys.argv[2]); corner=len(sys.argv)>3
for phi in [2,5,20,60,90]:
    first=run(eps0,theta,phi,corner)
    for (pose,op),r in sorted(first.items()):
        chain=[f"{eps0:g}:{r.split()[0]}({re.search(r'pred=(\S+)',r).group(1) if 'pred=' in r else ''})"]
        cur=r; e=eps0
        for _ in range(4):
            m=re.search(r"offer=([0-9.e+-]+)",cur)
            if not m: break
            e=0.9*float(m.group(1))
            cur=run(e,theta,phi,corner)[(pose,op)]
            chain.append(f"{e:.3g}:{cur.split()[0]}")
            if cur.startswith("BUILD"): break
        print(f"φ={phi} {pose} {op}: "+" -> ".join(chain) + ("" if cur.startswith("BUILD") or "offer=none" in cur else ""), "| final:", cur[:110])
