import React, { useState } from 'react';
import { FolderOpen, Link2Off, Plus, RotateCcw, Trash2 } from 'lucide-react';
import {
  crearProyectoConParticipantes,
  eliminarProyecto,
  eliminarRelacionesProyecto,
  getAllProyectosDetalle,
  getTauriErrorMessage,
  reactivarProyecto,
  type ProyectoDetalle,
} from '../services/tauriApi';
import { toast } from '../services/toast';
import { useFetchDocentes, useStableFetchData } from '../hooks/useFetch';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { FormModal } from './FormModal';
import { FormInput } from './FormInput';
import { DocentesChecklist } from './DocentesChecklist';
import { ConfirmDialog } from './ConfirmDialog';
import { SkeletonTable } from './Skeleton';
import { AppIcon } from './AppIcon';
import { TableActionButton } from './TableActionButton';

interface ProyectosTabProps {
  onProyectoCreated: () => void;
  refreshTrigger?: number;
}

export const ProyectosTab: React.FC<ProyectosTabProps> = ({ onProyectoCreated, refreshTrigger = 0 }) => {
  const [titulo, setTitulo] = useState('');
  const [docentesSeleccionados, setDocentesSeleccionados] = useState<string[]>([]);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [proyectoToDelete, setProyectoToDelete] = useState<ProyectoDetalle | null>(null);
  const [proyectoToDetach, setProyectoToDetach] = useState<ProyectoDetalle | null>(null);
  const [estadoFiltro, setEstadoFiltro] = useState<'todos' | 'activos' | 'inactivos'>('activos');
  const [busqueda, setBusqueda] = useState('');

  const {
    docentes,
    loading: loadingDocentes,
    refreshing: refreshingDocentes,
  } = useFetchDocentes(refreshTrigger);
  const {
    data: proyectos,
    loading: loadingProyectos,
    refreshing: refreshingProyectos,
    error: proyectosError,
    recargar: cargarProyectos,
  } = useStableFetchData<ProyectoDetalle[]>(
    getAllProyectosDetalle,
    refreshTrigger,
    'Error cargando proyectos',
    [],
  );

  useRefreshToast({
    refreshing: refreshingProyectos,
    message: 'Actualizando proyectos',
    toastKey: 'proyectos-refresh',
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!titulo.trim()) {
      toast.warning('Ingrese el título del proyecto');
      return;
    }

    if (docentesSeleccionados.length === 0) {
      toast.warning('Seleccione al menos un docente');
      return;
    }

    setIsLoading(true);
    try {
      await crearProyectoConParticipantes(titulo, docentesSeleccionados);
      toast.success('Proyecto creado exitosamente');
      setTitulo('');
      setDocentesSeleccionados([]);
      setIsFormOpen(false);
      onProyectoCreated();
      await cargarProyectos();
    } catch (error) {
      toast.error('Error al crear proyecto: ' + getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleDesvincularDocentes = async () => {
    if (!proyectoToDetach) return;
    try {
      await eliminarRelacionesProyecto(proyectoToDetach.id_proyecto);
      toast.success('Se eliminaron las relaciones docente-proyecto');
      setProyectoToDetach(null);
      await cargarProyectos();
      onProyectoCreated();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleEliminarProyecto = async () => {
    if (!proyectoToDelete) return;
    try {
      const resultado = await eliminarProyecto(proyectoToDelete.id_proyecto);
      toast.info(resultado.mensaje);
      setProyectoToDelete(null);
      await cargarProyectos();
      onProyectoCreated();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleReactivarProyecto = async (id: string) => {
    try {
      await reactivarProyecto(id);
      toast.success('Proyecto reactivado correctamente');
      await cargarProyectos();
      onProyectoCreated();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const totalActivos = proyectos.filter((proyecto) => proyecto.activo === 1).length;
  const totalInactivos = proyectos.filter((proyecto) => proyecto.activo === 0).length;

  const resetForm = () => {
    setTitulo('');
    setDocentesSeleccionados([]);
  };

  const handleOpenCreate = () => {
    resetForm();
    setIsFormOpen(true);
  };

  const handleCloseForm = () => {
    if (isLoading) return;
    resetForm();
    setIsFormOpen(false);
  };

  const proyectosFiltrados = proyectos.filter((proyecto) => {
    if (estadoFiltro === 'activos') return proyecto.activo === 1;
    if (estadoFiltro === 'inactivos') return proyecto.activo === 0;
    return true;
  }).filter((proyecto) => {
    const texto = busqueda.trim().toLowerCase();
    if (!texto) return true;
    return (
      proyecto.titulo_proyecto.toLowerCase().includes(texto) ||
      (proyecto.docentes || '').toLowerCase().includes(texto)
    );
  });

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
        <div className="filter-bar">
          <div className="filter-summary-group">
            <div className="filter-summary">Visibles: {proyectosFiltrados.length}</div>
            <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
            <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
            <span className="status-chip status-chip-total">Todos: {proyectos.length}</span>
          </div>
          <input
            className="form-input filter-search"
            placeholder="Buscar por título o docentes"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            aria-label="Buscar proyectos por título o docentes"
          />
          <select
            className="form-input filter-select"
            value={estadoFiltro}
            onChange={(e) => setEstadoFiltro(e.target.value as 'todos' | 'activos' | 'inactivos')}
            aria-label="Filtrar proyectos por estado"
          >
            <option value="todos">Todos</option>
            <option value="activos">Solo activos</option>
            <option value="inactivos">Solo inactivos</option>
          </select>
        </div>
        {loadingProyectos ? (
          <SkeletonTable columns={5} rows={6} />
        ) : proyectosFiltrados.length === 0 ? (
          <div className="empty-state">No hay proyectos para el filtro seleccionado</div>
        ) : (
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
              {proyectosFiltrados.map((p) => (
                <tr key={p.id_proyecto}>
                  <td>{p.titulo_proyecto}</td>
                  <td>{p.cantidad_docentes}</td>
                  <td>{p.docentes || '-'}</td>
                  <td>
                    {p.activo === 1 ? (
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
                      onClick={() => setProyectoToDetach(p)}
                      disabled={p.activo === 0}
                    />
                    {p.activo === 0 && (
                      <TableActionButton
                        className="btn-primary"
                        icon={RotateCcw}
                        label="Reactivar proyecto"
                        onClick={() => handleReactivarProyecto(p.id_proyecto)}
                      />
                    )}
                    <TableActionButton
                      className="btn-delete"
                      icon={Trash2}
                      label={p.activo === 1 ? 'Desactivar proyecto' : 'Mantener proyecto inactivo'}
                      onClick={() => setProyectoToDelete(p)}
                    />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <FormModal
        open={isFormOpen}
        title={(
          <span className="title-with-icon form-card-title">
            <AppIcon icon={FolderOpen} size={20} />
            <span>Registrar Nuevo Proyecto</span>
          </span>
        )}
        description="Defina el título del proyecto y seleccione los docentes participantes antes de registrar la relación completa."
        onClose={handleCloseForm}
        onSubmit={handleSubmit}
        submitText={(
          <span className="button-with-icon">
            <AppIcon icon={Plus} size={18} />
            <span>Crear Proyecto</span>
          </span>
        )}
        isLoading={isLoading}
        size="lg"
      >
        <FormInput
          label="Título del Proyecto"
          value={titulo}
          onChange={setTitulo}
          placeholder="Ej: Análisis de Microalgas en Agua Dulce"
          help="Registre el nombre con el que el proyecto será identificado en listados, reportes y relaciones con docentes."
          required
        />

        <DocentesChecklist
          docentes={docentes}
          selectedIds={docentesSeleccionados}
          onChange={setDocentesSeleccionados}
          loading={loadingDocentes}
          refreshing={refreshingDocentes}
        />
      </FormModal>

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