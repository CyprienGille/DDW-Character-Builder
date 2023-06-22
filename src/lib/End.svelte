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
      class="w-1/3 py-2 bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-full"
      on:click={save_file}>Save my character</button
    >
    <div class="w-1/3" />
  </div>
  <div class="py-5" />
</main>
