import React from 'react';
import { Plus } from 'lucide-react';
import { useProyectosTab } from './hooks/useProyectosTab';
import { ConfirmDialog } from '../../shared/overlays/ConfirmDialog';
import { AppIcon } from '../../shared/ui/AppIcon';
import { ProyectoCreateModal } from './components/ProyectoCreateModal';
import { ProyectosTableGrid } from './components/ProyectosTableGrid';
import { ProyectosToolbar } from './components/ProyectosToolbar';

interface ProyectosTabProps {
  onProyectoCreated: () => void;
  refreshTrigger?: number;
}

export const ProyectosTab: React.FC<ProyectosTabProps> = ({ onProyectoCreated, refreshTrigger = 0 }) => {
  const {
    busqueda,
    cargarProyectos,
    docentes,
    docentesSeleccionados,
    estadoFiltro,
    handleCloseForm,
    handleDesvincularDocentes,
    handleEliminarProyecto,
    handleOpenCreate,
    handleReactivarProyecto,
    handleSubmit,
    isFormOpen,
    isLoading,
    loadingDocentes,
    loadingProyectos,
    proyectoToDelete,
    proyectoToDetach,
    proyectos,
    proyectosError,
    proyectosFiltrados,
    refreshingDocentes,
    setBusqueda,
    setDocentesSeleccionados,
    setEstadoFiltro,
    setProyectoToDelete,
    setProyectoToDetach,
    setTitulo,
    titulo,
    totalActivos,
    totalInactivos,
  } = useProyectosTab(refreshTrigger, onProyectoCreated);

  return (
    <div className="tab-panel module-shell proyectos-module">
      <div className="table-container">
        <div className="section-header">
          <h2>Proyectos Registrados</h2>
          <div className="section-header-actions">
            <button type="button" className="btn-primary" onClick={handleOpenCreate}>
              <span className="button-with-icon">
                <AppIcon icon={Plus} size={18} />
                <span>Nuevo proyecto</span>
              </span>
            </button>
          </div>
        </div>
        {proyectosError && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se conservan los datos visibles.</span>
            <button type="button" className="btn-secondary" onClick={() => void cargarProyectos()}>
              Reintentar
            </button>
          </div>
        )}
        <ProyectosToolbar
          busqueda={busqueda}
          estadoFiltro={estadoFiltro}
          totalActivos={totalActivos}
          totalInactivos={totalInactivos}
          totalTodos={proyectos.length}
          totalVisibles={proyectosFiltrados.length}
          onBusquedaChange={setBusqueda}
          onEstadoFiltroChange={setEstadoFiltro}
        />
        <ProyectosTableGrid
          loading={loadingProyectos}
          proyectos={proyectosFiltrados}
          onDeactivate={setProyectoToDelete}
          onDetach={setProyectoToDetach}
          onReactivate={handleReactivarProyecto}
        />
      </div>

      <ProyectoCreateModal
        docentes={docentes}
        docentesSeleccionados={docentesSeleccionados}
        isLoading={isLoading}
        loadingDocentes={loadingDocentes}
        open={isFormOpen}
        refreshingDocentes={refreshingDocentes}
        titulo={titulo}
        onChangeDocentes={setDocentesSeleccionados}
        onClose={handleCloseForm}
        onSubmit={handleSubmit}
        onTituloChange={setTitulo}
      />

      <ConfirmDialog
        open={Boolean(proyectoToDetach)}
        title="Desvincular docentes del proyecto"
        message={`Se eliminarán todas las relaciones docentes del proyecto "${proyectoToDetach?.titulo_proyecto ?? ''}".`}
        confirmText="Sí, desvincular"
        cancelText="Cancelar"
        onConfirm={handleDesvincularDocentes}
        onCancel={() => setProyectoToDetach(null)}
      />

      <ConfirmDialog
        open={Boolean(proyectoToDelete)}
        title="Desactivar proyecto"
        message={`¿Desea desactivar el proyecto "${proyectoToDelete?.titulo_proyecto ?? ''}"? Solo se desactivará si no tiene docentes relacionados.`}
        confirmText="Sí, desactivar"
        cancelText="Cancelar"
        onConfirm={handleEliminarProyecto}
        onCancel={() => setProyectoToDelete(null)}
      />
    </div>
  );
};