import React, { useState } from 'react';
import { BadgeCheck, ChevronDown, ChevronUp, ExternalLink, GraduationCap, RefreshCw, TriangleAlert, UserRound, X } from 'lucide-react';
import { openUrl } from '@tauri-apps/plugin-opener';
import type { DocenteDetalle, RenacytFormacionAcademicaResumen } from '../api';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { InlineIconButton } from '../../../shared/ui/InlineIconButton';
import { toast } from '../../../services/toast';
import { formatRenacytNivel } from '../../../shared/utils/renacyt';

interface DocenteDetailModalProps {
  canRefreshRenacyt: boolean;
  docente: DocenteDetalle;
  onClose: () => void;
  onRefreshRenacytFormaciones: (id: string) => void;
  isRefreshingRenacyt: boolean;
}

type ExternalBrand = 'renacyt' | 'orcid' | 'scopus';

export const DocenteDetailModal: React.FC<DocenteDetailModalProps> = ({
  canRefreshRenacyt,
  docente,
  onClose,
  onRefreshRenacytFormaciones,
  isRefreshingRenacyt,
}) => {
  const proyectos = docente.proyectos ? docente.proyectos.split(' | ') : [];
  const tieneRenacyt = Boolean(docente.renacyt_codigo_registro || docente.renacyt_id_investigador);
  const [renacytExpanded, setRenacytExpanded] = useState(true);
  const [formacionesExpanded, setFormacionesExpanded] = useState(false);

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

  const handleOpenExternalUrl = async (url: string, errorMessage: string) => {
    try {
      await openUrl(url);
    } catch {
      toast.error(errorMessage);
    }
  };

  const scopusUrl = docente.renacyt_scopus_author_id
    ? `https://www.scopus.com/authid/detail.uri?authorId=${encodeURIComponent(docente.renacyt_scopus_author_id)}`
    : null;
  const orcidUrl = docente.renacyt_orcid
    ? `https://orcid.org/${encodeURIComponent(docente.renacyt_orcid)}`
    : null;
  const formacionesAcademicas = parseFormacionesAcademicas(docente.renacyt_formaciones_academicas_json);

  const renderBrandLabel = (
    label: string,
    brand?: ExternalBrand,
    action?: {
      tooltip: string;
      onClick: () => void;
    },
  ) => (
    <span className="renacyt-detail-label-row">
      <span className="renacyt-detail-label-main">
        {brand && (
          <span className={`brand-mark brand-mark-${brand}`} aria-hidden="true">
            {brand === 'renacyt' ? 'R' : brand === 'orcid' ? 'O' : 'S'}
          </span>
        )}
        <span className="renacyt-detail-label">{label}</span>
      </span>
      {action && (
        <InlineIconButton
          icon={ExternalLink}
          label={action.tooltip}
          onClick={action.onClick}
        />
      )}
    </span>
  );

  const renderFormacionDate = (value?: number | null) => {
    if (!value) {
      return 'No disponible';
    }

    return new Intl.DateTimeFormat('es-PE', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    }).format(value);
  };

  const renderLinkedIdentifier = (
    label: string,
    value: string | null | undefined,
    url: string | null,
    actionLabel: string,
    errorMessage: string,
    brand?: ExternalBrand,
  ) => (
    <div className="renacyt-detail-item renacyt-detail-item-linked">
      {renderBrandLabel(label, brand, url ? {
        tooltip: actionLabel,
        onClick: () => void handleOpenExternalUrl(url, errorMessage),
      } : undefined)}
      <div className="renacyt-detail-item-content">
        <strong>{value ?? 'No disponible'}</strong>
      </div>
    </div>
  );

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
            <button
              type="button"
              className="renacyt-detail-toggle"
              onClick={() => setRenacytExpanded((current) => !current)}
              aria-expanded={renacytExpanded}
            >
              <span className="renacyt-detail-toggle-copy">
                <span className="title-with-icon renacyt-detail-title">
                  <AppIcon icon={BadgeCheck} size={18} />
                  <span>Estado RENACYT</span>
                </span>
                {tieneRenacyt ? (
                  <span className="badge badge-success">Vinculado</span>
                ) : (
                  <span className="badge badge-warning">No registrado</span>
                )}
              </span>
              <span className="renacyt-detail-toggle-icon" aria-hidden="true">
                <AppIcon icon={renacytExpanded ? ChevronUp : ChevronDown} size={18} />
              </span>
            </button>

            {renacytExpanded && (tieneRenacyt ? (
              <>
                <div className="renacyt-detail-grid">
                  <div className="renacyt-detail-item">
                    {renderBrandLabel('Código', 'renacyt')}
                    <strong>{docente.renacyt_codigo_registro ?? 'No disponible'}</strong>
                  </div>
                  {renderLinkedIdentifier(
                    'ID investigador',
                    docente.renacyt_id_investigador,
                    docente.renacyt_ficha_url ?? null,
                    'Abrir ficha RENACYT',
                    'No se pudo abrir la ficha pública RENACYT.',
                    'renacyt',
                  )}
                  <div className="renacyt-detail-item">
                    <span className="renacyt-detail-label">Nivel</span>
                    <strong>{formatRenacytNivel(docente.renacyt_nivel) ?? 'No disponible'}</strong>
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
                  {renderLinkedIdentifier(
                    'ORCID',
                    docente.renacyt_orcid,
                    orcidUrl,
                    'Abrir ORCID',
                    'No se pudo abrir el perfil de ORCID.',
                    'orcid',
                  )}
                  {renderLinkedIdentifier(
                    'Scopus Author ID',
                    docente.renacyt_scopus_author_id,
                    scopusUrl,
                    'Abrir Scopus',
                    'No se pudo abrir el perfil de Scopus.',
                    'scopus',
                  )}
                </div>

                {canRefreshRenacyt && (
                  <div className="renacyt-detail-actions">
                    <button
                      type="button"
                      className="btn-secondary"
                      onClick={() => onRefreshRenacytFormaciones(docente.id_docente)}
                      disabled={isRefreshingRenacyt}
                    >
                      <span className="button-with-icon">
                        <AppIcon icon={RefreshCw} size={16} />
                        <span>
                          {isRefreshingRenacyt
                            ? 'Actualizando formación...'
                            : formacionesAcademicas.length > 0
                              ? 'Actualizar formación académica'
                              : 'Reintentar formación académica'}
                        </span>
                      </span>
                    </button>
                  </div>
                )}

                {canRefreshRenacyt && formacionesAcademicas.length === 0 && (
                  <div className="inline-feedback inline-feedback-info renacyt-formaciones-feedback">
                    <span>No hay formación académica RENACYT sincronizada para este docente. Puede reintentar la consulta.</span>
                  </div>
                )}

                {formacionesAcademicas.length > 0 && (
                  <div className="renacyt-subsection">
                    <button
                      type="button"
                      className="renacyt-subsection-toggle"
                      onClick={() => setFormacionesExpanded((current) => !current)}
                      aria-expanded={formacionesExpanded}
                    >
                      <span className="renacyt-subsection-toggle-copy">
                        <span className="title-with-icon renacyt-subsection-title">
                          <span className="brand-mark brand-mark-renacyt" aria-hidden="true">F</span>
                          <span>Formación académica</span>
                        </span>
                        <span className="badge badge-info">{formacionesAcademicas.length}</span>
                      </span>
                      <span className="renacyt-detail-toggle-icon" aria-hidden="true">
                        <AppIcon icon={formacionesExpanded ? ChevronUp : ChevronDown} size={18} />
                      </span>
                    </button>

                    {formacionesExpanded && (
                      <div className="renacyt-formaciones-list">
                        {formacionesAcademicas.map((formacion) => (
                          <article key={formacion.id} className="renacyt-formacion-card">
                            <div className="renacyt-formacion-head">
                              <strong>{formacion.titulo ?? 'Formación sin título'}</strong>
                              <span className={`badge ${formacion.considerado_para_cc ? 'badge-success' : 'badge-warning'}`}>
                                {formacion.considerado_para_cc ? 'Considerado CC' : 'Informativo'}
                              </span>
                            </div>
                            <div className="renacyt-formacion-grid">
                              <span><strong>Grado:</strong> {formacion.grado_academico ?? 'No disponible'}</span>
                              <span><strong>Centro:</strong> {formacion.centro_estudios ?? 'No disponible'}</span>
                              {hasFormacionDate(formacion.fecha_inicio) && (
                                <span><strong>Inicio:</strong> {renderFormacionDate(formacion.fecha_inicio)}</span>
                              )}
                              {hasFormacionDate(formacion.fecha_fin) && (
                                <span><strong>Fin:</strong> {renderFormacionDate(formacion.fecha_fin)}</span>
                              )}
                              <span><strong>Puntaje:</strong> {formacion.puntaje_obtenido ?? 'No disponible'}</span>
                              <span><strong>Origen:</strong> {formacion.indicador_importado ? 'Importado' : 'Manual'}</span>
                            </div>
                          </article>
                        ))}
                      </div>
                    )}
                  </div>
                )}

              </>
            ) : (
              <p className="renacyt-detail-empty">
                Este docente no tiene una clasificación RENACYT vinculada en su registro actual.
              </p>
            ))}
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

const parseFormacionesAcademicas = (value?: string | null): RenacytFormacionAcademicaResumen[] => {
  if (!value) {
    return [];
  }

  try {
    const parsed = JSON.parse(value);
    return Array.isArray(parsed) ? parsed as RenacytFormacionAcademicaResumen[] : [];
  } catch {
    return [];
  }
};

const hasFormacionDate = (value?: number | null) => {
  return Boolean(value && value > 0);
};