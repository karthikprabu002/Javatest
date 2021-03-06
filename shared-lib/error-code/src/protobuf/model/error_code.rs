// This file is generated by rust-protobuf 2.22.1. Do not edit
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
//! Generated file from `error_code.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    Internal = 0,
    UserUnauthorized = 2,
    RecordNotFound = 3,
    WorkspaceNameInvalid = 100,
    WorkspaceIdInvalid = 101,
    AppColorStyleInvalid = 102,
    WorkspaceDescTooLong = 103,
    WorkspaceNameTooLong = 104,
    AppIdInvalid = 110,
    AppNameInvalid = 111,
    ViewNameInvalid = 120,
    ViewThumbnailInvalid = 121,
    ViewIdInvalid = 122,
    ViewDescTooLong = 123,
    ViewDataInvalid = 124,
    ViewNameTooLong = 125,
    ConnectError = 200,
    EmailIsEmpty = 300,
    EmailFormatInvalid = 301,
    EmailAlreadyExists = 302,
    PasswordIsEmpty = 303,
    PasswordTooLong = 304,
    PasswordContainsForbidCharacters = 305,
    PasswordFormatInvalid = 306,
    PasswordNotMatch = 307,
    UserNameTooLong = 308,
    UserNameContainForbiddenCharacters = 309,
    UserNameIsEmpty = 310,
    UserIdInvalid = 311,
    UserNotExist = 312,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::Internal),
            2 => ::std::option::Option::Some(ErrorCode::UserUnauthorized),
            3 => ::std::option::Option::Some(ErrorCode::RecordNotFound),
            100 => ::std::option::Option::Some(ErrorCode::WorkspaceNameInvalid),
            101 => ::std::option::Option::Some(ErrorCode::WorkspaceIdInvalid),
            102 => ::std::option::Option::Some(ErrorCode::AppColorStyleInvalid),
            103 => ::std::option::Option::Some(ErrorCode::WorkspaceDescTooLong),
            104 => ::std::option::Option::Some(ErrorCode::WorkspaceNameTooLong),
            110 => ::std::option::Option::Some(ErrorCode::AppIdInvalid),
            111 => ::std::option::Option::Some(ErrorCode::AppNameInvalid),
            120 => ::std::option::Option::Some(ErrorCode::ViewNameInvalid),
            121 => ::std::option::Option::Some(ErrorCode::ViewThumbnailInvalid),
            122 => ::std::option::Option::Some(ErrorCode::ViewIdInvalid),
            123 => ::std::option::Option::Some(ErrorCode::ViewDescTooLong),
            124 => ::std::option::Option::Some(ErrorCode::ViewDataInvalid),
            125 => ::std::option::Option::Some(ErrorCode::ViewNameTooLong),
            200 => ::std::option::Option::Some(ErrorCode::ConnectError),
            300 => ::std::option::Option::Some(ErrorCode::EmailIsEmpty),
            301 => ::std::option::Option::Some(ErrorCode::EmailFormatInvalid),
            302 => ::std::option::Option::Some(ErrorCode::EmailAlreadyExists),
            303 => ::std::option::Option::Some(ErrorCode::PasswordIsEmpty),
            304 => ::std::option::Option::Some(ErrorCode::PasswordTooLong),
            305 => ::std::option::Option::Some(ErrorCode::PasswordContainsForbidCharacters),
            306 => ::std::option::Option::Some(ErrorCode::PasswordFormatInvalid),
            307 => ::std::option::Option::Some(ErrorCode::PasswordNotMatch),
            308 => ::std::option::Option::Some(ErrorCode::UserNameTooLong),
            309 => ::std::option::Option::Some(ErrorCode::UserNameContainForbiddenCharacters),
            310 => ::std::option::Option::Some(ErrorCode::UserNameIsEmpty),
            311 => ::std::option::Option::Some(ErrorCode::UserIdInvalid),
            312 => ::std::option::Option::Some(ErrorCode::UserNotExist),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::Internal,
            ErrorCode::UserUnauthorized,
            ErrorCode::RecordNotFound,
            ErrorCode::WorkspaceNameInvalid,
            ErrorCode::WorkspaceIdInvalid,
            ErrorCode::AppColorStyleInvalid,
            ErrorCode::WorkspaceDescTooLong,
            ErrorCode::WorkspaceNameTooLong,
            ErrorCode::AppIdInvalid,
            ErrorCode::AppNameInvalid,
            ErrorCode::ViewNameInvalid,
            ErrorCode::ViewThumbnailInvalid,
            ErrorCode::ViewIdInvalid,
            ErrorCode::ViewDescTooLong,
            ErrorCode::ViewDataInvalid,
            ErrorCode::ViewNameTooLong,
            ErrorCode::ConnectError,
            ErrorCode::EmailIsEmpty,
            ErrorCode::EmailFormatInvalid,
            ErrorCode::EmailAlreadyExists,
            ErrorCode::PasswordIsEmpty,
            ErrorCode::PasswordTooLong,
            ErrorCode::PasswordContainsForbidCharacters,
            ErrorCode::PasswordFormatInvalid,
            ErrorCode::PasswordNotMatch,
            ErrorCode::UserNameTooLong,
            ErrorCode::UserNameContainForbiddenCharacters,
            ErrorCode::UserNameIsEmpty,
            ErrorCode::UserIdInvalid,
            ErrorCode::UserNotExist,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorCode>("ErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Internal
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10error_code.proto*\xc4\x05\n\tErrorCode\x12\x0c\n\x08Internal\x10\0\
    \x12\x14\n\x10UserUnauthorized\x10\x02\x12\x12\n\x0eRecordNotFound\x10\
    \x03\x12\x18\n\x14WorkspaceNameInvalid\x10d\x12\x16\n\x12WorkspaceIdInva\
    lid\x10e\x12\x18\n\x14AppColorStyleInvalid\x10f\x12\x18\n\x14WorkspaceDe\
    scTooLong\x10g\x12\x18\n\x14WorkspaceNameTooLong\x10h\x12\x10\n\x0cAppId\
    Invalid\x10n\x12\x12\n\x0eAppNameInvalid\x10o\x12\x13\n\x0fViewNameInval\
    id\x10x\x12\x18\n\x14ViewThumbnailInvalid\x10y\x12\x11\n\rViewIdInvalid\
    \x10z\x12\x13\n\x0fViewDescTooLong\x10{\x12\x13\n\x0fViewDataInvalid\x10\
    |\x12\x13\n\x0fViewNameTooLong\x10}\x12\x11\n\x0cConnectError\x10\xc8\
    \x01\x12\x11\n\x0cEmailIsEmpty\x10\xac\x02\x12\x17\n\x12EmailFormatInval\
    id\x10\xad\x02\x12\x17\n\x12EmailAlreadyExists\x10\xae\x02\x12\x14\n\x0f\
    PasswordIsEmpty\x10\xaf\x02\x12\x14\n\x0fPasswordTooLong\x10\xb0\x02\x12\
    %\n\x20PasswordContainsForbidCharacters\x10\xb1\x02\x12\x1a\n\x15Passwor\
    dFormatInvalid\x10\xb2\x02\x12\x15\n\x10PasswordNotMatch\x10\xb3\x02\x12\
    \x14\n\x0fUserNameTooLong\x10\xb4\x02\x12'\n\"UserNameContainForbiddenCh\
    aracters\x10\xb5\x02\x12\x14\n\x0fUserNameIsEmpty\x10\xb6\x02\x12\x12\n\
    \rUserIdInvalid\x10\xb7\x02\x12\x11\n\x0cUserNotExist\x10\xb8\x02J\xf8\t\
    \n\x06\x12\x04\0\0!\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\
    \x12\x04\x02\0!\x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x0e\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03\x03\x04\x11\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\
    \x04\x0c\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x03\x0f\x10\n\x0b\n\x04\x05\
    \0\x02\x01\x12\x03\x04\x04\x19\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\
    \x04\x14\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x04\x17\x18\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\x05\x04\x17\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \x05\x04\x12\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\x15\x16\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\x06\x04\x1f\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\x06\x04\x18\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\x1b\x1e\n\x0b\
    \n\x04\x05\0\x02\x04\x12\x03\x07\x04\x1d\n\x0c\n\x05\x05\0\x02\x04\x01\
    \x12\x03\x07\x04\x16\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\x19\x1c\n\
    \x0b\n\x04\x05\0\x02\x05\x12\x03\x08\x04\x1f\n\x0c\n\x05\x05\0\x02\x05\
    \x01\x12\x03\x08\x04\x18\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x08\x1b\
    \x1e\n\x0b\n\x04\x05\0\x02\x06\x12\x03\t\x04\x1f\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\t\x04\x18\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\t\x1b\
    \x1e\n\x0b\n\x04\x05\0\x02\x07\x12\x03\n\x04\x1f\n\x0c\n\x05\x05\0\x02\
    \x07\x01\x12\x03\n\x04\x18\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\n\x1b\
    \x1e\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x0b\x04\x17\n\x0c\n\x05\x05\0\x02\
    \x08\x01\x12\x03\x0b\x04\x10\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x0b\
    \x13\x16\n\x0b\n\x04\x05\0\x02\t\x12\x03\x0c\x04\x19\n\x0c\n\x05\x05\0\
    \x02\t\x01\x12\x03\x0c\x04\x12\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x0c\
    \x15\x18\n\x0b\n\x04\x05\0\x02\n\x12\x03\r\x04\x1a\n\x0c\n\x05\x05\0\x02\
    \n\x01\x12\x03\r\x04\x13\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\r\x16\x19\n\
    \x0b\n\x04\x05\0\x02\x0b\x12\x03\x0e\x04\x1f\n\x0c\n\x05\x05\0\x02\x0b\
    \x01\x12\x03\x0e\x04\x18\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x0e\x1b\
    \x1e\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x0f\x04\x18\n\x0c\n\x05\x05\0\x02\
    \x0c\x01\x12\x03\x0f\x04\x11\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x0f\
    \x14\x17\n\x0b\n\x04\x05\0\x02\r\x12\x03\x10\x04\x1a\n\x0c\n\x05\x05\0\
    \x02\r\x01\x12\x03\x10\x04\x13\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x10\
    \x16\x19\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x11\x04\x1a\n\x0c\n\x05\x05\0\
    \x02\x0e\x01\x12\x03\x11\x04\x13\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\
    \x11\x16\x19\n\x0b\n\x04\x05\0\x02\x0f\x12\x03\x12\x04\x1a\n\x0c\n\x05\
    \x05\0\x02\x0f\x01\x12\x03\x12\x04\x13\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\
    \x03\x12\x16\x19\n\x0b\n\x04\x05\0\x02\x10\x12\x03\x13\x04\x17\n\x0c\n\
    \x05\x05\0\x02\x10\x01\x12\x03\x13\x04\x10\n\x0c\n\x05\x05\0\x02\x10\x02\
    \x12\x03\x13\x13\x16\n\x0b\n\x04\x05\0\x02\x11\x12\x03\x14\x04\x17\n\x0c\
    \n\x05\x05\0\x02\x11\x01\x12\x03\x14\x04\x10\n\x0c\n\x05\x05\0\x02\x11\
    \x02\x12\x03\x14\x13\x16\n\x0b\n\x04\x05\0\x02\x12\x12\x03\x15\x04\x1d\n\
    \x0c\n\x05\x05\0\x02\x12\x01\x12\x03\x15\x04\x16\n\x0c\n\x05\x05\0\x02\
    \x12\x02\x12\x03\x15\x19\x1c\n\x0b\n\x04\x05\0\x02\x13\x12\x03\x16\x04\
    \x1d\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\x16\x04\x16\n\x0c\n\x05\x05\0\
    \x02\x13\x02\x12\x03\x16\x19\x1c\n\x0b\n\x04\x05\0\x02\x14\x12\x03\x17\
    \x04\x1a\n\x0c\n\x05\x05\0\x02\x14\x01\x12\x03\x17\x04\x13\n\x0c\n\x05\
    \x05\0\x02\x14\x02\x12\x03\x17\x16\x19\n\x0b\n\x04\x05\0\x02\x15\x12\x03\
    \x18\x04\x1a\n\x0c\n\x05\x05\0\x02\x15\x01\x12\x03\x18\x04\x13\n\x0c\n\
    \x05\x05\0\x02\x15\x02\x12\x03\x18\x16\x19\n\x0b\n\x04\x05\0\x02\x16\x12\
    \x03\x19\x04+\n\x0c\n\x05\x05\0\x02\x16\x01\x12\x03\x19\x04$\n\x0c\n\x05\
    \x05\0\x02\x16\x02\x12\x03\x19'*\n\x0b\n\x04\x05\0\x02\x17\x12\x03\x1a\
    \x04\x20\n\x0c\n\x05\x05\0\x02\x17\x01\x12\x03\x1a\x04\x19\n\x0c\n\x05\
    \x05\0\x02\x17\x02\x12\x03\x1a\x1c\x1f\n\x0b\n\x04\x05\0\x02\x18\x12\x03\
    \x1b\x04\x1b\n\x0c\n\x05\x05\0\x02\x18\x01\x12\x03\x1b\x04\x14\n\x0c\n\
    \x05\x05\0\x02\x18\x02\x12\x03\x1b\x17\x1a\n\x0b\n\x04\x05\0\x02\x19\x12\
    \x03\x1c\x04\x1a\n\x0c\n\x05\x05\0\x02\x19\x01\x12\x03\x1c\x04\x13\n\x0c\
    \n\x05\x05\0\x02\x19\x02\x12\x03\x1c\x16\x19\n\x0b\n\x04\x05\0\x02\x1a\
    \x12\x03\x1d\x04-\n\x0c\n\x05\x05\0\x02\x1a\x01\x12\x03\x1d\x04&\n\x0c\n\
    \x05\x05\0\x02\x1a\x02\x12\x03\x1d),\n\x0b\n\x04\x05\0\x02\x1b\x12\x03\
    \x1e\x04\x1a\n\x0c\n\x05\x05\0\x02\x1b\x01\x12\x03\x1e\x04\x13\n\x0c\n\
    \x05\x05\0\x02\x1b\x02\x12\x03\x1e\x16\x19\n\x0b\n\x04\x05\0\x02\x1c\x12\
    \x03\x1f\x04\x18\n\x0c\n\x05\x05\0\x02\x1c\x01\x12\x03\x1f\x04\x11\n\x0c\
    \n\x05\x05\0\x02\x1c\x02\x12\x03\x1f\x14\x17\n\x0b\n\x04\x05\0\x02\x1d\
    \x12\x03\x20\x04\x17\n\x0c\n\x05\x05\0\x02\x1d\x01\x12\x03\x20\x04\x10\n\
    \x0c\n\x05\x05\0\x02\x1d\x02\x12\x03\x20\x13\x16b\x06proto3\
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
