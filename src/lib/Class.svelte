<script lang="ts">
  import { class_choices, asis } from "../stores";

  export let choice;
  export let level;

  let available_asis = [0, 1, 2, 3, 4];
  let taken_asis = [];
  let current_asi_n = -1;

  let skill_prof_choice_1 = { name: "skill_prof_choice_1", picked: "none" };
  let skill_prof_choice_2 = { name: "skill_prof_choice_2", picked: "none" };
  let asi_0 = { name: "asi_0", picked: "none" };
  let asi_0_1 = { name: "asi_0_1", picked: "none" };
  let asi_1 = { name: "asi_1", picked: "none" };
  let asi_1_1 = { name: "asi_1_1", picked: "none" };
  let asi_2 = { name: "asi_2", picked: "none" };
  let asi_2_1 = { name: "asi_2_1", picked: "none" };
  let asi_3 = { name: "asi_3", picked: "none" };
  let asi_3_1 = { name: "asi_3_1", picked: "none" };
  let asi_4 = { name: "asi_4", picked: "none" };
  let asi_4_1 = { name: "asi_4_1", picked: "none" };
  let asis_array = [
    [asi_0, asi_0_1],
    [asi_1, asi_1_1],
    [asi_2, asi_2_1],
    [asi_3, asi_3_1],
    [asi_4, asi_4_1],
  ];

  let picking_ok = false;

  function check_picking_ok() {
    // default: we don't care
    let sk1_picked = true;
    let sk2_picked = true;
    let asi_0_picked = true;
    let asi_0_1_picked = true;
    let asi_1_picked = true;
    let asi_1_1_picked = true;
    let asi_2_picked = true;
    let asi_2_1_picked = true;
    let asi_3_picked = true;
    let asi_3_1_picked = true;
    let asi_4_picked = true;
    let asi_4_1_picked = true;

    if (choice.profs.skill_choices != undefined) {
      sk1_picked = skill_prof_choice_1.picked != "none";
      if (choice.profs.skill_choices.num > 1) {
        sk2_picked = skill_prof_choice_2.picked != "none";
      }
    }
    picking_ok = sk1_picked && sk2_picked;
    return "";
  }

  function add_sign(num: number) {
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
      res += list[l - 1];
    }
    return res;
  }

  function reserve_one_asi() {
    current_asi_n = available_asis.shift();
    taken_asis.push(current_asi_n);
    taken_asis.sort();
    return "";
  }

  function free_one_asi() {
    if (taken_asis.includes(current_asi_n)) {
      available_asis.push(taken_asis.shift());
    }
    available_asis.sort();
    return "";
  }

  function get_current_n_asi() {
    return current_asi_n;
  }

  function check_exclu(other, picked, exclu) {
    if (picked == other.picked && exclu) {
      other.picked = "none";
    }
    return "";
  }

  function pick_this_class() {
    $class_choices = [];
    $class_choices.push({ name: "class", picked: choice });
    $class_choices.push({ name: "level", picked: level });
    $class_choices.push(skill_prof_choice_1);
    $class_choices.push(skill_prof_choice_2);
    // scroll to the bottom of the page
    window.scrollBy({
      left: 0,
      top: document.body.scrollHeight,
      behavior: "smooth",
    });
  }
</script>

