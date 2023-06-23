<script lang="ts">
  import Class from "./Class.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let picked_details = [];

  function toggle_details(i) {
    if (picked_details.includes(i)) {
      picked_details.splice(picked_details.indexOf(i), 1);
    } else {
      picked_details.push(i);
    }
    picked_details = picked_details;
  }

  let promise: Promise<any> = invoke("get_empty_options");

  function open_choices_file() {
    promise = invoke("open_options_file");
  }
</script>

<main>
  <div
    class="container flex flex-col items-center justify-center w-full mx-auto"
  >
    <div
      class="w-full px-4 py-5 mb-2 bg-white border rounded-md shadow sm:px-6"
    >
      <h3 class="text-lg font-medium leading-6 text-gray-900">Pick a Class.</h3>
      <p class="max-w-2xl mt-1 text-sm text-gray-500">
        This will give you features, proficiencies, equipment and hit die.
      </p>
    </div>
    <ul class="flex flex-col">
      {#await promise then lineages}
        {#each lineages as choice, i}
          <li class="flex flex-row mb-2 border-gray-400">
            <div
              class="transition duration-500 shadow ease-in-out transform hover:-translate-y-1 hover:shadow-lg select-none bg-white rounded-md flex flex-1 items-center px-4 py-2"
            >
              <div class="flex-1 pl-1 md:mr-16">
                <div class="font-medium text-lg">
                  {choice.name}
                </div>
                <div class="text-sm text-gray-600">
                  {choice.source}
                </div>
              </div>
              {#if !picked_details.includes(i)}
                <button
                  on:click={() => toggle_details(i)}
                  class="flex justify-end w-24 text-right text-blue-500 underline"
                >
                  Details
                </button>
              {:else}
                <button
                  on:click={() => toggle_details(i)}
                  class="flex justify-end w-24 text-right text-blue-500 underline"
                >
                  Hide Details
                </button>
                <div class="px-3">
                  <Class {choice} />
                </div>
              {/if}
            </div>
          </li>
        {/each}
      {/await}
    </ul>
  </div>
  <div class="flex">
    <div class="w-1/3" />
    <div class="w-1/3">
      <button
        class="py-4 w-full rounded-md font-bold bg-blue-200 text-slate-900 hover:bg-teal-400"
        on:click={open_choices_file}>Open Classes File</button
      >
    </div>
  </div>
</main>
