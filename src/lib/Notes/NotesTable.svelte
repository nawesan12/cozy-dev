<script>
  import NoteItem from "./NoteItem.svelte";
</script>

<table class="w-full divide-y-2 divide-gray-200 bg-white text-sm">
  <thead class="ltr:text-left rtl:text-right">
    <tr>
      <th
        class="whitespace-nowrap px-4 border-r-2 border-gray-200 py-2 font-medium text-gray-900"
      >
        Nombre
      </th>
      <th
        class="whitespace-nowrap px-4 border-x-2 border-gray-200 py-2 font-medium text-gray-900"
      >
        Creada en
      </th>
      <th
        class="whitespace-nowrap px-4 border-x-2 border-gray-200 py-2 font-medium text-gray-900"
      >
        Categoria
      </th>
      <th
        class="whitespace-nowrap px-4 border-l-2 border-gray-200 py-2 font-medium text-gray-900"
      >
        Resumen
      </th>
    </tr>
  </thead>

  <tbody class="divide-y divide-gray-200">
    <NoteItem
      noteInfo={{
        name: "Web Development",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Styling with CSS",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Hola hola hola",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Web Development",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Styling with CSS",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Hola hola hola",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Web Development",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Styling with CSS",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
    <NoteItem
      noteInfo={{
        name: "Hola hola hola",
        createdAt: "04/07/2023",
        category: "",
        resume: "",
      }}
    />
  </tbody>
</table>
