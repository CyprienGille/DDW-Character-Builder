<script lang="ts">
  import Class from "./Class.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let picked_details = [];

  let level = "1";

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
    <div class="w-full px-6 py-5 mb-2 bg-white border rounded-md shadow">
      <h3 class="text-lg font-medium leading-6 text-gray-900">Pick a Class.</h3>
      <p class="max-w-2xl mt-1 text-sm text-gray-500">
        This will give you features, proficiencies, equipment and hit die.
      </p>
      <div class="flex flex-row font-semibold">
        <div class="pr-2">Level :</div>
        <form class=" border-2 rounded-lg">
          <select bind:value={level}>
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
            <option value="4">4</option>
            <option value="5">5</option>
            <option value="6">6</option>
            <option value="7">7</option>
            <option value="8">8</option>
            <option value="9">9</option>
            <option value="10">10</option>
            <option value="11">11 </option>
            <option value="12">12</option>
            <option value="13">13</option>
            <option value="14">14</option>
            <option value="15">15</option>
            <option value="16">16</option>
            <option value="17">17</option>
            <option value="18">18</option>
            <option value="19">19</option>
            <option value="20">20</option>
          </select>
        </form>
      </div>
    </div>
    <ul class="flex flex-col">
      {#await promise then classes}
        {#each classes as choice, i}
          <li class="flex flex-row mb-2 border-gray-400">
            <div
              class="transition duration-500 shadow ease-in-out transform hover:-translate-y-1 hover:shadow-lg select-none bg-white rounded-md items-center px-4 py-2"
            >
              <div class="flex-1 pl-1 md:mr-16">
                <div class="font-medium text-xl">
                  {choice.name}
                </div>
                <div class="text-sm text-gray-600">
                  {choice.source}
                </div>
              </div>
              {#if !picked_details.includes(i)}
                <button
                  on:click={() => toggle_details(i)}
                  class="mx-auto w-full text-blue-500 underline"
                >
                  Details
                </button>
              {:else}
                <button
                  on:click={() => toggle_details(i)}
                  class="mx-auto w-full text-blue-500 underline"
                >
                  Hide Details
                </button>
                <div class="px-3">
                  <Class {choice} {level} />
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
