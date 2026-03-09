// tauri
use tauri::{AppHandle, Manager} ;

// sttk3
mod menu ;
use menu::create_menu ;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      let app_handle: AppHandle = app.handle().clone() ;

      // メニューを作る
      create_menu(&app_handle)? ;

      // ウインドウのタイトルを指定する
      let window_main: tauri::WebviewWindow = app.get_webview_window("main").unwrap() ;
      let package_info: &tauri::PackageInfo = app.package_info() ;
      window_main
        .set_title(&format!("{} {}", package_info.name, package_info.version))
        .expect("Failed to set window title");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application") ;
}
