<script lang="ts">
	import excelProcess from '$lib/animation/excel-process.json';
	import loading from '$lib/animation/loading.json';
	import folderOpen from '$lib/icon/folder-open.svg';
	import { open, save } from '@tauri-apps/api/dialog';
	import { LottiePlayer } from '@lottiefiles/svelte-lottie-player';
	import { transformXlsxFile, type TransformCommandResponse } from '$lib/tauriApi';
	import { goto } from '$app/navigation';
	import { resultStore } from '../stores/resultStore';

	let isLoading = false;

	let srcPath: string | null = null;
	let destPath: string | null = null;

	const excelFileExtensions = [
		'xlsx', // Excel Workbook
		'xls', // Excel 97-2003 Workbook
		'xlsm', // Excel Macro-Enabled Workbook
		'xlsb' // Excel Binary Workbook
	];

	async function selectSrcPath() {
		const resp = await open({
			multiple: false,
			filters: [
				{
					name: 'Image',
					extensions: excelFileExtensions
				}
			]
		});

		if (resp === null) {
			return;
		}

		if (Array.isArray(resp)) {
			srcPath = resp[0];
		} else {
			srcPath = resp;
		}
	}

	async function selectDestPath() {
		if (srcPath === null) {
			return;
		}

		destPath = await save({
			defaultPath: srcPath.substring(
				0,
				Math.max(srcPath.lastIndexOf('/'), srcPath.lastIndexOf('\\'))
			),
			filters: [{ name: 'Excel', extensions: excelFileExtensions }]
		});
	}

	async function doTransformFile(srcPath: string, destPath: string) {
		isLoading = true;
		try {
			const resp = await transformXlsxFile(srcPath, destPath);
			navigateWithStoreData(resp);
		} catch (e) {
			console.error(JSON.stringify(e, null, 2));
		} finally {
			isLoading = false;
		}
	}

	function navigateWithStoreData(result: TransformCommandResponse) {
		resultStore.update((_storeData) => result);
		goto('result');
	}
</script>

<section>
	<LottiePlayer
		src={isLoading ? loading : excelProcess}
		autoplay={true}
		loop={true}
		controls={false}
		renderer="svg"
		background="transparent"
		height={320}
		width={320}
	/>
</section>

<p>Select Excel file to convert</p>

<section class="file-select">
	<input disabled={true} value={srcPath} />
	<button
		class="icon-button"
		disabled={isLoading}
		on:click={() => selectSrcPath().catch((err) => console.error(JSON.stringify(err, null, 2)))}
		><img src={folderOpen} width="30" height="30" alt="Open Folder icon" /></button
	>
</section>

<section>
	<button
		class="label-button"
		disabled={srcPath === null || isLoading}
		on:click={async () => {
			await selectDestPath();

			if (srcPath === null || destPath === null) {
				return;
			}

			await doTransformFile(srcPath, destPath);
		}}>Convert</button
	>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	p {
		width: 100%;
		text-align: center;
		font-size: 1.5rem;
		margin-block-start: 0;
		margin-block-end: 0.3rem;
	}

	.file-select {
		display: flex;
		flex-direction: row;
	}
</style>
