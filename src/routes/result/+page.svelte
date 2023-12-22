<script lang="ts">
	import success from '$lib/animation/success.json';
	import failure from '$lib/animation/failure.json';
	import { LottiePlayer } from '@lottiefiles/svelte-lottie-player';
	import { resultStore } from '../../stores/resultStore';
	import warningIcon from '$lib/icon/warning.svg';
	import errorIcon from '$lib/icon/error.svg';

	let result = $resultStore;
	console.log(result);

	function isSuccess() {
		return result?.status.toLocaleLowerCase() === 'success';
	}
</script>

<h1 class="title">{isSuccess() ? 'File successfully converted!' : 'Could not convert file!'}</h1>

<section>
	<LottiePlayer
		src={isSuccess() ? success : failure}
		autoplay={true}
		loop={false}
		controls={false}
		renderer="svg"
		background="transparent"
	/>
</section>

{#if result?.error.length ?? 0}
	<section class="warnings">
		{#each result?.error as error (error)}
			<div class="row">
				<img class="icon" src={errorIcon} width="25" height="25" alt="Error icon" />
				<p>{error}</p>
			</div>
		{/each}
	</section>
{/if}

{#if result?.warning.length ?? 0}
	<section class="warnings">
		{#each result?.warning as warning (warning)}
			<div class="row">
				<img class="icon" src={warningIcon} width="25" height="25" alt="Warning icon" />
				<p>{warning}</p>
			</div>
		{/each}
	</section>
{/if}

<section class="file-select">
	<button class="label-button" on:click={() => window.history.back()}>Back</button>
</section>

<style>
	section {
		margin: 0 auto 0 auto;
	}

	.title {
		font-size: x-large;
		font-weight: bold;
	}

	.icon {
		margin-right: 0.5rem;
	}

	.row {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.warnings {
		margin: 0.4rem;
		border-radius: 1rem;
		background-color: white;
		border: 1px solid black;
		padding: 0.2rem 1rem;
	}
</style>
