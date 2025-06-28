use crate::models::circle::Circle;
use crate::models::inverse_pole_set::InversePoleSet;
use crate::models::segment::Segment;

#[derive(Clone, Copy)]
pub struct ApolloniusPair {
    pub circle_1: Option<Circle>,
    pub circle_2: Option<Circle>,
}
impl ApolloniusPair {
    pub fn get_apollonius_circles(
        inverse_pole_set: &Option<InversePoleSet>,
        ord: (i8, i8, i8),
    ) -> ApolloniusPair {
        let mut circle_1: Option<Circle> = None;
        let mut circle_2: Option<Circle> = None;

        if let Some(set) = inverse_pole_set {
            fn get_segment_point(num: i8, s: &Segment) -> egui::Pos2 {
                if num == 0 { s.0 } else { s.1 }
            }
            let get_option_point = |idx: usize, ord: i8| -> Option<egui::Pos2> {
                set.get_segment(idx)
                    .as_ref()
                    .map(|s| get_segment_point(ord, s))
            };

            circle_1 = Circle::get_circle_3_points(
                &get_option_point(0, ord.0),
                &get_option_point(1, ord.1),
                &get_option_point(2, ord.2),
            );
            circle_2 = Circle::get_circle_3_points(
                &get_option_point(0, 1 - ord.0),
                &get_option_point(1, 1 - ord.1),
                &get_option_point(2, 1 - ord.2),
            );
        }

        ApolloniusPair { circle_1, circle_2 }
    }
}
impl IntoIterator for ApolloniusPair {
    type Item = Option<Circle>;
    type IntoIter = std::array::IntoIter<Option<Circle>, 2>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.circle_1, self.circle_2])
    }
}
