<script lang="ts">
    import { c } from "../stores";
</script>

<main>
    <div class="px-4 py-5 mb-2 bg-white border rounded-md shadow sm:px-6">
        <h3 class="text-lg font-medium leading-6 text-gray-900">
            Pick a name, and character creation options.
        </h3>
        <p class="max-w-2xl mt-1 text-sm text-gray-500">
            Tip: You will always be able to modify your name in your character
            sheet directly! So if you want to pick your lineage and class
            beforehand, just proceed to the next step.
        </p>
    </div>
    <div class="w-1/3 flex">
        <input
            class="rounded-md w-full py-2 px-2 my-4 ml-2 border-slate-400 border-2"
            type="text"
            name="CName"
            id="cname"
            bind:value={$c.name}
        />
    </div>
</main>
