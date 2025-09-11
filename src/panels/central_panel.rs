use egui;

use crate::{
    MyApp,
    enums::{color_item_names::ColorItemNames, dragging::Dragging},
    models::{
        apollonius_pair::ApolloniusPair, app::InitialCircles, homothetic_set::HomotheticSet,
        inverse_pole_set::InversePoleSet, straightline::StraightLine,
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
                let union_3_circles_clipping_rect = app
                    .initial_circles
                    .circle_1
                    .get_circle_clipping_rect()
                    .union(app.initial_circles.circle_2.get_circle_clipping_rect())
                    .union(app.initial_circles.circle_3.get_circle_clipping_rect());

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

                // TODO: think of a way to nicely refactor the drawing steps

                // let inv_pole_sets: Vec<Option<InversePoleSet>> = {
                //     let mut sets: Vec<Option<InversePoleSet>> = vec![];
                //     let mut first = true;
                //     let same_radius = app.initial_circles.same_radius();
                //     println!("are they same radius {}", same_radius);

                //     for line in homothetic_set.lines {
                //         if first && same_radius {
                //             sets.push(InversePoleSet::new_special(&sorted_circles, radical_center));
                //             first = false;
                //             continue;
                //         }

                //         sets.push(InversePoleSet::new(line, &sorted_circles, radical_center));
                //     }
                //     sets
                // };

                // Inverse poles sets
                let inv_pole_set_1 = if !app.initial_circles.same_radius() {
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
                let apollonius_pair_1 =
                    ApolloniusPair::get_apollonius_circles(&inv_pole_set_1, (0, 0, 0));
                let apollonius_pair_2 =
                    ApolloniusPair::get_apollonius_circles(&inv_pole_set_2, (0, 0, 1));
                let apollonius_pair_3 =
                    ApolloniusPair::get_apollonius_circles(&inv_pole_set_3, (1, 0, 1));
                let apollonius_pair_4 =
                    ApolloniusPair::get_apollonius_circles(&inv_pole_set_4, (0, 1, 1));

                // Draw the shapes
                services::draw::draw_three_circles(
                    ui,
                    app.initial_circles.as_array(),
                    services::theme::get_color(ColorItemNames::InitialCircles, &app.theme_mode),
                );
                services::draw::draw_homothetic_centers(
                    ui,
                    &homothetic_set,
                    app.display_options.show_homothetic,
                    services::theme::get_color(ColorItemNames::HomotheticCenters, &app.theme_mode),
                );
                services::draw::draw_radical_center(
                    ui,
                    radical_center,
                    app.display_options.show_radical,
                    services::theme::get_color(ColorItemNames::Radical, &app.theme_mode),
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_1,
                    app.display_options.show_inverse_poles,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_2,
                    app.display_options.show_inverse_poles,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_3,
                    app.display_options.show_inverse_poles,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                );
                services::draw::draw_inverse_poles(
                    ui,
                    &inv_pole_set_4,
                    app.display_options.show_inverse_poles,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_1,
                    app.display_options.show_apollonius_circle_1,
                    services::theme::get_color(ColorItemNames::InversePoles1, &app.theme_mode),
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_2,
                    app.display_options.show_apollonius_circle_2,
                    services::theme::get_color(ColorItemNames::InversePoles2, &app.theme_mode),
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_3,
                    app.display_options.show_apollonius_circle_3,
                    services::theme::get_color(ColorItemNames::InversePoles3, &app.theme_mode),
                );
                services::draw::draw_apollonius_circles_pair(
                    ui,
                    &apollonius_pair_4,
                    app.display_options.show_apollonius_circle_4,
                    services::theme::get_color(ColorItemNames::InversePoles4, &app.theme_mode),
                );
            })
            .response;

        if response.double_clicked() {
            app.scene_rect = inner_rect;
        }
    });
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
