<script lang="ts">
  import { invoke, convertFileSrc } from '@tauri-apps/api/tauri'
	import UIkit from 'uikit'
	import Icons from 'uikit/dist/js/uikit-icons'
	import 'uikit/dist/css/uikit.css'
	import 'uikit/dist/css/uikit.min.css'

	UIkit.use(Icons);

  const dir = 'F:/kindletext/audio/';

  let info = {};
  invoke('get_audio_list', {path: dir})
  .then((l) => Object.values(l).forEach(item => info[item[0]] = item[1]));

  const record = (e) => {
    const target = e.target;
    console.log(target.currentTime);
    console.log(decodeURI(target.src.split('/').pop()));
  };

  const set_current_time = (e) => {
    const target = e.target;
    const time = info[decodeURI(target.src.split('/').pop())]
    target.currentTime = time;
  };
</script>

<main>
  <div class="uk-container uk-container-xsmall">
    <ul class="uk-list uk-list-divider">
      {#each Object.entries(info) as [name, time]}
        <li>
          <audio class="tmp" controls src={convertFileSrc(dir)+name} on:loadeddata="{set_current_time}" on:pause={record}/>
          {name}
        </li>
      {/each}
    </ul>
  </div>
</main>

<style lang="scss">
</style>
