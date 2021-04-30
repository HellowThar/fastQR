<script>

	export let genQrCode;

	let url;
	let svg;

	async function buttonHandler() {
		svg = await genQrCode(url);
	}

</script>

<main>
	<h1>Hello!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	{#if svg}
		{@html svg}
	{/if}
	<input type="text" bind:value="{url}">
	<button on:click={buttonHandler}></button>
</main>

<style>
	img {
		width: 100px;
		height: 100px;
		image-rendering: pixelated;
	}

	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>