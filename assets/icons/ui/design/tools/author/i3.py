from lib import *
from i2 import pt, arrowhead, arrow, poly_reg, dashes
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def stadium(cx,cy,L,w,ang):
    r=w/2; a=math.radians(ang); dx,dy=math.cos(a),math.sin(a); nx,ny=-dy,dx
    p0=(cx-dx*L/2,cy-dy*L/2); p1=(cx+dx*L/2,cy+dy*L/2)
    return p0,p1,(nx,ny),r
def star4(cx,cy,r,k=0.18):
    return 'M%s %sQ%s %s %s %sQ%s %s %s %sQ%s %s %s %sQ%s %s %s %sZ'%(f(cx),f(cy-r),f(cx+r*k),f(cy-r*k),f(cx+r),f(cy),f(cx+r*k),f(cy+r*k),f(cx),f(cy+r),f(cx-r*k),f(cy+r*k),f(cx-r),f(cy),f(cx-r*k),f(cy-r*k),f(cx),f(cy-r))
@icon
def chain():
    ic=Icon('chain','Chain')
    L,w=26,16
    def link(cx,cy,col):
        p0,p1,n,r=stadium(cx,cy,L,w,45)
        d='M%s %sL%s %s%sL%s %s%sZ'%(f(p0[0]+n[0]*r),f(p0[1]+n[1]*r),f(p1[0]+n[0]*r),f(p1[1]+n[1]*r),arcto(r,r,p1[0]-n[0]*r,p1[1]-n[1]*r,0,0),f(p0[0]-n[0]*r),f(p0[1]-n[1]*r),arcto(r,r,p0[0]+n[0]*r,p0[1]+n[1]*r,0,0))
        return d,(p0,p1,n,r)
    d1,g1=link(24,24,P3); d2,g2=link(40,40,A3)
    ic.stroke(d1,P3,AW); ic.stroke(d2,A3,AW)
    p0,p1,n,r=g1
    half='M%s %s%sL%s %s'%(f(p1[0]+n[0]*r),f(p1[1]+n[1]*r),'',f(p1[0]+n[0]*r),f(p1[1]+n[1]*r))
    # end cap half of link1 drawn over link2 (the side on +n)
    mid=(p1[0]+(-n[1])*r,p1[1]+(n[0])*r)
    ic.stroke('M%s %s%s'%(f(p1[0]+n[0]*r),f(p1[1]+n[1]*r),arcto(r,r,mid[0],mid[1],0,0)),P3,AW)
    return ic
@icon
def snap():
    ic=Icon('snap','Snap')
    th=math.radians(-135)                       # the poles point to the bottom left
    def R(x,y):
        x,y=x-32,y-35; return (32+x*math.cos(th)-y*math.sin(th), 32+x*math.sin(th)+y*math.cos(th))
    def pt_(x,y): X,Y=R(x,y); return '%s %s'%(f(X),f(Y))
    u='M%sL%sA10 10 0 0 0 %sL%s'%(pt_(22,22),pt_(22,36),pt_(42,36),pt_(42,22))
    ic.stroke(u,INK,16.4); ic.stroke(u,A2,11.3)
    def rounded(c,r):
        n=len(c); out=''
        for i in range(n):
            p0,p1,p2=c[i-1],c[i],c[(i+1)%n]
            a=(p1[0]-p0[0],p1[1]-p0[1]); b=(p2[0]-p1[0],p2[1]-p1[1]); la=math.hypot(*a); lb=math.hypot(*b)
            s_=(p1[0]-a[0]/la*r,p1[1]-a[1]/la*r); e_=(p1[0]+b[0]/lb*r,p1[1]+b[1]/lb*r)
            out+=('M' if i==0 else 'L')+f(s_[0])+' '+f(s_[1])+'Q'+f(p1[0])+' '+f(p1[1])+' '+f(e_[0])+' '+f(e_[1])
        return out+'Z'
    for x in (22,42):                           # poles as wide as the bent line, short, slightly rounded
        hw=8.2-OW/2
        ic.stroke(rounded([R(x-hw,13.5),R(x+hw,13.5),R(x+hw,23),R(x-hw,23)],2.4),INK,OW,fill=P1)
    return ic.recenter()
