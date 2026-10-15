from lib import *
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
def pt(ic,x,y,c=A3,r=3.07): ic.dot(x,y,r,c)
def arrowhead(x,y,dx,dy,L=9,W=8):
    n=math.hypot(dx,dy); dx,dy=dx/n,dy/n
    bx,by=x-dx*L,y-dy*L; px,py=-dy,dx
    return poly([(x,y),(bx+px*W/2,by+py*W/2),(bx-px*W/2,by-py*W/2)])
def arrow(ic,x0,y0,x1,y1,c=A3,w=AW,L=9,W=9):
    n=math.hypot(x1-x0,y1-y0); dx,dy=(x1-x0)/n,(y1-y0)/n
    ic.stroke(poly([(x0,y0),(x1-dx*(L-2),y1-dy*(L-2))],False),c,w)
    ic.fill(arrowhead(x1,y1,dx,dy,L,W),c)
def poly_reg(n,cx,cy,r,rot=-90): return [(cx+r*math.cos(math.radians(rot+360*i/n)),cy+r*math.sin(math.radians(rot+360*i/n))) for i in range(n)]

@icon
def line():
    ic=Icon('line','Line'); ic.stroke(poly([(15,49),(49,15)],False),A3,AW); pt(ic,15,49); pt(ic,49,15); return ic
@icon
def point():
    ic=Icon('point','Point')
    for d in ((0,-1),(0,1),(-1,0),(1,0)):
        ic.stroke(poly([(32+d[0]*10,32+d[1]*10),(32+d[0]*22,32+d[1]*22)],False),A3,AW)
    pt(ic,32,32,A3,6); return ic
@icon
def circle():
    ic=Icon('circle','Circle'); ic.fill(ell(32,32,22),A1); ic.stroke(ell(32,32,22),A3,AW); pt(ic,32,32); return ic
@icon
def ellipse():
    ic=Icon('ellipse','Ellipse'); ic.fill(ell(32,32,26,17),A1); ic.stroke(ell(32,32,26,17),A3,AW); pt(ic,32,32); return ic
@icon
def rect():
    ic=Icon('rect','Rectangle'); d=poly([(10,16),(54,16),(54,48),(10,48)])
    ic.fill(d,A1); ic.stroke(d,A3,AW); pt(ic,10,16); pt(ic,54,48); return ic
@icon
def polygon():
    ic=Icon('polygon','Polygon'); ps=poly_reg(6,32,32,24,-90)
    d=poly(ps); ic.fill(d,A1); ic.stroke(d,A3,AW); pt(ic,32,32); return ic
@icon
def slot():
    ic=Icon('slot','Slot'); d=rrect(7,21,57,43,11)
    ic.fill(d,A1); ic.stroke(d,A3,AW); pt(ic,18,32); pt(ic,46,32)
    ic.stroke(poly([(18,32),(46,32)],False),A3,1.8) if False else None
    return ic
@icon
def arc3():
    ic=Icon('arc3','Three-point arc'); ic.stroke('M8 46A24 24 0 0 1 56 46',A3,AW)
    pt(ic,8,46); pt(ic,56,46); pt(ic,32,22); return ic
@icon
def spline():
    ic=Icon('spline','Spline')
    ic.stroke('M9 44C17 14 27 14 32 30C37 46 47 46 55 20',A3,AW)
    for x,y in ((9,44),(32,30),(55,20)): pt(ic,x,y)
    return ic
@icon
def text():
    ic=Icon('text','Text')
    ic.stroke('M13 17L51 17M32 17L32 49M25 49L39 49',A3,AW)
    ic.stroke('M13 17L13 23M51 17L51 23',A3,AW)
    return ic
@icon
def construction():
    ic=Icon('construction','Construction'); 
    ic.stroke('M13 51L19 45M25 39L31 33M37 27L43 21M49 15L51 13',GUIDE,AW)
    ic.stroke('M10 54L12 52',GUIDE,AW)
    pt(ic,12,52,GUIDE); pt(ic,52,12,GUIDE)
    return ic
@icon
def dimension():
    ic=Icon('dimension','Dimension')
    ic.stroke('M10 14L10 50M54 14L54 50',INK,OW)
    ic.stroke('M18 32L46 32',A3,AW)
    ic.fill(arrowhead(10,32,-1,0,10,10),A3); ic.fill(arrowhead(54,32,1,0,10,10),A3)
    return ic
