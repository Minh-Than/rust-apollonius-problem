use egui::Pos2;

#[derive(Clone, Copy)]
pub struct HomotheticCenters {
    pub ex_12: Option<Pos2>,
    pub in_12: Option<Pos2>,
    pub ex_23: Option<Pos2>,
    pub in_23: Option<Pos2>,
    pub ex_31: Option<Pos2>,
    pub in_31: Option<Pos2>,
}
impl IntoIterator for HomotheticCenters {
    type Item = Option<Pos2>;
    type IntoIter = std::array::IntoIter<Option<Pos2>, 6>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([
            self.ex_12, self.in_12, self.ex_23, self.in_23, self.ex_31, self.in_31,
        ])
    }
}
