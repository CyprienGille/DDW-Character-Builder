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
  import { c } from "./stores";

  let current_page_id = 0;

  function pressedNext() {
    current_page_id += 1;
  }
  function pressedPrevious() {
    current_page_id -= 1;
  }
</script>

<main>
  {#if current_page_id == 0}
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
  {:else if current_page_id == 7}
    <End />
  {/if}

  <div class="flex w-full">
    {#if current_page_id == 0}
      <button class="w-1/3 py-2 rounded font-bold bg-slate-200 text-slate-900"
        >Previous</button
      >
    {:else}
      <button
        class="w-1/3 py-2 rounded font-bold bg-blue-200 text-slate-900 hover:bg-teal-400"
        on:click={pressedPrevious}>Previous</button
      >
    {/if}
    <div class="w-1/3" />
    {#if current_page_id == 7}
      <button class="w-1/3 py-2 rounded font-bold bg-slate-200 text-slate-900"
        >Next</button
      >
    {:else}
      <button
        class="w-1/3 py-2 rounded font-bold bg-blue-200 text-slate-900 hover:bg-teal-400"
        on:click={pressedNext}>Next</button
      >
    {/if}
  </div>
</main>
