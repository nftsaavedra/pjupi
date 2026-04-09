import { useMemo, useState } from 'react';
import { useStableFetchData } from '../../../shared/hooks/useStableFetch';
import { useRefreshToast } from '../../../shared/hooks/useRefreshToast';
import { toast } from '../../../services/toast';
import { eliminarDocente, getAllDocentesConProyectos, getTauriErrorMessage, reactivarDocente, refrescarFormacionAcademicaRenacytDocente, type DocenteDetalle } from '../api';
import { formatRenacytNivel, normalizeRenacytNivelSearch } from '../../../shared/utils/renacyt';

const normalizeText = (value: string | null | undefined) => (value ?? '').trim().toLowerCase();

export const useDocentesTable = (actorUserId: string, refreshTrigger = 0) => {
  const [selectedDocente, setSelectedDocente] = useState<DocenteDetalle | null>(null);
  const [docenteToDelete, setDocenteToDelete] = useState<DocenteDetalle | null>(null);
  const [estadoFiltro, setEstadoFiltro] = useState<'todos' | 'activos' | 'inactivos'>('todos');
  const [busqueda, setBusqueda] = useState('');
  const [gradoFiltro, setGradoFiltro] = useState('todos');
  const [renacytNivelFiltro, setRenacytNivelFiltro] = useState('todos');
  const [refreshingRenacytDocenteId, setRefreshingRenacytDocenteId] = useState<string | null>(null);

  const {
    data: docentes,
    loading,
    refreshing,
    error,
    recargar: cargarDocentes,
  } = useStableFetchData<DocenteDetalle[]>(
    () => getAllDocentesConProyectos(actorUserId),
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
      const resultado = await eliminarDocente(actorUserId, docenteToDelete.id_docente);
      toast.info(resultado.mensaje);
      setDocenteToDelete(null);
      await cargarDocentes();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleReactivarDocente = async (id: string) => {
    try {
      await reactivarDocente(actorUserId, id);
      toast.success('Docente reactivado correctamente');
      await cargarDocentes();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleRefreshRenacytFormaciones = async (id: string) => {
    setRefreshingRenacytDocenteId(id);
    try {
      const resultado = await refrescarFormacionAcademicaRenacytDocente(actorUserId, id);
      if (resultado.actualizada) {
        toast.success(resultado.mensaje);
      } else {
        toast.info(resultado.mensaje);
      }

      setSelectedDocente((current) => current?.id_docente === id ? resultado.docente : current);
      await cargarDocentes();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setRefreshingRenacytDocenteId(null);
    }
  };

  const totalActivos = useMemo(() => docentes.filter((docente) => docente.activo === 1).length, [docentes]);
  const totalInactivos = useMemo(() => docentes.filter((docente) => docente.activo === 0).length, [docentes]);
  const gradosDisponibles = useMemo(
    () => Array.from(new Set(docentes.map((docente) => normalizeText(docente.grado) ? docente.grado : 'Sin grado'))).sort((a, b) => a.localeCompare(b, 'es')),
    [docentes],
  );
  const nivelesRenacytDisponibles = useMemo(
    () => Array.from(new Set(docentes.map((docente) => formatRenacytNivel(docente.renacyt_nivel) ?? 'Sin nivel RENACYT'))).sort((a, b) => a.localeCompare(b, 'es')),
    [docentes],
  );

  const docentesFiltrados = useMemo(() => docentes.filter((docente) => {
    if (estadoFiltro === 'activos') return docente.activo === 1;
    if (estadoFiltro === 'inactivos') return docente.activo === 0;
    return true;
  }).filter((docente) => {
    if (gradoFiltro === 'todos') return true;
    return (normalizeText(docente.grado) ? docente.grado : 'Sin grado') === gradoFiltro;
  }).filter((docente) => {
    if (renacytNivelFiltro === 'todos') return true;
    return (formatRenacytNivel(docente.renacyt_nivel) ?? 'Sin nivel RENACYT') === renacytNivelFiltro;
  }).filter((docente) => {
    const texto = normalizeText(busqueda);
    if (!texto) return true;
    return (
      normalizeText(docente.nombres_apellidos).includes(texto) ||
      normalizeText(docente.dni).includes(texto) ||
      normalizeText(docente.grado).includes(texto) ||
      normalizeRenacytNivelSearch(docente.renacyt_nivel).includes(texto)
    );
  }), [busqueda, docentes, estadoFiltro, gradoFiltro, renacytNivelFiltro]);

  return {
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
  };
};