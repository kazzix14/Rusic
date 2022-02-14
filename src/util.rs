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
