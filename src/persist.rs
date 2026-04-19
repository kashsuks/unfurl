use crate::theme::Theme;

const KEY: &str = "unfurl_theme";

pub fn save_theme(theme: Theme) {
    let s = theme.label();

    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = config_path()  {
            let _ = std::fs::write(path, s);
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_storage() {
            let _ = storage.set_item(KEY, s);
        }
    }
}

pub fn load_theme() -> Theme {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = config_path() {
            if let Ok(contents) = std::fs::read_to_string(path) {
                return parse_theme(contents.trim());
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(storage) = web_storage() {
            if let Ok(Some(val)) = storage.get_item(KEY) {
                return parse_theme(&val);
            }
        }
    }

    Theme::default()
}

fn parse_theme(s: &str) -> Theme {
    match s {
        "Light" => Theme::Light,
        "Solarized" => Theme::Solarized,
        "Dracula" => Theme::Dracula,
        "Nord" => Theme::Nord,
        _ => Theme::Dark,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn config_path() -> Option<std::path::PathBuf> {
    // ~/.config/unfurl/theme (mac/linux)
    // %APPDATA%\unfurl\theme (windows)
    let base = dirs::config_dir()?;
    let dir = base.join("unfurl");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("theme"))
}

#[cfg(target_arch = "wasm32")]
fn web_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?;
}
