<script>
  import { onMount } from 'svelte';
  
  let container;
  let loading = true;
  let error = null;
  
  onMount(async () => {
    try {
      // Import the wasm module
      const wasm = await import('../../pkg/wasm_games.js');
      await wasm.default(); // Initialize the module
      
      // The Rust code will create and append its canvas to the body
      // We need to move it to our container
      const canvas = document.querySelector('canvas');
      if (canvas && canvas.parentNode === document.body) {
        document.body.removeChild(canvas);
        container.appendChild(canvas);
        
        // Optional: Set dimensions
        canvas.style.width = '100%';
        canvas.style.height = '100%';
      }
      
      loading = false;
    } catch (err) {
      console.error('Failed to load WebGL demo:', err);
      error = err.message;
      loading = false;
    }
  });
</script>

<div class="webgl-container" bind:this={container}>
  {#if loading}
    <div class="loading">Loading WebGL demo...</div>
  {:else if error}
    <div class="error">Error: {error}</div>
  {/if}
</div>

<style>
  .webgl-container {
    width: 100%;
    height: 100%;
    position: relative;
    background-color: #1a1a1a;
    border-radius: 4px;
    overflow: hidden;
  }
  
  .loading, .error {
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
    inset: 0;
    color: white;
  }
  
  .error {
    color: #ff6b6b;
  }
</style>