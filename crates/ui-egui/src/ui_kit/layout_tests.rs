use crate::ui_kit::test_support::*;
use crate::ui_kit::*;

use crate::{scenario, theme::Tokens};
use egui::Ui;

#[test]
fn theme_rail_cues_follow_hidden_content() {
    use crate::ui_kit::preferences::theme_rail_cues;
    assert_eq!(theme_rail_cues(0.0, 0.0), (false, false));
    assert_eq!(theme_rail_cues(0.0, 300.0), (false, true));
    assert_eq!(theme_rail_cues(150.0, 300.0), (true, true));
    assert_eq!(theme_rail_cues(300.0, 300.0), (true, false));
    assert_eq!(theme_rail_cues(0.25, 0.5), (false, false));
}

#[test]
fn theme_rail_arrows_scroll_one_card_and_hide_at_ends() {
    let ctx = crate::fonts::test_context();
    let t = Tokens::get();
    let width = t.form_label_width + t.space_24 + t.space_6;
    let draw = |ui: &mut Ui| {
        theme_rail(
            ui,
            |ui| {
                for i in 0..3 {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 80.0), egui::Sense::click());
                    scenario::publish_target(&format!("test:theme:{i}"), rect);
                }
            },
            |_| {},
        );
    };
    for _ in 0..3 {
        frame(&ctx, vec![], draw);
    }
    assert!(scenario::rect_of("prefs:theme_rail:Scroll left").is_none());
    let start = scenario::rect_of("test:theme:0").unwrap().left();
    let right = scenario::handle_at("prefs:theme_rail:Scroll right").unwrap();
    assert!((right.y - scenario::rect_of("test:theme:0").unwrap().center().y).abs() < 1.0);
    frame(&ctx, pointer(right, true), draw);
    frame(&ctx, pointer(right, false), draw);
    frame(&ctx, vec![], draw);
    let after = scenario::rect_of("test:theme:0").unwrap().left();
    assert!((start - after - width - t.space_8).abs() < 1.0);
    assert!(scenario::rect_of("prefs:theme_rail:Scroll left").is_some());
    for _ in 0..4 {
        if let Some(pos) = scenario::handle_at("prefs:theme_rail:Scroll right") {
            frame(&ctx, pointer(pos, true), draw);
            frame(&ctx, pointer(pos, false), draw);
            frame(&ctx, vec![], draw);
        }
    }
    assert!(scenario::rect_of("prefs:theme_rail:Scroll right").is_none());
    for _ in 0..4 {
        if let Some(pos) = scenario::handle_at("prefs:theme_rail:Scroll left") {
            frame(&ctx, pointer(pos, true), draw);
            frame(&ctx, pointer(pos, false), draw);
            frame(&ctx, vec![], draw);
        }
    }
    assert!(scenario::rect_of("prefs:theme_rail:Scroll left").is_none());
}

#[test]
fn theme_rail_fades_cover_only_overflowing_edges_with_the_page_background() {
    for dark in [false, true] {
        let t = Tokens::themed(dark, crate::theme::ColorVision::Off);
        let _restore = crate::theme::scoped_tokens(t);
        for count in [1, 6] {
            let ctx = crate::fonts::test_context();
            let draw = |events| {
                scenario::clear_handles();
                crate::test_support::frame(
                    &ctx,
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(320.0, 400.0))),
                        events,
                        ..Default::default()
                    },
                    |ui| {
                        ui.set_width(280.0);
                        theme_rail(
                            ui,
                            |ui| {
                                for _ in 0..count {
                                    let (rect, _) =
                                        ui.allocate_exact_size(egui::vec2(t.form_label_width + t.space_24 + t.space_6, 80.0), egui::Sense::hover());
                                    ui.painter().rect_filled(rect, 0.0, t.accent);
                                }
                            },
                            |_| {},
                        );
                    },
                )
            };
            let fades = |output: &egui::FullOutput| {
                output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Mesh(mesh)
                            if mesh.vertices.len() == 4 && mesh.vertices.iter().any(|v| v.color == egui::Color32::TRANSPARENT) =>
                        {
                            Some((shape.clip_rect, mesh.clone()))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            };
            for _ in 0..3 {
                draw(vec![]);
            }
            let output = draw(vec![]);
            let initial = fades(&output);
            if count == 1 {
                assert!(initial.is_empty(), "a rail that fits must have no fade");
                continue;
            }
            assert_eq!(initial.len(), 1, "only the right edge initially overflows");
            let right = scenario::handle_at("prefs:theme_rail:Scroll right").unwrap();
            draw(pointer(right, true));
            draw(pointer(right, false));
            let output = draw(vec![]);
            let middle = fades(&output);
            assert_eq!(middle.len(), 2, "both edges overflow after scrolling one card");
            for (direction, (clip, mesh)) in [-1.0, 1.0].into_iter().zip(middle) {
                assert_eq!(mesh.vertices.len(), 4);
                assert_eq!(mesh.indices.len(), 6);
                let outer = if direction < 0.0 { clip.left() } else { clip.right() };
                for vertex in mesh.vertices.iter().take(2) {
                    assert_eq!(vertex.pos.x, outer);
                    assert_eq!(vertex.color, t.dialog_body);
                    assert_eq!(vertex.color.a(), 255, "the edge must fully hide cards in light and dark themes");
                }
                for vertex in mesh.vertices.iter().skip(2) {
                    assert_eq!(vertex.color, egui::Color32::TRANSPARENT);
                    assert!((vertex.pos.x - (outer - direction * (t.space_24 * 2.0 + t.space_8))).abs() < 0.01);
                }
                assert_eq!(mesh.calc_bounds().height(), clip.height());
            }
        }
    }
}

