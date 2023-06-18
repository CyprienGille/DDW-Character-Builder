<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { c } from "../stores";

  let hide_details = true;

  function toggle_details() {
    hide_details = !hide_details;
  }

  let promise: Promise<any> = invoke("get_empty_choices");

  function open_choices_file() {
    promise = invoke("open_choices_file");
  }

  function pick_choice(choice) {
    invoke("handle_choice", { choice: choice, chara: $c }).then(
      (chara) => ($c = chara)
    );
    console.log($c);
  }
</script>

<main>
  <div
    class="container flex flex-col items-center justify-center w-full mx-auto"
  >
    <div
      class="w-full px-4 py-5 mb-2 bg-white border rounded-md shadow sm:px-6"
    >
      <h3 class="text-lg font-medium leading-6 text-gray-900">
        Pick a Lineage/Race.
      </h3>
      <p class="max-w-2xl mt-1 text-sm text-gray-500">
        This will decide your size, speed, and give you racial traits, languages
        and/or tool proficiencies.
      </p>
    </div>
    <ul class="flex flex-col">
      {#await promise then lineages}
        {#each lineages as choice}
          <li class="flex flex-row mb-2 border-gray-400">
            <div
              class="transition duration-500 shadow ease-in-out transform hover:-translate-y-1 hover:shadow-lg select-none bg-white rounded-md flex flex-1 items-center p-4"
            >
              <div class="flex-1 pl-1 md:mr-16">
                <div class="font-medium text-lg">{choice.name}</div>
                <div class="text-sm text-gray-600">
                  {choice.desc}
                </div>
              </div>
              {#if hide_details}
                <button
                  on:click={toggle_details}
                  class="flex justify-end w-24 text-right text-blue-500 underline"
                >
                  Details
                </button>
              {:else}
                <button
                  on:click={toggle_details}
                  class="flex justify-end w-24 text-right text-blue-500 underline"
                >
                  Hide Details
                </button>
                <div class="pl-3">
                  {#each choice.effects as effect}
                    <div>
                      {effect.desc}
                    </div>
                  {/each}
                </div>
                <div>
                  <button on:click={() => pick_choice(choice)}
                    >Pick Lineage</button
                  >
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
        on:click={open_choices_file}>Open Lineages File</button
      >
    </div>
  </div>
</main>
