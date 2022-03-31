<script lang="ts">
	import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri'

  invoke('test_play_')
		.then((msg) => {
      console.log()
		})
</script>

<main>
  <h1>hello</h1>
</main>

<style lang="scss">
</style>
