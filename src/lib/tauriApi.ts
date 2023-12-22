// index.js or index.ts
import { invoke } from '@tauri-apps/api/tauri';

export type TransformCommandResponse = {
	status: 'success' | 'error';
	num_rows: number;
	error: string[];
	warning: string[];
};

// Function to call the Tauri command with loading and error states
export async function transformXlsxFile(
	srcPath: string,
	destPath: string
): Promise<TransformCommandResponse> {
	// Invoke the Tauri command
	return await invoke('transform_xlsx_file', { srcPath, destPath });
}
