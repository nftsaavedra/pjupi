export const getTauriErrorMessage = (error: unknown): string => {
  if (!error) return 'Error desconocido';
  if (typeof error === 'string') return error;

  if (typeof error === 'object') {
    const maybe = error as Record<string, unknown>;

    if (typeof maybe.message === 'string' && maybe.message.trim()) {
      return maybe.message;
    }

    const keys = [
      'DatabaseError',
      'UniqueConstraintViolation',
      'NotFound',
      'InternalError',
      'ConfigurationError',
      'ExternalServiceError',
    ];

    for (const key of keys) {
      const value = maybe[key];
      if (typeof value === 'string' && value.trim()) {
        return value;
      }
    }

    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }

  return String(error);
};