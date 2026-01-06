<script lang="ts">
    import { page } from "$app/stores";

    const nav = [
        { label: "Timeline", href: "/" },
        { label: "Entities", href: "/entities" },
    ];
</script>

<div class="app">
    <aside class="sidebar">
        <div class="brand">
            <div class="logo" aria-hidden="true"></div>
            <div class="title">
                <div class="name">Runequill</div>
                <div class="subtitle">Drafting workspace</div>
            </div>
        </div>

        <nav class="nav">
            {#each nav as item}
                <a
                    class="navBtn"
                    class:active={$page.url.pathname === item.href}
                    href={item.href}
                >
                    {item.label}
                </a>
            {/each}
        </nav>

        <div class="sidebarFooter">
            <div class="hint">Tip: Start by creating your first Scene.</div>
        </div>
    </aside>

    <main class="main">
        <slot />
    </main>
</div>

<style>
    .app {
        display: grid;
        grid-template-columns: 260px 1fr;
        height: 100vh;
    }

    .sidebar {
        border-right: 1px solid var(--border);
        background: linear-gradient(
            180deg,
            rgba(50, 13, 109, 0.2),
            rgba(50, 13, 109, 0.05)
        );
        padding: 18px;
        display: flex;
        flex-direction: column;
        gap: 18px;
    }

    .brand {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px;
        border: 1px solid var(--border);
        background: var(--panel);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
    }

    .logo {
        width: 36px;
        height: 36px;
        border-radius: 10px;
        background: radial-gradient(
            circle at 30% 30%,
            var(--accent),
            var(--primary)
        );
    }

    .name {
        font-weight: 700;
        letter-spacing: 0.4px;
    }

    .subtitle {
        font-size: 12px;
        opacity: 0.8;
    }

    .nav {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .navBtn {
        display: block;
        text-decoration: none;
        padding: 12px 14px;
        border-radius: 10px;
        border: 1px solid var(--border);
        background: rgba(50, 13, 109, 0.25);
        transition:
            transform 120ms ease,
            background 120ms ease,
            border-color 120ms ease;
    }

    .navBtn:hover {
        transform: translateY(-1px);
        background: rgba(50, 13, 109, 0.4);
        border-color: rgba(127, 7, 153, 0.55);
    }

    .navBtn.active {
        background: rgba(127, 7, 153, 0.4);
        border-color: rgba(127, 7, 153, 0.75);
    }

    .sidebarFooter {
        margin-top: auto;
        padding: 12px;
        border: 1px solid var(--border);
        border-radius: var(--radius);
        background: var(--panel);
    }

    .hint {
        font-size: 12px;
        opacity: 0.85;
        line-height: 1.35;
    }

    .main {
        padding: 22px;
        overflow: auto;
    }
</style>
