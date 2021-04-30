import App from './App.svelte';

import wasm from '../../qr_code_encoder/Cargo.toml';
const init = async ()=> {
	const qrCodeEncoder = await wasm();

	const app = new App({
		target: document.body,
		props: {
			genQrCode: qrCodeEncoder.genQrCode,
		}
	});
}

init()
export default app;