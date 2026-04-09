import { useMemo, useState } from 'react';
import { useStableFetchData } from '../../../shared/hooks/useStableFetch';
import { useRefreshToast } from '../../../shared/hooks/useRefreshToast';
import { toast } from '../../../services/toast';
import { eliminarDocente, getAllDocentesConProyectos, getTauriErrorMessage, reactivarDocente, type DocenteDetalle } from '../api';

export const useDocentesTable = (refreshTrigger = 0) => {
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

  const totalActivos = useMemo(() => docentes.filter((docente) => docente.activo === 1).length, [docentes]);
  const totalInactivos = useMemo(() => docentes.filter((docente) => docente.activo === 0).length, [docentes]);

  const docentesFiltrados = useMemo(() => docentes.filter((docente) => {
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
  }), [busqueda, docentes, estadoFiltro]);

  return {
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
  };
};