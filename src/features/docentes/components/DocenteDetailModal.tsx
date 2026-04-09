import React from 'react';
import { BadgeCheck, ExternalLink, GraduationCap, TriangleAlert, UserRound, X } from 'lucide-react';
import { openUrl } from '@tauri-apps/plugin-opener';
import type { DocenteDetalle } from '../api';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { toast } from '../../../services/toast';

interface DocenteDetailModalProps {
  docente: DocenteDetalle;
  onClose: () => void;
}

export const DocenteDetailModal: React.FC<DocenteDetailModalProps> = ({ docente, onClose }) => {
  const proyectos = docente.proyectos ? docente.proyectos.split(' | ') : [];
  const tieneRenacyt = Boolean(docente.renacyt_codigo_registro || docente.renacyt_id_investigador);

  const formatDate = (value?: number | null) => {
    if (!value) {
      return 'No disponible';
    }

    return new Intl.DateTimeFormat('es-PE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    }).format(value);
  };

  const handleOpenRenacyt = async () => {
    if (!docente.renacyt_ficha_url) {
      return;
    }

    try {
      await openUrl(docente.renacyt_ficha_url);
    } catch {
      toast.error('No se pudo abrir la ficha pública RENACYT.');
    }
  };

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

          <div className="renacyt-detail-card">
            <div className="renacyt-detail-header">
              <h3 className="title-with-icon">
                <AppIcon icon={BadgeCheck} size={18} />
                <span>Estado RENACYT</span>
              </h3>
              {tieneRenacyt ? (
                <span className="badge badge-success">Vinculado</span>
              ) : (
                <span className="badge badge-warning">No registrado</span>
              )}
            </div>

            {tieneRenacyt ? (
              <>
                <div className="renacyt-detail-grid">
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Código</span>
                    <strong>{docente.renacyt_codigo_registro ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">ID investigador</span>
                    <strong>{docente.renacyt_id_investigador ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Nivel</span>
                    <strong>{docente.renacyt_nivel ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Grupo</span>
                    <strong>{docente.renacyt_grupo ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Condición</span>
                    <strong>{docente.renacyt_condicion ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Registro</span>
                    <strong>{formatDate(docente.renacyt_fecha_registro)}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Informe</span>
                    <strong>{formatDate(docente.renacyt_fecha_informe_calificacion)}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Última revisión</span>
                    <strong>{formatDate(docente.renacyt_fecha_ultima_revision)}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Última sincronización</span>
                    <strong>{formatDate(docente.renacyt_fecha_ultima_sincronizacion)}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">ORCID</span>
                    <strong>{docente.renacyt_orcid ?? 'No disponible'}</strong>
                  </div>
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Scopus Author ID</span>
                    <strong>{docente.renacyt_scopus_author_id ?? 'No disponible'}</strong>
                  </div>
                </div>

                {docente.renacyt_ficha_url && (
                  <div className="renacyt-detail-actions">
                    <button type="button" className="btn-secondary" onClick={handleOpenRenacyt}>
                      <span className="button-with-icon">
                        <AppIcon icon={ExternalLink} size={16} />
                        <span>Ver ficha RENACYT</span>
                      </span>
                    </button>
                  </div>
                )}
              </>
            ) : (
              <p className="renacyt-detail-empty">
                Este docente no tiene una clasificación RENACYT vinculada en su registro actual.
              </p>
            )}
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