use egui;

use crate::{
    MyApp,
    enums::{color_item_names::ColorItemNames, dragging::Dragging},
    models::{
        apollonius_pair::ApolloniusPair, app::InitialCircles, circle::Circle,
        homothetic_set::HomotheticSet, inverse_pole_set::InversePoleSet, segment::Segment,
        straightline::StraightLine,
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

                // Handle mouse dragging events
                let response_circles =
                    ui.allocate_rect(union_3_circles_clipping_rect, egui::Sense::click_and_drag());
                handle_circles_drag_events(
                    response_circles,
                    &mut app.initial_circles,
                    &mut app.is_dragging,
                );

                // Homothetic centers
                let mut sorted_circles = app.initial_circles.as_array();
                sorted_circles.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());
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
                // todo: do the same for appolonius pairs and refactor the drawing steps
                let inv_pole_sets: Vec<Option<InversePoleSet>> = {
                    let mut sets: Vec<Option<InversePoleSet>> = vec![];
                    let mut first = true;
                    let same_radius = app.initial_circles.same_radius();
                    println!("are they same radius {}", same_radius);

                    for line in homothetic_set.lines {
                        if first && same_radius {
                            sets.push(InversePoleSet::new_special(&sorted_circles, radical_center));
                            first = false;
                            continue;
                        }

                        sets.push(InversePoleSet::new(line, &sorted_circles, radical_center));
                    }
                    sets
                };

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

fn handle_circles_drag_events(
    response: egui::Response,
    initial_circles: &mut InitialCircles,
    is_dragging: &mut Dragging,
) {
    if response.drag_started() {
        let mut closest: Option<Dragging> = None;
        let mut min_distance = f32::INFINITY;

        if let Some(pos) = response.interact_pointer_pos() {
            for (dragging, circle) in [
                (Dragging::Circle1, &initial_circles.circle_1),
                (Dragging::Circle2, &initial_circles.circle_2),
                (Dragging::Circle3, &initial_circles.circle_3),
            ] {
                let dist = pos.distance(circle.center);
                if dist < circle.radius && dist < min_distance {
                    min_distance = dist;
                    closest = Some(dragging);
                }
            }
            if let Some(dragging) = closest {
                *is_dragging = dragging;
            }
        }
    }

    if response.dragged() {
        match is_dragging {
            Dragging::Circle1 => {
                initial_circles.circle_1.center += response.drag_delta();
            }
            Dragging::Circle2 => {
                initial_circles.circle_2.center += response.drag_delta();
            }
            Dragging::Circle3 => {
                initial_circles.circle_3.center += response.drag_delta();
            }
            Dragging::None => {
                for circle in initial_circles.as_array().iter_mut() {
                    circle.center += response.drag_delta();
                }
            }
        }
    }

    if response.drag_stopped() {
        *is_dragging = Dragging::None;
    }
}

fn get_apollonius_circles(
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

        circle_1 = services::calc::get_circle_3_points(
            &get_option_point(0, ord.0),
            &get_option_point(1, ord.1),
            &get_option_point(2, ord.2),
        );
        circle_2 = services::calc::get_circle_3_points(
            &get_option_point(0, 1 - ord.0),
            &get_option_point(1, 1 - ord.1),
            &get_option_point(2, 1 - ord.2),
        );
    }

    ApolloniusPair { circle_1, circle_2 }
}
