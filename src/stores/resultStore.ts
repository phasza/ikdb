import type { TransformCommandResponse } from '$lib/tauriApi';
import { writable } from 'svelte/store';

export const resultStore = writable<TransformCommandResponse>({
	status: 'success',
	num_rows: 0,
	error: [],
	warning: []
});