@icon
def dots():
    ic=Icon('dots','More')
    for x in (13,32,51): ic.dot(x,32,5,INK)
    return ic
@icon
def axis():
    ic=Icon('axis','Axis'); ic.stroke('M10 48L32 48L48 22',INK,OW) if False else None
    ic.stroke('M12 52L12 12',A3,AW); ic.stroke('M12 52L52 52',A3,AW)
    ic.fill(arrowhead(12,8,0,-1,9,9),A3); ic.fill(arrowhead(56,52,1,0,9,9),A3)
    pt(ic,12,52,INK,4); return ic
@icon
def origin():
    ic=Icon('origin','Origin')
    ic.stroke('M32 32L32 8',A3,AW); ic.stroke('M32 32L56 32',A3,AW); ic.stroke('M32 32L16 48',A3,AW)
    ic.fill(arrowhead(32,5,0,-1,8,8),A3); ic.fill(arrowhead(59,32,1,0,8,8),A3); ic.fill(arrowhead(12,52,-1,1,8,8),A3)
    pt(ic,32,32,INK,5); return ic

def dashes(ic,x0,y0,x1,y1,c,w=AW,n=3,fr=0.55):
    """n dashes along a segment"""
    for i in range(n):
        a=i/n; b=a+fr/n
        ic.stroke(poly([(x0+(x1-x0)*a,y0+(y1-y0)*a),(x0+(x1-x0)*b,y0+(y1-y0)*b)],False),c,w)
@icon
def trim():
    ic=Icon('trim','Trim')
    ic.stroke('M22 10L22 54M42 10L42 54',P3,AW)
    ic.stroke('M7 32L22 32M42 32L57 32',A3,AW)
    ic.stroke('M28 32L36 32',P3,AW)
    return ic
@icon
def extend():
    ic=Icon('extend','Extend')
    ic.stroke('M52 10L52 54',P3,AW)
    ic.stroke('M8 32L32 32',A3,AW); ic.stroke('M38 32L41 32',A3,AW); ic.stroke('M45 32L46 32',A3,AW)
    ic.fill(arrowhead(50,32,1,0,9,10),A3) if False else None
    return ic
@icon
def break_():
    ic=Icon('break','Break')
    ic.stroke('M7 32L23 32M41 32L57 32',A3,AW); pt(ic,23,32); pt(ic,41,32)
    ic.stroke('M32 22L32 42',INK,OW)
    return ic
@icon
def offset():
    ic=Icon('offset','Offset')
    ic.stroke('M8 56L8 28A20 20 0 0 1 28 8L56 8',P3,AW)
    ic.stroke('M24 56L24 32A8 8 0 0 1 32 24L56 24',A3,AW)
    ic.stroke('M44 8L44 24',INK,OW) if False else None
    return ic
def tri_pair(ic,fill_l,fill_r,sl,sr,w):
    L=[(8,50),(28,50),(28,16)]; R=[(56,50),(36,50),(36,16)]
    ic.fill(poly(L),fill_l); ic.fill(poly(R),fill_r)
    ic.stroke(poly(L),sl,w); ic.stroke(poly(R),sr,w)
@icon
def mirror():
    ic=Icon('mirror','Mirror')
    ic.stroke('M32 6L32 12M32 20L32 26M32 34L32 40M32 48L32 54',GUIDE,2.0); tri_pair(ic,P2,A2,INK,INK,OW)
    return ic
@icon
def sketch_mirror():
    ic=Icon('sketch_mirror','Sketch mirror')
    ic.stroke('M32 6L32 12M32 20L32 26M32 34L32 40M32 48L32 54',GUIDE,AW)
    ic.stroke(poly([(8,50),(28,50),(28,16)]),P3,AW); ic.stroke(poly([(56,50),(36,50),(36,16)]),A3,AW)
    return ic
@icon
def sketch_fillet():
    ic=Icon('sketch_fillet','Sketch fillet')
    ic.stroke('M12 34L12 12L34 12',GUIDE,OW)
    ic.stroke('M12 54L12 34A22 22 0 0 1 34 12L54 12',A3,AW)
    pt(ic,12,54); pt(ic,54,12); return ic
@icon
def sketch_chamfer():
    ic=Icon('sketch_chamfer','Sketch chamfer')
    ic.stroke('M12 32L12 12L32 12',GUIDE,OW)
    ic.stroke('M12 54L12 32L32 12L54 12',A3,AW)
    pt(ic,12,54); pt(ic,54,12); return ic
