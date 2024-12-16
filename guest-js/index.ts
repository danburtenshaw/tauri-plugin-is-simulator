import { invoke } from "@tauri-apps/api/core";

export async function isSimulator(): Promise<boolean> {
  return await invoke<{ value: boolean }>(
    "plugin:is-simulator|is_simulator",
  ).then((r) => r.value);
}
