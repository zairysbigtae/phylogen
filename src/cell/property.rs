pub const BASE_RESPIRATION_RATE: f32 = 0.5;

#[derive(Default)]
pub struct CellProperties {
    pub sharpness: f32,

    pub poison: f32,
    pub venom: f32,

    pub metabolism_rate: f32,
    pub respiration_rate: f32,
}
