# from gi.repository import Gtk, Gdk, cairo
import sys

from cffi import FFI

# Dictionary storing the library filename, keyed by platform
LIB_PREFIX = {'win32': ''}
LIB_EXT = {'win32': '.dll', 'darwin': '.dylib'}


# FFI instance
ffi = FFI()
# Open the library, filename depending on platform
ffi_module = ffi.dlopen(r"target/debug/{0}lspace{1}".format(LIB_PREFIX.get(sys.platform, 'lib'),
							    LIB_EXT.get(sys.platform, '.so')))
