<script lang="ts">
  import ThreePaneLayout from '$lib/layout/ThreePaneLayout.svelte';
</script>

<ThreePaneLayout>
  {#snippet left()}
    <div class="placeholder">Left (file tree)</div>
  {/snippet}
  {#snippet center()}
    <div class="placeholder">Center (terminal)</div>
  {/snippet}
  {#snippet right()}
    <div class="placeholder">Right (context panel)</div>
  {/snippet}
</ThreePaneLayout>

<style>
  .placeholder {
    padding: 12px;
    color: var(--fg-2);
    font-size: 12px;
  }
</style>
