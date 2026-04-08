import React, { useState } from 'react';
import { BookPlus, Pencil, Plus, RotateCcw, Save, Trash2 } from 'lucide-react';
import { crearGrado, actualizarGrado, eliminarGrado, reactivarGrado, getTauriErrorMessage, type GradoAcademico } from '../services/tauriApi';
import { toast } from '../services/toast';
import { useFetchGrados } from '../hooks/useFetch';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { AppIcon } from './AppIcon';
import { FormModal } from './FormModal';
import { FormInput } from './FormInput';
import { ConfirmDialog } from './ConfirmDialog';
import { SkeletonTable } from './Skeleton';

interface GradosTabProps {
  onGradoModified: () => void;
  refreshTrigger?: number;
}

export const GradosTab: React.FC<GradosTabProps> = ({ onGradoModified, refreshTrigger = 0 }) => {
  const [nombre, setNombre] = useState('');
  const [descripcion, setDescripcion] = useState('');
  const [editingGrado, setEditingGrado] = useState<GradoAcademico | null>(null);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [gradoToDelete, setGradoToDelete] = useState<GradoAcademico | null>(null);
  const [estadoFiltro, setEstadoFiltro] = useState<'todos' | 'activos' | 'inactivos'>('todos');
  const [busqueda, setBusqueda] = useState('');

  const { grados, loading, refreshing, error, recargar } = useFetchGrados(refreshTrigger);

  useRefreshToast({
    refreshing,
    message: 'Actualizando grados',
    toastKey: 'grados-refresh',
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!nombre.trim()) {
      toast.warning('Ingrese el nombre del grado');
      return;
    }

    setIsLoading(true);
    try {
      if (editingGrado) {
        await actualizarGrado(editingGrado.id_grado, nombre, descripcion || undefined);
        toast.success('Grado actualizado');
      } else {
        await crearGrado(nombre, descripcion || undefined);
        toast.success('Grado creado');
      }
      setNombre('');
      setDescripcion('');
      setEditingGrado(null);
      setIsFormOpen(false);
      await recargar();
      onGradoModified();
    } catch (error) {
      toast.error('Error: ' + getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleEditar = (grado: GradoAcademico) => {
    setEditingGrado(grado);
    setNombre(grado.nombre);
    setDescripcion(grado.descripcion || '');
    setIsFormOpen(true);
  };

  const resetForm = () => {
    setEditingGrado(null);
    setNombre('');
    setDescripcion('');
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

  const handleEliminar = async () => {
    if (!gradoToDelete) return;
    try {
      const resultado = await eliminarGrado(gradoToDelete.id_grado);
      if (resultado.accion === 'desactivado') {
        toast.info(resultado.mensaje);
      } else {
        toast.success(resultado.mensaje);
      }
      setGradoToDelete(null);
      await recargar();
      onGradoModified();
    } catch (error) {
      toast.error('Error: ' + getTauriErrorMessage(error));
    }
  };

  const handleReactivar = async (id: string) => {
    try {
      await reactivarGrado(id);
      toast.success('Grado reactivado correctamente');
      await recargar();
      onGradoModified();
    } catch (error) {
      toast.error('Error: ' + getTauriErrorMessage(error));
    }
  };

  const totalActivos = grados.filter((grado) => grado.activo !== 0).length;
  const totalInactivos = grados.filter((grado) => grado.activo === 0).length;

  const gradosFiltrados = grados.filter((grado) => {
    if (estadoFiltro === 'activos') return grado.activo !== 0;
    if (estadoFiltro === 'inactivos') return grado.activo === 0;
    return true;
  }).filter((grado) => {
    const texto = busqueda.trim().toLowerCase();
    if (!texto) return true;
    return (
      grado.nombre.toLowerCase().includes(texto) ||
      (grado.descripcion || '').toLowerCase().includes(texto)
    );
  });

  return (
    <div className="tab-panel">
      <div className="table-container">
        <div className="section-header">
          <h2>Grados Registrados</h2>
          <div className="section-header-actions">
            <button type="button" className="btn-primary" onClick={handleOpenCreate}>
              <span className="button-with-icon">
                <AppIcon icon={Plus} size={18} />
                <span>Nuevo grado</span>
              </span>
            </button>
          </div>
        </div>
        {error && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se mantienen los datos previos.</span>
            <button type="button" className="btn-secondary" onClick={() => void recargar()}>
              Reintentar
            </button>
          </div>
        )}
        <div className="filter-bar">
          <div className="filter-summary-group">
            <div className="filter-summary">Visibles: {gradosFiltrados.length}</div>
            <span className="status-chip status-chip-total">Todos: {grados.length}</span>
            <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
            <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
          </div>
          <input
            className="form-input filter-search"
            placeholder="Buscar por nombre o descripción"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            aria-label="Buscar grados por nombre o descripción"
          />
          <select
            className="form-input filter-select"
            value={estadoFiltro}
            onChange={(e) => setEstadoFiltro(e.target.value as 'todos' | 'activos' | 'inactivos')}
            aria-label="Filtrar grados por estado"
          >
            <option value="todos">Todos</option>
            <option value="activos">Solo activos</option>
            <option value="inactivos">Solo inactivos</option>
          </select>
        </div>
        {loading ? (
          <SkeletonTable columns={3} rows={5} />
        ) : gradosFiltrados.length > 0 ? (
          <table className="table" aria-label="Tabla de grados académicos registrados">
            <thead>
              <tr>
                <th>Nombre</th>
                <th>Descripción</th>
                <th>Acciones</th>
              </tr>
            </thead>
            <tbody>
              {gradosFiltrados.map((grado) => (
                <tr key={grado.id_grado}>
                  <td>{grado.nombre}</td>
                  <td>{grado.descripcion || '-'}</td>
                  <td className="table-actions">
                    {grado.activo === 0 && <span className="badge badge-warning">Inactivo</span>}
                    {grado.activo === 0 && (
                      <button className="btn-primary" onClick={() => handleReactivar(grado.id_grado)}>
                        <span className="button-with-icon">
                          <AppIcon icon={RotateCcw} size={18} />
                          <span>Reactivar</span>
                        </span>
                      </button>
                    )}
                    <button className="btn-edit" onClick={() => handleEditar(grado)}>
                      <span className="button-with-icon">
                        <AppIcon icon={Pencil} size={16} />
                        <span>Editar</span>
                      </span>
                    </button>
                    <button className="btn-delete" onClick={() => setGradoToDelete(grado)}>
                      <span className="button-with-icon">
                        <AppIcon icon={Trash2} size={16} />
                        <span>Desactivar o eliminar</span>
                      </span>
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <div className="empty-state">No hay grados para el filtro seleccionado</div>
        )}
      </div>

      <FormModal
        open={isFormOpen}
        title={(
          <span className="title-with-icon form-card-title">
            <AppIcon icon={editingGrado ? Pencil : BookPlus} size={20} />
            <span>{editingGrado ? 'Editar Grado Académico' : 'Crear Grado Académico'}</span>
          </span>
        )}
        description="Complete la información base del catálogo académico y guarde los cambios para refrescar la lista visible."
        onClose={handleCloseForm}
        onSubmit={handleSubmit}
        submitText={(
          <span className="button-with-icon">
            <AppIcon icon={Save} size={18} />
            <span>{editingGrado ? 'Actualizar' : 'Crear'}</span>
          </span>
        )}
        isLoading={isLoading}
      >
        <FormInput
          label="Nombre del Grado"
          value={nombre}
          onChange={setNombre}
          placeholder="Ej: Licenciado"
          required
        />

        <FormInput
          label="Descripción"
          value={descripcion}
          onChange={setDescripcion}
          placeholder="Ej: Licenciatura en Ciencias"
        />
      </FormModal>

      <ConfirmDialog
        open={Boolean(gradoToDelete)}
        title="Desactivar o eliminar grado académico"
        message={`Esta acción intentará eliminar el grado "${gradoToDelete?.nombre ?? ''}". Si tiene docentes relacionados, se desactivará para conservar la integridad de la información.`}
        confirmText="Sí, continuar"
        cancelText="No, cancelar"
        onConfirm={handleEliminar}
        onCancel={() => setGradoToDelete(null)}
      />
    </div>
  );
};