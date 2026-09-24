use eframe::egui;

pub(crate) enum NavigationIcon {
    Back,
    Forward,
    Reload,
}

pub(crate) fn navigation_button(
    ui: &mut egui::Ui,
    icon: NavigationIcon,
    enabled: bool,
) -> egui::Response {
    let response = ui.add_enabled(
        enabled,
        egui::Button::new("").min_size(egui::vec2(34.0, 32.0)),
    );
    let color = if enabled {
        ui.visuals().text_color()
    } else {
        ui.visuals().weak_text_color()
    };
    let stroke = egui::Stroke::new(1.8, color);
    let center = response.rect.center();
    let painter = ui.painter();

    match icon {
        NavigationIcon::Back => {
            let tip = egui::pos2(center.x - 5.0, center.y);
            painter.line_segment([egui::pos2(center.x + 5.0, center.y), tip], stroke);
            painter.line_segment([tip, egui::pos2(center.x + 1.0, center.y - 5.0)], stroke);
            painter.line_segment([tip, egui::pos2(center.x + 1.0, center.y + 5.0)], stroke);
        }
        NavigationIcon::Forward => {
            let tip = egui::pos2(center.x + 5.0, center.y);
            painter.line_segment([egui::pos2(center.x - 5.0, center.y), tip], stroke);
            painter.line_segment([tip, egui::pos2(center.x - 1.0, center.y - 5.0)], stroke);
            painter.line_segment([tip, egui::pos2(center.x - 1.0, center.y + 5.0)], stroke);
        }
        NavigationIcon::Reload => {
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                "↻",
                egui::FontId::proportional(20.0),
                color,
            );
        }
    }

    response
}
