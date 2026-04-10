import { useMemo, useState } from 'react';
import { useFetchDocentes } from '../../docentes/hooks/useFetchDocentes';
import { useStableFetchData } from '../../../shared/hooks/useStableFetch';
import { useRefreshToast } from '../../../shared/hooks/useRefreshToast';
import { toast } from '../../../services/toast';
import {
  actualizarProyectoConParticipantes,
  crearProyectoConParticipantes,
  eliminarProyecto,
  getAllProyectosDetalle,
  getTauriErrorMessage,
  reactivarProyecto,
  type ProyectoParticipantesPayload,
  type ProyectoDetalle,
} from '../api';

export const useProyectosTab = (refreshTrigger = 0, onProyectoCreated: () => void) => {
  const [titulo, setTitulo] = useState('');
  const [docentesSeleccionados, setDocentesSeleccionados] = useState<string[]>([]);
  const [docenteResponsableId, setDocenteResponsableId] = useState<string | null>(null);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [proyectoToDelete, setProyectoToDelete] = useState<ProyectoDetalle | null>(null);
  const [proyectoToEdit, setProyectoToEdit] = useState<ProyectoDetalle | null>(null);
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
    () => getAllProyectosDetalle(),
    refreshTrigger,
    'Error cargando proyectos',
    [],
  );

  useRefreshToast({
    refreshing: refreshingProyectos,
    message: 'Actualizando proyectos',
    toastKey: 'proyectos-refresh',
  });

  const resetForm = () => {
    setTitulo('');
    setDocentesSeleccionados([]);
    setDocenteResponsableId(null);
  };

  const handleChangeDocentesSeleccionados = (ids: string[]) => {
    setDocentesSeleccionados(ids);
    setDocenteResponsableId((current) => {
      if (ids.length === 0) {
        return null;
      }
      if (current && ids.includes(current)) {
        return current;
      }
      return ids[0] ?? null;
    });
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

    if (!docenteResponsableId) {
      toast.warning('Seleccione un docente responsable para el proyecto');
      return;
    }

    setIsLoading(true);
    try {
      await crearProyectoConParticipantes(titulo, docentesSeleccionados, docenteResponsableId);
      toast.success('Proyecto creado exitosamente');
      resetForm();
      setIsFormOpen(false);
      onProyectoCreated();
      await cargarProyectos();
    } catch (error) {
      toast.error('Error al crear proyecto: ' + getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleActualizarProyecto = async (idProyecto: string, payload: ProyectoParticipantesPayload) => {
    if (!payload.titulo_proyecto.trim()) {
      toast.warning('Ingrese el título del proyecto');
      return;
    }

    if (payload.docentes_ids.length > 0 && !payload.docente_responsable_id) {
      toast.warning('Seleccione un docente responsable antes de guardar los cambios');
      return;
    }

    setIsLoading(true);
    try {
      await actualizarProyectoConParticipantes(idProyecto, payload);
      toast.success('Proyecto actualizado correctamente');
      setProyectoToEdit(null);
      await cargarProyectos();
      onProyectoCreated();
    } catch (error) {
      toast.error('Error actualizando proyecto: ' + getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
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

  const totalActivos = useMemo(() => proyectos.filter((proyecto) => proyecto.activo === 1).length, [proyectos]);
  const totalInactivos = useMemo(() => proyectos.filter((proyecto) => proyecto.activo === 0).length, [proyectos]);

  const proyectosFiltrados = useMemo(() => proyectos.filter((proyecto) => {
    if (estadoFiltro === 'activos') return proyecto.activo === 1;
    if (estadoFiltro === 'inactivos') return proyecto.activo === 0;
    return true;
  }).filter((proyecto) => {
    const texto = busqueda.trim().toLowerCase();
    if (!texto) return true;
    return (
      proyecto.titulo_proyecto.toLowerCase().includes(texto) ||
      (proyecto.docente_responsable || '').toLowerCase().includes(texto) ||
      (proyecto.docentes || '').toLowerCase().includes(texto)
    );
  }), [busqueda, estadoFiltro, proyectos]);

  return {
    busqueda,
    docentes,
    docenteResponsableId,
    docentesSeleccionados,
    estadoFiltro,
    handleCloseForm,
    handleActualizarProyecto,
    handleChangeDocentesSeleccionados,
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
    cargarProyectos,
  };
};