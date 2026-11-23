import json,sys,collections,re
name=sys.argv[1] if len(sys.argv)>1 else "ladder-eps"
floor={"1e-9":1e8,"1e-12":1e5}
for e in ("1e-9","1e-12"):
    B=json.load(open(f"base-{e}/{name}{e}.judged.json")); H=json.load(open(f"head-{e}/{name}{e}.judged.json"))
    key=lambda j:(j["door"],j["lane"],j["fat"],j["target"],j["case"],j["shift"])
    Bm={key(j):j for j in B}; Hm={key(j):j for j in H}
    st=lambda j: "refused" if j["said"]=="refused" else ("WRONG" if j["wrong"] else "right")
    def row(f):
        m=re.search(r'row: "([a-z_0-9]+)"',f or ""); 
        if m: return m.group(1)
        m=re.search(r'Unsettled\((\w+)',f or ""); 
        return m.group(1) if m else (f or "")[:40]
    c=collections.Counter(); ex={}
    for k in Bm:
        b,h=Bm[k],Hm[k]
        if st(b)=="right" and st(h)=="refused" and k[1]=="f64":
            below = k[3] < floor[e]
            kk=("below-floor" if below else "at/above", h["truth"], h["place"], k[0], row(h["fault"]))
            c[kk]+=1; ex.setdefault(kk,(k,h["gap"],b["said"]))
    print("==",e,"f64 right->refused")
    for k,v in sorted(c.items()): print("  ",k,v,"e.g.",ex[k])
