<script>
  import Profits from "../lib/Home/Profits.svelte";
  import Header from "../lib/Home/HomeHeader.svelte";
  import WeatherWidget from "../lib/Home/WeatherWidget.svelte";
  import People from "../lib/Home/People.svelte";
</script>

<Header />
<section class="grid grid-cols-12 grid-rows-2 gap-8 p-8 max-h-screen">
  <Profits />
  <People />
  <WeatherWidget />
</section>
