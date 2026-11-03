import collections,re,sys
def load(f):
    d={}
    for line in open(f):
        if ' tr-' not in line: continue
        tag,key,cat=[x.strip() for x in line.replace("PROBE ","").split('|')]
        m=re.match(r'tr-(\w+)-k(.+)-s(.+)$',tag)
        d[(m[1],m[2],m[3],key)]=cat
    return d
d=load(sys.argv[1])
print(len(d))
c=collections.Counter()
for (bn,k,s,key),v in d.items():
    base=d.get((bn,k,'0e0',key))
    if base is None or v==base: continue
    kind=key.split()[0]+' '+key.split()[1].rstrip('0123456789')
    c[(bn,k,s,kind,base,v)]+=1
for x,n in sorted(c.items()): print(n,x)
