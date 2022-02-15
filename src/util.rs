use rutie::{AnyObject, Object, TryConvert, VerifiedObject};

pub trait ConvertOrPanic<T> {
    fn convert_or_panic(self) -> T;
}

impl<T, U> ConvertOrPanic<U> for T
where
    U: VerifiedObject,
    T: Object,
{
    fn convert_or_panic(self) -> U {
        self.try_convert_to::<U>()
            .expect("panic trying to force convert @ util.rs")
    }
}

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
        impl $class {
            pub fn to_any_object(&self) -> AnyObject {
                AnyObject::from(self.value())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_inner {
    ($class: ident, $inner: ident, $wrapper: ident) => {
        impl $class {
            pub fn inner(&self) -> &$inner {
                self.get_data(&*$wrapper)
            }

            pub fn inner_mut(&mut self) -> &mut $inner {
                self.get_data_mut(&*$wrapper)
            }
        }
    };
}
