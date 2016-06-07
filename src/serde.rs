extern crate serde;

mod note_freq {

    mod portamento {
        use note_freq::Portamento;
        use super::super::serde;

        impl serde::Serialize for Portamento {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                serializer.serialize_newtype_struct("Portamento", self.0)
            }
        }

        impl serde::Deserialize for Portamento {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = Portamento;

                    fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                        where E: serde::de::Error,
                    {
                        Ok(Portamento(v))
                    }

                    fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                        where D: serde::Deserializer,
                    {
                        Ok(Portamento(try!(serde::de::Deserialize::deserialize(deserializer))))
                    }
                }

                deserializer.deserialize_newtype_struct("Portamento", Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let portamento = Portamento(1000);
            let serialized = serde_json::to_string(&portamento).unwrap();

            println!("{}", serialized);
            assert_eq!("1000", &serialized);
            
            let deserialized: Portamento = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(portamento, deserialized);
        }
    }

    mod portamento_freq {
        use note_freq::PortamentoFreq;
        use super::super::serde;

        impl serde::Serialize for PortamentoFreq {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                struct Visitor<'a> {
                    t: &'a PortamentoFreq,
                    field_idx: u8,
                }

                impl<'a> serde::ser::MapVisitor for Visitor<'a> {
                    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                        where S: serde::Serializer,
                    {
                        match self.field_idx {
                            0 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("current_sample",
                                                                             self.t.current_sample))))
                            },
                            1 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("target_samples",
                                                                             self.t.target_samples))))
                            },
                            2 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("start_mel",
                                                                             self.t.start_mel))))
                            },
                            3 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("target_mel",
                                                                             self.t.target_mel))))
                            },
                            _ => Ok(None),
                        }
                    }

                    fn len(&self) -> Option<usize> {
                        Some(4)
                    }
                }

                serializer.serialize_struct("PortamentoFreq", Visitor { t: self, field_idx: 0 })
            }
        }

        impl serde::Deserialize for PortamentoFreq {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = PortamentoFreq;

                    fn visit_map<V>(&mut self, mut visitor: V) -> Result<PortamentoFreq, V::Error>
                        where V: serde::de::MapVisitor,
                    {
                        let mut current_sample = None;
                        let mut target_samples = None;
                        let mut start_mel = None;
                        let mut target_mel = None;

                        enum Field {
                            CurrentSample,
                            TargetSamples,
                            StartMel,
                            TargetMel,
                        }

                        impl serde::Deserialize for Field {
                            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                                where D: serde::de::Deserializer,
                            {
                                struct FieldVisitor;

                                impl serde::de::Visitor for FieldVisitor {
                                    type Value = Field;

                                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                                        where E: serde::de::Error,
                                    {
                                        match value {
                                            "current_sample" => Ok(Field::CurrentSample),
                                            "target_samples" => Ok(Field::TargetSamples),
                                            "start_mel" => Ok(Field::StartMel),
                                            "target_mel" => Ok(Field::TargetMel),
                                            _ => Err(serde::de::Error::custom(
                                                "expected current_sample, target_samples, \
                                                start_mel or target_mel"
                                            )),
                                        }
                                    }
                                }

                                deserializer.deserialize(FieldVisitor)
                            }
                        }

                        loop {
                            match try!(visitor.visit_key()) {
                                Some(Field::CurrentSample) => { current_sample = Some(try!(visitor.visit_value())); },
                                Some(Field::TargetSamples) => { target_samples = Some(try!(visitor.visit_value())); },
                                Some(Field::StartMel) => { start_mel = Some(try!(visitor.visit_value())); },
                                Some(Field::TargetMel) => { target_mel = Some(try!(visitor.visit_value())); },
                                None => { break; }
                            }
                        }

                        let current_sample = match current_sample {
                            Some(current_sample) => current_sample,
                            None => return Err(serde::de::Error::missing_field("current_sample")),
                        };

                        let target_samples = match target_samples {
                            Some(target_samples) => target_samples,
                            None => return Err(serde::de::Error::missing_field("target_samples")),
                        };

                        let start_mel = match start_mel {
                            Some(start_mel) => start_mel,
                            None => return Err(serde::de::Error::missing_field("start_mel")),
                        };

                        let target_mel = match target_mel {
                            Some(target_mel) => target_mel,
                            None => return Err(serde::de::Error::missing_field("target_mel")),
                        };

                        try!(visitor.end());

                        Ok(PortamentoFreq {
                            current_sample: current_sample,
                            target_samples: target_samples,
                            start_mel: start_mel,
                            target_mel: target_mel,
                        })
                    }
                }

                static FIELDS: &'static [&'static str] = &[
                    "current_sample",
                    "target_samples",
                    "start_mel",
                    "target_mel",
                ];

                deserializer.deserialize_struct("PortamentoFreq", FIELDS, Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let porta_freq = PortamentoFreq {
                current_sample: 0,
                target_samples: 10_000,
                start_mel: 10.0,
                target_mel: 20.0,
            };
            let serialized = serde_json::to_string(&porta_freq).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"current_sample\":0,\"target_samples\":10000,\"start_mel\":10,\"target_mel\":20}", serialized);
            
            let deserialized: PortamentoFreq = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(porta_freq, deserialized);
        }

    }

    mod dynamic_generator {
        use note_freq::DynamicGenerator;
        use super::super::serde;

        impl serde::Serialize for DynamicGenerator {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                match *self {
                    DynamicGenerator::Portamento(p) =>
                        serializer.serialize_newtype_variant("DynamicGenrator", 0, "Portamento", p),
                    DynamicGenerator::Constant =>
                        serializer.serialize_unit_variant("DynamicGenrator", 1, "Constant"),
                }
            }
        }

        impl serde::Deserialize for DynamicGenerator {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                enum Variant {
                    Portamento,
                    Constant,
                }

                impl serde::de::Deserialize for Variant {
                    fn deserialize<D>(deserializer: &mut D) -> Result<Variant, D::Error>
                        where D: serde::Deserializer,
                    {
                        struct VariantVisitor;

                        impl serde::de::Visitor for VariantVisitor {
                            type Value = Variant;

                            fn visit_str<E>(&mut self, value: &str) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                match value {
                                    "Portamento" => Ok(Variant::Portamento),
                                    "Constant" => Ok(Variant::Constant),
                                    _ => Err(serde::de::Error::unknown_field(value)),
                                }
                            }
                        }

                        deserializer.deserialize(VariantVisitor)
                    }
                }

                struct Visitor;

                impl serde::de::EnumVisitor for Visitor {
                    type Value = DynamicGenerator;

                    fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                        where V: serde::de::VariantVisitor,
                    {
                        match try!(visitor.visit_variant()) {
                            Variant::Portamento => {
                                let porta = try!(visitor.visit_newtype());
                                Ok(DynamicGenerator::Portamento(porta))
                            },
                            Variant::Constant => {
                                try!(visitor.visit_unit());
                                Ok(DynamicGenerator::Constant)
                            },
                        }
                    }
                }

                const VARIANTS: &'static [&'static str] = &["Portamento", "Constant"];

                deserializer.deserialize_enum("DynamicGenerator", VARIANTS, Visitor)
            }
        }

        #[test]
        fn test() {
            use note_freq::Portamento;
            extern crate serde_json;

            let porta_freq = DynamicGenerator::Portamento(Portamento(20));
            let serialized = serde_json::to_string(&porta_freq).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"Portamento\":20}", serialized);
            
            let deserialized: DynamicGenerator = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(porta_freq, deserialized);
        }
    }

    mod dynamic {
        use note_freq::Dynamic;
        use super::super::serde;

        impl serde::Serialize for Dynamic {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                match *self {
                    Dynamic::Portamento(p) =>
                        serializer.serialize_newtype_variant("Dynamic", 0, "Portamento", p),
                    Dynamic::Constant(hz) =>
                        serializer.serialize_newtype_variant("Dynamic", 1, "Constant", hz),
                }
            }
        }

        impl serde::Deserialize for Dynamic {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                enum Variant {
                    Portamento,
                    Constant,
                }

                impl serde::de::Deserialize for Variant {
                    fn deserialize<D>(deserializer: &mut D) -> Result<Variant, D::Error>
                        where D: serde::Deserializer,
                    {
                        struct VariantVisitor;

                        impl serde::de::Visitor for VariantVisitor {
                            type Value = Variant;

                            fn visit_str<E>(&mut self, value: &str) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                match value {
                                    "Portamento" => Ok(Variant::Portamento),
                                    "Constant" => Ok(Variant::Constant),
                                    _ => Err(serde::de::Error::unknown_field(value)),
                                }
                            }
                        }

                        deserializer.deserialize(VariantVisitor)
                    }
                }

                struct Visitor;

                impl serde::de::EnumVisitor for Visitor {
                    type Value = Dynamic;

                    fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                        where V: serde::de::VariantVisitor,
                    {
                        match try!(visitor.visit_variant()) {
                            Variant::Portamento => {
                                let porta = try!(visitor.visit_newtype());
                                Ok(Dynamic::Portamento(porta))
                            },
                            Variant::Constant => {
                                let hz = try!(visitor.visit_newtype());
                                Ok(Dynamic::Constant(hz))
                            },
                        }
                    }
                }

                const VARIANTS: &'static [&'static str] = &["Portamento", "Constant"];

                deserializer.deserialize_enum("Dynamic", VARIANTS, Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let hz = Dynamic::Constant(440.0);
            let serialized = serde_json::to_string(&hz).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"Constant\":440}", serialized);
            
            let deserialized: Dynamic = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(hz, deserialized);
        }
    }
}

