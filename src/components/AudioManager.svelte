<script script lang="ts">
	import { onMount } from "svelte";
  import { dialog, path } from '@tauri-apps/api'
	import { readDir } from '@tauri-apps/api/fs';
	import { convertFileSrc } from "@tauri-apps/api/tauri";

	const KEY_DIR = "key_dir";
	const KEY_INFO = "info";

	class OneSoundInfo {
		time: number;
		volume: number;

		constructor(time: number = 0.0, volume: number = 1.0) {
			this.time = time;
			this.volume = volume;
		}
	}

	// 選択ディレクトリ取得
	$:dir = "change dir";
	const get_dir = () => {
    dialog.open({directory: true, multiple: false, recursive: false})
			.then((d: string) => {
				if (d === null) { return; }

				dir = d;
				make_info();
				localStorage.setItem(KEY_DIR, d);
			})
	};

	// 選択ディレクトリのファイルから音声ファイル情報作成
	$: info = {};
	const SOUND_EXTENSIONS = ['ogg', 'wav', 'mp3', 'aac', 'flac', 'opus'];
	const make_info = () => {
		// 選択ディレクトリのファイル一覧から音声ファイル一覧取得
		readDir(dir).then((list) => {
			let info_ = {};

			list
				.filter((l) => SOUND_EXTENSIONS.includes(l.name.split('.').pop()))
				.forEach((l) => info_[l.path.replaceAll('\\', '/')] = new OneSoundInfo());

			// ローカルストレージから音声ファイル情報取得
			let inf_from_local = localStorage.getItem(KEY_INFO);
			if (inf_from_local) {
				Object.entries(JSON.parse(inf_from_local)).forEach(([path, one_sound_info]) => {
					// ディレクトリに存在する音声ファイルのみからなる情報作成
					if (path in info_) { info_[path] = one_sound_info }
				})
			}
			info = info_;
		});
	};

	// audioコントロールに時間設定
	const set_info = (e) => {
		e.target.currentTime = info[e.target.id].time
		e.target.volume = info[e.target.id].volume
	}

	// 音声情報保存（別ディレクトリ情報は残す）
	const record_info = (e) => {
		let info_ = {}
		Array.from(document.getElementsByTagName('audio')).forEach(audio => {
			info_[audio.id] = new OneSoundInfo(audio.currentTime, audio.volume)
		});

		// 既存の音声情報に新規情報を上書き
		let inf_from_local = localStorage.getItem(KEY_INFO)
		if (inf_from_local) { info_ = Object.assign(JSON.parse(inf_from_local), info_) }

		// 保存
		localStorage.setItem(KEY_INFO, JSON.stringify(info_));
	};

	onMount(() => {
		// 音声ファイル場所情報取得
		dir = localStorage.getItem(KEY_DIR);

		// 音声情報作成
		make_info();
	});
</script>


<main>
	<div class="dir">
		<button on:click={get_dir}>{ dir }</button>
	</div>

		{#each Object.entries(info) as [path, time]}
		<div class="per_audio">
			<div>
				{ path.split('/').pop() }
			</div>

			<div>
				<audio controls preload="metadata"
					id={path}
					src={convertFileSrc(path)}
					on:pause={record_info}
					on:seeked={record_info}
					on:volumechange={record_info}
					on:loadstart={set_info}
				/>
			</div>

		</div>
		{/each}

</main>


<style>
	.dir {
		padding: 20px 0;
	}

	.per_audio {
		padding-bottom: 15px;
	}

	audio {
		width: 100%;
		height: 30px;
		padding-top: 5px;
	}
</style>