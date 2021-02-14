use super::unit::Unit;

#[derive(Debug, Clone)]
pub struct Val {
    pub num: f64,
    pub unit: Unit,
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}
