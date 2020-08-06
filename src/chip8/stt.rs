pub struct Settings {
    // Where to load the ROM in memory
    pub rom_addr: u16,

    pub cpu_freq: u16,
    pub dt_freq: u16,
    pub st_freq: u16,

    pub rng_seed: u32,

    // Some games expect STRR (FX55) and LDRR (FX65) instructions to increment the I register.
    // When load_store_quirk is true, I will be incremented after STRR and LDRR.
    pub load_store_quirk: bool,
    // Some games expect SHR (8xy6) and SHL (8xyE) operations to always shift Vx and ignore Vy.
    // When shift_quirk is true, Vy will be set to Vx before executing the instruction.
    pub shift_quirk: bool,
    // Some games expect ADDA (Fx1E) to set the VF flag register when overflow occurs.
    // When address_overflow_quirk is true, the VF will be set.
    pub address_overflow_quirk: bool,

    // Wrap drawing on the screen vertically
    pub vertical_wrap: bool,

    pub mute: bool,

    pub print_rom: bool,
    pub print_opcodes: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            rom_addr: 0x200,
            cpu_freq: 700,
            dt_freq: 60,
            st_freq: 60,
            rng_seed: 0,
            load_store_quirk: false,
            shift_quirk: true,
            address_overflow_quirk: false,
            vertical_wrap: false,
            mute: false,
            print_rom: false,
            print_opcodes: false,
        }
    }
}