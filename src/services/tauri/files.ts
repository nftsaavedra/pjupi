import { invoke } from './client';

export const writeExportFile = async (filePath: string, bytes: Uint8Array): Promise<void> => {
  await invoke('write_export_file', {
    filePath,
    bytes: Array.from(bytes),
  });
};