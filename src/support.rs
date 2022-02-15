use rutie::{methods, Class, Float, Module, Object};

pub fn define_class(super_class: &Class) {
    //parent.define_nested_module("Support").define(|module| {
    //    module.def("ru_sin", support__ru_sin);
    //    module.def("ru_saw", support__ru_saw);
    //    module.def("ru_sq", support__ru_sq);
    //});
}

module!(Support);
methods!(
    Support,
    itself,
    fn support__ru_sin(hz: Float, t: Float) -> Float {
        Float::new((hz.unwrap().to_f64() * std::f64::consts::TAU * t.unwrap().to_f64()).sin())
    },
    fn support__ru_saw(hz: Float, t: Float) -> Float {
        Float::new(((hz.unwrap().to_f64() * t.unwrap().to_f64()) % 1.0) * 2.0 - 1.0)
    },
    fn support__ru_sq(hz: Float, t: Float) -> Float {
        Float::new({
            match hz.unwrap().to_f64() * t.unwrap().to_f64() % 1.0 {
                f if 0.5 < f => 1.0,
                _ => -1.0,
            }
        })
    },
);
