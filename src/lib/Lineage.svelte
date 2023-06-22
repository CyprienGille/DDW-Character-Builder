<script lang="ts">
  import { each } from "svelte/internal";
  import { lineage_choices } from "../stores";
  export let choice;

  let tool_prof_choice = { name: "tool_prof_choice", picked: "none" };
  let draconic_choice = { name: "draconic_ancestry", picked: "none" };
  let language_choice = { name: "language_choice", picked: "none" };

  let picking_ok = false;

  function check_picking_ok() {
    let tp_picked = true; // default: we don't care
    let da_picked = true;
    let lan_picked = true;
    if (choice.tool_prof_choices != undefined) {
      tp_picked = tool_prof_choice.picked != "none";
    }
    if (choice.draconic_ancestries != undefined) {
      da_picked = draconic_choice.picked != "none";
    }
    if (choice.language_choices != undefined) {
      lan_picked = language_choice.picked != "none";
    }

    picking_ok = tp_picked && da_picked && lan_picked;
    return "";
  }

  function add_sign(num) {
    let res = "";
    if (num >= 0) {
      res = "+ " + num;
    } else {
      res = "-" + num;
    }
    return res;
  }

  function insert_between(list, elem) {
    let res = "";
    let l = list.length;
    if (l > 1) {
      for (var i = 0; i < l - 1; i++) {
        res += list[i] + elem;
      }
    }
    res += list[l - 1];
    return res;
  }

  function pick_this_lineage() {
    $lineage_choices = [];
    $lineage_choices.push({ name: "lineage", picked: choice.name });
    $lineage_choices.push(tool_prof_choice);
    $lineage_choices.push(draconic_choice);
    $lineage_choices.push(language_choice);
    console.log($lineage_choices);
  }
</script>

