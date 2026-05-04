import React from 'react';
import { Plus } from 'lucide-react';
import { useProyectosTab } from './hooks/useProyectosTab';
import { ConfirmDialog } from '@/shared/overlays/ConfirmDialog';
import { AppIcon } from '@/shared/ui/AppIcon';
import { ProyectoCreateModal } from './components/ProyectoCreateModal';
import { ProyectoEditModal } from './components/ProyectoEditModal';
import { ProyectosTableGrid } from './components/ProyectosTableGrid';
import { ProyectosToolbar } from './components/ProyectosToolbar';

interface ProyectosTabProps {
  canManage: boolean;
  onProyectoCreated: () => void;
  refreshTrigger?: number;
}

export const ProyectosTab: React.FC<ProyectosTabProps> = ({ canManage, onProyectoCreated, refreshTrigger = 0 }) => {
  const {
    busqueda,
    cargarProyectos,
    docentes,
    docenteResponsableId,
    docentesSeleccionados,
    estadoFiltro,
    handleActualizarProyecto,
    handleChangeDocentesSeleccionados,
    handleCloseForm,
    handleEliminarProyecto,
    handleOpenCreate,
    handleReactivarProyecto,
    handleSubmit,
    isFormOpen,
    isLoading,
    loadingDocentes,
    loadingProyectos,
    proyectoToDelete,
    proyectoToEdit,
    proyectos,
    proyectosError,
    proyectosFiltrados,
    refreshingDocentes,
    setBusqueda,
    setDocenteResponsableId,
    setEstadoFiltro,
    setProyectoToDelete,
    setProyectoToEdit,
    setTitulo,
    titulo,
    totalActivos,
    totalInactivos,
    // Recursos
    patentes,
    productos,
    equipamientos,
    financiamientos,
    handlePatentesChange,
    handleProductosChange,
    handleEquipamientosChange,
    handleFinanciamientosChange,
  } = useProyectosTab(refreshTrigger, onProyectoCreated);

  return (
    <div className="tab-panel module-shell proyectos-module">
      <div className="table-container">
        <div className="section-header">
          <h2>Proyectos Registrados</h2>
          {canManage && (
            <div className="section-header-actions">
              <button type="button" className="btn-primary" onClick={handleOpenCreate}>
                <span className="button-with-icon">
                  <AppIcon icon={Plus} size={18} />
                  <span>Nuevo proyecto</span>
                </span>
              </button>
            </div>
          )}
        </div>
        {proyectosError && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se conservan los datos visibles.</span>
            <button type="button" className="btn-secondary" onClick={() => void cargarProyectos()}>
              Reintentar
            </button>
          </div>
        )}
        {!canManage && (
          <div className="inline-feedback inline-feedback-info">
            <span>Modo consulta: puede revisar proyectos y participantes, pero no crear, desvincular, desactivar ni reactivar.</span>
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
          onEdit={setProyectoToEdit}
          onReactivate={(id) => { void handleReactivarProyecto(id); }}
          canManage={canManage}
        />
      </div>

      {canManage && (
        <ProyectoCreateModal
          docentes={docentes}
          docentesSeleccionados={docentesSeleccionados}
          docenteResponsableId={docenteResponsableId}
          isLoading={isLoading}
          loadingDocentes={loadingDocentes}
          open={isFormOpen}
          refreshingDocentes={refreshingDocentes}
          titulo={titulo}
          patentes={patentes.map((p) => ({ ...p, id: p.id_patente }))}
          productos={productos.map((p) => ({ ...p, id: p.id_producto }))}
          equipamientos={equipamientos.map((e) => ({ ...e, id: e.id_equipamiento }))}
          financiamientos={financiamientos.map((f) => ({ ...f, id: f.id_financiamiento }))}
          onChangeDocentes={handleChangeDocentesSeleccionados}
          onChangeResponsable={setDocenteResponsableId}
          onClose={handleCloseForm}
          onSubmit={(e) => { void handleSubmit(e); }}
          onTituloChange={setTitulo}
          onPatentesChange={(items) => { void handlePatentesChange(items); }}
          onProductosChange={(items) => { void handleProductosChange(items); }}
          onEquipamientosChange={(items) => { void handleEquipamientosChange(items); }}
          onFinanciamientosChange={(items) => { void handleFinanciamientosChange(items); }}
        />
      )}

      {canManage && (
        <ProyectoEditModal
          docentes={docentes}
          isLoading={isLoading}
          loadingDocentes={loadingDocentes}
          open={Boolean(proyectoToEdit)}
          proyecto={proyectoToEdit}
          refreshingDocentes={refreshingDocentes}
          patentes={patentes.map((p) => ({ ...p, id: p.id_patente }))}
          productos={productos.map((p) => ({ ...p, id: p.id_producto }))}
          equipamientos={equipamientos.map((e) => ({ ...e, id: e.id_equipamiento }))}
          financiamientos={financiamientos.map((f) => ({ ...f, id: f.id_financiamiento }))}
          onClose={() => { setProyectoToEdit(null); }}
          onSubmit={(id, payload) => { void handleActualizarProyecto(id, payload); }}
          onPatentesChange={(items) => { void handlePatentesChange(items); }}
          onProductosChange={(items) => { void handleProductosChange(items); }}
          onEquipamientosChange={(items) => { void handleEquipamientosChange(items); }}
          onFinanciamientosChange={(items) => { void handleFinanciamientosChange(items); }}
        />
      )}

      {canManage && (
        <ConfirmDialog
          open={Boolean(proyectoToDelete)}
          title="Desactivar proyecto"
          message={`¿Desea desactivar el proyecto "${proyectoToDelete?.titulo_proyecto ?? ''}"? Solo se desactivará si no tiene docentes relacionados.`}
          confirmText="Sí, desactivar"
          cancelText="Cancelar"
          onConfirm={() => { void handleEliminarProyecto(); }}
          onCancel={() => { setProyectoToDelete(null); }}
        />
      )}
    </div>
  );
};