#[test]
fn theme_rail_chevrons_have_no_idle_box_and_show_hover_and_focus() {
    let ctx = crate::fonts::test_context();
    let t = Tokens::get();
    let rect = egui::Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(t.control_height, t.control_height));
    let draw = |events| {
        crate::test_support::frame(&ctx, egui::RawInput { events, focused: true, ..Default::default() }, |ui| {
            crate::ui_kit::preferences::theme_rail_arrow(ui, rect, 1.0, "Scroll right");
        })
    };
    let idle = draw(vec![]);
    assert!(!idle.shapes.iter().any(|s| matches!(s.shape, egui::Shape::Rect(_))));
    assert!(!idle.shapes.iter().any(|s| matches!(s.shape, egui::Shape::Circle(_))), "no backing until hovered");
    let hover = draw(vec![egui::Event::PointerMoved(rect.center())]);
    assert_eq!(hover.platform_output.cursor_icon, egui::CursorIcon::PointingHand);
    assert!(hover.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Circle(c) if c.fill == t.hover && c.radius == rect.width() / 2.0)));
    draw(pointer(rect.center(), true));
    draw(pointer(rect.center(), false));
    let focus = draw(vec![egui::Event::PointerMoved(egui::pos2(200.0, 200.0))]);
    assert!(
        !focus.shapes.iter().any(|s| matches!(&s.shape, egui::Shape::Rect(r) if r.stroke.color == t.form_primary)),
        "a mouse click leaves no outline"
    );
}

#[test]
#[ignore = "set SOLIDCRAFT_THEME_RAIL_EVIDENCE and run explicitly for Preferences screenshots"]
fn theme_rail_preferences_evidence() {
    use crate::ui_kit::checks::tests::{raster, textures_update};
    let dir = std::path::PathBuf::from(std::env::var("SOLIDCRAFT_THEME_RAIL_EVIDENCE").unwrap());
    std::fs::create_dir_all(&dir).unwrap();
    for dark in [false, true] {
        let _restore = crate::theme::scoped_tokens(Tokens::themed(dark, crate::theme::ColorVision::Off));
        let ctx = crate::fonts::test_context();
        ctx.set_visuals(if dark { egui::Visuals::dark() } else { egui::Visuals::light() });
        let mut app = crate::test_support::sample_app();
        crate::prefs::open_at(&mut app, 1);
        let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1200.0, 1000.0));
        let mut textures = std::collections::HashMap::new();
        for frame in 0..8 {
            let events = match frame {
                3 => vec![egui::Event::PointerMoved(scenario::handle_at("prefs:theme_rail:Scroll right").unwrap())],
                4 | 5 => pointer(scenario::handle_at("prefs:theme_rail:Scroll right").unwrap(), frame == 4),
                _ => vec![egui::Event::PointerMoved(egui::pos2(0.0, 0.0))],
            };
            scenario::clear_handles();
            let mut output = ctx.run_ui(egui::RawInput { screen_rect: Some(bounds), events, focused: true, ..Default::default() }, |ui| {
                ui.painter().rect_filled(bounds, 0.0, Tokens::get().panel);
                crate::prefs::show(&mut app, ui.ctx());
            });
            textures_update(&mut textures, &output.textures_delta);
            output.textures_delta.clear();
            if let Some(state) = match frame {
                2 => Some("start"),
                3 => Some("hover"),
                7 => Some("middle"),
                _ => None,
            } {
                raster(&ctx, output.shapes, output.pixels_per_point, bounds, &textures)
                    .save(dir.join(format!("preferences-{}-{state}.png", if dark { "dark" } else { "light" })))
                    .unwrap();
            }
        }
    }
}

