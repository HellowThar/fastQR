<script>

	export let genQrCode;

	let url;
	let svg;

	async function buttonHandler() {
		svg = await genQrCode('url');
	}

	async function download() {
		let canvas = document.createElement('canvas');
		let img = document.getElementsByTagName('svg');
		img = img[0];
		console.log(img)
		let xml = new XMLSerializer().serializeToString(img);
		let svg64 = btoa(xml);
		let image64 = `data:image/svg+xml;base64,${svg64}`;
		canvas.getContext('2d').drawImage(svg, 0, 0);
		let dt = canvas.toDataURL('image/png');
		dt = dt.replace(/^data:image\/[^;]*/, 'data:application/octet-stream');
		dt = dt.replace(/^data:application\/octet-stream/, 'data:application/octet-stream;headers=Content-Disposition%3A%20attachment%3B%20filname=Canvas.png')
		this.href = dt
	}

</script>

<main>
	<h1>Hello!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	{#if svg}
		{@html svg}
	{/if}
	<input type="text" bind:value="{url}">
	<button on:click={buttonHandler}>Generate QR Code</button>
	<button on:click={download}>Download</button>
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