mod mode {

    mod mono_kind {
        use mode::MonoKind;
        use super::super::serde;

        impl serde::Serialize for MonoKind {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                match *self {
                    MonoKind::Retrigger => serializer.serialize_unit_variant("MonoKind", 0, "Retrigger"),
                    MonoKind::Legato    => serializer.serialize_unit_variant("MonoKind", 1, "Legato"),
                }
            }
        }

        impl serde::Deserialize for MonoKind {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                enum Variant {
                    Retrigger,
                    Legato,
                }

                impl serde::de::Deserialize for Variant {
                    fn deserialize<D>(deserializer: &mut D) -> Result<Variant, D::Error>
                        where D: serde::Deserializer,
                    {
                        struct VariantVisitor;

                        impl serde::de::Visitor for VariantVisitor {
                            type Value = Variant;

                            fn visit_usize<E>(&mut self, value: usize) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                let var = match value {
                                    0 => Variant::Retrigger,
                                    1 => Variant::Legato,
                                    _ => return Err(serde::de::Error::unknown_field(&value.to_string())),
                                };
                                Ok(var)
                            }

                            fn visit_str<E>(&mut self, value: &str) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                match value {
                                    "Retrigger" => Ok(Variant::Retrigger),
                                    "Legato"    => Ok(Variant::Legato),
                                    _ => Err(serde::de::Error::unknown_field(value)),
                                }
                            }
                        }

