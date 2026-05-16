unsafe extern "C" {
    pub fn cgevent_type_raw_value(index: u32) -> u32;
    pub fn cgevent_type_mask_bit(r#type: u32) -> u64;
    pub fn cgevent_mask_for_all_events() -> u64;
}
