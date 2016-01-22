from lspace_ffi import ffi, ffi_module
from pyrs import rust_type, RustObject, RustMethod



@rust_type(ffi, ffi_module)
class Colour (RustObject):
	__destructor_name__ = 'destroy_colour'

	__new_colour = RustMethod('Colour*', 'new_colour',
				       ['double r', 'double g', 'double b', 'double a'])

	def __init__(self, r, g, b, a):
		super(Colour, self).__init__(self.__new_colour(r, g, b, a))




@rust_type(ffi, ffi_module)
class TextStyleParams (RustObject):
	__destructor_name__ = 'destroy_text_style_params'

	__new_text_style_params = RustMethod('TextStyleParams*', 'new_text_style_params',
				       ['const char *font_family', 'unsigned short bold',
					'unsigned short italic', 'double size', 'Colour *colour'])

	__new_text_style_params_default = RustMethod('TextStyleParams*', 'new_text_style_params_default', [])

	def __init__(self, font_family, bold, italic, size, colour):
		bold = 1 if bold else 0
		italic = 1 if italic else 0
		super(TextStyleParams, self).__init__(self.__new_text_style_params(font_family, bold, italic, size,
										   colour._rust_obj))

	@staticmethod
	def new_default():
		obj = TextStyleParams.__new__(TextStyleParams)
		RustObject.__init__(obj, obj.__new_text_style_params_default())
		return obj




@rust_type(ffi, ffi_module, struct_name='Pres')
class Text (RustObject):
	__destructor_name__ = 'destroy_pres'

	__new_text = RustMethod('Pres*', 'new_text',
				       ['const char *text', 'TextStyleParams *style'])

	def __init__(self, text, style):
		super(Text, self).__init__(self.__new_text(text, style._rust_obj))