                        deserializer.deserialize(VariantVisitor)
                    }
                }

                struct Visitor;

                impl serde::de::EnumVisitor for Visitor {
                    type Value = MonoKind;

                    fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                        where V: serde::de::VariantVisitor,
                    {
                        let kind = match try!(visitor.visit_variant()) {
                            Variant::Retrigger => MonoKind::Retrigger,
                            Variant::Legato => MonoKind::Legato,
                        };
                        try!(visitor.visit_unit());
                        Ok(kind)
                    }
                }

                const VARIANTS: &'static [&'static str] = &["Retrigger", "Legato"];

                deserializer.deserialize_enum("MonoKind", VARIANTS, Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let kind = MonoKind::Retrigger;
            let serialized = serde_json::to_string(&kind).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"Retrigger\":[]}", &serialized);
            
            let deserialized: MonoKind = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(kind, deserialized);
        }

    }

    mod mono {
        use mode::Mono;
        use super::super::serde;

        impl serde::Serialize for Mono {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                struct Visitor<'a> {
                    t: &'a Mono,
                    field_idx: u8,
                }

                impl<'a> serde::ser::SeqVisitor for Visitor<'a> {
                    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                        where S: serde::Serializer,
                    {
                        match self.field_idx {
                            0 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.0))))
                            },
                            1 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_tuple_struct_elt(&self.t.1))))
                            },
                            _ => Ok(None),
                        }
                    }

                    fn len(&self) -> Option<usize> {
                        Some(2)
                    }
                }

                serializer.serialize_tuple_struct("Mono", Visitor { t: self, field_idx: 0 })
            }
        }

        impl serde::Deserialize for Mono {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = Mono;

                    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Mono, V::Error>
                        where V: serde::de::SeqVisitor,
                    {
                        let kind = try!(visitor.visit());
                        let notes = try!(visitor.visit());

                        let kind = match kind {
                            Some(kind) => kind,
                            None => return Err(serde::de::Error::missing_field("kind")),
                        };

                        let notes = match notes {
                            Some(notes) => notes,
                            None => return Err(serde::de::Error::missing_field("notes")),
                        };

                        try!(visitor.end());

                        Ok(Mono(kind, notes))
                    }
                }

                deserializer.deserialize_tuple_struct("Mono", 2, Visitor)
            }
        }

        #[test]
        fn test() {
            use mode::MonoKind;
            extern crate serde_json;

            let mono = Mono(MonoKind::Retrigger, vec![440.0]);
            let serialized = serde_json::to_string(&mono).unwrap();

            println!("{}", serialized);
            assert_eq!("[{\"Retrigger\":[]},[440]]", &serialized);
            
            let deserialized: Mono = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(mono, deserialized);
        }

    }

    mod poly {
        use mode::Poly;
        use super::super::serde;

        impl serde::Serialize for Poly {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                serializer.serialize_unit_struct("Poly")
            }
        }

        impl serde::Deserialize for Poly {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = Poly;

                    fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
                        where E: serde::de::Error,
                    {
                        Ok(Poly)
                    }
                }

                deserializer.deserialize_unit_struct("Poly", Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let poly = Poly;
            let serialized = serde_json::to_string(&poly).unwrap();

            println!("{}", serialized);
            assert_eq!("null", &serialized);
            
            let deserialized: Poly = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(poly, deserialized);
        }
    }

    mod dynamic {
        use mode::Dynamic;
        use super::super::serde;

        impl serde::Serialize for Dynamic {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                match *self {
                    Dynamic::Mono(ref m) =>
                        serializer.serialize_newtype_variant("Dynamic", 0, "Mono", m),
                    Dynamic::Poly(ref p) =>
                        serializer.serialize_newtype_variant("Dynamic", 1, "Poly", p),
                }
            }
        }

        impl serde::Deserialize for Dynamic {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                enum Variant { Mono, Poly }

                impl serde::de::Deserialize for Variant {
                    fn deserialize<D>(deserializer: &mut D) -> Result<Variant, D::Error>
                        where D: serde::Deserializer,
                    {
                        struct VariantVisitor;

                        impl serde::de::Visitor for VariantVisitor {
                            type Value = Variant;

                            fn visit_str<E>(&mut self, value: &str) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                match value {
                                    "Mono" => Ok(Variant::Mono),
                                    "Poly" => Ok(Variant::Poly),
                                    _ => Err(serde::de::Error::unknown_field(value)),
                                }
                            }
                        }

                        deserializer.deserialize(VariantVisitor)
                    }
                }

                struct Visitor;

                impl serde::de::EnumVisitor for Visitor {
                    type Value = Dynamic;

                    fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                        where V: serde::de::VariantVisitor,
                    {
                        match try!(visitor.visit_variant()) {
                            Variant::Mono => {
                                let mono = try!(visitor.visit_newtype());
                                Ok(Dynamic::Mono(mono))
                            },
                            Variant::Poly => {
                                let poly = try!(visitor.visit_newtype());
                                Ok(Dynamic::Poly(poly))
                            },
                        }
                    }
                }

                const VARIANTS: &'static [&'static str] = &["Mono", "Poly"];

                deserializer.deserialize_enum("Dynamic", VARIANTS, Visitor)
            }
        }

        #[test]
        fn test() {
            use mode::Poly;
            extern crate serde_json;

            let poly = Dynamic::Poly(Poly);
            let serialized = serde_json::to_string(&poly).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"Poly\":null}", serialized);
            
            let deserialized: Dynamic = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(poly, deserialized);
        }
    }
}

