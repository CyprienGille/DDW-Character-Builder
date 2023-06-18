<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { c } from "../stores";

  async function save_file() {
    invoke("save_character_to_file", { c: $c });
  }
</script>

<main>
  <div class="text-center text-2xl py-20">
    Congratulations, you just finished building your character! Now, click on
    the button below to save it as a DnDWithin-compatible .json file!
  </div>
  <div class="text-center flex">
    <div class="w-1/3" />
    <button
      class="w-1/3 py-2 rounded-md font-bold text-xl bg-blue-200 text-slate-900 hover:bg-teal-400"
      on:click={save_file}>Save my character</button
    >
    <div class="w-1/3" />
  </div>
  <div class="py-5" />
</main>
