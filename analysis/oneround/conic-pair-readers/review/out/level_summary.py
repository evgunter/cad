import json,collections,sys
# usage: level_summary.py level-head-<eps>.judged.json ...  (probe_level.py's output)
for path in sys.argv[1:]:
    out=json.load(open(path)); c=collections.Counter()
    for j in out:
        zero,esc=j["zero"],j["esc"]
        place="near" if abs(j["theta"]-j["theta0"])<0.3 else "far"
        seam="seam at the anchor" if j["seam"]==0 else "re-seamed a quarter turn"
        v=j["verdict"]; d=j["dist"]
        if v=="on" and d>esc: k="WRONG: On past K eps"
        elif v=="off" and d<=zero: k="WRONG: Off within eps"
        elif v=="off" and d<=esc: k="Off inside the gap"
        elif v=="on" and d>zero: k="On inside the gap"
        elif v=="refused": k="refused"
        else: k="right"
        c[(seam,place,k)]+=1
    print(path,"zero",zero,"esc",esc)
    for k,v in sorted(c.items()): print("   ",*k,v,sep="\t")
