import sys,re,collections
def load(p):
    d={}
    for l in open(p):
        if " => " in l:
            l=l.split(" ... ")[-1]
            k,_,v=l.partition(" => ")
            d[k.strip()]=v.strip()
    return d
def cls(v):
    if "cert=false" in v and "SOUND" in v: return "SOUND-NOCERT"
    if v.startswith("OK SOUND") or v.startswith("SOUND") or v.startswith("EMPTY ok"): return "SOUND"
    if v.startswith("OK BAD") or v.startswith("BAD") or "WRONG" in v: return "BAD"
    if v.startswith("NONOP"): return "NONOP"
    if "UNMEAS" in v: return "UNMEAS"
    if v.startswith("ERR"):
        m=re.match(r"ERR (\w+(?:\(\w+)?(?: \{ \w+)?)",v); return "ERR:"+(m.group(1) if m else v[:40])
    return v[:30]
a=load(sys.argv[1]); b=load(sys.argv[2])
print("poses",len(a),len(b),"common",len(set(a)&set(b)))
ca=collections.Counter(cls(v) for v in a.values()); cb=collections.Counter(cls(v) for v in b.values())
print("base:",dict(ca)); print("head:",dict(cb))
t=collections.Counter()
ex={}
for k in a:
    if k in b:
        x,y=cls(a[k]),cls(b[k])
        if x!=y or (x.startswith("ERR") and a[k]!=b[k] and False):
            t[(x,y)]+=1; ex.setdefault((x,y),k)
for k,v in sorted(t.items(),key=lambda kv:-kv[1]): print(v,k[0],"->",k[1]," e.g.",ex[k])
