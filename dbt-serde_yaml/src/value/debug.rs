use crate::mapping::Mapping;
use crate::value::{Number, Value};
use std::fmt::{self, Debug, Display};

impl Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // TODO: print the span as well
        match self {
            Value::Null(span) => write!(formatter, "Null({:?})", span),
            Value::Bool(boolean, span) => write!(formatter, "Bool({:?}, {:?})", boolean, span),
            Value::Number(number, span) => write!(formatter, "Number({:?}, {:?})", number, span),
            Value::String(string, span) => write!(formatter, "String({:?}, {:?})", string, span),
            Value::Sequence(sequence, span) => {
                write!(formatter, "Sequence({:?}, {:?})", sequence, span)
            }
            Value::Mapping(mapping, ..) => Debug::fmt(mapping, formatter),
            Value::Tagged(tagged, ..) => Debug::fmt(tagged, formatter),
        }
    }
}

struct DisplayNumber<'a>(&'a Number);

impl Debug for DisplayNumber<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self.0, formatter)
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Number({})", self)
    }
}

impl Debug for Mapping {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Mapping ")?;
        let mut debug = formatter.debug_map();
        for (k, v) in self {
            let tmp;
            debug.entry(
                match k {
                    Value::Bool(boolean, ..) => boolean,
                    Value::Number(number, ..) => {
                        tmp = DisplayNumber(number);
                        &tmp
                    }
                    Value::String(string, ..) => string,
                    _ => k,
                },
                v,
            );
        }
        debug.finish()
    }
}
