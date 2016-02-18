# Function to convert python Cairo Context to a `cairo_t*` pointer to give to Rust
def pycairo_to_cairo_t_ptr(ffi, ctx):
    return ffi.cast('void **', id(ctx) + object.__basicsize__)[0]


_declared_types = set()
_declared_destructors = set()


def rust_type(ffi, ffi_module, struct_name=None):
    def decorate_class(cls):
        cdef_type_name = struct_name
        class_name = cls.__name__

        if cdef_type_name is None:
            cdef_type_name = class_name

        if cdef_type_name not in _declared_types:
            # Declare structure tupe
            ffi.cdef("""
			    typedef struct {{
				int dummy;
			    }} {0};
			""".format(cdef_type_name))
            _declared_types.add(cdef_type_name)

        # Declare method functions
        for key, value in cls.__dict__.items():
            if isinstance(value, RustMethod):
                value.ffi_init(ffi, ffi_module)

        # Declare destructor
        try:
            destructor_name = cls.__destructor_name__
        except AttributeError:
            raise TypeError('__destructor_name__ not defined for class {0}'.format(cls.__name__))
        if destructor_name not in _declared_destructors:
            ffi.cdef('void {0}({1} *w);'.format(destructor_name, cdef_type_name))
            _declared_destructors.add(destructor_name)
        destructor = getattr(ffi_module, destructor_name)
        cls.__ffi_destructor__ = destructor
        return cls

    return decorate_class


class RustObject(object):
    def __init__(self, rust_obj):
        self._rust_obj = rust_obj


def unwrap_for_rust(ffi, x):
    if isinstance(x, RustObject):
        return x._rust_obj
    elif isinstance(x, tuple):
        return tuple([unwrap_for_rust(ffi, el) for el in x])
    elif isinstance(x, list):
        return [unwrap_for_rust(ffi, el) for el in x]
    elif x is None:
        return ffi.NULL
    else:
        return x


class RustMethod(object):
    def __init__(self, return_type_cdef, name, param_cdefs):
        if not isinstance(param_cdefs, (list, tuple)):
            raise TypeError('param_cdefs should be a list or tuple of strings')
        self._return_type_cdef = return_type_cdef
        self._name = name
        self._param_cdefs = param_cdefs
        self._ffi_method = None
        self._ffi = None

    def ffi_init(self, ffi, ffi_module):
        self._ffi = ffi
        cdecl = '{0} {1}({2});'.format(self._return_type_cdef, self._name, ', '.join(self._param_cdefs))
        ffi.cdef(cdecl)
        self._ffi_method = getattr(ffi_module, self._name)

    def invoke(self, *args, **kwargs):
        args = tuple([unwrap_for_rust(self._ffi, x) for x in args])
        kwargs = {name: unwrap_for_rust(self._ffi, value) for name, value in kwargs.items()}
        return self._ffi_method(*args, **kwargs)
