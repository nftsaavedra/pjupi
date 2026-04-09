import React from 'react';
import { Link2Off, RotateCcw, Trash2 } from 'lucide-react';
import type { ProyectoDetalle } from '../api';
import { SkeletonTable } from '../../../shared/ui/Skeleton';
import { TableActionButton } from '../../../shared/ui/TableActionButton';

interface ProyectosTableGridProps {
  loading: boolean;
  proyectos: ProyectoDetalle[];
  onDeactivate: (proyecto: ProyectoDetalle) => void;
  onDetach: (proyecto: ProyectoDetalle) => void;
  onReactivate: (id: string) => void;
}

export const ProyectosTableGrid: React.FC<ProyectosTableGridProps> = ({
  loading,
  proyectos,
  onDeactivate,
  onDetach,
  onReactivate,
}) => {
  if (loading) {
    return <SkeletonTable columns={5} rows={6} />;
  }

  if (proyectos.length === 0) {
    return <div className="empty-state">No hay proyectos para el filtro seleccionado</div>;
  }

  return (
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
        {proyectos.map((proyecto) => (
          <tr key={proyecto.id_proyecto}>
            <td>{proyecto.titulo_proyecto}</td>
            <td>{proyecto.cantidad_docentes}</td>
            <td>{proyecto.docentes || '-'}</td>
            <td>
              {proyecto.activo === 1 ? (
                <span className="badge badge-success">Activo</span>
              ) : (
                <span className="badge badge-warning">Inactivo</span>
              )}
            </td>
            <td className="table-actions">
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
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};