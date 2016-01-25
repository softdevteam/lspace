# from gi.repository import Gtk, Gdk, cairo
from gi.repository import Gtk, Gdk

from pylspace.lspace_area import LSpaceArea
from pylspace import pres


border = pres.GraphicsBorder.solid(1.5, 3.0, 4.0, pres.Colour(0.6, 0.6, 0.6, 1.0), None)
base_style = pres.TextStyleParams.new_default()
title_style = pres.TextStyleParams('Sans serif', False, False, 64.0, pres.Colour(0.0, 0.0, 0.0, 1.0))
title = pres.Text("Hello from LSpace", title_style)
body = pres.Text("This is a test of LSpace being called from Python", base_style)
p = pres.Column([border.surround(title), body], 5.0)
area = LSpaceArea(p)


def size_allocate(wid,rect):
	area.on_size_allocate(rect.width, rect.height)

def draw(wid,ctx):
	area.on_draw(ctx)

	return False


def close_window(wid):
	Gtk.main_quit()

if __name__ == '__main__':
	win = Gtk.Window(Gtk.WindowType.TOPLEVEL)
	win.set_title('Drawing Area')
	win.connect('destroy',close_window)
	win.set_border_width(8)

	frame = Gtk.Frame()
	frame.set_shadow_type(Gtk.ShadowType.IN)
	win.add(frame)

	da = Gtk.DrawingArea()
	da.set_size_request(800,600)
	da.connect('size-allocate', size_allocate)
	da.connect('draw', draw)
	frame.add(da)
	da.queue_draw()


	win.show_all()
	Gtk.main()