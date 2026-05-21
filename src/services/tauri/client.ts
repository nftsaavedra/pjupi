import { invoke } from '@tauri-apps/api/core';
import { getTauriErrorMessage } from './error';

export class AppError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'AppError';
  }
}

export async function tauriCmd<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    throw new AppError(getTauriErrorMessage(error));
  }
}

export { invoke };