mod voice {

    mod note_state {
        use voice::NoteState;
        use super::super::serde;

        impl serde::Serialize for NoteState {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                match *self {
                    NoteState::Playing =>
                        serializer.serialize_unit_variant("NoteState", 0, "Playing"),
                    NoteState::Released(playhead) =>
                        serializer.serialize_newtype_variant("NoteState", 1, "Released", playhead),
                }
            }
        }

        impl serde::Deserialize for NoteState {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                enum Variant {
                    Playing,
                    Released,
                }

                impl serde::de::Deserialize for Variant {
                    fn deserialize<D>(deserializer: &mut D) -> Result<Variant, D::Error>
                        where D: serde::Deserializer,
                    {
                        struct VariantVisitor;

                        impl serde::de::Visitor for VariantVisitor {
                            type Value = Variant;

                            fn visit_str<E>(&mut self, value: &str) -> Result<Variant, E>
                                where E: serde::de::Error,
                            {
                                match value {
                                    "Playing" => Ok(Variant::Playing),
                                    "Released" => Ok(Variant::Released),
                                    _ => Err(serde::de::Error::unknown_field(value)),
                                }
                            }
                        }

                        deserializer.deserialize(VariantVisitor)
                    }
                }

                struct Visitor;

                impl serde::de::EnumVisitor for Visitor {
                    type Value = NoteState;

