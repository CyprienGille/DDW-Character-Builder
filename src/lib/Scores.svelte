<script lang="ts">
  import { c, lineage_choices, asis, rolled_scores } from "../stores";

  let rolled_str = 0;
  let rolled_dex = 0;
  let rolled_con = 0;
  let rolled_int = 0;
  let rolled_wis = 0;
  let rolled_cha = 0;

  function add_sign(num: number) {
    let res = "";
    if (num >= 0) {
      res = "+" + num;
    } else {
      res = "-" + num;
    }
    return res;
  }

  function input_scores() {
    $rolled_scores = [];
    $rolled_scores.push(rolled_str);
    $rolled_scores.push(rolled_dex);
    $rolled_scores.push(rolled_con);
    $rolled_scores.push(rolled_int);
    $rolled_scores.push(rolled_wis);
    $rolled_scores.push(rolled_cha);
    $rolled_scores = $rolled_scores;
    return "";
  }
</script>

<main>
  {input_scores()}
  <div
    class="container flex flex-col items-center justify-center w-full mx-auto"
  >
    <div
      class="w-full px-4 py-5 mb-2 bg-white border rounded-md shadow sm:px-6"
    >
      <h3 class="text-lg font-medium leading-6 text-gray-900">
        Set your ability scores.
      </h3>
      <p class="max-w-2xl mt-1 text-sm text-gray-500">
        This will decide your general abilities, and which skills your character
        is the most competent in. The classical way of rolling ability scores is
        to roll 4d6, drop the lowest. You can also assign the standard
        8,10,12,13,14,15 array to your base scores. Note that unless specified
        otherwise, scores cannot exceed 20.
      </p>
    </div>
    <div class="flex flex-row">
      <div class="shadow-sm border-2 rounded text-center">
        <div class="text-lg font-bold pl-2">Strength</div>
        <div class="flex flex-row mb-2 pl-2 columns-2">
          <div class="italic w-full">Base Score:</div>
          <div class="font-semibold mx-2 w-full">
            <input
              class="border-2 rounded-lg text-center"
              type="number"
              bind:value={rolled_str}
              on:change={input_scores}
            />
          </div>
        </div>
        <div class="flex flex-row mb-2 pl-2 columns-2">
          <div class="italic w-full">Lineage Bonus:</div>
          <div class="font-semibold mx-auto w-full">
            {#if $lineage_choices[0] != undefined}
              {add_sign($lineage_choices[0].picked.scores.str)}
            {:else}
              +0
            {/if}
          </div>
        </div>
        <div class="flex flex-row mb-2 pl-2 columns-2">
          <div class="italic w-full">Score Improvements:</div>
          <div class="font-semibold mx-auto w-full">
            {add_sign($asis[0].reduce((a, b) => a + b, 0))}
          </div>
        </div>
        <div class="flex flex-row mb-2 pl-2 columns-2">
          <div class="italic w-full">Total:</div>
          <div class="font-semibold mx-auto w-full">
            {#if $lineage_choices[0] != undefined}
              {$rolled_scores[0] +
                $lineage_choices[0].picked.scores.str +
                $asis[0].reduce((a, b) => a + b, 0)}
            {:else}
              {$rolled_scores[0] + $asis[0].reduce((a, b) => a + b, 0)}
            {/if}
          </div>
        </div>
      </div>
      <div>Dexterity</div>
      <div>Constitution</div>
    </div>
    <div class="flex flex-row">
      <div>Intelligence</div>
      <div>Wisdom</div>
      <div>Charisma</div>
    </div>
  </div>
</main>
