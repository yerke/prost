use prost::Message;
use prost_types::FileDescriptorProto;

fn sint32(i: i32) -> i32 {
    let mut i1 = (i >> 1);
    if i & 1 == 1 {
        i1 = !i1
    }
    i1
}

fn sint64(i: i64) -> i64 {
    let mut i1 = (i >> 1);
    if i & 1 == 1 {
        i1 = !i1
    }
    i1
}

pub mod ext {
    include!(concat!(env!("OUT_DIR"), "/ext.rs"));
}

#[test]
fn decode_custom_options() {
    let fdp: FileDescriptorProto =
        FileDescriptorProto::decode(ext::EXT_FILE_DESCRIPTOR_PROTO).expect("failed to get fdp");
    let options = fdp.message_type[1].options.as_ref().unwrap();
    assert_eq!(10.5, ext::ext_ext::double_field.get(options).unwrap().unwrap());
    assert_eq!(-8.5, ext::ext_ext::float_field.get(options).unwrap().unwrap());
    assert_eq!(-3, ext::ext_ext::int32_field.get(options).unwrap().unwrap());
    assert_eq!(-13, ext::ext_ext::int64_field.get(options).unwrap().unwrap());
    assert_eq!(
        -4,
        sint32(ext::ext_ext::sint32_field.get(options).unwrap().unwrap())
    );
    assert_eq!(
        -14,
        sint64(ext::ext_ext::sint64_field.get(options).unwrap().unwrap())
    );
    assert_eq!(5, ext::ext_ext::uint32_field.get(options).unwrap().unwrap());
    assert_eq!(15, ext::ext_ext::uint64_field.get(options).unwrap().unwrap());
    assert_eq!(6, ext::ext_ext::fixed32_field.get(options).unwrap().unwrap());
    assert_eq!(16, ext::ext_ext::fixed64_field.get(options).unwrap().unwrap());
    assert_eq!(7, ext::ext_ext::sfixed32_field.get(options).unwrap().unwrap());
    assert_eq!(-17, ext::ext_ext::sfixed64_field.get(options).unwrap().unwrap());
    assert_eq!(true, ext::ext_ext::bool_field.get(options).unwrap().unwrap());
    assert_eq!(
        "Hello world!",
        ext::ext_ext::string_field.get(options).unwrap().unwrap()
    );
    assert_eq!(
        ext::TestEnum::Red as i32,
        ext::ext_ext::enum_field.get(options).unwrap().unwrap()
    );
    assert_eq!(
        Some(22),
        ext::ext_ext::message_field.get(options).unwrap().unwrap().n
    );
}
