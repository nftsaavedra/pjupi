import { dirname, join } from '@tauri-apps/api/path';
import { save } from '@tauri-apps/plugin-dialog';
import { writeExportFile } from '../../services/tauri/files';

const LAST_EXPORT_DIRECTORY_KEY = 'pjupi.exports.lastDirectory';

interface SaveDesktopFileOptions {
  suggestedName: string;
  bytes: Uint8Array;
  filters: Array<{
    name: string;
    extensions: string[];
  }>;
  mimeType: string;
}

const isTauriRuntime = () => {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
};

const getStoredDirectory = () => {
  if (typeof window === 'undefined') {
    return null;
  }

  return window.localStorage.getItem(LAST_EXPORT_DIRECTORY_KEY);
};

const setStoredDirectory = (directory: string) => {
  if (typeof window === 'undefined') {
    return;
  }

  window.localStorage.setItem(LAST_EXPORT_DIRECTORY_KEY, directory);
};

const getDefaultPath = async (suggestedName: string) => {
  const lastDirectory = getStoredDirectory();

  if (!lastDirectory) {
    return suggestedName;
  }

  try {
    return await join(lastDirectory, suggestedName);
  } catch {
    return suggestedName;
  }
};

const downloadInBrowser = ({ suggestedName, bytes, mimeType }: SaveDesktopFileOptions) => {
  const blob = new Blob([bytes], { type: mimeType });
  const objectUrl = URL.createObjectURL(blob);
  const anchor = document.createElement('a');
  anchor.href = objectUrl;
  anchor.download = suggestedName;
  anchor.click();
  URL.revokeObjectURL(objectUrl);
};

export const saveDesktopFile = async ({
  suggestedName,
  bytes,
  filters,
  mimeType,
}: SaveDesktopFileOptions): Promise<string | null> => {
  if (!isTauriRuntime()) {
    downloadInBrowser({ suggestedName, bytes, filters, mimeType });
    return suggestedName;
  }

  const filePath = await save({
    defaultPath: await getDefaultPath(suggestedName),
    filters,
  });

  if (!filePath) {
    return null;
  }

  await writeExportFile(filePath, bytes);

  try {
    const directory = await dirname(filePath);
    setStoredDirectory(directory);
  } catch {
    // Ignore path parsing failures after a successful save.
  }

  return filePath;
};