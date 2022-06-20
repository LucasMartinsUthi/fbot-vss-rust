// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct Ball {
    // message fields
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Ball {
    fn default() -> &'a Ball {
        <Ball as ::protobuf::Message>::default_instance()
    }
}

impl Ball {
    pub fn new() -> Ball {
        ::std::default::Default::default()
    }

    // double x = 1;


    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = v;
    }

    // double y = 2;


    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = v;
    }

    // double z = 3;


    pub fn get_z(&self) -> f64 {
        self.z
    }
    pub fn clear_z(&mut self) {
        self.z = 0.;
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f64) {
        self.z = v;
    }

    // double vx = 4;


    pub fn get_vx(&self) -> f64 {
        self.vx
    }
    pub fn clear_vx(&mut self) {
        self.vx = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vx(&mut self, v: f64) {
        self.vx = v;
    }

    // double vy = 5;


    pub fn get_vy(&self) -> f64 {
        self.vy
    }
    pub fn clear_vy(&mut self) {
        self.vy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vy(&mut self, v: f64) {
        self.vy = v;
    }

    // double vz = 6;


    pub fn get_vz(&self) -> f64 {
        self.vz
    }
    pub fn clear_vz(&mut self) {
        self.vz = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vz(&mut self, v: f64) {
        self.vz = v;
    }
}

impl ::protobuf::Message for Ball {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.y = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.z = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vx = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vy = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vz = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.x != 0. {
            my_size += 9;
        }
        if self.y != 0. {
            my_size += 9;
        }
        if self.z != 0. {
            my_size += 9;
        }
        if self.vx != 0. {
            my_size += 9;
        }
        if self.vy != 0. {
            my_size += 9;
        }
        if self.vz != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.x != 0. {
            os.write_double(1, self.x)?;
        }
        if self.y != 0. {
            os.write_double(2, self.y)?;
        }
        if self.z != 0. {
            os.write_double(3, self.z)?;
        }
        if self.vx != 0. {
            os.write_double(4, self.vx)?;
        }
        if self.vy != 0. {
            os.write_double(5, self.vy)?;
        }
        if self.vz != 0. {
            os.write_double(6, self.vz)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Ball {
        Ball::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "x",
                |m: &Ball| { &m.x },
                |m: &mut Ball| { &mut m.x },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "y",
                |m: &Ball| { &m.y },
                |m: &mut Ball| { &mut m.y },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "z",
                |m: &Ball| { &m.z },
                |m: &mut Ball| { &mut m.z },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vx",
                |m: &Ball| { &m.vx },
                |m: &mut Ball| { &mut m.vx },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vy",
                |m: &Ball| { &m.vy },
                |m: &mut Ball| { &mut m.vy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vz",
                |m: &Ball| { &m.vz },
                |m: &mut Ball| { &mut m.vz },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Ball>(
                "Ball",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Ball {
        static instance: ::protobuf::rt::LazyV2<Ball> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Ball::new)
    }
}

impl ::protobuf::Clear for Ball {
    fn clear(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.z = 0.;
        self.vx = 0.;
        self.vy = 0.;
        self.vz = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ball {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ball {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Robot {
    // message fields
    pub robot_id: u32,
    pub x: f64,
    pub y: f64,
    pub orientation: f64,
    pub vx: f64,
    pub vy: f64,
    pub vorientation: f64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Robot {
    fn default() -> &'a Robot {
        <Robot as ::protobuf::Message>::default_instance()
    }
}

impl Robot {
    pub fn new() -> Robot {
        ::std::default::Default::default()
    }

    // uint32 robot_id = 1;


    pub fn get_robot_id(&self) -> u32 {
        self.robot_id
    }
    pub fn clear_robot_id(&mut self) {
        self.robot_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_robot_id(&mut self, v: u32) {
        self.robot_id = v;
    }

    // double x = 2;


    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn clear_x(&mut self) {
        self.x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f64) {
        self.x = v;
    }

    // double y = 3;


    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn clear_y(&mut self) {
        self.y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f64) {
        self.y = v;
    }

    // double orientation = 4;


    pub fn get_orientation(&self) -> f64 {
        self.orientation
    }
    pub fn clear_orientation(&mut self) {
        self.orientation = 0.;
    }

    // Param is passed by value, moved
    pub fn set_orientation(&mut self, v: f64) {
        self.orientation = v;
    }

    // double vx = 5;


    pub fn get_vx(&self) -> f64 {
        self.vx
    }
    pub fn clear_vx(&mut self) {
        self.vx = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vx(&mut self, v: f64) {
        self.vx = v;
    }

    // double vy = 6;


    pub fn get_vy(&self) -> f64 {
        self.vy
    }
    pub fn clear_vy(&mut self) {
        self.vy = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vy(&mut self, v: f64) {
        self.vy = v;
    }

    // double vorientation = 7;


    pub fn get_vorientation(&self) -> f64 {
        self.vorientation
    }
    pub fn clear_vorientation(&mut self) {
        self.vorientation = 0.;
    }

    // Param is passed by value, moved
    pub fn set_vorientation(&mut self, v: f64) {
        self.vorientation = v;
    }
}

impl ::protobuf::Message for Robot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.robot_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.x = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.y = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.orientation = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vx = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vy = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.vorientation = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.robot_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.robot_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.x != 0. {
            my_size += 9;
        }
        if self.y != 0. {
            my_size += 9;
        }
        if self.orientation != 0. {
            my_size += 9;
        }
        if self.vx != 0. {
            my_size += 9;
        }
        if self.vy != 0. {
            my_size += 9;
        }
        if self.vorientation != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.robot_id != 0 {
            os.write_uint32(1, self.robot_id)?;
        }
        if self.x != 0. {
            os.write_double(2, self.x)?;
        }
        if self.y != 0. {
            os.write_double(3, self.y)?;
        }
        if self.orientation != 0. {
            os.write_double(4, self.orientation)?;
        }
        if self.vx != 0. {
            os.write_double(5, self.vx)?;
        }
        if self.vy != 0. {
            os.write_double(6, self.vy)?;
        }
        if self.vorientation != 0. {
            os.write_double(7, self.vorientation)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Robot {
        Robot::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "robot_id",
                |m: &Robot| { &m.robot_id },
                |m: &mut Robot| { &mut m.robot_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "x",
                |m: &Robot| { &m.x },
                |m: &mut Robot| { &mut m.x },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "y",
                |m: &Robot| { &m.y },
                |m: &mut Robot| { &mut m.y },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "orientation",
                |m: &Robot| { &m.orientation },
                |m: &mut Robot| { &mut m.orientation },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vx",
                |m: &Robot| { &m.vx },
                |m: &mut Robot| { &mut m.vx },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vy",
                |m: &Robot| { &m.vy },
                |m: &mut Robot| { &mut m.vy },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "vorientation",
                |m: &Robot| { &m.vorientation },
                |m: &mut Robot| { &mut m.vorientation },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Robot>(
                "Robot",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Robot {
        static instance: ::protobuf::rt::LazyV2<Robot> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Robot::new)
    }
}

impl ::protobuf::Clear for Robot {
    fn clear(&mut self) {
        self.robot_id = 0;
        self.x = 0.;
        self.y = 0.;
        self.orientation = 0.;
        self.vx = 0.;
        self.vy = 0.;
        self.vorientation = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Robot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Robot {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Field {
    // message fields
    pub width: f64,
    pub length: f64,
    pub goal_width: f64,
    pub goal_depth: f64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Field {
    fn default() -> &'a Field {
        <Field as ::protobuf::Message>::default_instance()
    }
}

impl Field {
    pub fn new() -> Field {
        ::std::default::Default::default()
    }

    // double width = 1;


    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn clear_width(&mut self) {
        self.width = 0.;
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: f64) {
        self.width = v;
    }

    // double length = 2;


    pub fn get_length(&self) -> f64 {
        self.length
    }
    pub fn clear_length(&mut self) {
        self.length = 0.;
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: f64) {
        self.length = v;
    }

    // double goal_width = 3;


    pub fn get_goal_width(&self) -> f64 {
        self.goal_width
    }
    pub fn clear_goal_width(&mut self) {
        self.goal_width = 0.;
    }

    // Param is passed by value, moved
    pub fn set_goal_width(&mut self, v: f64) {
        self.goal_width = v;
    }

    // double goal_depth = 4;


    pub fn get_goal_depth(&self) -> f64 {
        self.goal_depth
    }
    pub fn clear_goal_depth(&mut self) {
        self.goal_depth = 0.;
    }

    // Param is passed by value, moved
    pub fn set_goal_depth(&mut self, v: f64) {
        self.goal_depth = v;
    }
}

impl ::protobuf::Message for Field {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.width = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.length = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.goal_width = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.goal_depth = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.width != 0. {
            my_size += 9;
        }
        if self.length != 0. {
            my_size += 9;
        }
        if self.goal_width != 0. {
            my_size += 9;
        }
        if self.goal_depth != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.width != 0. {
            os.write_double(1, self.width)?;
        }
        if self.length != 0. {
            os.write_double(2, self.length)?;
        }
        if self.goal_width != 0. {
            os.write_double(3, self.goal_width)?;
        }
        if self.goal_depth != 0. {
            os.write_double(4, self.goal_depth)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Field {
        Field::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "width",
                |m: &Field| { &m.width },
                |m: &mut Field| { &mut m.width },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "length",
                |m: &Field| { &m.length },
                |m: &mut Field| { &mut m.length },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "goal_width",
                |m: &Field| { &m.goal_width },
                |m: &mut Field| { &mut m.goal_width },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "goal_depth",
                |m: &Field| { &m.goal_depth },
                |m: &mut Field| { &mut m.goal_depth },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Field>(
                "Field",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Field {
        static instance: ::protobuf::rt::LazyV2<Field> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Field::new)
    }
}

impl ::protobuf::Clear for Field {
    fn clear(&mut self) {
        self.width = 0.;
        self.length = 0.;
        self.goal_width = 0.;
        self.goal_depth = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Field {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Frame {
    // message fields
    pub ball: ::protobuf::SingularPtrField<Ball>,
    pub robots_yellow: ::protobuf::RepeatedField<Robot>,
    pub robots_blue: ::protobuf::RepeatedField<Robot>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Frame {
    fn default() -> &'a Frame {
        <Frame as ::protobuf::Message>::default_instance()
    }
}

impl Frame {
    pub fn new() -> Frame {
        ::std::default::Default::default()
    }

    // .fira_message.Ball ball = 1;


    pub fn get_ball(&self) -> &Ball {
        self.ball.as_ref().unwrap_or_else(|| <Ball as ::protobuf::Message>::default_instance())
    }
    pub fn clear_ball(&mut self) {
        self.ball.clear();
    }

    pub fn has_ball(&self) -> bool {
        self.ball.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ball(&mut self, v: Ball) {
        self.ball = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ball(&mut self) -> &mut Ball {
        if self.ball.is_none() {
            self.ball.set_default();
        }
        self.ball.as_mut().unwrap()
    }

    // Take field
    pub fn take_ball(&mut self) -> Ball {
        self.ball.take().unwrap_or_else(|| Ball::new())
    }

    // repeated .fira_message.Robot robots_yellow = 2;


    pub fn get_robots_yellow(&self) -> &[Robot] {
        &self.robots_yellow
    }
    pub fn clear_robots_yellow(&mut self) {
        self.robots_yellow.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots_yellow(&mut self, v: ::protobuf::RepeatedField<Robot>) {
        self.robots_yellow = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots_yellow(&mut self) -> &mut ::protobuf::RepeatedField<Robot> {
        &mut self.robots_yellow
    }

    // Take field
    pub fn take_robots_yellow(&mut self) -> ::protobuf::RepeatedField<Robot> {
        ::std::mem::replace(&mut self.robots_yellow, ::protobuf::RepeatedField::new())
    }

    // repeated .fira_message.Robot robots_blue = 3;


    pub fn get_robots_blue(&self) -> &[Robot] {
        &self.robots_blue
    }
    pub fn clear_robots_blue(&mut self) {
        self.robots_blue.clear();
    }

    // Param is passed by value, moved
    pub fn set_robots_blue(&mut self, v: ::protobuf::RepeatedField<Robot>) {
        self.robots_blue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_robots_blue(&mut self) -> &mut ::protobuf::RepeatedField<Robot> {
        &mut self.robots_blue
    }

    // Take field
    pub fn take_robots_blue(&mut self) -> ::protobuf::RepeatedField<Robot> {
        ::std::mem::replace(&mut self.robots_blue, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Frame {
    fn is_initialized(&self) -> bool {
        for v in &self.ball {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.robots_yellow {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.robots_blue {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ball)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots_yellow)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.robots_blue)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.ball.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.robots_yellow {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.robots_blue {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ball.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.robots_yellow {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.robots_blue {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Frame {
        Frame::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ball>>(
                "ball",
                |m: &Frame| { &m.ball },
                |m: &mut Frame| { &mut m.ball },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Robot>>(
                "robots_yellow",
                |m: &Frame| { &m.robots_yellow },
                |m: &mut Frame| { &mut m.robots_yellow },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Robot>>(
                "robots_blue",
                |m: &Frame| { &m.robots_blue },
                |m: &mut Frame| { &mut m.robots_blue },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Frame>(
                "Frame",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Frame {
        static instance: ::protobuf::rt::LazyV2<Frame> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Frame::new)
    }
}

impl ::protobuf::Clear for Frame {
    fn clear(&mut self) {
        self.ball.clear();
        self.robots_yellow.clear();
        self.robots_blue.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Frame {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccommon.proto\x12\x0cfira_message\"`\n\x04Ball\x12\x0c\n\x01x\x18\
    \x01\x20\x01(\x01R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x01R\x01y\x12\
    \x0c\n\x01z\x18\x03\x20\x01(\x01R\x01z\x12\x0e\n\x02vx\x18\x04\x20\x01(\
    \x01R\x02vx\x12\x0e\n\x02vy\x18\x05\x20\x01(\x01R\x02vy\x12\x0e\n\x02vz\
    \x18\x06\x20\x01(\x01R\x02vz\"\xa4\x01\n\x05Robot\x12\x19\n\x08robot_id\
    \x18\x01\x20\x01(\rR\x07robotId\x12\x0c\n\x01x\x18\x02\x20\x01(\x01R\x01\
    x\x12\x0c\n\x01y\x18\x03\x20\x01(\x01R\x01y\x12\x20\n\x0borientation\x18\
    \x04\x20\x01(\x01R\x0borientation\x12\x0e\n\x02vx\x18\x05\x20\x01(\x01R\
    \x02vx\x12\x0e\n\x02vy\x18\x06\x20\x01(\x01R\x02vy\x12\"\n\x0cvorientati\
    on\x18\x07\x20\x01(\x01R\x0cvorientation\"s\n\x05Field\x12\x14\n\x05widt\
    h\x18\x01\x20\x01(\x01R\x05width\x12\x16\n\x06length\x18\x02\x20\x01(\
    \x01R\x06length\x12\x1d\n\ngoal_width\x18\x03\x20\x01(\x01R\tgoalWidth\
    \x12\x1d\n\ngoal_depth\x18\x04\x20\x01(\x01R\tgoalDepth\"\x9f\x01\n\x05F\
    rame\x12&\n\x04ball\x18\x01\x20\x01(\x0b2\x12.fira_message.BallR\x04ball\
    \x128\n\rrobots_yellow\x18\x02\x20\x03(\x0b2\x13.fira_message.RobotR\x0c\
    robotsYellow\x124\n\x0brobots_blue\x18\x03\x20\x03(\x0b2\x13.fira_messag\
    e.RobotR\nrobotsBlueb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}