import { describe, it, expect } from 'vitest';
import { getTauriErrorMessage } from './error';

describe('getTauriErrorMessage', () => {
  it('handles null/undefined', () => {
    expect(getTauriErrorMessage(null)).toBe('Error desconocido');
    expect(getTauriErrorMessage(undefined)).toBe('Error desconocido');
  });

  it('extracts string directly', () => {
    expect(getTauriErrorMessage('test error')).toBe('test error');
  });

  it('extracts message from error object', () => {
    const err = new Error('something broke');
    expect(getTauriErrorMessage(err)).toBe('something broke');
  });

  it('extracts from Tauri AppError variants', () => {
    expect(getTauriErrorMessage({ NotFound: 'Docente no encontrado.' })).toBe('Docente no encontrado.');
    expect(getTauriErrorMessage({ DatabaseError: 'Connection refused' })).toBe('Connection refused');
    expect(getTauriErrorMessage({ InternalError: 'System error' })).toBe('System error');
  });

  it('falls back to JSON.stringify for unknown objects', () => {
    expect(getTauriErrorMessage({ code: 500 })).toBe(JSON.stringify({ code: 500 }));
  });
});
