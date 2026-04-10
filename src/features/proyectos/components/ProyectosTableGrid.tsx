import React, { useState } from 'react';
import { Pencil, RotateCcw, Trash2, Users, X } from 'lucide-react';
import type { ProyectoDetalle, ProyectoParticipanteResumen } from '../api';
import { SkeletonTable } from '../../../shared/ui/Skeleton';
import { TableActionButton } from '../../../shared/ui/TableActionButton';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { formatRenacytNivel } from '../../../shared/utils/renacyt';
import { getResponsableProyecto, parseParticipantesProyecto } from '../participantes';

interface ProyectosTableGridProps {
  canManage: boolean;
  loading: boolean;
  proyectos: ProyectoDetalle[];
  onDeactivate: (proyecto: ProyectoDetalle) => void;
  onEdit: (proyecto: ProyectoDetalle) => void;
  onReactivate: (id: string) => void;
}

export const ProyectosTableGrid: React.FC<ProyectosTableGridProps> = ({
  canManage,
  loading,
  proyectos,
  onDeactivate,
  onEdit,
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
            <th>Responsable</th>
            <th>Docentes</th>
            <th>Estado</th>
            <th>Acciones</th>
          </tr>
        </thead>
        <tbody>
          {proyectos.map((proyecto) => {
            const participantes = parseParticipantesProyecto(proyecto.participantes_json);
            const responsable = getResponsableProyecto(participantes);

            return (
              <tr key={proyecto.id_proyecto}>
                <td>{proyecto.titulo_proyecto}</td>
                <td>
                  {responsable ? (
                    <div className="project-responsable-cell">
                      <strong>{responsable.nombre}</strong>
                    </div>
                  ) : (
                    <span className="project-responsable-empty">Sin responsable</span>
                  )}
                </td>
                <td>
                  <button
                    type="button"
                    className="project-docentes-trigger"
                    onClick={() => setSelectedProyecto({ titulo: proyecto.titulo_proyecto, participantes })}
                    disabled={participantes.length === 0}
                  >
                    <span className="button-with-icon">
                      <AppIcon icon={Users} size={15} />
                      <span>{proyecto.cantidad_docentes} docente{proyecto.cantidad_docentes === 1 ? '' : 's'}</span>
                    </span>
                  </button>
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
                      {proyecto.activo === 1 && (
                        <TableActionButton
                          className="btn-secondary"
                          icon={Pencil}
                          label="Editar proyecto"
                          onClick={() => onEdit(proyecto)}
                        />
                      )}
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
                    <div className="project-participant-card-head">
                      <strong>{participante.nombre}</strong>
                      {participante.es_responsable && <span className="badge badge-info">Responsable</span>}
                    </div>
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