@icon
def blend():
    ic=Icon('blend','Blend curve')
    ic.stroke('M6 48L18 48M46 16L58 16',P3,AW)
    ic.stroke('M18 48C36 48 28 16 46 16',A3,AW); pt(ic,18,48); pt(ic,46,16); return ic
@icon
def conic():
    ic=Icon('conic','Conic')
    ic.stroke(ell(32,34,24,17),GUIDE,OW)
    ic.stroke(arcpts(32,34,24,17,195,345),A3,AW)
    pt(ic,32-24*math.cos(math.radians(15)),34-17*math.sin(math.radians(15))); pt(ic,32+24*math.cos(math.radians(15)),34-17*math.sin(math.radians(15)))
    return ic
@icon
def centerline():
    ic=Icon('centerline','Centerline')
    ic.stroke('M5 32L24 32M40 32L59 32',A3,AW); ic.dot(32,32,3.6,A3)
    ic.stroke('M13 18L51 18M13 46L51 46',GUIDE,OW) if False else None
    return ic
@icon
def line_mid():
    ic=Icon('line_mid','Midpoint line')
    ic.stroke('M12 50L52 14',A3,AW); pt(ic,12,50); pt(ic,52,14)
    ic.dot(32,32,6.5,A3); ic.ring(32,32,6.5,INK,OW) if False else None
    ic.ring(32,32,11,INK,OW)
    return ic
def arc_pts(cx,cy,r,deg): a=math.radians(deg); return cx+r*math.cos(a),cy+r*math.sin(a)
@icon
def arc_slot():
    ic=Icon('arc_slot','Arc slot'); cx,cy,R,hw=32,46,21,7; a0,a1=205,335
    o0=arc_pts(cx,cy,R+hw,a0); o1=arc_pts(cx,cy,R+hw,a1); i1=arc_pts(cx,cy,R-hw,a1); i0=arc_pts(cx,cy,R-hw,a0)
    d='M%s %s%s%s%s%s'%(f(o0[0]),f(o0[1]),arcto(R+hw,R+hw,o1[0],o1[1],0,1),arcto(hw,hw,i1[0],i1[1],0,1),arcto(R-hw,R-hw,i0[0],i0[1],0,0),arcto(hw,hw,o0[0],o0[1],0,1))+'Z'
    ic.fill(d,A1); ic.stroke(d,A3,AW)
    c0=arc_pts(cx,cy,R,a0); c1=arc_pts(cx,cy,R,a1); pt(ic,*c0); pt(ic,*c1); return ic
@icon
def spun():
    ic=Icon('spun','Spun profile')
    d='M12 10L28 10C40 22 24 38 36 54L12 54Z'
    ic.fill(d,A1); ic.stroke(d,A3,AW)
    ic.stroke('M6 5L6 59',INK,OW)
    ic.stroke('M46 16A22 22 0 0 1 46 48',GUIDE,OW); ic.fill(arrowhead(46,48,-1,0.2,8,8),GUIDE)
    return ic
@icon
def constrainer():
    ic=Icon('constrainer','Constrainer')
    d=poly([(14,16),(50,16),(50,48),(14,48)]); ic.stroke(d,A3,AW)
    ic.stroke('M26 10L38 10M26 55L38 55',INK,OW) if False else None
    ic.stroke('M24 29L40 29M24 35L40 35',INK,OW)
    pt(ic,14,16); pt(ic,50,48); return ic
@icon
def finish():
    ic=Icon('finish','Finish')
    ic.stroke('M12 34L26 48L52 16',A3,NW); return ic
@icon
def svg():
    ic=Icon('svg','Insert SVG'); doc='M14 6L38 6L52 20L52 58L14 58Z'
    ic.fill(doc,P1); ic.stroke(doc,INK,OW); ic.stroke('M38 6L38 20L52 20',INK,OW)
    ic.stroke('M21 46C23 30 33 30 34 38C35 44 42 40 45 30',A3,AW) if False else None
    ic.stroke('M20 46C22 32 30 30 33 38C36 46 42 42 46 30',A3,AW*0.8); pt(ic,20,46,A3,2.6); pt(ic,46,30,A3,2.6)
    return ic
@icon
def fit_section():
    ic=Icon('fit_section','Fit curves to section')
    for x in (14,32,50): ic.stroke(poly([(x,10),(x,54)],False),GUIDE,OW)
    ic.stroke('M6 42C20 42 24 20 32 28C40 36 44 22 58 22',A3,AW)
    return ic
