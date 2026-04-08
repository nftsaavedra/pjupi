import React, { useId } from 'react';

interface FormSelectProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  options: Array<{ value: string; label: string }>;
  placeholder?: string;
  required?: boolean;
}

export const FormSelect: React.FC<FormSelectProps> = ({
  label,
  value,
  onChange,
  options,
  placeholder = '-- Seleccionar --',
  required = false,
}) => {
  const selectId = useId();

  return (
  <div className="form-group">
    <label htmlFor={selectId}>
      {label}
      {required && ' *'}
    </label>
    <select
      id={selectId}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      required={required}
      className="form-input"
    >
      <option value="">{placeholder}</option>
      {options.map((opt) => (
        <option key={opt.value} value={opt.value}>
          {opt.label}
        </option>
      ))}
    </select>
  </div>
  );
};