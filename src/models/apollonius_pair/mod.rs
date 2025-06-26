use crate::models::circle::Circle;

#[derive(Clone, Copy)]
pub struct ApolloniusPair {
    pub circle_1: Option<Circle>,
    pub circle_2: Option<Circle>,
}
impl IntoIterator for ApolloniusPair {
    type Item = Option<Circle>;
    type IntoIter = std::array::IntoIter<Option<Circle>, 2>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.circle_1, self.circle_2])
    }
}
