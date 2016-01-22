from lspace_ffi import ffi, ffi_module
from pyrs import rust_type, RustObject, RustMethod, pycairo_to_cairo_t_ptr
from pylspace import pres



@rust_type(ffi, ffi_module)
class LSpaceArea (RustObject):
	__destructor_name__ = 'destroy_lspace_area'

	__new_lspace_area = RustMethod('LSpaceArea*', 'new_lspace_area',
				       ['Pres *content'])
	__lspace_area_on_draw = RustMethod('void', 'lspace_area_on_draw',
					      ['LSpaceArea *area', 'void *ctx'])
	__lspace_area_on_size_allocate = RustMethod('void', 'lspace_area_on_size_allocate',
					      ['LSpaceArea *area', 'int w', 'int h'])

	def __init__(self, content):
		super(LSpaceArea, self).__init__(self.__new_lspace_area(content._rust_obj))

	def on_size_allocate(self, width, height):
		self.__lspace_area_on_size_allocate(self._rust_obj, width, height)

	def on_draw(self, cairo_ctx):
		self.__lspace_area_on_draw(self._rust_obj, pycairo_to_cairo_t_ptr(ffi, cairo_ctx))




