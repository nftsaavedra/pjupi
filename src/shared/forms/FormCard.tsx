import React from 'react';

interface FormCardProps {
  title: React.ReactNode;
  onSubmit: (e: React.FormEvent) => void;
  children: React.ReactNode;
  submitText?: React.ReactNode;
  isLoading?: boolean;
}

export const FormCard: React.FC<FormCardProps> = ({
  title,
  onSubmit,
  children,
  submitText = 'Guardar',
  isLoading = false,
}) => (
  <div className="form-card">
    <h2>{title}</h2>
    <form onSubmit={onSubmit} className="form">
      {children}
      <button type="submit" className="btn-primary" disabled={isLoading}>
        {isLoading ? 'Procesando...' : submitText}
      </button>
    </form>
  </div>
);