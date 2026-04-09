import React, { useId } from 'react';
import { FieldHelpTooltip } from './FieldHelpTooltip';

interface FormInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: string;
  maxLength?: number;
  required?: boolean;
  help?: React.ReactNode;
  readOnly?: boolean;
  disabled?: boolean;
  containerClassName?: string;
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
  readOnly = false,
  disabled = false,
  containerClassName,
}) => {
  const inputId = useId();
  const helpId = help ? `${inputId}-help` : undefined;

  return (
    <div className={containerClassName ? `form-group ${containerClassName}` : 'form-group'}>
      <div className="form-label-row">
        <label htmlFor={inputId} className="form-label-text">
          {label}
          {required && ' *'}
        </label>
        {help && <FieldHelpTooltip content={help} label={`Ayuda para ${label}`} />}
      </div>
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
        readOnly={readOnly}
        disabled={disabled}
      />
      {help && <span id={helpId} className="visually-hidden">{help}</span>}
    </div>
  );
};