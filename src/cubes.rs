#[derive(Default, Clone, Copy)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub fn is_possible(limit: CubeSet, grab: CubeSet) -> bool {
    !(limit.red < grab.red || limit.green < grab.green || limit.blue < grab.blue)
}
