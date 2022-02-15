#[macro_use]
extern crate rutie;
#[macro_use]
extern crate lazy_static;

//mod inner;
mod instrument;
mod meta;
mod piece;
mod section;
mod support;
mod time;
mod track;
mod util;

use crate::{instrument::transfer, util::ConvertOrPanic};
use rutie::{
    class, methods, module, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class,
    Hash, Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, GC, VM,
};

#[macro_export]
macro_rules! ruby_class {
    ($class: ident) => {
        impl From<rutie::types::Value> for $class {
            fn from(value: rutie::types::Value) -> Self {
                $class { value: value }
            }
        }

        impl rutie::Object for $class {
            #[inline]
            fn value(&self) -> rutie::types::Value {
                self.value
            }
        }

        impl rutie::VerifiedObject for $class {
            fn is_correct_type<T: Object>(object: &T) -> bool {
                Class::from_existing(stringify!($class)).case_equals(object)
            }

            fn error_message() -> &'static str {
                &concat!("Error converting to ", stringify!($class))
            }
        }
        impl TryFrom<rutie::AnyObject> for $class {
            type Error = std::io::Error;

            fn try_from(obj: AnyObject) -> Result<$class, Self::Error> {
                if Class::from_existing(stringify!($class)).case_equals(&obj) {
                    Ok($class::from(obj.value()))
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::AddrInUse,
                        "aaaaaaaaaokkkkkkk",
                    ))
                }
            }
        }
    };
}

module!(Jungru);

#[no_mangle]
pub extern "C" fn init_jungru() {
    let data_class = Class::from_existing("Object");

    Module::new("Jungru").define(|module| {
        piece::define(module, &data_class);
        instrument::define(module, &data_class);
        meta::define(module, &data_class);
        track::define(module, &data_class);
        section::define(module, &data_class);
        transfer::define(module, &data_class);
        support::define(module, &data_class);
    });
}
