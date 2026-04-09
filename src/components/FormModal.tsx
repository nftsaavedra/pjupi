import React, { useEffect, useId } from 'react';
import { X } from 'lucide-react';
import { AppIcon } from './AppIcon';

interface FormModalProps {
  open: boolean;
  title: React.ReactNode;
  description?: React.ReactNode;
  children: React.ReactNode;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => void;
  submitText?: React.ReactNode;
  cancelText?: string;
  isLoading?: boolean;
  submitDisabled?: boolean;
  size?: 'md' | 'lg';
}

export const FormModal: React.FC<FormModalProps> = ({
  open,
  title,
  description,
  children,
  onClose,
  onSubmit,
  submitText = 'Guardar',
  cancelText = 'Cancelar',
  isLoading = false,
  submitDisabled = false,
  size = 'md',
}) => {
  const titleId = useId();

  useEffect(() => {
    if (!open) return;

    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = 'hidden';

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && !isLoading) {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      document.body.style.overflow = previousOverflow;
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [isLoading, onClose, open]);

  if (!open) return null;

  return (
    <div className="modal-overlay" onClick={() => { if (!isLoading) onClose(); }}>
      <div
        className={`modal-content form-modal ${size === 'lg' ? 'form-modal-lg' : ''}`}
        onClick={(event) => event.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
      >
        <div className="modal-header form-modal-header">
          <h2 id={titleId}>{title}</h2>
          <button
            type="button"
            className="modal-close"
            onClick={onClose}
            aria-label="Cerrar formulario"
            disabled={isLoading}
          >
            <AppIcon icon={X} size={18} />
          </button>
        </div>

        <form onSubmit={onSubmit} className="form form-modal-form">
          <div className="modal-body form-modal-body">
            {description && <p className="form-modal-description">{description}</p>}
            {children}
          </div>

          <div className="modal-footer form-modal-footer">
            <button type="button" className="btn-secondary" onClick={onClose} disabled={isLoading}>
              {cancelText}
            </button>
            <button type="submit" className="btn-primary" disabled={isLoading || submitDisabled}>
              {isLoading ? 'Procesando...' : submitText}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};