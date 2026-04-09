import React, { useState } from 'react';
import { Eye, GraduationCap, Plus, RotateCcw, TriangleAlert, Trash2, UserRound, X } from 'lucide-react';
import { eliminarDocente, getAllDocentesConProyectos, getTauriErrorMessage, reactivarDocente, type DocenteDetalle } from '../services/tauriApi';
import { AppIcon } from './AppIcon';
import { ConfirmDialog } from './ConfirmDialog';
import { TableActionButton } from './TableActionButton';
import { toast } from '../services/toast';
import { useStableFetchData } from '../hooks/useFetch';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { SkeletonTable } from './Skeleton';

interface DocenteDetalleModalProps {
  docente: DocenteDetalle;
  onClose: () => void;
}

const DocenteDetalleModal: React.FC<DocenteDetalleModalProps> = ({ docente, onClose }) => {
  const proyectos = docente.proyectos ? docente.proyectos.split(' | ') : [];

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2 className="title-with-icon">
            <AppIcon icon={UserRound} size={20} />
            <span>Detalles de Docente</span>
          </h2>
          <button type="button" className="modal-close" onClick={onClose} aria-label="Cerrar detalles del docente">
            <AppIcon icon={X} size={18} />
          </button>
        </div>

        <div className="modal-body">
          <div className="docente-info">
            <div className="info-row">
              <label>Nombre:</label>
              <span>{docente.nombres_apellidos}</span>
            </div>
            <div className="info-row">
              <label>DNI:</label>
              <span>{docente.dni}</span>
            </div>
            <div className="info-row">
              <label>Grado Académico:</label>
              <span>{docente.grado}</span>
            </div>
            <div className="info-row highlight">
              <label>Proyectos Asignados:</label>
              <span className="badge">{docente.cantidad_proyectos}</span>
            </div>
          </div>

          {docente.cantidad_proyectos > 0 ? (
            <div className="proyectos-section">
              <h3 className="title-with-icon">
                <AppIcon icon={GraduationCap} size={18} />
                <span>Proyectos en los que Participa</span>
              </h3>
              <ul className="proyectos-list">
                {proyectos.map((proyecto, idx) => (
                  <li key={idx} className="proyecto-item">
                    <span className="proyecto-number">{idx + 1}</span>
                    <span className="proyecto-title">{proyecto}</span>
                  </li>
                ))}
              </ul>
            </div>
          ) : (
            <div className="empty-state">
              <p className="title-with-icon empty-state-inline">
                <AppIcon icon={TriangleAlert} size={18} />
                <span>Este docente no tiene proyectos asignados</span>
              </p>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn-secondary" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
};

interface DocentesTabMejoradaProps {
  onCreateClick: () => void;
  refreshTrigger?: number;
}

export const DocentesTabMejorada: React.FC<DocentesTabMejoradaProps> = ({
  onCreateClick,
  refreshTrigger = 0,
}) => {
  const [selectedDocente, setSelectedDocente] = useState<DocenteDetalle | null>(null);
  const [docenteToDelete, setDocenteToDelete] = useState<DocenteDetalle | null>(null);
  const [estadoFiltro, setEstadoFiltro] = useState<'todos' | 'activos' | 'inactivos'>('todos');
  const [busqueda, setBusqueda] = useState('');
  const {
    data: docentes,
    loading,
    refreshing,
    error,
    recargar: cargarDocentes,
  } = useStableFetchData<DocenteDetalle[]>(
    getAllDocentesConProyectos,
    refreshTrigger,
    'Error cargando docentes',
    [],
  );

  useRefreshToast({
    refreshing,
    message: 'Actualizando docentes',
    toastKey: 'docentes-refresh',
  });

  const handleEliminarDocente = async () => {
    if (!docenteToDelete) return;
    try {
      const resultado = await eliminarDocente(docenteToDelete.id_docente);
      toast.info(resultado.mensaje);
      setDocenteToDelete(null);
      await cargarDocentes();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleReactivarDocente = async (id: string) => {
    try {
      await reactivarDocente(id);
      toast.success('Docente reactivado correctamente');
      await cargarDocentes();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const totalActivos = docentes.filter((docente) => docente.activo === 1).length;
  const totalInactivos = docentes.filter((docente) => docente.activo === 0).length;

  const docentesFiltrados = docentes.filter((docente) => {
    if (estadoFiltro === 'activos') return docente.activo === 1;
    if (estadoFiltro === 'inactivos') return docente.activo === 0;
    return true;
  }).filter((docente) => {
    const texto = busqueda.trim().toLowerCase();
    if (!texto) return true;
    return (
      docente.nombres_apellidos.toLowerCase().includes(texto) ||
      docente.dni.includes(texto) ||
      docente.grado.toLowerCase().includes(texto)
    );
  });

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
        <div className="filter-bar">
          <div className="filter-summary-group">
            <div className="filter-summary">Visibles: {docentesFiltrados.length}</div>
            <span className="status-chip status-chip-total">Todos: {docentes.length}</span>
            <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
            <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
          </div>
          <input
            className="form-input filter-search"
            placeholder="Buscar por nombre, DNI o grado"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            aria-label="Buscar docentes por nombre, DNI o grado"
          />
          <select
            className="form-input filter-select"
            value={estadoFiltro}
            onChange={(e) => setEstadoFiltro(e.target.value as 'todos' | 'activos' | 'inactivos')}
            aria-label="Filtrar docentes por estado"
          >
            <option value="todos">Todos</option>
            <option value="activos">Solo activos</option>
            <option value="inactivos">Solo inactivos</option>
          </select>
        </div>
        {loading ? (
          <SkeletonTable columns={6} rows={6} />
        ) : docentesFiltrados.length > 0 ? (
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
              {docentesFiltrados.map((docente) => (
                <tr
                  key={docente.id_docente}
                  className={docente.cantidad_proyectos === 0 ? 'unassigned' : ''}
                >
                  <td className="font-semibold">{docente.nombres_apellidos}</td>
                  <td>{docente.dni}</td>
                  <td>{docente.grado}</td>
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
                      onClick={() => setSelectedDocente(docente)}
                    />
                    {docente.activo === 0 && (
                      <TableActionButton
                        className="btn-primary"
                        icon={RotateCcw}
                        iconSize={18}
                        label="Reactivar docente"
                        onClick={() => handleReactivarDocente(docente.id_docente)}
                      />
                    )}
                    <TableActionButton
                      className="btn-delete"
                      icon={Trash2}
                      label={docente.activo === 1 ? 'Desactivar docente' : 'Mantener docente inactivo'}
                      onClick={() => setDocenteToDelete(docente)}
                    />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <div className="empty-state">No hay docentes para el filtro seleccionado</div>
        )}
      </div>

      {selectedDocente && (
        <DocenteDetalleModal
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