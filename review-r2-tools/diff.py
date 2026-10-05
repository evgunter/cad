import sys,collections
c=collections.Counter(); ex={}
def cls(s):
    s=s.split(': ',1)[1]
    if s.startswith('OK'): return ' '.join(s.split()[:2])
    if s.startswith('ERR'):
        k=s.split()[1]
        if 'what:' in s: k+=' '+s.split('what:')[1][:35]
        if 'MarginDiag' in s: k='MarginDiag'
        return k
    return s[:30]
A=[l.rstrip('\n') for l in open(sys.argv[1])]; B=[l.rstrip('\n') for l in open(sys.argv[2])]
assert len(A)==len(B),(len(A),len(B))
heads=collections.Counter()
for a,b in zip(A,B):
    ta,tb=a.split(': ')[0],b.split(': ')[0]
    assert ta==tb,(ta,tb)
    heads[cls(b)]+=1
    if a==b: c['identical']+=1; continue
    k=(cls(a),cls(b)); c[k]+=1; ex.setdefault(k,ta)
for k,v in c.most_common(): print(v,k,ex.get(k,''))
print('head outcomes:',dict(heads))
