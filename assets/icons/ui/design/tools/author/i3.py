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
    u='M16 10L16 34A16 16 0 0 0 48 34L48 10'
    ic.stroke(u,INK,14.6); ic.stroke(u,A2,9.5)
    for x in (16,48):
        ic.stroke(poly([(x,10),(x,18)],False),INK,14.6); ic.stroke(poly([(x,10),(x,18)],False),P1,9.5)
    ic.stroke('M16 10L16 18M48 10L48 18',INK,14.6) if False else None
    pt(ic,32,57,A3,3.5)
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
    ic.fill(star4(26,28,20,0.1),A3); ic.stroke(star4(47,47,11,0.1),INK,OW,fill=P1)
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
@icon
def c_coincident():
    ic=Icon('c_coincident','Coincident')
    ic.stroke('M7 46L32 32M32 32L57 18',P3,AW) if False else None
    ic.stroke('M8 48L30 33',A3,AW); ic.stroke('M34 31L56 16',A3,AW)
    ic.dot(32,32,5.5,A3); ic.ring(32,32,11,INK,OW); return ic
@icon
def c_collinear():
    ic=Icon('c_collinear','Collinear')
    ic.stroke('M6 38L24 26M40 38L58 26',GUIDE,OW) if False else None
    ic.stroke('M6 44L26 32',A3,AW); ic.stroke('M38 32L58 20',A3,AW) if False else None
    ic.stroke('M6 45L25 33.8',A3,AW); ic.stroke('M39 25.6L58 14.4',A3,AW) if False else None
    ic.stroke('M6 46L26 34M38 27L58 15',A3,AW) if False else None
    ic.stroke('M6 46L26 34.2',A3,AW); ic.stroke('M38 26.7L58 14.9',A3,AW)
    ic.stroke('M26 34.2L38 26.7',GUIDE,OW)
    pt(ic,6,46); pt(ic,58,14.9); return ic
@icon
def c_concentric():
    ic=Icon('c_concentric','Concentric')
    ic.stroke(ell(32,32,23),A3,AW); ic.stroke(ell(32,32,11),P3,AW); ic.dot(32,32,3.07,A3); return ic
@icon
def c_equal():
    ic=Icon('c_equal','Equal')
    ic.stroke('M12 23L52 23M12 41L52 41',A3,NW); return ic
@icon
def c_fix():
    ic=Icon('c_fix','Fixed')
    ic.stroke('M32 8L32 38',A3,AW); ic.dot(32,10,4.5,A3)
    ic.stroke('M12 38L52 38',INK,AW)
    ic.stroke('M16 54L24 46M28 54L36 46M40 54L48 46',INK,OW); return ic
@icon
def c_hv():
    ic=Icon('c_hv','Horizontal / vertical')
    ic.stroke('M6 20L34 20',A3,AW); ic.stroke('M50 30L50 58',A3,AW)
    pt(ic,6,20); pt(ic,34,20); pt(ic,50,30); pt(ic,50,58)
    ic.stroke('M38 24L50 24',GUIDE,OW) if False else None
    return ic
@icon
def c_midpoint():
    ic=Icon('c_midpoint','Midpoint')
    ic.stroke('M8 48L56 16',P3,AW); ic.dot(8,48,3.07,P3); ic.dot(56,16,3.07,P3)
    ic.dot(32,32,6,A3); ic.ring(32,32,11,INK,OW)
    return ic
@icon
def c_parallel():
    ic=Icon('c_parallel','Parallel')
    ic.stroke('M8 40L40 8M24 56L56 24',A3,AW); return ic
@icon
def c_perpendicular():
    ic=Icon('c_perpendicular','Perpendicular')
    ic.stroke('M12 8L12 52L56 52',A3,AW); ic.stroke('M12 38L26 38L26 52',INK,OW); return ic
@icon
def c_polygon():
    ic=Icon('c_polygon','Regular polygon')
    ps=poly_reg(5,32,34,24,-90); ic.stroke(poly(ps),P3,AW)
    for i in range(5):
        a,b=ps[i],ps[(i+1)%5]; mx,my=(a[0]+b[0])/2,(a[1]+b[1])/2; dx,dy=b[0]-a[0],b[1]-a[1]; n=math.hypot(dx,dy); nx,ny=-dy/n,dx/n
        ic.stroke(poly([(mx+nx*5,my+ny*5),(mx-nx*5,my-ny*5)],False),A3,AW)
    return ic
@icon
def c_smooth():
    ic=Icon('c_smooth','Curvature')
    ic.stroke('M6 50C28 50 34 14 58 14',A3,AW)
    ic.stroke(ell(32,32,5),INK,OW)
    return ic
@icon
def c_symmetry():
    ic=Icon('c_symmetry','Symmetry')
    ic.stroke('M32 6L32 14M32 22L32 30M32 38L32 46M32 54L32 58',GUIDE,AW) if False else None
    ic.stroke('M32 6L32 16M32 24L32 40M32 48L32 58',GUIDE,OW) if False else None
    ic.stroke('M32 6L32 14M32 22L32 28M32 36L32 42M32 50L32 58',GUIDE,AW)
    ic.dot(11,32,6,A3); ic.dot(53,32,6,A3)
    ic.stroke('M19 32L25 32M39 32L45 32',A3,OW) if False else None
    ic.fill(arrowhead(26,32,1,0,7,8),INK) if False else None
    return ic
@icon
def c_tangent():
    ic=Icon('c_tangent','Tangent')
    ic.stroke(ell(32,38,18),A3,AW); ic.stroke('M6 20L58 20',A3,AW); pt(ic,32,20,INK,3.5) if False else None
    return ic
