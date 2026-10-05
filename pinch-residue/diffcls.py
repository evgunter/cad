import sys,re,collections
def key(l):
    i=l.find(': ')
    return l[:i], l[i+2:]
def cls(o):
    if o.startswith('OK SOUND'): return 'SOUND'
    if o.startswith('EMPTY ok'): return 'SOUND'
    if o.startswith('OK BAD') or o.startswith('EMPTY WRONG'): return 'BAD'
    if o.startswith('PANIC'): return 'PANIC'
    return 'REF'
def word(o):
    m=re.match(r'ERR (\w+)(?:\s*\{\s*(?:what|errors): \[?"?(\w+)?)?',o); return (m.group(1)+(':'+m.group(2) if m.group(2) else '')) if m else o[:20]
a=[l.rstrip('\n') for l in open(sys.argv[1]) if ': ' in l and re.search(r': (OK|ERR|EMPTY|PANIC)',l)]
b=[l.rstrip('\n') for l in open(sys.argv[2]) if ': ' in l and re.search(r': (OK|ERR|EMPTY|PANIC)',l)]
A=dict(key(l) for l in a); B=dict(key(l) for l in b)
c=collections.Counter(); ex=collections.defaultdict(list)
for k in A:
    if k not in B: c['missing-in-head']+=1; continue
    x,y=A[k],B[k]
    if x==y: c['same']+=1; continue
    t=f"{cls(x)}->{cls(y)}"
    if cls(x)=='REF' and cls(y)=='REF': t=f"REF {word(x)} -> {word(y)}"
    if cls(x)=='REF' and cls(y)!='REF': t+=f" (from {word(x)})"
    c[t]+=1; ex[t].append(k)
c['only-in-head']=len([k for k in B if k not in A])
print(len(A),len(B))
for t,n in c.most_common(): print(f"  {n:6} {t}", ('e.g. '+ex[t][0]) if ex[t] else '')
