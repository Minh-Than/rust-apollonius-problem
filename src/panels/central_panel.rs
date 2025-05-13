use egui::{Color32, Context, Pos2, Rect, Response, Shape, Stroke, Ui, epaint::CircleShape};

use crate::{
    MyApp,
    calc::{self, get_circle_3_points},
    enums::{color_item_names::ColorItemNames, dragging::Dragging, theme_mode::ThemeMode},
    models::{segment::Segment, straightline::StraightLine},
    theme_handler,
};

#[derive(Clone)]
struct HomotheticCenters {
    ex_1: Pos2,
    in_1: Pos2,
    ex_2: Pos2,
    in_2: Pos2,
    ex_3: Pos2,
    in_3: Pos2,
}
impl IntoIterator for HomotheticCenters {
    type Item = Pos2;
    type IntoIter = std::array::IntoIter<Pos2, 6>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([
            self.ex_1, self.in_1, self.ex_2, self.in_2, self.ex_3, self.in_3,
        ])
    }
}

#[derive(Clone)]
struct InversePoleSet {
    p1: Pos2,
    p2: Pos2,
    p3: Pos2,
    s1: Segment,
    s2: Segment,
    s3: Segment,
}
impl IntoIterator for InversePoleSet {
    type Item = Pos2;
    type IntoIter = std::array::IntoIter<Pos2, 3>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.p1, self.p2, self.p3])
    }
}

#[derive(Clone, Copy)]
struct ApolloniusCirclesPair {
    c1: Option<CircleShape>,
    c2: Option<CircleShape>,
}
impl IntoIterator for ApolloniusCirclesPair {
    type Item = Option<CircleShape>;

    type IntoIter = std::array::IntoIter<Option<CircleShape>, 2>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.c1, self.c2])
    }
}

