use egui;

use crate::{
    MyApp,
    enums::{color_item_names::ColorItemNames, dragging::Dragging},
    models::{
        apollonius_pair::ApolloniusPair, circle::Circle, homothetic_set::HomotheticSet,
        inverse_pole_set::InversePoleSet, segment::Segment, straightline::StraightLine,
    },
    services,
};

pub fn get(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let scene = egui::Scene::new().zoom_range(0.1..=50.0);

        let mut inner_rect = egui::Rect::NAN;
        let response = scene
            .show(ui, &mut app.scene_rect, |ui: &mut egui::Ui| {
                inner_rect = ui.min_rect();

                // Clipping rect bounding all 3 circles for handing indiviual circle dragging
                let union_3_circles_clipping_rect =
                    get_circle_clipping_rect(app.initial_circles.circle_1)
                        .union(get_circle_clipping_rect(app.initial_circles.circle_2))
                        .union(get_circle_clipping_rect(app.initial_circles.circle_3));

                let response_circles =
                    ui.allocate_rect(union_3_circles_clipping_rect, egui::Sense::click_and_drag());
                if response_circles.drag_started() {
                    let mut closest: Option<Dragging> = None;
                    let mut min_distance = f32::INFINITY;

                    if let Some(pos) = response_circles.interact_pointer_pos() {
                        for (dragging, circle) in [
                            (Dragging::Circle1, &app.initial_circles.circle_1),
                            (Dragging::Circle2, &app.initial_circles.circle_2),
                            (Dragging::Circle3, &app.initial_circles.circle_3),
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
                }

                match app.is_dragging {
                    Dragging::Circle1 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.initial_circles.circle_1,
                    ),
                    Dragging::Circle2 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.initial_circles.circle_2,
                    ),
                    Dragging::Circle3 => handle_circle_drag(
                        response_circles,
                        &mut app.is_dragging,
                        &mut app.initial_circles.circle_3,
                    ),
                    Dragging::None => {
                        if response_circles.dragged() {
                            for circle in app.initial_circles.as_array().iter_mut() {
                                circle.center += response_circles.drag_delta();
                            }
                        }
                        if response_circles.drag_stopped() {
                            app.is_dragging = Dragging::None;
                        }
                    }
                }

                let mut sorted_circles = app.initial_circles.as_array();
                sorted_circles.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());

                // Homothetic centers
                let homothetic_set: HomotheticSet = HomotheticSet::new(&sorted_circles);

                // Radical center
                let radical_axes: [StraightLine; 2] = [
                    services::calc::get_radical_axis(
                        app.initial_circles.circle_1,
                        app.initial_circles.circle_2,
                    ),
                    services::calc::get_radical_axis(
                        app.initial_circles.circle_2,
                        app.initial_circles.circle_3,
                    ),
                ];
                let radical_center: egui::Pos2 =
                    services::calc::find_intersection(&radical_axes[0], &radical_axes[1]);

                // Inverse poles sets
                let inv_pole_set_1 = if !(app.initial_circles.circle_1.radius
                    == app.initial_circles.circle_2.radius
                    && app.initial_circles.circle_2.radius == app.initial_circles.circle_3.radius)
                {
                    InversePoleSet::new(homothetic_set.lines[0], &sorted_circles, radical_center)
                } else {
                    InversePoleSet::new_special(&sorted_circles, radical_center)
                };
                let inv_pole_set_2 =
                    InversePoleSet::new(homothetic_set.lines[1], &sorted_circles, radical_center);
                let inv_pole_set_3 =
                    InversePoleSet::new(homothetic_set.lines[2], &sorted_circles, radical_center);
                let inv_pole_set_4 =
                    InversePoleSet::new(homothetic_set.lines[3], &sorted_circles, radical_center);

                // Apollonius pairs
                let apollonius_pair_1 = get_apollonius_circles(&inv_pole_set_1, (0, 0, 0));
                let apollonius_pair_2 = get_apollonius_circles(&inv_pole_set_2, (0, 0, 1));
                let apollonius_pair_3 = get_apollonius_circles(&inv_pole_set_3, (1, 0, 1));
                let apollonius_pair_4 = get_apollonius_circles(&inv_pole_set_4, (0, 1, 1));

                // Draw the shapes
                services::draw::draw_three_circles(
                    ui,
                    app.initial_circles.as_array(),
                    &app.theme_mode,
                );
                services::draw::draw_homothetic_centers(
                    ui,
                    &homothetic_set,
                    app.display_options.show_homothetic,
                    &app.theme_mode,
                );
                services::draw::draw_radical_center(
                    ui,
                    radical_center,
                    app.display_options.show_radical,
                    &app.theme_mode,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.display_options.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.display_options.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.display_options.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.display_options.show_inverse_poles,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.display_options.show_apollonius_circle_1,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.display_options.show_apollonius_circle_2,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.display_options.show_apollonius_circle_3,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.display_options.show_apollonius_circle_4,
                );
            })
            .response;

        if response.double_clicked() {
            app.scene_rect = inner_rect;
        }
    });
}

fn get_circle_clipping_rect(c: Circle) -> egui::Rect {
    egui::Rect {
        min: egui::Pos2 {
            x: c.center.x - c.radius,
            y: c.center.y - c.radius,
        },
        max: egui::Pos2 {
            x: c.center.x + c.radius,
            y: c.center.y + c.radius,
        },
    }
}

fn handle_circle_drag(response: egui::Response, is_dragging: &mut Dragging, c: &mut Circle) {
    if response.dragged() {
        c.center += response.drag_delta();
    }
    if response.drag_stopped() {
        *is_dragging = Dragging::None;
    }
}

fn get_apollonius_circles(
    inverse_pole_set: &Option<InversePoleSet>,
    ord: (i8, i8, i8),
) -> ApolloniusPair {
    match inverse_pole_set {
        Some(set) => {
            fn get_segment_point(num: i8, s: &Segment) -> egui::Pos2 {
                if num == 0 { s.0 } else { s.1 }
            }
            let get_option_point = |idx: usize, ord: i8| -> Option<egui::Pos2> {
                set.get_segment(idx)
                    .as_ref()
                    .map(|s| get_segment_point(ord, s))
            };

            ApolloniusPair {
                c1: services::calc::get_circle_3_points(
                    &get_option_point(0, ord.0),
                    &get_option_point(1, ord.1),
                    &get_option_point(2, ord.2),
                ),
                c2: services::calc::get_circle_3_points(
                    &get_option_point(0, 1 - ord.0),
                    &get_option_point(1, 1 - ord.1),
                    &get_option_point(2, 1 - ord.2),
                ),
            }
        }
        None => ApolloniusPair { c1: None, c2: None },
    }
}
