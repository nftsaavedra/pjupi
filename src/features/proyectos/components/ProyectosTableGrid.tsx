import React, { useState } from 'react';
import { Link2Off, RotateCcw, Trash2, Users, X } from 'lucide-react';
import type { ProyectoDetalle, ProyectoParticipanteResumen } from '../api';
import { SkeletonTable } from '../../../shared/ui/Skeleton';
import { TableActionButton } from '../../../shared/ui/TableActionButton';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { formatRenacytNivel } from '../../../shared/utils/renacyt';

interface ProyectosTableGridProps {
  canManage: boolean;
  loading: boolean;
  proyectos: ProyectoDetalle[];
  onDeactivate: (proyecto: ProyectoDetalle) => void;
  onDetach: (proyecto: ProyectoDetalle) => void;
  onReactivate: (id: string) => void;
}

export const ProyectosTableGrid: React.FC<ProyectosTableGridProps> = ({
  canManage,
  loading,
  proyectos,
  onDeactivate,
  onDetach,
  onReactivate,
}) => {
  const [selectedProyecto, setSelectedProyecto] = useState<{
    titulo: string;
    participantes: ProyectoParticipanteResumen[];
  } | null>(null);

  if (loading) {
    return <SkeletonTable columns={5} rows={6} />;
  }

  if (proyectos.length === 0) {
    return <div className="empty-state">No hay proyectos para el filtro seleccionado</div>;
  }

  return (
    <>
      <table className="table" aria-label="Tabla de proyectos registrados">
        <thead>
          <tr>
            <th>Título</th>
            <th>Docentes relacionados</th>
            <th>Docentes</th>
            <th>Estado</th>
            <th>Acciones</th>
          </tr>
        </thead>
        <tbody>
          {proyectos.map((proyecto) => {
            const participantes = parseParticipantes(proyecto.participantes_json);
            const participantesPreview = participantes.slice(0, 2);
            const showResumenCompacto = participantes.length > 2;

            return (
              <tr key={proyecto.id_proyecto}>
                <td>{proyecto.titulo_proyecto}</td>
                <td>{proyecto.cantidad_docentes}</td>
                <td>
                  {participantes.length > 0 ? (
                    <div className="project-participants-preview">
                      <div className="project-participants-list project-participants-list-compact">
                        {participantesPreview.map((participante, index) => (
                          <article key={`${proyecto.id_proyecto}-${participante.nombre}-${index}`} className="project-participant-card">
                            <strong>{participante.nombre}</strong>
                            <span>{participante.grado}</span>
                            <span>{formatRenacytNivel(participante.renacyt_nivel) ?? 'Sin nivel RENACYT'}</span>
                          </article>
                        ))}
                      </div>
                      {showResumenCompacto && (
                        <button
                          type="button"
                          className="project-participants-more"
                          onClick={() => setSelectedProyecto({ titulo: proyecto.titulo_proyecto, participantes })}
                        >
                          <span className="button-with-icon">
                            <AppIcon icon={Users} size={15} />
                            <span>Ver {participantes.length} participantes</span>
                          </span>
                        </button>
                      )}
                    </div>
                  ) : (
                    proyecto.docentes || '-'
                  )}
                </td>
                <td>
                  {proyecto.activo === 1 ? (
                    <span className="badge badge-success">Activo</span>
                  ) : (
                    <span className="badge badge-warning">Inactivo</span>
                  )}
                </td>
                <td className="table-actions">
                  {canManage ? (
                    <>
                      <TableActionButton
                        className="btn-secondary"
                        icon={Link2Off}
                        label="Desvincular docentes del proyecto"
                        onClick={() => onDetach(proyecto)}
                        disabled={proyecto.activo === 0}
                      />
                      {proyecto.activo === 0 && (
                        <TableActionButton
                          className="btn-primary"
                          icon={RotateCcw}
                          label="Reactivar proyecto"
                          onClick={() => onReactivate(proyecto.id_proyecto)}
                        />
                      )}
                      <TableActionButton
                        className="btn-delete"
                        icon={Trash2}
                        label={proyecto.activo === 1 ? 'Desactivar proyecto' : 'Mantener proyecto inactivo'}
                        onClick={() => onDeactivate(proyecto)}
                      />
                    </>
                  ) : (
                    <span className="table-actions-empty">Solo lectura</span>
                  )}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>

      {selectedProyecto && (
        <div className="modal-overlay" onClick={() => setSelectedProyecto(null)}>
          <div className="modal-content project-participants-modal" onClick={(event) => event.stopPropagation()}>
            <div className="modal-header">
              <h2 className="title-with-icon">
                <AppIcon icon={Users} size={20} />
                <span>Participantes del proyecto</span>
              </h2>
              <button type="button" className="modal-close" onClick={() => setSelectedProyecto(null)} aria-label="Cerrar participantes del proyecto">
                <AppIcon icon={X} size={18} />
              </button>
            </div>

            <div className="modal-body project-participants-modal-body">
              <div className="project-participants-modal-intro">
                <strong>{selectedProyecto.titulo}</strong>
                <span>{selectedProyecto.participantes.length} docentes relacionados</span>
              </div>
              <div className="project-participants-list">
                {selectedProyecto.participantes.map((participante, index) => (
                  <article key={`${selectedProyecto.titulo}-${participante.nombre}-${index}`} className="project-participant-card">
                    <strong>{participante.nombre}</strong>
                    <span>{participante.grado}</span>
                    <span>{formatRenacytNivel(participante.renacyt_nivel) ?? 'Sin nivel RENACYT'}</span>
                  </article>
                ))}
              </div>
            </div>

            <div className="modal-footer">
              <button type="button" className="btn-secondary" onClick={() => setSelectedProyecto(null)}>
                Cerrar
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

const parseParticipantes = (value?: string | null): ProyectoParticipanteResumen[] => {
  if (!value) {
    return [];
  }

  try {
    const parsed = JSON.parse(value);
    return Array.isArray(parsed) ? parsed as ProyectoParticipanteResumen[] : [];
  } catch {
    return [];
  }
};