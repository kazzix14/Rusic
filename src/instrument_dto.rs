use crate::{impl_inner, inner::instrument_dto::*, ruby_class, util::ConvertOrPanic};

use rutie::{methods, types::Value, AnyObject, Class, Float, NilClass, Object, Symbol, GC};

pub fn define_class(super_class: &Class) {
    Class::new("InstrumentDto", Some(super_class)).define(|class| {
        class.def("load", instrument_dto_load);
        class.def("save", instrument_dto_save);
        class.def("offset", instrument_dto_offset);
        class.def("out", instrument_dto_out);
    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct InstrumentDto {
    value: Value,
}

impl InstrumentDto {
    pub fn new() -> AnyObject {
        let inner = InstrumentDtoInner::new();

        Class::from_existing("InstrumentDto").wrap_data(inner, &*TRANSFER_WRAPPER)
    }

    pub fn load(&mut self, key: Symbol) -> AnyObject {
        let instrument_dto = self.get_data_mut(&*TRANSFER_WRAPPER);

        instrument_dto
            .store
            .get(key.to_str())
            .cloned()
            .expect("key: `{key}` does not exist")
    }

    pub fn save(&mut self, key: Symbol, value: AnyObject) -> NilClass {
        let instrument_dto = self.get_data_mut(&*TRANSFER_WRAPPER);

        //GC::mark(&value);
        instrument_dto.store.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn offset(&mut self, offset: Float) -> NilClass {
        let instrument_dto = self.get_data_mut(&*TRANSFER_WRAPPER);

        instrument_dto.offset = offset.to_f64() as f32;

        NilClass::new()
    }

    pub fn out(&mut self, signal: AnyObject) -> NilClass {
        let instrument_dto = self.get_data_mut(&*TRANSFER_WRAPPER);

        let signal = match signal {
            _ if Class::from_existing("NilClass").case_equals(&signal) => None,
            _ if Class::from_existing("Float").case_equals(&signal) => {
                let signal: Float = signal.convert_or_panic();
                Some(signal.to_f64() as f32)
            }
            _ => panic!("signal is invalid"),
        };

        instrument_dto.out = signal;

        NilClass::new()
    }

    pub fn reset(&mut self) {
        self.get_data_mut(&*TRANSFER_WRAPPER).out = None;
    }
}

ruby_class!(InstrumentDto);
impl_inner!(InstrumentDto, InstrumentDtoInner, TRANSFER_WRAPPER);
methods!(
    InstrumentDto,
    itself,
    fn instrument_dto_load(key: Symbol) -> AnyObject {
        itself.load(key.unwrap())
    },
    fn instrument_dto_save(key: Symbol, value: AnyObject) -> NilClass {
        itself.save(key.unwrap(), value.unwrap())
    },
    fn instrument_dto_offset(offset: Float) -> NilClass {
        itself.offset(offset.unwrap())
    },
    fn instrument_dto_out(signal: AnyObject) -> NilClass {
        itself.out(signal.unwrap())
    },
);
