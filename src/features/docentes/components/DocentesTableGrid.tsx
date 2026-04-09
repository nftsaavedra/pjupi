import React from 'react';
import { Eye, RefreshCw, RotateCcw, Trash2 } from 'lucide-react';
import type { DocenteDetalle } from '../api';
import { SkeletonTable } from '../../../shared/ui/Skeleton';
import { TableActionButton } from '../../../shared/ui/TableActionButton';

interface DocentesTableGridProps {
  docentes: DocenteDetalle[];
  loading: boolean;
  onView: (docente: DocenteDetalle) => void;
  onRefreshRenacyt: (id: string) => void;
  onReactivate: (id: string) => void;
  onDeactivate: (docente: DocenteDetalle) => void;
  refreshingRenacytDocenteId: string | null;
}

export const DocentesTableGrid: React.FC<DocentesTableGridProps> = ({
  docentes,
  loading,
  onView,
  onRefreshRenacyt,
  onReactivate,
  onDeactivate,
  refreshingRenacytDocenteId,
}) => {
  if (loading) {
    return <SkeletonTable columns={6} rows={6} />;
  }

  if (docentes.length === 0) {
    return <div className="empty-state">No hay docentes para el filtro seleccionado</div>;
  }

  return (
    <table className="table table-interactive" aria-label="Tabla de docentes registrados">
      <thead>
        <tr>
          <th>Nombre</th>
          <th>DNI</th>
          <th>Grado Académico</th>
          <th>Proyectos</th>
          <th>Estado</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        {docentes.map((docente) => (
          (() => {
            const tieneRenacyt = Boolean(docente.renacyt_codigo_registro || docente.renacyt_id_investigador);
            const tieneFormaciones = Boolean(docente.renacyt_formaciones_academicas_json?.trim());
            const estaActualizando = refreshingRenacytDocenteId === docente.id_docente;

            return (
          <tr
            key={docente.id_docente}
            className={docente.cantidad_proyectos === 0 ? 'unassigned' : ''}
          >
            <td className="font-semibold">{docente.nombres_apellidos || 'Sin nombre registrado'}</td>
            <td>{docente.dni || 'Sin DNI'}</td>
            <td>{docente.grado || 'Sin grado'}</td>
            <td>
              <span
                className={`badge badge-${
                  docente.cantidad_proyectos === 0 ? 'warning' : 'success'
                }`}
              >
                {docente.cantidad_proyectos}
              </span>
            </td>
            <td>
              {docente.activo === 1 ? (
                <span className="badge badge-success">Activo</span>
              ) : (
                <span className="badge badge-warning">Inactivo</span>
              )}
            </td>
            <td className="table-actions">
              <TableActionButton
                className="btn-view"
                icon={Eye}
                label="Ver detalles"
                onClick={() => onView(docente)}
              />
              {tieneRenacyt && (
                <TableActionButton
                  className="btn-secondary"
                  icon={RefreshCw}
                  label={estaActualizando ? 'Actualizando formación RENACYT' : tieneFormaciones ? 'Actualizar formación RENACYT' : 'Reintentar formación RENACYT'}
                  onClick={() => onRefreshRenacyt(docente.id_docente)}
                  disabled={estaActualizando}
                />
              )}
              {docente.activo === 0 && (
                <TableActionButton
                  className="btn-primary"
                  icon={RotateCcw}
                  iconSize={18}
                  label="Reactivar docente"
                  onClick={() => onReactivate(docente.id_docente)}
                />
              )}
              {docente.activo === 1 && (
                <TableActionButton
                  className="btn-delete"
                  icon={Trash2}
                  label="Desactivar docente"
                  onClick={() => onDeactivate(docente)}
                />
              )}
            </td>
          </tr>
            );
          })()
        ))}
      </tbody>
    </table>
  );
};