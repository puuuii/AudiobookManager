<script lang="ts">
	import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri'

  invoke('hello', {name: 'world'})
		.then((msg) => {
      console.log()
		})
</script>

<main>
  <h1>hello</h1>
</main>

<style lang="scss">
</style>