pub fn central_panel(app: &mut MyApp, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let scene = egui::Scene::new().zoom_range(0.1..=50.0);

        let mut inner_rect = egui::Rect::NAN;
        let response = scene
            .show(ui, &mut app.scene_rect, |ui: &mut Ui| {
                inner_rect = ui.min_rect();

                // Clipping rect bounding all 3 circles for handing indiviual circle dragging
                // TODO: make circles scalable

                let union_3_circles_clipping_rect = get_circle_clipping_rect(app.circle_1)
                    .union(get_circle_clipping_rect(app.circle_2))
                    .union(get_circle_clipping_rect(app.circle_3));

                let response_circles =
                    ui.allocate_rect(union_3_circles_clipping_rect, egui::Sense::click_and_drag());

                if response_circles.drag_started() {
                    let mut closest: Option<Dragging> = None;
                    let mut min_distance = f32::INFINITY;

                    match response_circles.interact_pointer_pos() {
                        Some(pos) => {
                            for (dragging, circle) in [
                                (Dragging::Circle1, &app.circle_1),
                                (Dragging::Circle2, &app.circle_2),
                                (Dragging::Circle3, &app.circle_3),
                            ] {
                                let dist = pos.distance(circle.center);
                                if dist < circle.radius && dist < min_distance {
                                    min_distance = dist;
                                    closest = Some(dragging);
                                }
                            }
                            if let Some(dragging) = closest {
                                app.is_dragging = dragging;
                            }
                        }
                        _ => (),
                    }
                }

                match app.is_dragging {
                    Dragging::Circle1 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.circle_1,
                    ),
                    Dragging::Circle2 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.circle_2,
                    ),
                    Dragging::Circle3 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.circle_3,
                    ),
                    Dragging::None => {
                        if response_circles.dragged() {
                            app.circle_1.center += response_circles.drag_delta();
                            app.circle_2.center += response_circles.drag_delta();
                            app.circle_3.center += response_circles.drag_delta();
                        }
                        if response_circles.drag_stopped() {
                            app.is_dragging = Dragging::None;
                        }
                    }
                }

                // Homothetic centers
                let homothetic_centers: HomotheticCenters = HomotheticCenters {
                    ex_1: calc::get_external_homothetic_center(app.circle_1, app.circle_2),
                    in_1: calc::get_internal_homothetic_center(app.circle_1, app.circle_2),
                    ex_2: calc::get_external_homothetic_center(app.circle_2, app.circle_3),
                    in_2: calc::get_internal_homothetic_center(app.circle_2, app.circle_3),
                    ex_3: calc::get_external_homothetic_center(app.circle_3, app.circle_1),
                    in_3: calc::get_internal_homothetic_center(app.circle_3, app.circle_1),
                };
                let line_1: Segment = Segment(homothetic_centers.ex_1, homothetic_centers.ex_3);
                let line_2: Segment = Segment(homothetic_centers.ex_1, homothetic_centers.in_2);
                let line_3: Segment = Segment(homothetic_centers.ex_3, homothetic_centers.in_1);
                let line_4: Segment = Segment(homothetic_centers.ex_2, homothetic_centers.in_1);

                // Radical center
                let radical_axes: [StraightLine; 2] = [
                    calc::get_radical_axis(app.circle_1, app.circle_2),
                    calc::get_radical_axis(app.circle_2, app.circle_3),
                ];
                let radical_center: Pos2 =
                    calc::find_intersection(&radical_axes[0], &radical_axes[1]);

                // Inverse poles sets
                let inv_pole_set_1: InversePoleSet = InversePoleSet {
                    p1: calc::get_inverse_pole(&line_1, app.circle_1),
                    p2: calc::get_inverse_pole(&line_1, app.circle_2),
                    p3: calc::get_inverse_pole(&line_1, app.circle_3),
                    s1: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_1, app.circle_1),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_1,
                    ),
                    s2: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_1, app.circle_2),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_2,
                    ),
                    s3: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_1, app.circle_3),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_3,
                    ),
                };
                let inv_pole_set_2: InversePoleSet = InversePoleSet {
                    p1: calc::get_inverse_pole(&line_2, app.circle_1),
                    p2: calc::get_inverse_pole(&line_2, app.circle_2),
                    p3: calc::get_inverse_pole(&line_2, app.circle_3),
                    s1: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_2, app.circle_1),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_1,
                    ),
                    s2: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_2, app.circle_2),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_2,
                    ),
                    s3: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_2, app.circle_3),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_3,
                    ),
                };
                let inv_pole_set_3: InversePoleSet = InversePoleSet {
                    p1: calc::get_inverse_pole(&line_3, app.circle_1),
                    p2: calc::get_inverse_pole(&line_3, app.circle_2),
                    p3: calc::get_inverse_pole(&line_3, app.circle_3),
                    s1: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_3, app.circle_1),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_1,
                    ),
                    s2: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_3, app.circle_2),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_2,
                    ),
                    s3: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_3, app.circle_3),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_3,
                    ),
                };
                let inv_pole_set_4: InversePoleSet = InversePoleSet {
                    p1: calc::get_inverse_pole(&line_4, app.circle_1),
                    p2: calc::get_inverse_pole(&line_4, app.circle_2),
                    p3: calc::get_inverse_pole(&line_4, app.circle_3),
                    s1: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_4, app.circle_1),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_1,
                    ),
                    s2: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_4, app.circle_2),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_2,
                    ),
                    s3: calc::get_circle_straight_line_intersection(
                        &Segment(
                            calc::get_inverse_pole(&line_4, app.circle_3),
                            radical_center,
                        )
                        .as_straight_line(),
                        app.circle_3,
                    ),
                };

                let apollonius_pair_1: ApolloniusCirclesPair = ApolloniusCirclesPair {
                    c1: get_circle_3_points(
                        inv_pole_set_1.s1.0,
                        inv_pole_set_1.s2.0,
                        inv_pole_set_1.s3.0,
                    ),
                    c2: get_circle_3_points(
                        inv_pole_set_1.s1.1,
                        inv_pole_set_1.s2.1,
                        inv_pole_set_1.s3.1,
                    ),
                };
                let apollonius_pair_2: ApolloniusCirclesPair = ApolloniusCirclesPair {
                    c1: get_circle_3_points(
                        inv_pole_set_2.s1.0,
                        inv_pole_set_2.s2.0,
                        inv_pole_set_2.s3.1,
                    ),
                    c2: get_circle_3_points(
                        inv_pole_set_2.s1.1,
                        inv_pole_set_2.s2.1,
                        inv_pole_set_2.s3.0,
                    ),
                };
                let apollonius_pair_3: ApolloniusCirclesPair = ApolloniusCirclesPair {
                    c1: get_circle_3_points(
                        inv_pole_set_3.s1.1,
                        inv_pole_set_3.s2.0,
                        inv_pole_set_3.s3.1,
                    ),
                    c2: get_circle_3_points(
                        inv_pole_set_3.s1.0,
                        inv_pole_set_3.s2.1,
                        inv_pole_set_3.s3.0,
                    ),
                };
                let apollonius_pair_4: ApolloniusCirclesPair = ApolloniusCirclesPair {
                    c1: get_circle_3_points(
                        inv_pole_set_4.s1.0,
                        inv_pole_set_4.s2.1,
                        inv_pole_set_4.s3.1,
                    ),
                    c2: get_circle_3_points(
                        inv_pole_set_4.s1.1,
                        inv_pole_set_4.s2.0,
                        inv_pole_set_4.s3.0,
                    ),
                };

                // Drawing
                draw_three_circles(
                    ui,
                    [app.circle_1, app.circle_2, app.circle_3],
                    &app.theme_mode,
                );
                draw_homothetis_centers(
                    ui,
                    &homothetic_centers,
                    app.show_homothetic,
                    &app.theme_mode,
                );
                draw_radical_center(ui, radical_center, app.show_radical, &app.theme_mode);
                draw_inverse_poles(
                    ui,
                    &inv_pole_set_1,
                    theme_handler::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_inverse_poles,
                );
                draw_inverse_poles(
                    ui,
                    &inv_pole_set_2,
                    theme_handler::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_inverse_poles,
                );
                draw_inverse_poles(
                    ui,
                    &inv_pole_set_3,
                    theme_handler::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_inverse_poles,
                );
                draw_inverse_poles(
                    ui,
                    &inv_pole_set_4,
                    theme_handler::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_inverse_poles,
                );

                draw_connectors(
                    ui,
                    &inv_pole_set_1,
                    theme_handler::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_connectors,
                );
                draw_connectors(
                    ui,
                    &inv_pole_set_2,
                    theme_handler::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_connectors,
                );
                draw_connectors(
                    ui,
                    &inv_pole_set_3,
                    theme_handler::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_connectors,
                );
                draw_connectors(
                    ui,
                    &inv_pole_set_4,
                    theme_handler::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_connectors,
                );

                draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_1,
                    theme_handler::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_apollonius_circle_1,
                );
                draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_2,
                    theme_handler::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_apollonius_circle_2,
                );
                draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_3,
                    theme_handler::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_apollonius_circle_3,
                );
                draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_4,
                    theme_handler::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_apollonius_circle_4,
                );
            })
            .response;

        if response.double_clicked() {
            app.scene_rect = inner_rect;
        }
    });
}

