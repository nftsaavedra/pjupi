import React, { useId } from 'react';
import { FieldHelpTooltip } from './FieldHelpTooltip';

interface FormSelectProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  options: Array<{ value: string; label: string }>;
  placeholder?: string;
  required?: boolean;
  help?: React.ReactNode;
  disabled?: boolean;
  containerClassName?: string;
}

export const FormSelect: React.FC<FormSelectProps> = ({
  label,
  value,
  onChange,
  options,
  placeholder = '-- Seleccionar --',
  required = false,
  help,
  disabled = false,
  containerClassName,
}) => {
  const selectId = useId();
  const helpId = help ? `${selectId}-help` : undefined;

  return (
    <div className={containerClassName ? `form-group ${containerClassName}` : 'form-group'}>
      <div className="form-label-row">
        <label htmlFor={selectId} className="form-label-text">
          {label}
          {required && ' *'}
        </label>
        {help && <FieldHelpTooltip content={help} label={`Ayuda para ${label}`} />}
      </div>
      <select
        id={selectId}
        value={value}
        onChange={(e) => { onChange(e.target.value); }}
        required={required}
        className="form-input"
        aria-describedby={helpId}
        disabled={disabled}
      >
        <option value="">{placeholder}</option>
        {options.map((opt) => (
          <option key={opt.value} value={opt.value}>
            {opt.label}
          </option>
        ))}
      </select>
      {help && <span id={helpId} className="visually-hidden">{help}</span>}
    </div>
  );
};