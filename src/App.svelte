<script lang="ts">
  //@ts-nocheck
  import Layout from "./lib/Layout.svelte";

  import { Router, Route } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Notas from "./routes/Notas.svelte";
  import Proyectos from "./routes/Proyectos.svelte";
  import AccessForm from "./lib/AccessForm.svelte";

  const user = JSON.parse(localStorage.getItem("cozy-user"));
</script>

{#if user != undefined}
  <Layout>
    <Router>
      <Route path="/" component={Home} />
      <Route path="/notas" component={Notas} />
      <Route path="proyectos" component={Proyectos} />
    </Router>
  </Layout>
{:else}
  <AccessForm />
{/if}
