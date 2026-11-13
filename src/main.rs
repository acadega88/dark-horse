mod browser_app;
mod icons;
mod user_data;

use eframe::egui;

use browser_app::BrowserApp;

fn app_icon() -> egui::IconData {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/dark-horse-icon.png"))
        .expect("application icon must be a valid PNG");

    #[cfg(target_os = "macos")]
    {
        let mut background = vec![false; (icon.width * icon.height) as usize];
        let mut pending = vec![0_usize];
        while let Some(index) = pending.pop() {
            if background[index] {
                continue;
            }
            let source = index * 4;
            if icon.rgba[source..source + 3]
                .iter()
                .any(|channel| *channel < 220)
            {
                continue;
            }
            background[index] = true;
            let column = index % icon.width as usize;
            let row = index / icon.width as usize;
            if column > 0 {
                pending.push(index - 1);
            }
            if column + 1 < icon.width as usize {
                pending.push(index + 1);
            }
            if row > 0 {
                pending.push(index - icon.width as usize);
            }
            if row + 1 < icon.height as usize {
                pending.push(index + icon.width as usize);
            }
        }

        let margin = icon.width.min(icon.height) / 10;
        let content_width = icon.width - 2 * margin;
        let content_height = icon.height - 2 * margin;
        let radius = content_width.min(content_height) as f32 * 0.22;
        let mut pixels = vec![0; icon.rgba.len()];

        for y in 0..content_height {
            for x in 0..content_width {
                let source_x = x * icon.width / content_width;
                let source_y = y * icon.height / content_height;
                let source = ((source_y * icon.width + source_x) * 4) as usize;
                let target = (((y + margin) * icon.width + x + margin) * 4) as usize;
                let point_x = x as f32 + 0.5;
                let point_y = y as f32 + 0.5;
                let center_x = point_x.clamp(radius, content_width as f32 - radius);
                let center_y = point_y.clamp(radius, content_height as f32 - radius);
                let distance = ((point_x - center_x).powi(2) + (point_y - center_y).powi(2)).sqrt();
                let coverage = (radius + 0.5 - distance).clamp(0.0, 1.0);
                pixels[target..target + 4].copy_from_slice(&icon.rgba[source..source + 4]);
                if background[source / 4] {
                    let brightness = (u16::from(icon.rgba[source])
                        + u16::from(icon.rgba[source + 1])
                        + u16::from(icon.rgba[source + 2]))
                        / 3;
                    let gray = (brightness * 220 / 255) as u8;
                    pixels[target..target + 3].fill(gray);
                }
                pixels[target + 3] = (f32::from(pixels[target + 3]) * coverage).round() as u8;
            }
        }

        return egui::IconData {
            rgba: pixels,
            ..icon
        };
    }

    #[cfg(not(target_os = "macos"))]
    icon
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1800.0, 1100.0])
            .with_icon(app_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Dark Horse Browser",
        options,
        Box::new(|_creation_context| Ok(Box::<BrowserApp>::default())),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn app_icon_is_valid() {
        let icon = super::app_icon();
        assert_eq!((icon.width, icon.height), (1024, 1024));
        #[cfg(target_os = "macos")]
        {
            assert_eq!(icon.rgba[3], 0);
            let top_center = (icon.width / 2 * 4 + 3) as usize;
            assert_eq!(icon.rgba[top_center], 0);
            let background = ((icon.height / 2 * icon.width + icon.width / 8) * 4) as usize;
            assert_eq!(icon.rgba[background], icon.rgba[background + 1]);
            assert_eq!(icon.rgba[background + 1], icon.rgba[background + 2]);
            assert!((190..=220).contains(&icon.rgba[background]));
            let center = ((icon.height / 2 * icon.width + icon.width / 2) * 4 + 3) as usize;
            assert_eq!(icon.rgba[center], 255);
        }
    }
}
