<script lang="ts">
  import { writeTextFile } from "@tauri-apps/api/fs";
  import { desktopDir } from "@tauri-apps/api/path";

  import type monaco from "monaco-editor";
  import { onMount } from "svelte";
  import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
  import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
  import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
  import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
  import type Module from "module";

  let elementForEditor: HTMLDivElement = null;
  let editor: monaco.editor.IStandaloneCodeEditor;
  let Monaco: any | null;

  onMount(async () => {
    // @ts-ignore
    self.MonacoEnvironment = {
      getWorker: function (_moduleId: any, label: string) {
        if (label === "json") {
          return new jsonWorker();
        }
        if (label === "css" || label === "scss" || label === "less") {
          return new cssWorker();
        }
        if (label === "html" || label === "handlebars" || label === "razor") {
          return new htmlWorker();
        }
        if (label === "typescript" || label === "javascript") {
          return new tsWorker();
        }
        return new editorWorker();
      },
    };

    Monaco = await import("monaco-editor");
    editor = Monaco.editor.create(elementForEditor, {
      value: ["function x() {", '\tconsole.log("Hello world!");', "}"].join(
        "\n"
      ),
      language: "javascript",
      themes: "vs-dark",
    });

    return () => {
      editor.dispose();
    };
  });
</script>

<div bind:this={elementForEditor} class="h-screen" />
