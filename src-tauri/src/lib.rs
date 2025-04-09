#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use specta::Type;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};
use fast_qr::convert::{svg::SvgBuilder, Builder as ImageBuilder, Shape};
use fast_qr::qr::QRBuilder;

#[derive(Serialize, Deserialize, Type)]
struct GreetResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Type)]
struct QrResponse {
    url: String,
    qr_code: String,
    base64_image: String,
}

#[tauri::command]
#[specta::specta]
fn greet(name: String) -> GreetResponse {
    GreetResponse {
        message: format!("Hello, {}!", name),
    }
}

#[tauri::command]
#[specta::specta]
fn qr(url: String) -> QrResponse {
    let qrcode = QRBuilder::new(url.clone())
        .build()
        .unwrap();

    let svg_content = SvgBuilder::default()
        .shape(Shape::RoundedSquare).image_size(100.0).to_str(&qrcode);

    let base64_img = format!(
        "data:image/svg+xml;base64,{}",
        base64::encode(svg_content)
    );

    QrResponse {
        url,
        qr_code: qrcode.to_str(),
        base64_image: base64_img
    }
}

pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![greet, qr]);
    #[cfg(debug_assertions)] // Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export TypeScript bindings");
    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}