import React, { useId } from 'react';

interface FormInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: string;
  maxLength?: number;
  required?: boolean;
  help?: string;
}

export const FormInput: React.FC<FormInputProps> = ({
  label,
  value,
  onChange,
  placeholder,
  type = 'text',
  maxLength,
  required = false,
  help,
}) => {
  const inputId = useId();
  const helpId = help ? `${inputId}-help` : undefined;

  return (
  <div className="form-group">
    <label htmlFor={inputId}>
      {label}
      {required && ' *'}
    </label>
    <input
      id={inputId}
      type={type}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      placeholder={placeholder}
      maxLength={maxLength}
      required={required}
      className="form-input"
      aria-describedby={helpId}
    />
    {help && <small id={helpId}>{help}</small>}
  </div>
  );
};