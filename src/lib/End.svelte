<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { c, lineage_choices } from "../stores";

  async function finish_building() {
    console.log($lineage_choices);
    invoke("finish_building", { c: $c, lineageChoices: $lineage_choices });
  }
</script>

<main>
  <div class="text-center text-2xl py-20">
    Congratulations, you just finished creating {$c.name}! Now, click on the
    button below to save your work as a DnDWithin-compatible .json file!
  </div>
  <div class="text-center flex">
    <div class="w-1/3" />
    <button
      class="w-1/3 py-2 bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-full"
      on:click={finish_building}>Save my character</button
    >
    <div class="w-1/3" />
  </div>
  <div class="py-5" />
</main>
