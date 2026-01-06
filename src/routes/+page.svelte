<script lang="ts">
  type SceneBlock = { id: string; name: string };

  let blocks: SceneBlock[] = [];

  function createBlock(atIndex: number) {
    const id = crypto.randomUUID();
    const name = blocks.length === 0 ? 'First Scene' : `Scene ${blocks.length + 1}`;
    blocks = [...blocks.slice(0, atIndex), { id, name }, ...blocks.slice(atIndex)];
  }
</script>

<header class="header">
  <div>
    <h1>Timeline</h1>
    <p>Arrange scenes in order. Insert scenes anywhere.</p>
  </div>
</header>

<section class="timeline">
  {#if blocks.length === 0}
    <div class="emptyRow">
      <div class="arrow" aria-hidden="true"></div>
      <button class="plus" on:click={() => createBlock(0)} aria-label="Create your first scene">
        +
      </button>
      <div class="emptyHint">Create your first block</div>
    </div>
  {:else}
    <div class="row">
      {#each blocks as b, i (b.id)}
        <!-- Insert point BEFORE block i -->
        <div class="insert">
          <div class="arrow" aria-hidden="true"></div>
          <button class="plus small" on:click={() => createBlock(i)} aria-label="Insert scene here">+</button>
        </div>

        <!-- Block -->
        <div class="block">
          <div class="blockTop">
            <div class="blockName">{b.name}</div>
            <div class="pill">Scene</div>
          </div>
          <div class="blockBody">
            <div class="muted">Click later to edit text, tags, etc.</div>
          </div>
        </div>
      {/each}

      <!-- Insert point AFTER last block -->
      <div class="insert">
        <div class="arrow" aria-hidden="true"></div>
        <button class="plus small" on:click={() => createBlock(blocks.length)} aria-label="Add scene to end">+</button>
      </div>
    </div>
  {/if}
</section>

<style>
  .header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  h1 {
    margin: 0;
    font-size: 26px;
    letter-spacing: 0.3px;
  }

  p {
    margin: 6px 0 0;
    opacity: 0.85;
  }

  .timeline {
    border: 1px solid var(--border);
    background: var(--panel);
    border-radius: var(--radius);
    padding: 18px;
    box-shadow: var(--shadow);
    min-height: 240px;
  }

  .emptyRow {
    display: grid;
    grid-template-columns: 48px 56px 1fr;
    align-items: center;
    gap: 14px;
    padding: 18px;
    border-radius: 14px;
    border: 1px dashed rgba(242, 247, 242, 0.25);
    background: rgba(30, 32, 25, 0.25);
  }

  .emptyHint {
    opacity: 0.85;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 14px;
    flex-wrap: wrap;
  }

  .insert {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .arrow {
    width: 48px;
    height: 10px;
    position: relative;
    opacity: 0.9;
  }

  .arrow::before {
    content: '';
    position: absolute;
    inset: 4px 10px 4px 0;
    background: rgba(242, 247, 242, 0.55);
    border-radius: 999px;
  }

  .arrow::after {
    content: '';
    position: absolute;
    right: 0;
    top: 0;
    width: 0;
    height: 0;
    border-top: 5px solid transparent;
    border-bottom: 5px solid transparent;
    border-left: 10px solid rgba(242, 247, 242, 0.55);
  }

  .plus {
    width: 56px;
    height: 56px;
    border-radius: 16px;
    border: 1px solid rgba(127, 7, 153, 0.55);
    background: rgba(50, 13, 109, 0.55);
    box-shadow: 0 10px 25px rgba(0,0,0,0.25);
    cursor: pointer;
    font-size: 30px;
    line-height: 1;
    display: grid;
    place-items: center;
    transition: transform 120ms ease, background 120ms ease, border-color 120ms ease;
  }

  .plus:hover {
    transform: translateY(-1px);
    background: rgba(127, 7, 153, 0.55);
    border-color: rgba(127, 7, 153, 0.85);
  }

  .plus.small {
    width: 38px;
    height: 38px;
    border-radius: 12px;
    font-size: 22px;
  }

  .block {
    width: 240px;
    border-radius: 16px;
    border: 1px solid rgba(242, 247, 242, 0.16);
    background: rgba(30, 32, 25, 0.35);
    overflow: hidden;
  }

  .blockTop {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 12px 12px 10px;
    background: linear-gradient(135deg, rgba(50,13,109,0.55), rgba(127,7,153,0.25));
    border-bottom: 1px solid rgba(242, 247, 242, 0.12);
  }

  .blockName {
    font-weight: 700;
    letter-spacing: 0.2px;
  }

  .pill {
    font-size: 11px;
    padding: 4px 8px;
    border-radius: 999px;
    border: 1px solid rgba(242, 247, 242, 0.25);
    background: rgba(242, 247, 242, 0.06);
    opacity: 0.95;
  }

  .blockBody {
    padding: 12px;
  }

  .muted {
    font-size: 12px;
    opacity: 0.75;
  }
</style>
