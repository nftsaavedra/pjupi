import React from 'react';
import { GraduationCap, TriangleAlert, UserRound, X } from 'lucide-react';
import type { DocenteDetalle } from '../api';
import { AppIcon } from '../../../shared/ui/AppIcon';

interface DocenteDetailModalProps {
  docente: DocenteDetalle;
  onClose: () => void;
}

export const DocenteDetailModal: React.FC<DocenteDetailModalProps> = ({ docente, onClose }) => {
  const proyectos = docente.proyectos ? docente.proyectos.split(' | ') : [];

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2 className="title-with-icon">
            <AppIcon icon={UserRound} size={20} />
            <span>Detalles de Docente</span>
          </h2>
          <button type="button" className="modal-close" onClick={onClose} aria-label="Cerrar detalles del docente">
            <AppIcon icon={X} size={18} />
          </button>
        </div>

        <div className="modal-body">
          <div className="docente-info">
            <div className="info-row">
              <label>Nombre:</label>
              <span>{docente.nombres_apellidos}</span>
            </div>
            <div className="info-row">
              <label>DNI:</label>
              <span>{docente.dni}</span>
            </div>
            <div className="info-row">
              <label>Grado Académico:</label>
              <span>{docente.grado}</span>
            </div>
            <div className="info-row highlight">
              <label>Proyectos Asignados:</label>
              <span className="badge">{docente.cantidad_proyectos}</span>
            </div>
          </div>

          {docente.cantidad_proyectos > 0 ? (
            <div className="proyectos-section">
              <h3 className="title-with-icon">
                <AppIcon icon={GraduationCap} size={18} />
                <span>Proyectos en los que Participa</span>
              </h3>
              <ul className="proyectos-list">
                {proyectos.map((proyecto, idx) => (
                  <li key={idx} className="proyecto-item">
                    <span className="proyecto-number">{idx + 1}</span>
                    <span className="proyecto-title">{proyecto}</span>
                  </li>
                ))}
              </ul>
            </div>
          ) : (
            <div className="empty-state">
              <p className="title-with-icon empty-state-inline">
                <AppIcon icon={TriangleAlert} size={18} />
                <span>Este docente no tiene proyectos asignados</span>
              </p>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn-secondary" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
};