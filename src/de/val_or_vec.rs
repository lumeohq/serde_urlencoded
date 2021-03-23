use serde::de::{
    self,
    value::{Error, SeqDeserializer},
    Deserializer, IntoDeserializer,
};

pub enum ValOrVec<T> {
    Val(T),
    Vec(Vec<T>),
}

impl<T> ValOrVec<T> {
    fn deserialize_val<U, E, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
        E: de::Error,
    {
        match self {
            ValOrVec::Val(val) => f(val),
            ValOrVec::Vec(_) => Err(de::Error::custom("unsupported value")),
        }
    }
}

impl<'de, T> IntoDeserializer<'de> for ValOrVec<T>
where
    T: IntoDeserializer<'de> + Deserializer<'de, Error = Error>,
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_to_part {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                self.deserialize_val(move |val| val.$method(visitor))
            }
        )*
    }
}

impl<'de, T> Deserializer<'de> for ValOrVec<T>
where
    T: IntoDeserializer<'de> + Deserializer<'de, Error = Error>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            ValOrVec::Val(val) => val.deserialize_any(visitor),
            ValOrVec::Vec(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            ValOrVec::Val(val) => val.deserialize_seq(visitor),
            ValOrVec::Vec(vec) => {
                visitor.visit_seq(SeqDeserializer::new(vec.into_iter()))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| {
            val.deserialize_enum(name, variants, visitor)
        })
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_tuple(len, visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| {
            val.deserialize_struct(name, fields, visitor)
        })
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| {
            val.deserialize_unit_struct(name, visitor)
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| {
            val.deserialize_tuple_struct(name, len, visitor)
        })
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_val(move |val| {
            val.deserialize_newtype_struct(name, visitor)
        })
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    forward_to_part! {
        deserialize_bool,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_unit,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_f32,
        deserialize_f64,
        deserialize_option,
        deserialize_identifier,
        deserialize_map,
    }
}
