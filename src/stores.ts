import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";

export const c = writable<any>(await invoke("get_default_character"));

export const lineage_choices = writable<any>(await invoke("get_empty_choices"));