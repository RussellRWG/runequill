<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let message: string = "Loading...";

    onMount(() => {
        (async () => {
            try {
                message = await invoke<string>("hello_world");
            } catch (err) {
                console.error(err);
                message = "Error calling Rust (see console)";
            }
        })();
    });
</script>

<h1>Welcome to SvelteKit</h1>
<p>
    Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the
    documentation
</p>
<h2>{message}</h2>
