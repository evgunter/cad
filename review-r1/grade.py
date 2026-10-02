exec(open('review-r1/oracle.py').read())
import sys
cache={}
def load(p):
    d={}
    for l in open(p):
        if "POSE " not in l or " => " not in l: continue
        l=l[l.index("POSE ")+5:].strip()
        k,_,v=l.partition(" => ")
        d[k]=v
    return d
def grade(k,v):
    c=v.split()[0]
    if c=="ERR":
        m=re.match(r"ERR (\w+(?:\(\w+)?)",v); return "ERR:"+m.group(1)
    if c!="BAD" or "empty" in v: return c
    m=re.match(r"(c\d+) (\w+) (\[.*\])(?: DECL)? (\w) (\w\w)$",k)
    mv=re.search(r" v=([-\d.e]+)",v)
    if not m or not mv: return "BAD"
    if not all(x in v for x in ("t2=true","t3p=true","cert=true")): return "BAD"
    case,z,chs,op,order=m.groups()
    if chs not in cache: cache[chs]=inside_area(ast.literal_eval(chs),1)
    A,Ai=cache[chs]; z0,h=Z[z]; dd=min(z0+h,1)-max(z0,0); ov=Ai*dd
    want={"U":4+A*h-ov,"I":ov,"S":(4-ov) if order=="BT" else A*h-ov}[op]
    if abs(float(mv.group(1))-want)<1e-6:
        return "SOUND*" if "op=ok" in v else "NONOP*"
    return "BAD"
a=load(sys.argv[1]); b=load(sys.argv[2])
ga={k:grade(k,v) for k,v in a.items()}; gb={k:grade(k,v) for k,v in b.items()}
print("poses",len(a),len(b))
print("main:",dict(collections.Counter(ga.values())))
print("head:",dict(collections.Counter(gb.values())))
t=collections.Counter(); ex={}
for k in ga:
    if k in gb and ga[k]!=gb[k]: t[(ga[k],gb[k])]+=1; ex.setdefault((ga[k],gb[k]),k)
for (x,y),n in sorted(t.items(),key=lambda kv:-kv[1]): print(f"  {n} {x} -> {y}   e.g. {ex[(x,y)][:90]}")
for k in gb:
    if gb[k]=="BAD": print("  HEAD BAD:",k[:100],b[k][:160])
