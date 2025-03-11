<script>
  import { onMount } from 'svelte';
  let container;
  let loading = true;
  let error = null;

  onMount(async () => {
    try {
      // Import the wasm module
      const wasm = await import('../../wasm-games/pkg/wasm_games.js');
      await wasm.default(); // Initialize the module
      
      // Pass the container element to the wasm module
      wasm.initialize(container);
      
      loading = false;
    } catch (err) {
      console.error('Failed to load WebGL demo:', err);
      error = err.message;
      loading = false;
    }
  });
</script>

<div class="game-container" bind:this={container}>
  <!-- {#if loading}
    <p>Loading WebGL demo...</p>
  {:else if error}
    <p class="error">Error: {error}</p>
  {/if} -->
</div>

<style>
  .game-container {
    width: 100%;
    height: 100%;
    position: relative;
    border: 1px solid #ccc;
  }
  
  .error {
    color: red;
  }
</style>