<main>
  {check_picking_ok()}
  <div>
    <div class="mx-4">
      <div class="font-bold italic">Hit die: {choice.hit_dice}</div>
      <div class="text-lg font-bold italic">Proficiencies:</div>
      <div class="flex flex-row">
        <div class="font-semibold mr-1">Armor:</div>
        <div>
          {insert_between(choice.profs.armor, ", ")}
        </div>
      </div>
      <div class="flex flex-row">
        <div class="font-semibold mr-1">Weapons:</div>
        <div>
          {insert_between(choice.profs.weapon, ", ")}
        </div>
      </div>
      <div class="flex flex-row">
        <div class="font-semibold mr-1">Tools:</div>
        <div>
          {insert_between(choice.profs.tool, ", ")}
        </div>
      </div>
      <div class="flex flex-row mt-1">
        <div class="font-semibold mr-1">Skills:</div>
        {#if choice.profs.skill_choices != undefined}
          <div class="my-1 flex flex-row">
            {#if skill_prof_choice_1.picked == "none"}
              <form class="border-2 rounded-md border-red-100">
                <select bind:value={skill_prof_choice_1.picked}>
                  {#each choice.profs.skill_choices.options as option}
                    <option value={option}>
                      {option}
                    </option>
                  {/each}
                </select>
              </form>
            {:else}
              {check_picking_ok()}
              {check_exclu(
                skill_prof_choice_2,
                skill_prof_choice_1.picked,
                choice.profs.skill_choices.exclu
              )}
              <form class="border-2 rounded-md border-green-200">
                <select
                  bind:value={skill_prof_choice_1.picked}
                  on:change={() =>
                    check_exclu(
                      skill_prof_choice_2,
                      skill_prof_choice_1.picked,
                      choice.profs.skill_choices.exclu
                    )}
                >
                  {#each choice.profs.skill_choices.options as option}
                    <option value={option}>
                      {option}
                    </option>
                  {/each}
                </select>
              </form>
            {/if}
            {#if choice.profs.skill_choices.num > 1}
              and
              {#if skill_prof_choice_2.picked == "none"}
                {check_picking_ok()}
                <form class="border-2 rounded-md border-red-100">
                  <select bind:value={skill_prof_choice_2.picked}>
                    {#each choice.profs.skill_choices.options as option}
                      {#if choice.profs.skill_choices.exclu && skill_prof_choice_1.picked == option}
                        <div />
                      {:else}
                        <option value={option}>
                          {option}
                        </option>
                      {/if}
                    {/each}
                  </select>
                </form>
              {:else}
                {check_picking_ok()}
                {check_exclu(
                  skill_prof_choice_1,
                  skill_prof_choice_2.picked,
                  choice.profs.skill_choices.exclu
                )}
                <form class="border-2 rounded-md border-green-200">
                  <select
                    bind:value={skill_prof_choice_2.picked}
                    on:change={() =>
                      check_exclu(
                        skill_prof_choice_1,
                        skill_prof_choice_2.picked,
                        choice.profs.skill_choices.exclu
                      )}
                  >
                    {#each choice.profs.skill_choices.options as option}
                      {#if choice.profs.skill_choices.exclu && skill_prof_choice_1.picked == option}
                        <div />
                      {:else}
                        <option value={option}>
                          {option}
                        </option>
                      {/if}
                    {/each}
                  </select>
                </form>
              {/if}
            {/if}
            .
          </div>
        {/if}
      </div>
      <div class="text-xl font-bold italic mt-2">Class Features</div>
      {#each choice.features as feature}
        {#if feature.lvl <= level}
          <div class="mb-3">
            <div class="text-lg font-bold">
              {feature.name} -- Level {feature.lvl}
            </div>
            <div class="text-sm text-gray-500 whitespace-pre-line">
              {feature.desc}
            </div>
            {#if feature.name == "Ability Score Improvement"}
              {reserve_one_asi()}
              {#if asis_array[get_current_n_asi()][0].picked == "none"}
                <form class="border-2 rounded-md border-red-100">
                  <select
                    bind:value={asis_array[get_current_n_asi()][0].picked}
                  >
                    <option value="str">Str</option>
                    <option value="dex">Dex</option>
                    <option value="con">Con</option>
                    <option value="int">Int</option>
                    <option value="wis">Wis</option>
                    <option value="cha">Cha</option>
                  </select>
                </form>
              {:else}
                {check_picking_ok()}
                <form class="border-2 rounded-md border-green-200">
                  <select
                    bind:value={asis_array[get_current_n_asi()][0].picked}
                  >
                    <option value="str">Str</option>
                    <option value="dex">Dex</option>
                    <option value="con">Con</option>
                    <option value="int">Int</option>
                    <option value="wis">Wis</option>
                    <option value="cha">Cha</option>
                  </select>
                </form>
              {/if}
              {#if asis_array[get_current_n_asi()][1].picked == "none"}
                <form class="border-2 rounded-md border-red-100">
                  <select
                    bind:value={asis_array[get_current_n_asi()][1].picked}
                  >
                    <option value="str">Str</option>
                    <option value="dex">Dex</option>
                    <option value="con">Con</option>
                    <option value="int">Int</option>
                    <option value="wis">Wis</option>
                    <option value="cha">Cha</option>
                  </select>
                </form>
              {:else}
                {check_picking_ok()}
                <form class="border-2 rounded-md border-green-200">
                  <select
                    bind:value={asis_array[get_current_n_asi()][1].picked}
                  >
                    <option value="str">Str</option>
                    <option value="dex">Dex</option>
                    <option value="con">Con</option>
                    <option value="int">Int</option>
                    <option value="wis">Wis</option>
                    <option value="cha">Cha</option>
                  </select>
                </form>
              {/if}
            {/if}
          </div>
        {:else if feature.name == "Ability Score Improvement"}
          {free_one_asi()}
        {/if}
      {/each}
    </div>

    <div class="my-auto">
      {#if !picking_ok}
        <button
          disabled
          class="py-2 px-2 flex justify-center items-center bg-slate-500 text-white w-full text-center text-base font-semibold shadow-md rounded-full"
          >Pick Class
        </button>
      {:else}
        <button
          class="py-2 px-2 flex justify-center items-center bg-green-500 hover:bg-green-700 focus:ring-green-500 focus:ring-offset-green-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2 rounded-full"
          on:click={pick_this_class}
          >Pick Class
        </button>
      {/if}
    </div>
  </div>
</main>
