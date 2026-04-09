import React from 'react';
import { GraduationCap, Plus } from 'lucide-react';
import { useDocentesTable } from '../hooks/useDocentesTable';
import { DocenteDetailModal } from './DocenteDetailModal';
import { DocentesTableGrid } from './DocentesTableGrid';
import { DocentesTableToolbar } from './DocentesTableToolbar';
import { ConfirmDialog } from '../../../shared/overlays/ConfirmDialog';
import { AppIcon } from '../../../shared/ui/AppIcon';

interface DocentesTableProps {
  onCreateClick: () => void;
  refreshTrigger?: number;
}

export const DocentesTable: React.FC<DocentesTableProps> = ({
  onCreateClick,
  refreshTrigger = 0,
}) => {
  const {
    busqueda,
    cargarDocentes,
    docenteToDelete,
    docentes,
    docentesFiltrados,
    error,
    estadoFiltro,
    handleEliminarDocente,
    handleReactivarDocente,
    loading,
    selectedDocente,
    setBusqueda,
    setDocenteToDelete,
    setEstadoFiltro,
    setSelectedDocente,
    totalActivos,
    totalInactivos,
  } = useDocentesTable(refreshTrigger);

  return (
    <div className="tab-panel docentes-list-panel">
      <div className="table-container">
        <div className="section-header">
          <h2 className="title-with-icon">
            <AppIcon icon={GraduationCap} size={20} />
            <span>Docentes Registrados</span>
          </h2>
          <div className="section-header-actions">
            <button type="button" className="btn-primary" onClick={onCreateClick}>
              <span className="button-with-icon">
                <AppIcon icon={Plus} size={18} />
                <span>Nuevo docente</span>
              </span>
            </button>
          </div>
        </div>
        {error && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se muestran los datos ya cargados.</span>
            <button type="button" className="btn-secondary" onClick={() => void cargarDocentes()}>
              Reintentar
            </button>
          </div>
        )}
        <DocentesTableToolbar
          busqueda={busqueda}
          estadoFiltro={estadoFiltro}
          totalVisibles={docentesFiltrados.length}
          totalTodos={docentes.length}
          totalActivos={totalActivos}
          totalInactivos={totalInactivos}
          onBusquedaChange={setBusqueda}
          onEstadoFiltroChange={setEstadoFiltro}
        />
        <DocentesTableGrid
          docentes={docentesFiltrados}
          loading={loading}
          onView={setSelectedDocente}
          onReactivate={handleReactivarDocente}
          onDeactivate={setDocenteToDelete}
        />
      </div>

      {selectedDocente && (
        <DocenteDetailModal
          docente={selectedDocente}
          onClose={() => setSelectedDocente(null)}
        />
      )}

      <ConfirmDialog
        open={Boolean(docenteToDelete)}
        title="Desactivar docente"
        message={`¿Desea desactivar al docente "${docenteToDelete?.nombres_apellidos ?? ''}"? Su historial y relaciones se conservarán para mantener la trazabilidad.`}
        confirmText="Sí, desactivar"
        cancelText="Cancelar"
        onConfirm={handleEliminarDocente}
        onCancel={() => setDocenteToDelete(null)}
      />
    </div>
  );
};