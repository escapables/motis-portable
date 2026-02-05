import { client } from '@motis-project/motis-client';
import { browser } from '$app/environment';
import type { QuerySerializerOptions } from '@hey-api/client-fetch';

export const prerender = true;

if (browser) {
	const params = new URL(window.location.href).searchParams;
	const defaultProtocol = window.location.protocol;
	const defaultHost = window.location.hostname;
	const defaultPort = '8080';
	const motisParam = params.get('motis');
	
	// Check if running in Tauri (native app with IPC backend)
	const isTauri = !!(window as any).__TAURI__ || !!(window as any).__TAURI_INTERNALS__;
	
	let baseUrl: string;
	if (isTauri) {
		// Tauri native app: use the custom protocol for IPC communication.
		// No HTTP server is needed; requests are intercepted by the Rust backend.
		// CSP also allows motis.localhost variants for compatibility.
		baseUrl = 'motis://localhost';
	} else if (motisParam) {
		if (/^[0-9]+$/.test(motisParam)) {
			baseUrl = defaultProtocol + '//' + defaultHost + ':' + motisParam;
		} else if (!motisParam.includes(':')) {
			baseUrl = defaultProtocol + '//' + motisParam + ':' + defaultPort;
		} else if (!motisParam.startsWith('http:') && !motisParam.startsWith('https:')) {
			baseUrl = defaultProtocol + '//' + motisParam;
		} else {
			baseUrl = motisParam;
		}
	} else {
		baseUrl = String(window.location.origin + window.location.pathname);
	}
	
	const querySerializer = { array: { explode: false } } as QuerySerializerOptions;
	client.setConfig({ baseUrl, querySerializer });
	
	console.log('[MOTIS] API client configured with baseUrl:', baseUrl);
}
