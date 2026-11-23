import json,sys,collections
name=sys.argv[1] if len(sys.argv)>1 else "ladder-eps"
for e in ("1e-9","1e-12"):
    B=json.load(open(f"base-{e}/{name}{e}.judged.json")); H=json.load(open(f"head-{e}/{name}{e}.judged.json"))
    key=lambda j:(j["door"],j["lane"],j["fat"],j["target"],j["case"],j["shift"])
    Bm={key(j):j for j in B}; Hm={key(j):j for j in H}
    assert len(Bm)==len(B) and len(Hm)==len(H) and set(Bm)==set(Hm)
    st=lambda j: "refused" if j["said"]=="refused" else ("WRONG" if j["wrong"] else "right")
    mv=collections.Counter(); apart_meet=[]; lost=collections.Counter(); lostfault=collections.Counter()
    for k in Bm:
        b,h=Bm[k],Hm[k]
        mv[(k[1],st(b),st(h))]+=1
        if h["truth"]=="apart" and h["said"]=="meets": apart_meet.append(k)
        if st(b)=="right" and st(h)!="right":
            lost[(k[1],k[0],"fat" if k[2] else "thin",k[3],h["place"],st(h))]+=1
            lostfault[(k[1],k[0],h["fault"].split("{")[0][:90] if h["fault"] else None)]+=1
    print("==",e,"moves (lane, base->head):"); [print("  ",k,v) for k,v in sorted(mv.items())]
    print("  apart twins read as a meeting on head:",len(apart_meet), apart_meet[:5])
    # also between-truth said meets
    print("  head said-by-truth:",sorted(collections.Counter((j["lane"],j["truth"],j["said"]) for j in H).items()))
    print("  right on base, not right on head, by (lane,door,shape,target,place,now):")
    for k,v in sorted(lost.items()): print("    ",k,v)
    print("  their faults:"); [print("    ",k,v) for k,v in sorted(lostfault.items(),key=str)]
    print("  head WRONG:"); 
    for j in H:
        if j["wrong"]: print("    ",j["door"],j["lane"],"fat" if j["fat"] else "thin",j["target"],j["place"],"theta=%.4g"%j["theta"],"gamma=%.3g"%j["gamma"],"shift=%g"%j["shift"],"gap=%.3g"%j["gap"],j["truth"],j["said"],["%.3g"%o for o in j["off"]])
