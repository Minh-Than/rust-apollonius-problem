use egui;

use crate::{
    MyApp,
    enums::{color_item_names::ColorItemNames, dragging::Dragging},
    models::{
        apollonius_pair::ApolloniusPair, circle::Circle, homothetic_centers::HomotheticCenters,
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

                let mut sorted_circles = vec![app.circle_1, app.circle_2, app.circle_3];
                sorted_circles.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());

                // Homothetic centers
                let homothetic_centers: HomotheticCenters = HomotheticCenters {
                    ex_12: services::calc::get_external_homothetic_center(
                        sorted_circles[0],
                        sorted_circles[1],
                    ),
                    in_12: services::calc::get_internal_homothetic_center(
                        sorted_circles[0],
                        sorted_circles[1],
                    ),
                    ex_23: services::calc::get_external_homothetic_center(
                        sorted_circles[1],
                        sorted_circles[2],
                    ),
                    in_23: services::calc::get_internal_homothetic_center(
                        sorted_circles[1],
                        sorted_circles[2],
                    ),
                    ex_31: services::calc::get_external_homothetic_center(
                        sorted_circles[2],
                        sorted_circles[0],
                    ),
                    in_31: services::calc::get_internal_homothetic_center(
                        sorted_circles[2],
                        sorted_circles[0],
                    ),
                };
                let line_1: Option<Segment> = Segment::get_any_valid_segment(vec![
                    homothetic_centers.ex_31,
                    homothetic_centers.ex_23,
                    homothetic_centers.ex_12,
                ]);
                let line_2: Option<Segment> = Segment::get_any_valid_segment(vec![
                    homothetic_centers.ex_12,
                    homothetic_centers.in_31,
                    homothetic_centers.in_23,
                ]);
                let line_3: Option<Segment> = Segment::get_any_valid_segment(vec![
                    homothetic_centers.ex_31,
                    homothetic_centers.in_23,
                    homothetic_centers.in_12,
                ]);
                let line_4: Option<Segment> = Segment::get_any_valid_segment(vec![
                    homothetic_centers.ex_23,
                    homothetic_centers.in_31,
                    homothetic_centers.in_12,
                ]);

                // Radical center
                let radical_axes: [StraightLine; 2] = [
                    services::calc::get_radical_axis(app.circle_1, app.circle_2),
                    services::calc::get_radical_axis(app.circle_2, app.circle_3),
                ];
                let radical_center: egui::Pos2 =
                    services::calc::find_intersection(&radical_axes[0], &radical_axes[1]);

                // Inverse poles sets
                let inv_pole_set_1: Option<InversePoleSet> =
                    InversePoleSet::new(line_1, &sorted_circles, radical_center);
                let inv_pole_set_2: Option<InversePoleSet> =
                    InversePoleSet::new(line_2, &sorted_circles, radical_center);
                let inv_pole_set_3: Option<InversePoleSet> =
                    InversePoleSet::new(line_3, &sorted_circles, radical_center);
                let inv_pole_set_4: Option<InversePoleSet> =
                    InversePoleSet::new(line_4, &sorted_circles, radical_center);

                // Apollonius pairs
                let apollonius_pair_1 = ApolloniusPair {
                    c1: services::calc::get_circle_3_points(
                        &inv_pole_set_1.as_ref().and_then(|set| set.s1).map(|s| s.0),
                        &inv_pole_set_1.as_ref().and_then(|set| set.s2).map(|s| s.0),
                        &inv_pole_set_1.as_ref().and_then(|set| set.s3).map(|s| s.0),
                    ),
                    c2: services::calc::get_circle_3_points(
                        &inv_pole_set_1.as_ref().and_then(|set| set.s1).map(|s| s.1),
                        &inv_pole_set_1.as_ref().and_then(|set| set.s2).map(|s| s.1),
                        &inv_pole_set_1.as_ref().and_then(|set| set.s3).map(|s| s.1),
                    ),
                };
                let apollonius_pair_2 = ApolloniusPair {
                    c1: services::calc::get_circle_3_points(
                        &inv_pole_set_2.as_ref().and_then(|set| set.s1).map(|s| s.0),
                        &inv_pole_set_2.as_ref().and_then(|set| set.s2).map(|s| s.0),
                        &inv_pole_set_2.as_ref().and_then(|set| set.s3).map(|s| s.1),
                    ),
                    c2: services::calc::get_circle_3_points(
                        &inv_pole_set_2.as_ref().and_then(|set| set.s1).map(|s| s.1),
                        &inv_pole_set_2.as_ref().and_then(|set| set.s2).map(|s| s.1),
                        &inv_pole_set_2.as_ref().and_then(|set| set.s3).map(|s| s.0),
                    ),
                };
                let apollonius_pair_3 = ApolloniusPair {
                    c1: services::calc::get_circle_3_points(
                        &inv_pole_set_3.as_ref().and_then(|set| set.s1).map(|s| s.1),
                        &inv_pole_set_3.as_ref().and_then(|set| set.s2).map(|s| s.0),
                        &inv_pole_set_3.as_ref().and_then(|set| set.s3).map(|s| s.1),
                    ),
                    c2: services::calc::get_circle_3_points(
                        &inv_pole_set_3.as_ref().and_then(|set| set.s1).map(|s| s.0),
                        &inv_pole_set_3.as_ref().and_then(|set| set.s2).map(|s| s.1),
                        &inv_pole_set_3.as_ref().and_then(|set| set.s3).map(|s| s.0),
                    ),
                };
                let apollonius_pair_4 = ApolloniusPair {
                    c1: services::calc::get_circle_3_points(
                        &inv_pole_set_4.as_ref().and_then(|set| set.s1).map(|s| s.0),
                        &inv_pole_set_4.as_ref().and_then(|set| set.s2).map(|s| s.1),
                        &inv_pole_set_4.as_ref().and_then(|set| set.s3).map(|s| s.1),
                    ),
                    c2: services::calc::get_circle_3_points(
                        &inv_pole_set_4.as_ref().and_then(|set| set.s1).map(|s| s.1),
                        &inv_pole_set_4.as_ref().and_then(|set| set.s2).map(|s| s.0),
                        &inv_pole_set_4.as_ref().and_then(|set| set.s3).map(|s| s.0),
                    ),
                };

                // Draw the shapes
                services::draw::draw_three_circles(
                    ui,
                    [app.circle_1, app.circle_2, app.circle_3],
                    &app.theme_mode,
                );
                services::draw::draw_homothetis_centers(
                    ui,
                    &homothetic_centers,
                    app.show_homothetic,
                    &app.theme_mode,
                );
                services::draw::draw_radical_center(
                    ui,
                    radical_center,
                    app.show_radical,
                    &app.theme_mode,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_inverse_poles,
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_inverse_poles,
                );
                services::draw::draw_connectors(
                    ui,
                    &inv_pole_set_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_connectors,
                );
                services::draw::draw_connectors(
                    ui,
                    &inv_pole_set_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_connectors,
                );
                services::draw::draw_connectors(
                    ui,
                    &inv_pole_set_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_connectors,
                );
                services::draw::draw_connectors(
                    ui,
                    &inv_pole_set_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_connectors,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                    app.show_apollonius_circle_1,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                    app.show_apollonius_circle_2,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                    app.show_apollonius_circle_3,
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                    app.show_apollonius_circle_4,
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