def snap():
    ic=Icon('snap','Snap')
    th=math.radians(-135)                       # the poles point to the bottom left
    def R(x,y):
        x,y=x-32,y-35; return (32+x*math.cos(th)-y*math.sin(th), 32+x*math.sin(th)+y*math.cos(th))
    def pt_(x,y): X,Y=R(x,y); return '%s %s'%(f(X),f(Y))
    u='M%sL%sA10 10 0 0 0 %sL%s'%(pt_(22,16),pt_(22,36),pt_(42,36),pt_(42,16))
    ic.stroke(u,INK,16.4); ic.stroke(u,A2,11.3)
    for x in (22,42):
        t='M%sL%s'%(pt_(x,16),pt_(x,25))
        ic.stroke(t,INK,16.4); ic.stroke(t,P1,11.3)
    return ic.recenter()
def snap():
    ic=Icon('snap','Snap')
    th=math.radians(-135)                       # the poles point to the bottom left
    def R(x,y):
        x,y=x-32,y-30; return (32+x*math.cos(th)-y*math.sin(th), 32+x*math.sin(th)+y*math.cos(th))
    def pt_(x,y): X,Y=R(x,y); return '%s %s'%(f(X),f(Y))
    u='M%sL%sA16 16 0 0 0 %sL%s'%(pt_(16,10),pt_(16,34),pt_(48,34),pt_(48,10))
    ic.stroke(u,INK,14.6); ic.stroke(u,A2,9.5)
    for x in (16,48):
        t='M%sL%s'%(pt_(x,10),pt_(x,18))
        ic.stroke(t,INK,14.6); ic.stroke(t,P1,9.5)
    return ic
@icon
def select():
    ic=Icon('select','Select')
    ic.stroke('M18 7L18 50L28.5 40.5L35.5 56L43 52.5L36 37.5L50 37.5Z',INK,OW,fill=INK); return ic
@icon
def move():
    ic=Icon('move','Move')
    ic.stroke('M32 14L32 50M14 32L50 32',A3,AW)
    for dx,dy in ((0,-1),(0,1),(-1,0),(1,0)): ic.fill(arrowhead(32+dx*27,32+dy*27,dx,dy,10,12),A3)
    return ic
@icon
def project():
    ic=Icon('project','Project')
    pl='M5 46L28 34L59 46L36 58Z'; ic.fill(pl,P1); ic.stroke(pl,INK,OW)
    ic.stroke('M15 42C25 38 34 48 47 44',A3,AW)
    ic.stroke('M15 8C25 4 34 14 47 10',A3,AW)
    ic.stroke('M15 14L15 37M47 16L47 40M31 8L31 43',GUIDE,OW) if False else None
    ic.stroke('M15 16L15 36M47 18L47 38',GUIDE,OW)
    ic.fill(arrowhead(15,40,0,1,7,7),GUIDE); ic.fill(arrowhead(47,42,0,1,7,7),GUIDE)
    return ic
@icon
def include():
    ic=Icon('include','Include')
    I=Iso(1,32,32+4); s=24
    I.box(ic,0,0,0,s,s,s,(P1,P2,P3))
    a=I.p(s,s,s); b=I.p(s,0,s); c=I.p(0,s,s)
    ic.stroke(poly([c,a,b],False),GAP,AW+2.4); ic.dot(c[0],c[1],3.07+1.2,GAP); ic.dot(b[0],b[1],3.07+1.2,GAP)   # very thin punch-out
    ic.stroke(poly([c,a,b],False),A3,AW); pt(ic,*c); pt(ic,*b)
    return ic
@icon
def filter():
    ic=Icon('filter','Filter')
    d='M7 11L57 11L38 33L38 54L26 48L26 33Z'
    ic.fill(d,P1); ic.stroke(d,INK,OW); ic.fill('M10.3 14.5L53.7 14.5L50 19L14 19Z',A2) if False else None
    ic.fill('M7 11L57 11L48 22L16 22Z',A1); ic.stroke(d,INK,OW); ic.stroke('M16 22L48 22',INK,OW)
    return ic
@icon
def eye():
    ic=Icon('eye','Eye')
    d='M4 32C16 14 48 14 60 32C48 50 16 50 4 32Z'
    ic.fill(d,P1); ic.stroke(d,INK,OW); ic.fill(ell(32,32,10),A2); ic.stroke(ell(32,32,10),INK,OW); ic.dot(32,32,4,INK)
    return ic
