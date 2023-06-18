import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";

export const c = writable<any>(await invoke("get_default"))