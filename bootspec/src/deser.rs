// #[cfg(any())]
pub mod s {
    use std::collections::HashMap;
    use std::fmt;

    use serde::de::{Deserializer, MapAccess, Visitor};

    use crate::{generation::Generation, Extension};

    struct BootSpecExtensionMapVisitor;

    impl<'de> Visitor<'de> for BootSpecExtensionMapVisitor {
        type Value = HashMap<String, Extension>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of bootspec extensions")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let generation_fields = Generation::field_names();

            let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

            while let Some((key, value)) = access.next_entry::<String, Extension>()? {
                if generation_fields.contains(&key.as_str()) {
                    continue;
                }

                map.insert(key, value);
            }

            Ok(map)
        }
    }

    pub fn temp_serde_fix<'de, D>(deserializer: D) -> Result<HashMap<String, Extension>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BootSpecExtensionMapVisitor)
    }
}

#[cfg(any())]
pub mod t {
    use std::{collections::HashMap, fmt};

    use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};

    use crate::{generation::Generation, BootJson, Extension};

    extern crate serde as _serde;
    impl<'de> _serde::Deserialize<'de> for BootJson {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field<'de> {
                __other(_serde::__private::de::Content<'de>),
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        _ => {
                            let __value = _serde::__private::de::Content::String(
                                _serde::__private::ToString::to_string(__value),
                            );
                            _serde::__private::Ok(__Field::__other(__value))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BootJson>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BootJson;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct BootJson")
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __collect = _serde::__private::Vec::<
                        _serde::__private::Option<(
                            _serde::__private::de::Content,
                            _serde::__private::de::Content,
                        )>,
                    >::new();
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__other(__name) => {
                                __collect.push(_serde::__private::Some((
                                    __name,
                                    match _serde::de::MapAccess::next_value(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                )));
                            }
                        }
                    }
                    let __field0: Generation = match _serde::de::Deserialize::deserialize(
                        _serde::__private::de::FlatMapDeserializer(
                            &mut __collect,
                            _serde::__private::PhantomData,
                        ),
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    // dbg!(&__collect);
                    __collect.retain(|content| {
                        dbg!(&content);
                        if let Some((_serde::__private::de::Content::String(name), _)) = content {
                            dbg!(__field0.field_names());
                            if __field0.field_names().contains(&name.as_str()) {
                                dbg!(&name);
                                return false;
                            }
                        }
                        // dbg!(&content);
                        true
                    });
                    dbg!(&__collect);
                    // panic!();
                    let __field1: HashMap<String, Extension> =
                        match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    _serde::__private::Ok(BootJson {
                        generation: __field0,
                        extensions: __field1,
                    })
                }
            }
            _serde::Deserializer::deserialize_map(
                __deserializer,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BootJson>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }

    // impl<'de> Deserialize<'de> for BootJson {
    //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //     where
    //         D: Deserializer<'de>,
    //     {
    //         #[derive(serde::Deserialize)]
    //         #[serde(field_identifier, rename_all = "lowercase")]
    //         enum Field {
    //             Generation,
    //             Extensions,
    //         }

    //         struct BootJsonVisitor;

    //         impl<'de> Visitor<'de> for BootJsonVisitor {
    //             type Value = BootJson;

    //             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    //                 formatter.write_str("struct BootJson")
    //             }

    //             fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    //             where
    //                 V: MapAccess<'de>,
    //             {
    //                 let mut generation = None;
    //                 let mut extensions = None;
    //                 dbg!();
    //                 while let Some(key) = map.next_key()? {
    //                     match key {
    //                         Field::Generation => {
    //                             if generation.is_some() {
    //                                 return Err(de::Error::duplicate_field("generation"));
    //                             }
    //                             generation = Some(map.next_value()?);
    //                             dbg!(&generation);
    //                         }
    //                         Field::Extensions => {
    //                             if extensions.is_some() {
    //                                 return Err(de::Error::duplicate_field("extensions"));
    //                             }
    //                             extensions = Some(map.next_value()?);
    //                             dbg!(&extensions);
    //                         }
    //                     }
    //                 }
    //                 dbg!();

    //                 panic!();
    //                 let generation =
    //                     generation.ok_or_else(|| de::Error::missing_field("generation"))?;
    //                 let extensions =
    //                     extensions.ok_or_else(|| de::Error::missing_field("extensions"))?;

    //                 Ok(BootJson {
    //                     generation,
    //                     extensions,
    //                 })
    //             }
    //         }

    //         const FIELDS: &'static [&'static str] = &["generation", "extensions"];
    //         deserializer.deserialize_struct("BootJson", FIELDS, BootJsonVisitor)
    //     }
    // }
}
