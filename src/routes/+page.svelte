<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <button class="bg-yellow-500 mx-64">Hello</button>
    <div class="mx-32 my-4 text-center border-4 rounded-md min-h-32">
      <output>This is where output goes</output>
    </div>
  </div>
</main>
