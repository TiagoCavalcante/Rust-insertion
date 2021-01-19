let wasm;

export function square(number) {
	return wasm.square(number);
}

async function init() {
	const { instance } = await WebAssembly.instantiateStreaming(
		fetch(import.meta.url.replace(/js/, 'wasm')),
		{}
	);

	wasm = instance.exports;
}

export default init;