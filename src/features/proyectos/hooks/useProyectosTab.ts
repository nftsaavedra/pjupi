import { useMemo, useState } from 'react';
import { useFetchDocentes } from '../../docentes/hooks/useFetchDocentes';
import { useStableFetchData } from '../../../shared/hooks/useStableFetch';
import { useRefreshToast } from '../../../shared/hooks/useRefreshToast';
import { toast } from '../../../services/toast';
import {
  crearProyectoConParticipantes,
  eliminarProyecto,
  eliminarRelacionesProyecto,
  getAllProyectosDetalle,
  getTauriErrorMessage,
  reactivarProyecto,
  type ProyectoDetalle,
} from '../api';

export const useProyectosTab = (refreshTrigger = 0, onProyectoCreated: () => void) => {
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

    setIsLoading(true);
    try {
      await crearProyectoConParticipantes(titulo, docentesSeleccionados);
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
      (proyecto.docentes || '').toLowerCase().includes(texto)
    );
  }), [busqueda, estadoFiltro, proyectos]);

  return {
    busqueda,
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
    cargarProyectos,
  };
};