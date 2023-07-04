<script>
  import Aside from "./Aside.svelte";
  import ChatBot from "./ChatBot.svelte";
</script>

<main class="grid grid-cols-12">
  <Aside />
  <section class="outlet col-span-10 p-2">
    <slot />
  </section>
</main>
<ChatBot />
