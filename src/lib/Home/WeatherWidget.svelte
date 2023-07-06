<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let data: any = {};

  async function getWeatherData(city: string) {
    const response: any = await invoke("get_weather_data_from_api", { city });
    data = {
      ...response.currentConditions,
      max: response.days[0].feelslikemin,
      min: response.days[0].feelslikemax,
    };
    console.log(data);
  }

  onMount(() => getWeatherData("Mar del Plata"));

  const today = new Date();
  const actualMonthIndex = today.getMonth();
  const months = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
  ];

  const actualDayIndex = today.getDay();
  const days = [
    "Domingo",
    "Lunes",
    "Martes",
    "Miercoles",
    "Jueves",
    "Viernes",
    "Sabado",
  ];
</script>

<section
  class="col-start-10 col-span-3 row-span-2 w-full h-full bg-white rounded-lg grid place-items-center"
>
  <div class="flex flex-col gap-3">
    <div class="font-bold text-xl">Mar del Plata</div>
    <div class="text-sm text-gray-500">
      {days[actualDayIndex]}
      {today.getDate()} de
      {months[actualMonthIndex]} del {today.getFullYear()}
    </div>
    <div
      class="mt-6 text-6xl self-center inline-flex items-center justify-center rounded-lg text-indigo-400 h-24 w-24"
    >
      <svg
        class="w-32 h-32"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        ><path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"
        /></svg
      >
    </div>
    <div class="flex flex-row items-center justify-center mt-6">
      <div class="font-medium text-6xl">{data.feelslike}°</div>
      <div class="flex flex-col items-center ml-6">
        <div>Cloudy</div>
        <div class="mt-1">
          <span class="text-sm"><i class="far fa-long-arrow-up" /></span>
          <span class="text-sm font-light text-gray-500">{data.max}°C</span>
        </div>
        <div>
          <span class="text-sm"><i class="far fa-long-arrow-down" /></span>
          <span class="text-sm font-light text-gray-500">{data.min}°C</span>
        </div>
      </div>
    </div>
    <div class="flex flex-row justify-between mt-6">
      <div class="flex flex-col items-center">
        <div class="font-medium text-sm">Wind</div>
        <div class="text-sm text-gray-500">{data.windspeed}k/h</div>
      </div>
      <div class="flex flex-col items-center">
        <div class="font-medium text-sm">Humidity</div>
        <div class="text-sm text-gray-500">{data.humidity}%</div>
      </div>
      <div class="flex flex-col items-center">
        <div class="font-medium text-sm">Visibility</div>
        <div class="text-sm text-gray-500">{data.visibility}km</div>
      </div>
    </div>
  </div>
</section>
