<script lang="ts">
    let { name, text }: { name: String; text: String } = $props();
</script>

<div class="card">
    <h3 class="card-name">{name}</h3>
    <p class="card-text">
        {text}
    </p>
    <button class="card-button">Edit</button>
</div>

<style>
    .card {
        max-width: 320px;
        padding: 16px;
        padding-top: 8px;
        margin: 0 auto;
        color: var(--text);
        background: var(--primary);

        border: 2px solid var(--text);
        border-radius: 16px;
    }
    .card-button {
        display: block;
        width: 160px;
        height: 32px;
        background: var(--accent);
        border: 2px solid var(--text);
        border-radius: 16px;
        margin: 0 auto;
    }

    .card-name {
        margin-top: 0px;
        margin-bottom: 0px;
    }
    .card-text {
        margin-top: 8px;
    }
</style>
