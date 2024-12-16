# Tauri Plugin is-simulator

A simple Tauri plugin to check if the app is running in a simulator.

## Supported Platforms
- iOS
- Android

## Installation

### Rust

Run the following command in the src-tauri folder to add the plugin to the project’s dependencies in Cargo.toml:

```bash
cargo add tauri-plugin-is-simulator
```

Modify `lib.rs` to initialize the plugin:

```rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_is_simulator::init());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript Bindings

Install the JavaScript bindings:

```bash
npm install tauri-plugin-is-simulator
```

## Usage

```typescript
import { isSimulator } from 'tauri-plugin-is-simulator';

console.log(`Is simulator: ${await isSimulator()}`);
```