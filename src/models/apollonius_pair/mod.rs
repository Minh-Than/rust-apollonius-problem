use crate::models::circle::Circle;

#[derive(Clone, Copy)]
pub struct ApolloniusPair {
    pub c1: Option<Circle>,
    pub c2: Option<Circle>,
}
impl IntoIterator for ApolloniusPair {
    type Item = Option<Circle>;
    type IntoIter = std::array::IntoIter<Option<Circle>, 2>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.c1, self.c2])
    }
}