@icon
def canvas():
    ic=Icon('canvas','Canvas')
    fr=rrect(6,10,58,54,5); ic.fill(fr,P1)
    ic.fill('M7 50L7 47L22 29L33 42L41 34L57 51L57 50A4 4 0 0 1 53 54L11 54A4 4 0 0 1 7 50Z',A2)
    ic.fill('M22 29L33 42L26 54L7 54L7 47Z',A3) if False else None
    ic.dot(44,23,5,A3)
    ic.stroke(fr,INK,OW); return ic
@icon
def dxf():
    ic=Icon('dxf','DXF'); doc='M14 6L38 6L52 20L52 58L14 58Z'
    ic.fill(doc,P1); ic.stroke(doc,INK,OW); ic.stroke('M38 6L38 20L52 20',INK,OW)
    ic.stroke(ell(29,43,8),A3,AW*0.8); ic.stroke('M22 30L44 30L44 36',A3,AW*0.8) if False else ic.stroke('M21 29L43 29',A3,AW*0.8)
    return ic
@icon
def auto_constrain():
    ic=Icon('auto_constrain','Auto constrain')
    ic.stroke(star4(26,28,19,0.1),A3,3.2,fill=A1); ic.stroke(star4(47,47,11,0.1),A3,2.8,fill=A1)
    return ic
@icon
def align():
    ic=Icon('align','Align')
    for (y0,x1,c) in ((10,48,P3),(25,34,P2),(40,54,P3)):
        d=poly([(14,y0),(x1,y0),(x1,y0+13),(14,y0+13)]); ic.fill(d,c); ic.stroke(d,INK,OW)
    ic.stroke('M8 5L8 59',A3,AW)
    return ic
@icon
def pattern():
    ic=Icon('pattern','Pattern'); I=Iso(1,32,32); s=11.5; pitch=19
    pos=[(0,0),(pitch,0),(0,pitch),(pitch,pitch)]
    cx=(pitch+s)/2
    I.ox=32; I.oy=32-0+ (0)  
    def P(x,y,z): return I.p(x-cx,y-cx,z-s/2)
    for i,(x,y) in enumerate(sorted(pos,key=lambda q:q[0]+q[1])):
        cols=(P1,P2,P3) if (x,y)==(0,0) else (A1,A2,A3)
        I.box(ic,x-cx,y-cx,-s/2,x-cx+s,y-cx+s,s/2,cols,OW)
    return ic

def pat_sq(ic,x,y,first,s=11,rot=0):
    """square centred on (x, y), turned by rot degrees"""
    c,sn=math.cos(math.radians(rot)),math.sin(math.radians(rot)); h=s/2
    d=poly([(x+c*px-sn*py,y+sn*px+c*py) for px,py in ((-h,-h),(h,-h),(h,h),(-h,h))])
    if first: ic.stroke(d,P3,3.0,fill=P1)
    else: ic.stroke(d,A3,3.0,fill=A1)
def pat_dots(ic,pts,r=1.7):
    for x,y in pts: ic.dot(x,y,r,GUIDE)
def line_dots(a,b,n):
    return [(a[0]+(b[0]-a[0])*i/n,a[1]+(b[1]-a[1])*i/n) for i in range(n+1)]
@icon
def pattern_rect():
    ic=Icon('pattern_rect','Rectangular pattern')
    xs=(10,32,54); ys=(18,46)
    for y in ys: pat_dots(ic,line_dots((xs[0],y),(xs[-1],y),11))
    for x in xs: pat_dots(ic,line_dots((x,ys[0]),(x,ys[-1]),5))
    for j,y in enumerate(ys):
        for i,x in enumerate(xs): pat_sq(ic,x,y,i==0 and j==0)
    return ic
@icon
def pattern_circ():
    ic=Icon('pattern_circ','Circular pattern')
    R=22
    pat_dots(ic,[(32+R*math.cos(math.radians(a)),32+R*math.sin(math.radians(a))) for a in range(0,360,12)])
    for i in range(6):
        a=math.radians(-90+60*i); pat_sq(ic,32+R*math.cos(a),32+R*math.sin(a),i==0,rot=60*i)
    return ic
