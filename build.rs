extern crate build_const;

include!("src/util.rs");

const ALLOW_UNREADABLE_LITERAL_LINT: &str = "#[allow(clippy::unreadable_literal)]";

#[allow(non_snake_case)]
fn create_constants() {
    let mut crc16 = build_const::ConstWriter::for_build("crc16_constants")
        .unwrap()
        .finish_dependencies();

    let X25: u16 = 0x1021;
    crc16.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc16.add_value("X25", "u16", X25);
    crc16.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc16.add_array("X25_TABLE", "u16", &make_table_crc16(X25, true));

    let USB: u16 = 0x8005;
    crc16.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc16.add_value("USB", "u16", USB);
    crc16.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc16.add_array("USB_TABLE", "u16", &make_table_crc16(USB, true));

    crc16.finish();

    let mut crc32 = build_const::ConstWriter::for_build("crc32_constants")
        .unwrap()
        .finish_dependencies();

    let CASTAGNOLI: u32 = 0x1EDC_6F41;
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_value("CASTAGNOLI", "u32", CASTAGNOLI);
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_array(
        "CASTAGNOLI_TABLE",
        "u32",
        &make_table_crc32(CASTAGNOLI, true),
    );

    let IEEE: u32 = 0x04C1_1DB7;
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_value("IEEE", "u32", IEEE);
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_array("IEEE_TABLE", "u32", &make_table_crc32(IEEE, true));

    let KOOPMAN: u32 = 0x741B_8CD7;
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_value("KOOPMAN", "u32", KOOPMAN);
    crc32.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc32.add_array("KOOPMAN_TABLE", "u32", &make_table_crc32(KOOPMAN, true));

    crc32.finish();

    let mut crc64 = build_const::ConstWriter::for_build("crc64_constants")
        .unwrap()
        .finish_dependencies();

    let ECMA: u64 = 0x42F0_E1EB_A9EA_3693;
    crc64.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc64.add_value("ECMA", "u64", ECMA);
    crc64.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc64.add_array("ECMA_TABLE", "u64", &make_table_crc64(ECMA, true));

    let ISO: u64 = 0x0000_0000_0000_001B;
    crc64.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc64.add_value("ISO", "u64", ISO);
    crc64.add_raw(ALLOW_UNREADABLE_LITERAL_LINT);
    crc64.add_array("ISO_TABLE", "u64", &make_table_crc64(ISO, true));

    crc64.finish();
}

fn main() {
    create_constants();
}