<main>
  {check_picking_ok()}
  <div class="flex flex-row">
    <div class="flex flex-col mx-4">
      {#if choice.scores.str != 0}
        <div class="my-1">Str {add_sign(choice.scores.str)}</div>
      {/if}
      {#if choice.scores.dex != 0}
        <div class="my-1">Dex {add_sign(choice.scores.dex)}</div>
      {/if}
      {#if choice.scores.con != 0}
        <div class="my-1">Con {add_sign(choice.scores.con)}</div>
      {/if}
      {#if choice.scores.int != 0}
        <div class="my-1">Int {add_sign(choice.scores.int)}</div>
      {/if}
      {#if choice.scores.wis != 0}
        <div class="my-1">Wis {add_sign(choice.scores.wis)}</div>
      {/if}
      {#if choice.scores.cha != 0}
        <div class="my-1">Cha {add_sign(choice.scores.cha)}</div>
      {/if}
      {#if choice.features != undefined}
        {#each choice.features as feature}
          <div class="my-1">{feature}</div>
        {/each}
      {/if}
      {#if choice.speeds.walk != undefined}
        <div class="my-1">
          You have a {choice.speeds.walk} ft. walking speed.
        </div>
      {/if}
      {#if choice.speeds.swim != undefined}
        <div class="my-1">
          You have a {choice.speeds.swim} ft. swimming speed.
        </div>
      {/if}
      {#if choice.speeds.fly != undefined}
        <div class="my-1">
          You have a {choice.speeds.fly} ft. flying speed.
        </div>
      {/if}
      {#if choice.speeds.burrow != undefined}
        <div class="my-1">
          You have a {choice.speeds.burrow} ft. burrowing speed.
        </div>
      {/if}
      <div class="my-1">Your size is {choice.size}.</div>
      {#if choice.st_mods != undefined}
        {#each choice.st_mods as st_mod}
          <div class="my-1">
            Your saving throws have {st_mod}.
          </div>
        {/each}
      {/if}
      {#if choice.darkvision != undefined}
        You have darkvision up to {choice.darkvision} ft.
      {/if}

      {#if choice.languages != undefined}
        <div class="my-1">
          You know {insert_between(choice.languages, " and ")}.
        </div>
      {/if}
      {#if choice.weapon_profs != undefined}
        <div class="my-1">
          You are proficient with the {insert_between(
            choice.weapon_profs,
            ", the "
          )}.
        </div>
      {/if}
      {#if choice.dmg_res != undefined}
        {#each choice.dmg_res as dmg_type}
          <div class="my-1">
            You have resistance to {dmg_type} damage.
          </div>
        {/each}
      {/if}

      {#if choice.tool_profs != undefined}
        <div class="my-1">
          You are proficient with {insert_between(choice.tool_profs, " and ")}.
        </div>
      {/if}

      {#if choice.tool_prof_choices != undefined}
        <div class="my-1 flex flex-row">
          You are proficient with
          {#if tool_prof_choice.picked == "none"}
            <form class="border-2 rounded-md border-red-100">
              <select bind:value={tool_prof_choice.picked}>
                {#each choice.tool_prof_choices as option}
                  <option value={option}>
                    {option}.
                  </option>
                {/each}
              </select>
            </form>
          {:else}
            {check_picking_ok()}
            <form class="border-2 rounded-md border-green-200">
              <select bind:value={tool_prof_choice.picked}>
                {#each choice.tool_prof_choices as option}
                  <option value={option}>
                    {option}.
                  </option>
                {/each}
              </select>
            </form>
          {/if}
        </div>
      {/if}
      {#if choice.language_choices != undefined}
        <div class="my-1 flex flex-row">
          Pick an extra language:
          {#if language_choice.picked == "none"}
            <form class="border-2 rounded-md border-red-100">
              <select bind:value={language_choice.picked}>
                {#each choice.language_choices as option}
                  <option value={option}>
                    {option}
                  </option>
                {/each}
              </select>
            </form>
          {:else}
            {check_picking_ok()}
            <form class="border-2 rounded-md border-green-200">
              <select bind:value={language_choice.picked}>
                {#each choice.language_choices as option}
                  <option value={option}>
                    {option}
                  </option>
                {/each}
              </select>
            </form>
          {/if}
          .
        </div>
      {/if}

      {#if choice.draconic_ancestries != undefined}
        <div class="my-1">
          <div class="flex flex-row">
            Pick a draconic ancestry:
            <div class="px-2" />
            {#if draconic_choice.picked == "none"}
              <form class="border-2 rounded-md border-red-100">
                <select bind:value={draconic_choice.picked}>
                  {#each choice.draconic_ancestries as option}
                    <option value={option.dragon}>
                      {option.dragon}
                    </option>
                  {/each}
                </select>
              </form>
            {:else}
              {check_picking_ok()}
              <form class="border-2 rounded-md border-green-200">
                <select bind:value={draconic_choice.picked}>
                  {#each choice.draconic_ancestries as option}
                    <option value={option.dragon}>
                      {option.dragon}
                    </option>
                  {/each}
                </select>
              </form>
            {/if}
            . The details for each ancestry are shown in the table below.
          </div>
          <table class="table p-4 bg-white rounded-lg shadow w-full">
            <thead>
              <tr>
                <th
                  class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
                >
                  Dragon
                </th>
                <th
                  class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
                >
                  Damage Type
                </th>
                <th
                  class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
                >
                  Breath Weapon
                </th>
              </tr>
            </thead>
            <tbody>
              {#each choice.draconic_ancestries as da}
                <tr class="text-gray-700">
                  <td class="border-b-2 p-2 dark:border-dark-5">
                    {da.dragon}
                  </td>
                  <td class="border-b-2 p-2 dark:border-dark-5">
                    {da.dmg_type}
                  </td>
                  <td class="border-b-2 p-2 dark:border-dark-5">
                    {da.breath_weapon}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      {#if choice.age_fluff != undefined}
        <div class="my-1">{choice.age_fluff}</div>
      {/if}
      {#if choice.size_fluff != undefined}
        <div class="my-1">{choice.size_fluff}</div>
      {/if}
    </div>

    <div class="my-auto">
      {#if !picking_ok}
        <button
          disabled
          class="py-2 px-2 flex justify-center items-center bg-slate-500 text-white w-full text-center text-base font-semibold shadow-md rounded-full"
          on:click={() => pick_this_lineage()}
          >Pick Lineage
        </button>
      {:else}
        <button
          class="py-2 px-2 flex justify-center items-center bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-full"
          on:click={() => pick_this_lineage()}
          >Pick Lineage
        </button>
      {/if}
    </div>
  </div>
</main>
