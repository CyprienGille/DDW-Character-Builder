<script lang="ts">
  import Choice_Lineage from "./lib/Choice_Lineage.svelte";
  import Choice_Class from "./lib/Choice_Class.svelte";
  import Scores from "./lib/Scores.svelte";
  import Description from "./lib/Description.svelte";
  import Equipment from "./lib/Equipment.svelte";
  import End from "./lib/End.svelte";

  let current_page_id = 0;
  let show_about = false;

  function toggle_about() {
    show_about = !show_about;
  }

  function pressedNext() {
    current_page_id += 1;
  }
  function pressedPrevious() {
    current_page_id -= 1;
  }
</script>

<main class="overscroll-none">
  <div class="h-screen flex flex-col">
    {#if current_page_id == 0}
      <div class="relative h-screen overflow-hidden bg-indigo-900">
        <img
          src="/landing.jpg"
          class="absolute object-cover w-full h-full"
          alt="An aurora borealis over a dimly lit wooden inn atop a river."
        />
        <div class="absolute inset-0 bg-black opacity-30" />
        <header class="absolute top-0 left-0 right-0 z-20">
          <nav class="container px-6 py-4 mx-auto md:px-12">
            <div class="items-center justify-between md:flex">
              <div class="flex items-center justify-between">
                <div class="hidden">
                  <button class="text-white focus:outline-none">
                    Hi there
                  </button>
                </div>
              </div>
              <div class="items-center hidden md:flex">
                <button
                  class="mx-3 text-lg text-white uppercase cursor-pointer hover:text-gray-300"
                  on:click={toggle_about}
                >
                  About
                </button>
                <a
                  href="https://github.com/CyprienGille/DDW-Character-Builder"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="mx-3 text-lg text-white uppercase cursor-pointer hover:text-gray-300"
                >
                  Repository
                </a>
              </div>
            </div>
          </nav>
        </header>
        <div
          class="container relative z-10 flex items-center px-6 py-32 mx-auto md:px-12 xl:py-40"
        >
          <div
            class="relative z-10 flex flex-col items-start lg:w-3/5 xl:w-2/5"
          >
            <span class="font-bold text-green-300 tracking-widest">
              DnDWithin Character Builder
            </span>
            <h1
              class="mt-4 text-6xl font-bold leading-tight text-white sm:text-7xl"
            >
              Get ready to build your next character
            </h1>
            <div
              class="block px-4 py-3 mt-10 text-lg font-bold text-gray-800 uppercase bg-white rounded-lg hover:bg-gray-100"
            >
              <button on:click={pressedNext}> Get Started </button>
            </div>
            {#if show_about}
              <div class="text-white mt-5 font-semibold tracking-wide">
                Made with ❤️ by CyprienGille
                <br />
                Background image by Raphael Lacoste
              </div>
            {/if}
          </div>
        </div>
      </div>
    {:else if current_page_id == 1}
      <Choice_Lineage />
    {:else if current_page_id == 2}
      <Choice_Class />
    {:else if current_page_id == 3}
      <Description />
    {:else if current_page_id == 4}
      <Scores />
    {:else if current_page_id == 5}
      <Equipment />
    {:else if current_page_id == 6}
      <End />
    {/if}

    {#if current_page_id != 0}
      <div class="mb-auto" />
      <div class="flex w-full mb-10">
        <button
          class="w-1/3 py-2 rounded font-bold bg-blue-200 text-slate-900 hover:bg-teal-400"
          on:click={pressedPrevious}>Previous</button
        >
        <div class="w-1/3" />
        {#if current_page_id == 6}
          <button
            disabled
            class="w-1/3 py-2 rounded font-bold bg-slate-200 text-slate-900"
            >Next</button
          >
        {:else}
          <button
            class="w-1/3 py-2 rounded font-bold bg-blue-200 text-slate-900 hover:bg-teal-400"
            on:click={pressedNext}>Next</button
          >
        {/if}
      </div>
    {/if}
  </div>
</main>
