<script lang="ts">
  import Profits from "../lib/Home/Profits.svelte";
  import Header from "../lib/Home/HomeHeader.svelte";
  import WeatherWidget from "../lib/Home/WeatherWidget.svelte";
  import People from "../lib/Home/People.svelte";
</script>

<Header />
<section class="grid grid-cols-12 grid-rows-4 gap-4 p-8 h-full max-h-screen">
  <Profits />
  <WeatherWidget />
</section>