                    fn visit<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                        where V: serde::de::VariantVisitor,
                    {
                        match try!(visitor.visit_variant()) {
                            Variant::Playing => {
                                try!(visitor.visit_unit());
                                Ok(NoteState::Playing)
                            },
                            Variant::Released => {
                                let playhead = try!(visitor.visit_newtype());
                                Ok(NoteState::Released(playhead))
                            },
                        }
                    }
                }

                const VARIANTS: &'static [&'static str] = &["Playing", "Released"];

                deserializer.deserialize_enum("NoteState", VARIANTS, Visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let state = NoteState::Playing;
            let serialized = serde_json::to_string(&state).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"Playing\":[]}", serialized);
            
            let deserialized: NoteState = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(state, deserialized);
        }
    }

    mod voice {
        use voice::Voice;
        use super::super::serde;

        impl<NF> serde::Serialize for Voice<NF>
            where NF: serde::Serialize,
        {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer,
            {
                struct Visitor<'a, NF: 'a> {
                    t: &'a Voice<NF>,
                    field_idx: u8,
                }

                impl<'a, NF> serde::ser::MapVisitor for Visitor<'a, NF>
                    where NF: serde::Serialize,
                {
                    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                        where S: serde::Serializer,
                    {
                        match self.field_idx {
                            0 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("note", &self.t.note))))
                            },
                            1 => {
                                self.field_idx += 1;
                                Ok(Some(try!(serializer.serialize_struct_elt("playhead", self.t.playhead))))
                            },
                            _ => Ok(None),
                        }
                    }

                    fn len(&self) -> Option<usize> {
                        Some(2)
                    }
                }

                serializer.serialize_struct("Voice", Visitor { t: self, field_idx: 0 })
            }
        }

        impl<NF> serde::Deserialize for Voice<NF>
            where NF: serde::Deserialize,
        {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: serde::Deserializer,
            {
                use std;

                struct Visitor<NF> { note_freq: std::marker::PhantomData<NF> };

                impl<NF> serde::de::Visitor for Visitor<NF>
                    where NF: serde::Deserialize,
                {
                    type Value = Voice<NF>;

                    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Voice<NF>, V::Error>
                        where V: serde::de::MapVisitor,
                    {
                        let mut note = None;
                        let mut playhead = None;

                        enum Field {
                            Note,
                            Playhead,
                        }

                        impl serde::Deserialize for Field {
                            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                                where D: serde::de::Deserializer,
                            {
                                struct FieldVisitor;

                                impl serde::de::Visitor for FieldVisitor {
                                    type Value = Field;

                                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                                        where E: serde::de::Error,
                                    {
                                        match value {
                                            "note" => Ok(Field::Note),
                                            "playhead" => Ok(Field::Playhead),
                                            _ => Err(serde::de::Error::custom("expected note or playhead")),
                                        }
                                    }
                                }

                                deserializer.deserialize(FieldVisitor)
                            }
                        }

                        loop {
                            match try!(visitor.visit_key()) {
                                Some(Field::Note) => { note = Some(try!(visitor.visit_value())); },
                                Some(Field::Playhead) => { playhead = Some(try!(visitor.visit_value())); },
                                None => { break; }
                            }
                        }

                        let note = match note {
                            Some(note) => note,
                            None => return Err(serde::de::Error::missing_field("note")),
                        };

                        let playhead = match playhead {
                            Some(playhead) => playhead,
                            None => return Err(serde::de::Error::missing_field("playhead")),
                        };

                        try!(visitor.end());

                        Ok(Voice {
                            note: note,
                            playhead: playhead,
                        })
                    }
                }

                static FIELDS: &'static [&'static str] = &[
                    "note",
                    "playhead",
                ];

                let visitor = Visitor { note_freq: std::marker::PhantomData };
                deserializer.deserialize_struct("Voice", FIELDS, visitor)
            }
        }

        #[test]
        fn test() {
            extern crate serde_json;

            let voice = Voice {
                note: None,
                playhead: 10_000,
            };
            let serialized = serde_json::to_string(&voice).unwrap();

            println!("{}", serialized);
            assert_eq!("{\"note\":null,\"playhead\":10000}", serialized);
            
            let deserialized: Voice<()> = serde_json::from_str(&serialized).unwrap();

            println!("{:?}", deserialized);
            assert_eq!(voice, deserialized);
        }
    }
}