#[test]
fn row_height_and_label_column_are_fixed_for_long_labels_and_expressions() {
    let ctx = crate::fonts::test_context();
    let mut value = "width / 2 + 10 mm".to_string();
    for _ in 0..3 {
        frame(&ctx, vec![], |ui| {
            row(ui, "A long parameter label that must elide", |ui| {
                field(ui, &mut value);
            });
            row(ui, "Radius", |ui| {
                field(ui, &mut value);
            });
        });
    }
    let first = scenario::rect_of("dialog:row:A long parameter label that must elide").unwrap();
    let second = scenario::rect_of("dialog:row:Radius").unwrap();
    let label = scenario::rect_of("dialog:label:Radius").unwrap();
    let control = scenario::rect_of("dialog:control:Radius").unwrap();
    assert_eq!(first.height(), Tokens::get().control_height);
    assert_eq!(second.height(), Tokens::get().control_height);
    // The design's grid: 28 pt controls with a 5 pt row gap and a 70 pt label column (wider only for long labels).
    assert!(
        (second.top() - first.bottom() - Tokens::get().form_gap_y).abs() < 0.01,
        "Plex Mono fields must retain the row gap: {first:?}, {second:?}"
    );
    // The long label widens the column for every row, up to 1.6 times the design's width.
    assert_eq!(label.width(), Tokens::get().form_label_width * 1.6);
    assert_eq!(scenario::rect_of("dialog:label:A long parameter label that must elide").unwrap().width(), label.width());
    assert!(control.left() > label.right());
    assert!(second.contains_rect(control));
    assert_eq!(value, "width / 2 + 10 mm");
}

#[test]
fn section_toggles_persist_per_dialog() {
    let ctx = crate::fonts::test_context();
    let mut a = 0;
    let mut b = 0;
    let mut draw = |ui: &mut Ui| {
        ui.push_id("fillet", |ui| {
            section(ui, "opts", "Options", false, |_| a += 1);
        });
        ui.push_id("hole", |ui| {
            section(ui, "opts", "Hole options", false, |_| b += 1);
        });
    };
    frame(&ctx, vec![], &mut draw);
    let pos = scenario::handle_at("dialog:section:Options").unwrap();
    frame(&ctx, pointer(pos, true), &mut draw);
    frame(&ctx, pointer(pos, false), &mut draw);
    frame(&ctx, vec![], &mut draw);
    assert!(a >= 2);
    assert_eq!(b, 0);
    let before = a;
    frame(&ctx, pointer(pos, true), |ui| {
        ui.push_id("fillet", |ui| {
            section(ui, "opts", "Options", false, |_| a += 1);
        });
    });
    frame(&ctx, pointer(pos, false), |ui| {
        ui.push_id("fillet", |ui| {
            section(ui, "opts", "Options", false, |_| a += 1);
        });
    });
    assert_eq!(a, before + 1);
}

#[test]
fn short_labels_keep_the_designs_label_column_and_long_ones_widen_it_for_every_row() {
    fn widths(labels: [&str; 2]) -> (f32, f32) {
        let ctx = egui::Context::default();
        crate::fonts::setup_fonts(&ctx);
        let mut value = String::new();
        for _ in 0..3 {
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                ui.set_width(300.0);
                scope(ui, |ui| {
                    for label in labels {
                        row(ui, label, |ui| {
                            field(ui, &mut value);
                        });
                    }
                });
            });
            output.textures_delta.clear();
        }
        (
            scenario::rect_of(&format!("dialog:label:{}", labels[0])).unwrap().width(),
            scenario::rect_of(&format!("dialog:label:{}", labels[1])).unwrap().width(),
        )
    }
    let t = Tokens::get();
    let (a, b) = widths(["Edges", "Radius"]);
    assert_eq!((a, b), (t.form_label_width, t.form_label_width));
    let (a, b) = widths(["Edges", "Counterbore depth"]);
    assert_eq!(a, b, "one column per form");
    assert!(a > t.form_label_width && a <= t.form_label_width * 1.6, "{a}");
}
