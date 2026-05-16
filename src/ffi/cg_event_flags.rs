unsafe extern "C" {
    pub fn cgevent_flags_raw_value(index: u32) -> u64;
    pub fn cgevent_flags_known_mask() -> u64;
}
