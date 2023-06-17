<script lang="ts">
  import Landing from "./lib/Landing.svelte";
  import Menu from "./lib/Menu.svelte";
  import Lineage from "./lib/Lineage.svelte";
  import Class from "./lib/Class.svelte";
  import Scores from "./lib/Scores.svelte";
  import Description from "./lib/Description.svelte";
  import Equipment from "./lib/Equipment.svelte";
  import End from "./lib/End.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let current_page_id = 0;
</script>

<main>
  {#if current_page_id <= 0}
    <Landing />
  {:else if current_page_id == 1}
    <Menu />
  {:else if current_page_id == 2}
    <Lineage />
  {:else if current_page_id == 3}
    <Class />
  {:else if current_page_id == 4}
    <Scores />
  {:else if current_page_id == 5}
    <Description />
  {:else if current_page_id == 6}
    <Equipment />
  {:else if current_page_id >= 7}
    <End />
  {/if}
</main>