fn get_circle_clipping_rect(c: CircleShape) -> Rect {
    Rect {
        min: Pos2 {
            x: c.center.x - c.radius,
            y: c.center.y - c.radius,
        },
        max: Pos2 {
            x: c.center.x + c.radius,
            y: c.center.y + c.radius,
        },
    }
}

fn handle_circle_drag(response: Response, is_dragging: &mut Dragging, c: &mut CircleShape) {
    if response.dragged() {
        c.center += response.drag_delta();
    }
    if response.drag_stopped() {
        *is_dragging = Dragging::None;
    }
}

fn draw_three_circles(ui: &mut Ui, circles: [CircleShape; 3], theme_mode: &ThemeMode) {
    for circle in circles {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center: circle.center,
            radius: circle.radius,
            fill: theme_handler::get_color(ColorItemNames::InitialCircles, theme_mode),
            stroke: Stroke::NONE,
        }));
    }
}

fn draw_homothetis_centers(
    ui: &mut Ui,
    homothetic_centers: &HomotheticCenters,
    condition: bool,
    theme_mode: &ThemeMode,
) {
    if !condition {
        return;
    }
    for center in homothetic_centers.clone().into_iter() {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center,
            radius: 2.0,
            fill: theme_handler::get_color(ColorItemNames::HomotheticCenters, theme_mode),
            stroke: Stroke::NONE,
        }));
    }
}

fn draw_radical_center(ui: &mut Ui, radical_center: Pos2, condition: bool, theme_mode: &ThemeMode) {
    if !condition {
        return;
    }
    ui.painter().add(egui::Shape::Circle(CircleShape {
        center: radical_center,
        radius: 4.0,
        fill: theme_handler::get_color(ColorItemNames::Radical, theme_mode),
        stroke: Stroke::NONE,
    }));
}

fn draw_inverse_poles(ui: &mut Ui, poles_set: &InversePoleSet, fill: Color32, condition: bool) {
    if !condition {
        return;
    }
    for center in poles_set.clone().into_iter() {
        ui.painter().add(egui::Shape::Circle(CircleShape {
            center,
            radius: 2.0,
            fill,
            stroke: Stroke::NONE,
        }));
    }
}

fn draw_connectors(
    ui: &mut Ui,
    poles_set: &InversePoleSet,
    stroke_color: Color32,
    condition: bool,
) {
    if !condition {
        return;
    }

    for segment in [poles_set.s1, poles_set.s2, poles_set.s3] {
        ui.painter().add(Shape::LineSegment {
            points: [segment.0, segment.1],
            stroke: Stroke {
                width: 1.0,
                color: stroke_color,
            },
        });
    }
}

fn draw_apollonius_circles_pair(
    ui: &mut Ui,
    circle_pair: &ApolloniusCirclesPair,
    stroke: Color32,
    condition: bool,
) {
    if !condition {
        return;
    }

    for circle in circle_pair.into_iter() {
        if let Some(c) = circle {
            ui.painter().add(egui::Shape::Circle(CircleShape {
                center: c.center,
                radius: c.radius,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(1.0, stroke),
            }));
        }
    }
}
