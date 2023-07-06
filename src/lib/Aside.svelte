<script>
  let actualPath = location.pathname;

  console.log(actualPath);
</script>

<nav class="p-4 col-span-2">
  <ul class="space-y-4 select-none fixed">
    <li
      class={actualPath === "/"
        ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
        : ""}
    >
      <a
        href="/"
        class="flex items-center gap-2 rounded-lg px-4 py-2 text-gray-950"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 opacity-75"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>

        <span class="text-md font-medium"> Inicio</span>
      </a>
    </li>

    <li
      class={actualPath === "/amigos"
        ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
        : ""}
    >
      <details class="group [&_summary::-webkit-details-marker]:hidden">
        <summary
          class="group flex items-center justify-between rounded-lg px-4 py-2 text-black hover:bg-gray-100 hover:text-gray-950"
        >
          <div class="flex items-center gap-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 opacity-75"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
              />
            </svg>

            <span class="text-md font-medium"> Friends </span>
          </div>
        </summary>

        <ul class="mt-2 space-y-1 px-4">
          <li
            class={actualPath === "/hola"
              ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
              : ""}
          >
            <a
              href=""
              class="block rounded-lg px-4 py-2 text-md font-medium text-black hover:bg-gray-100 hover:text-gray-950"
            >
              Banned Users
            </a>
          </li>

          <li
            class={actualPath === "/explorar"
              ? "bg-blue-300 bg-opacity-5 border-blue-700"
              : ""}
          >
            <a
              href=""
              class="block rounded-lg px-4 py-2 text-md font-medium text-black hover:bg-gray-100 hover:text-gray-950"
            >
              Calendar
            </a>
          </li>
        </ul>
      </details>
    </li>

    <li
      class={actualPath === "/notas"
        ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
        : ""}
    >
      <a
        href="/notas"
        class="flex items-center gap-2 rounded-lg px-4 py-2 text-black hover:bg-gray-100 hover:text-gray-950"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 opacity-75"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
          />
        </svg>

        <span class="text-md font-medium"> Notas </span>
      </a>
    </li>

    <li
      class={actualPath === "/proyectos"
        ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
        : ""}
    >
      <a
        href="/proyectos"
        class="flex items-center gap-2 rounded-lg px-4 py-2 text-black hover:bg-gray-100 hover:text-gray-950"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5 opacity-75"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"
          />
        </svg>

        <span class="text-md font-medium"> Proyectos </span>
      </a>
    </li>

    <li
      class={actualPath === "/perfil"
        ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
        : ""}
    >
      <details class="group [&_summary::-webkit-details-marker]:hidden">
        <summary
          class="group flex items-center justify-between rounded-lg px-4 py-2 text-black hover:bg-gray-100 hover:text-gray-950"
        >
          <div class="flex items-center gap-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 opacity-75"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              stroke-width="2"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>

            <span class="text-md font-medium"> Perfil </span>
          </div>
        </summary>

        <ul
          class={actualPath === "/detalles"
            ? "bg-blue-300 bg-opacity-5 mt-2 space-y-1 px-4 border-blue-700"
            : "mt-2 space-y-1 px-4"}
        >
          <li
            class={actualPath === "/perfil"
              ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
              : ""}
          >
            <a
              href="/perfil"
              class="block rounded-lg px-4 py-2 text-md font-medium text-black hover:bg-gray-100 hover:text-gray-950"
            >
              Detalles
            </a>
          </li>

          <li
            class={actualPath === "/seguridad"
              ? "bg-blue-300 bg-opacity-5 border-blue-700"
              : ""}
          >
            <a
              href="/seguridad"
              class="block rounded-lg px-4 py-2 text-md font-medium text-black hover:bg-gray-100 hover:text-gray-950"
            >
              Seguridad
            </a>
          </li>

          <li
            class={actualPath === "/salir"
              ? "bg-blue-300 bg-opacity-70 border-l-2 border-blue-700"
              : ""}
          >
            <form action="/salir">
              <button
                type="submit"
                class="w-full rounded-lg px-4 py-2 text-md font-medium text-black [text-align:_inherit] hover:bg-gray-100 hover:text-gray-950"
              >
                Salir
              </button>
            </form>
          </li>
        </ul>
      </details>
    </li>
  </ul>
</nav>
