<script lang="ts">
  export let noteInfo;
</script>

<tr>
  <td class="whitespace-nowrap px-4 py-4 font-medium text-gray-900 text-md">
    {noteInfo.name}
  </td>
  <td class="whitespace-nowrap px-4 py-6 text-gray-700">{noteInfo.createdAt}</td
  >
  <td class="whitespace-nowrap px-4 py-6 text-gray-700">{noteInfo.category}</td>
  <td class="whitespace-nowrap px-4 py-6 text-gray-700">{noteInfo.resume}</td>
</tr>
