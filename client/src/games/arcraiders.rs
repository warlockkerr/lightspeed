use super::GameConfig;

pub struct ArcRaidersConfig;

impl GameConfig for ArcRaidersConfig {
    fn name(&self) -> &str {
        "ARC Raiders"
    }

    fn process_names(&self) -> &[&str] {
        &["PioneerGame.exe"]
    }

    fn ports(&self) -> (u16, u16) {
        (44153, 44153)
    }

    fn anti_cheat(&self) -> &str {
        "Denuvo Anti-Cheat"
    }

    fn uses_sdr(&self) -> bool {
        false
    }

    fn dynamic_server(&self) -> bool {
        true
    }

    fn typical_pps(&self) -> u32 {
        30
    }

    fn packet_size_range(&self) -> (usize, usize) {
        (64, 1400)
    }
}