mod instrument {
    use instrument::Instrument;
    use note_freq::NoteFreqGenerator;
    use super::serde;

    impl<M, NFG> serde::Serialize for Instrument<M, NFG>
        where M: serde::Serialize,
              NFG: serde::Serialize + NoteFreqGenerator,
              NFG::NoteFreq: serde::Serialize,
    {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            struct Visitor<'a, M: 'a, NFG: 'a>
                where NFG: NoteFreqGenerator,
            {
                t: &'a Instrument<M, NFG>,
                field_idx: u8,
            }

            impl<'a, M, NFG> serde::ser::MapVisitor for Visitor<'a, M, NFG>
                where M: serde::Serialize,
                      NFG: serde::Serialize + NoteFreqGenerator,
                      NFG::NoteFreq: serde::Serialize,
            {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("mode", &self.t.mode))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("voices", &self.t.voices))))
                        },
                        2 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("detune", self.t.detune))))
                        },
                        3 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("note_freq_gen", &self.t.note_freq_gen))))
                        },
                        4 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("attack_ms", self.t.attack_ms))))
                        },
                        5 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_struct_elt("release_ms", self.t.release_ms))))
                        },
                        _ => Ok(None),
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some(6)
                }
            }

            serializer.serialize_struct("Instrument", Visitor { t: self, field_idx: 0 })
        }
    }

    impl<M, NFG> serde::Deserialize for Instrument<M, NFG>
        where M: serde::Deserialize,
              NFG: serde::Deserialize + NoteFreqGenerator,
              NFG::NoteFreq: serde::Deserialize,
    {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            use std;

            struct Visitor<M, NFG> {
                mode: std::marker::PhantomData<M>,
                note_freq_gen: std::marker::PhantomData<NFG>,
            };

            impl<M, NFG> serde::de::Visitor for Visitor<M, NFG>
                where M: serde::Deserialize,
                      NFG: serde::Deserialize + NoteFreqGenerator,
                      NFG::NoteFreq: serde::Deserialize,
            {
                type Value = Instrument<M, NFG>;

                fn visit_map<V>(&mut self, mut visitor: V) -> Result<Instrument<M, NFG>, V::Error>
                    where V: serde::de::MapVisitor,
                {
                    let mut mode = None;
                    let mut voices = None;
                    let mut detune = None;
                    let mut note_freq_gen = None;
                    let mut attack_ms = None;
                    let mut release_ms = None;

                    enum Field {
                        Mode,
                        Voices,
                        Detune,
                        NoteFreqGen,
                        AttackMs,
                        ReleaseMs,
                    }

                    impl serde::Deserialize for Field {
                        fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                            where D: serde::de::Deserializer,
                        {
                            struct FieldVisitor;

                            impl serde::de::Visitor for FieldVisitor {
                                type Value = Field;

                                fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                                    where E: serde::de::Error,
                                {
                                    match value {
                                        "mode" => Ok(Field::Mode),
                                        "voices" => Ok(Field::Voices),
                                        "detune" => Ok(Field::Detune),
                                        "note_freq_gen" => Ok(Field::NoteFreqGen),
                                        "attack_ms" => Ok(Field::AttackMs),
                                        "release_ms" => Ok(Field::ReleaseMs),
                                        _ => Err(serde::de::Error::custom("expected mode, voices, \
                                        detune, note_freq_gen, attack_ms or release_ms")),
                                    }
                                }
                            }

                            deserializer.deserialize(FieldVisitor)
                        }
                    }

                    loop {
                        match try!(visitor.visit_key()) {
                            Some(Field::Mode)        => { mode = Some(try!(visitor.visit_value())); },
                            Some(Field::Voices)      => { voices = Some(try!(visitor.visit_value())); },
                            Some(Field::Detune)      => { detune = Some(try!(visitor.visit_value())); },
                            Some(Field::NoteFreqGen) => { note_freq_gen = Some(try!(visitor.visit_value())); },
                            Some(Field::AttackMs)    => { attack_ms = Some(try!(visitor.visit_value())); },
                            Some(Field::ReleaseMs)   => { release_ms = Some(try!(visitor.visit_value())); },
                            None => { break; }
                        }
                    }

                    let mode = match mode {
                        Some(mode) => mode,
                        None => return Err(serde::de::Error::missing_field("mode")),
                    };

                    let voices = match voices {
                        Some(voices) => voices,
                        None => return Err(serde::de::Error::missing_field("voices")),
                    };

                    let detune = match detune {
                        Some(detune) => detune,
                        None => return Err(serde::de::Error::missing_field("detune")),
                    };

                    let note_freq_gen = match note_freq_gen {
                        Some(note_freq_gen) => note_freq_gen,
                        None => return Err(serde::de::Error::missing_field("note_freq_gen")),
                    };

                    let attack_ms = match attack_ms {
                        Some(attack_ms) => attack_ms,
                        None => return Err(serde::de::Error::missing_field("attack_ms")),
                    };

                    let release_ms = match release_ms {
                        Some(release_ms) => release_ms,
                        None => return Err(serde::de::Error::missing_field("release_ms")),
                    };

                    try!(visitor.end());

                    Ok(Instrument {
                        mode: mode,
                        voices: voices,
                        detune: detune,
                        note_freq_gen: note_freq_gen,
                        attack_ms: attack_ms,
                        release_ms: release_ms,
                    })
                }
            }

            static FIELDS: &'static [&'static str] = &[
                "mode",
                "voices",
                "detune",
                "note_freq_gen",
                "attack_ms",
                "release_ms",
            ];

            let visitor = Visitor {
                mode: std::marker::PhantomData,
                note_freq_gen: std::marker::PhantomData,
            };
            deserializer.deserialize_struct("Instrument", FIELDS, visitor)
        }
    }

    #[test]
    fn test() {
        use mode::Poly;
        extern crate serde_json;

        let instrument = Instrument {
            mode: Poly,
            voices: vec![],
            detune: 0.25,
            note_freq_gen: (),
            attack_ms: 10.0.into(),
            release_ms: 100.0.into(),
        };
        let serialized = serde_json::to_string(&instrument).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"mode\":null,\"voices\":[],\"detune\":0.25,\"note_freq_gen\":null,\"attack_ms\":10,\"release_ms\":100}", serialized);
        
        let deserialized: Instrument<Poly, ()> = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(instrument, deserialized);
    }
}
