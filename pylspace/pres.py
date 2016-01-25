from lspace_ffi import ffi, ffi_module
from pyrs import rust_type, RustObject, RustMethod, unwrap_for_rust


@rust_type(ffi, ffi_module)
class Colour(RustObject):
    __destructor_name__ = 'destroy_colour'

    __new_colour = RustMethod('Colour*', 'new_colour',
                              ['double r', 'double g', 'double b', 'double a'])

    def __init__(self, r, g, b, a):
        super(Colour, self).__init__(self.__new_colour.invoke(r, g, b, a))

Colour.BLACK = Colour(0.0, 0.0, 0.0, 1.0)


@rust_type(ffi, ffi_module)
class GraphicsBorder(RustObject):
    __destructor_name__ = 'destroy_gfx_border'

    __new_solid_border = RustMethod('GraphicsBorder*', 'new_solid_border',
                                    ['double thickness', 'double inset', 'double rounding',
                                     'Colour *border_colour', 'Colour *background_colour'])
    __new_filled_border = RustMethod('GraphicsBorder*', 'new_filled_border',
                                     ['double left_margin', 'double right_margin',
                                      'double top_margin', 'double bottom_margin',
                                      'Colour *background_colour'])

    @staticmethod
    def solid(thickness, inset, rounding, border_colour, background_colour=None):
        return GraphicsBorder(GraphicsBorder.__new_solid_border.invoke(thickness, inset, rounding, border_colour,
                                                                       background_colour))

    @staticmethod
    def filled(left_margin, right_margin, top_margin, bottom_margin, background_colour=None):
        return GraphicsBorder(GraphicsBorder.__new_filled_border.invoke(left_margin, right_margin,
                                                                        top_margin, bottom_margin,
                                                                        background_colour))


    def surround(self, child):
        return Border(child, self)


@rust_type(ffi, ffi_module)
class TextStyleParams(RustObject):
    __destructor_name__ = 'destroy_text_style_params'

    __new_text_style_params = RustMethod('TextStyleParams*', 'new_text_style_params',
                                         ['const char *font_family', 'unsigned short bold',
                                          'unsigned short italic', 'double size', 'Colour *colour'])

    def __init__(self, font_family, bold, italic, size, colour):
        bold = 1 if bold else 0
        italic = 1 if italic else 0
        super(TextStyleParams, self).__init__(self.__new_text_style_params.invoke(font_family, bold, italic, size,
                                                                                  colour))

    @staticmethod
    def new_default():
        return TextStyleParams('Sans serif', False, False, 12.0, Colour.BLACK)


@rust_type(ffi, ffi_module, struct_name='Pres')
class Text(RustObject):
    __destructor_name__ = 'destroy_pres'

    __new_text = RustMethod('Pres*', 'new_text',
                            ['const char *text', 'TextStyleParams *style'])

    def __init__(self, text, style):
        super(Text, self).__init__(self.__new_text.invoke(text, style))


@rust_type(ffi, ffi_module, struct_name='Pres')
class Border(RustObject):
    __destructor_name__ = 'destroy_pres'

    __new_border = RustMethod('Pres*', 'new_border_pres',
                              ['Pres *child', 'GraphicsBorder *border'])

    def __init__(self, child, border):
        super(Border, self).__init__(self.__new_border.invoke(child, border))


@rust_type(ffi, ffi_module, struct_name='Pres')
class Column(RustObject):
    __destructor_name__ = 'destroy_pres'

    __new_column = RustMethod('Pres*', 'new_column',
                              ['Pres **children', 'size_t n_children', 'double y_spacing'])

    def __init__(self, children, y_spacing=0.0):
        super(Column, self).__init__(self.__new_column.invoke(ffi.new('Pres*[]', unwrap_for_rust(ffi, children)),
                                                              len(children), y_spacing))


@rust_type(ffi, ffi_module, struct_name='Pres')
class Row(RustObject):
    __destructor_name__ = 'destroy_pres'

    __new_row = RustMethod('Pres*', 'new_row',
                           ['Pres **children', 'size_t n_children', 'double x_spacing'])

    def __init__(self, children, x_spacing=0.0):
        super(Row, self).__init__(self.__new_row.invoke(children, len(children), x_spacing))


@rust_type(ffi, ffi_module)
class FlowIndent(RustObject):
    __destructor_name__ = 'destroy_flow_indent'

    __new_flow_indent_no_indent = RustMethod('FlowIndent*', 'new_flow_indent_no_indent', [])
    __new_flow_indent_first = RustMethod('FlowIndent*', 'new_flow_indent_first',
                                         ['double indent'])
    __new_flow_indent_except_first = RustMethod('FlowIndent*', 'new_flow_indent_except_first',
                                                ['double indent'])

    __no_indent = None

    @staticmethod
    def no_indent():
        if FlowIndent.__no_indent is None:
            FlowIndent.__no_indent = FlowIndent(FlowIndent.__new_flow_indent_no_indent.invoke())
        return FlowIndent.__no_indent

    @staticmethod
    def first(indent):
        return FlowIndent(FlowIndent.__new_flow_indent_first.invoke(indent))

    @staticmethod
    def except_first(indent):
        return FlowIndent(FlowIndent.__new_flow_indent_except_first.invoke(indent))


@rust_type(ffi, ffi_module, struct_name='Pres')
class Flow(RustObject):
    __destructor_name__ = 'destroy_pres'

    __new_flow = RustMethod('Pres*', 'new_flow',
                           ['Pres **children', 'size_t n_children', 'double x_spacing',
                            'double y_spacing', 'FlowIndent *indent'])

    def __init__(self, children, x_spacing=0.0, y_spacing=0.0, indent=None):
        if indent is None:
            indent = FlowIndent.no_indent()
        super(Flow, self).__init__(self.__new_flow.invoke(children, len(children), x_spacing,
                                                          y_spacing, indent))
