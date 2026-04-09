const normalizeText = (value: string | null | undefined) => (value ?? '').trim().toLowerCase();

export const formatRenacytNivel = (value: string | null | undefined) => {
  const trimmedValue = value?.trim();

  if (!trimmedValue) {
    return null;
  }

  if (trimmedValue.toUpperCase() === 'ID') {
    return 'Distinguido';
  }

  return trimmedValue;
};

export const normalizeRenacytNivelSearch = (value: string | null | undefined) => {
  const formattedValue = formatRenacytNivel(value);
  const normalizedFormattedValue = normalizeText(formattedValue);
  const normalizedRawValue = normalizeText(value);

  if (!normalizedFormattedValue) {
    return '';
  }

  if (!normalizedRawValue || normalizedRawValue === normalizedFormattedValue) {
    return normalizedFormattedValue;
  }

  return `${normalizedFormattedValue} ${normalizedRawValue}`;
};