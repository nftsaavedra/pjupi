import React from 'react';
import { GraduationCap, Plus } from 'lucide-react';
import { useDocentesTable } from '../hooks/useDocentesTable';
import { DocenteDetailModal } from './DocenteDetailModal';
import { DocentesTableGrid } from './DocentesTableGrid';
import { DocentesTableToolbar } from './DocentesTableToolbar';
import { ConfirmDialog } from '@/shared/overlays/ConfirmDialog';
import { AppIcon } from '@/shared/ui/AppIcon';

interface DocentesTableProps {
  canManage: boolean;
  onCreateClick: () => void;
  refreshTrigger?: number;
}

export const DocentesTable: React.FC<DocentesTableProps> = ({
  canManage,
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
    gradoFiltro,
    gradosDisponibles,
    handleEliminarDocente,
    handleRefreshRenacytFormaciones,
    handleReactivarDocente,
    loading,
    nivelesRenacytDisponibles,
    renacytNivelFiltro,
    refreshingRenacytDocenteId,
    selectedDocente,
    setBusqueda,
    setDocenteToDelete,
    setEstadoFiltro,
    setGradoFiltro,
    setRenacytNivelFiltro,
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
          {canManage && (
            <div className="section-header-actions">
              <button type="button" className="btn-primary" onClick={onCreateClick}>
                <span className="button-with-icon">
                  <AppIcon icon={Plus} size={18} />
                  <span>Nuevo docente</span>
                </span>
              </button>
            </div>
          )}
        </div>
        {error && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se muestran los datos ya cargados.</span>
            <button type="button" className="btn-secondary" onClick={() => void cargarDocentes()}>
              Reintentar
            </button>
          </div>
        )}
        {!canManage && (
          <div className="inline-feedback inline-feedback-info">
            <span>Modo consulta: puede revisar docentes y su detalle, pero no crear, desactivar, reactivar ni refrescar información RENACYT.</span>
          </div>
        )}
        <DocentesTableToolbar
          busqueda={busqueda}
          estadoFiltro={estadoFiltro}
          gradoFiltro={gradoFiltro}
          gradosDisponibles={gradosDisponibles}
          nivelesRenacytDisponibles={nivelesRenacytDisponibles}
          renacytNivelFiltro={renacytNivelFiltro}
          totalVisibles={docentesFiltrados.length}
          totalTodos={docentes.length}
          totalActivos={totalActivos}
          totalInactivos={totalInactivos}
          onBusquedaChange={setBusqueda}
          onEstadoFiltroChange={setEstadoFiltro}
          onGradoFiltroChange={setGradoFiltro}
          onRenacytNivelFiltroChange={setRenacytNivelFiltro}
        />
        <DocentesTableGrid
          docentes={docentesFiltrados}
          loading={loading}
          onView={setSelectedDocente}
          onRefreshRenacyt={(id) => { void handleRefreshRenacytFormaciones(id); }}
          onReactivate={(id) => { void handleReactivarDocente(id); }}
          onDeactivate={setDocenteToDelete}
          refreshingRenacytDocenteId={refreshingRenacytDocenteId}
          canManage={canManage}
        />
      </div>

      {selectedDocente && (
        <DocenteDetailModal
          docente={selectedDocente}
          onClose={() => { setSelectedDocente(null); }}
          onRefreshRenacytFormaciones={(id) => { void handleRefreshRenacytFormaciones(id); }}
          isRefreshingRenacyt={refreshingRenacytDocenteId === selectedDocente.id_docente}
          canRefreshRenacyt={canManage}
          canSyncPure={canManage}
        />
      )}

      {canManage && (
        <ConfirmDialog
          open={Boolean(docenteToDelete)}
          title="Desactivar docente"
          message={`¿Desea desactivar al docente "${docenteToDelete?.nombres_apellidos ?? ''}"? Su historial y relaciones se conservarán para mantener la trazabilidad.`}
          confirmText="Sí, desactivar"
          cancelText="Cancelar"
          onConfirm={() => { void handleEliminarDocente(); }}
          onCancel={() => { setDocenteToDelete(null); }}
        />
      )}
    </div>
  );
};