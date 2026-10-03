import sys,re,collections
def load(p):
    d={};extra=collections.Counter()
    for l in open(p):
        if "J3R2 " not in l: continue
        l=l[l.index("J3R2 ")+5:].strip()
        if " => " in l:
            k,_,v=l.partition(" => "); d[k]=v
        elif "BAD" in l or "MISMATCH" in l: extra[l[:120]]+=1
    return d,extra
def cls(v):
    w=v.split()
    if not w: return "?"
    if w[0] in("SOUND","OK","EMPTY"):
        if "NONOP" in v or "op=NONOP" in v: return "NONOP"
        return "SOUND"
    if w[0]=="ERR":
        m=re.match(r"ERR (\w+(?:\(\w+)?(?: \{ \w+)?)",v); return "ERR:"+(m.group(1) if m else v[:40])
    return w[0]
a,ea=load(sys.argv[1]); b,eb=load(sys.argv[2])
print("poses",len(a),len(b))
print("main:",dict(collections.Counter(map(cls,a.values()))))
print("head:",dict(collections.Counter(map(cls,b.values()))))
print("extra main",dict(ea)); print("extra head",dict(eb))
t=collections.Counter();ex={}
for k in a:
    if k in b and cls(a[k])!=cls(b[k]): t[(cls(a[k]),cls(b[k]))]+=1; ex.setdefault((cls(a[k]),cls(b[k])),k)
for (x,y),n in sorted(t.items(),key=lambda kv:-kv[1]): print(f"  {n} {x} -> {y}  e.g. {ex[(x,y)][:80]}")
nk=collections.Counter((re.search(r"NONOP\S* (\w+\(\w+\))",v).group(1) if re.search(r"NONOP\S* (\w+\(\w+\))",v) else v[-60:]) for v in b.values() if cls(v)=="NONOP"); print("head nonop kinds",dict(nk))
nk=collections.Counter((re.search(r"NONOP\S* (\w+\(\w+\))",v).group(1) if re.search(r"NONOP\S* (\w+\(\w+\))",v) else v[-60:]) for v in a.values() if cls(v)=="NONOP"); print("main nonop kinds",dict(nk))
