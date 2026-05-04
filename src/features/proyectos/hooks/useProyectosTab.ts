import { useEffect, useMemo, useState } from 'react';
import { useRecursoCrud } from '@/shared/hooks/useRecursoCrud';
import { useFetchDocentes } from '../../docentes/hooks/useFetchDocentes';
import { useStableFetchData } from '@/shared/hooks/useStableFetch';
import { useRefreshToast } from '@/shared/hooks/useRefreshToast';
import { toast } from '@/services/toast';
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
import type { Patente, Producto, Equipamiento, Financiamiento } from '@/services/tauri/types';
import {
  crearPatente, getPatentesProyecto, eliminarPatente,
  crearProducto, getProductosProyecto, eliminarProducto,
  crearEquipamiento, getEquipamientosProyecto, eliminarEquipamiento,
  crearFinanciamiento, getFinanciamientosProyecto, eliminarFinanciamiento,
  type CreatePatentePayload,
  type CreateProductoPayload,
  type CreateEquipamientoPayload,
  type CreateFinanciamientoPayload,
} from '@/services/tauri/recursos';

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

  const getProyectoId = (): string | undefined => proyectoToEdit?.id_proyecto;

  const patentesCrud = useRecursoCrud<Patente, CreatePatentePayload>(
    getPatentesProyecto, crearPatente, eliminarPatente,
    (raw, pid) => ({
      proyecto_id: pid,
      titulo: (raw.titulo_patente as string) || (raw.titulo as string) || '',
      numero_patente: raw.numero_patente as string,
      estado: raw.estado as string,
    }),
    (p) => p.id_patente,
    getProyectoId(),
  );

  const productosCrud = useRecursoCrud<Producto, CreateProductoPayload>(
    getProductosProyecto, crearProducto, eliminarProducto,
    (raw, pid) => ({
      proyecto_id: pid,
      nombre: (raw.nombre_producto as string) || (raw.nombre as string) || '',
      tipo: raw.tipo as string,
      etapa: raw.etapa as string,
      descripcion: raw.descripcion as string,
    }),
    (p) => p.id_producto,
    getProyectoId(),
  );

  const equipamientosCrud = useRecursoCrud<Equipamiento, CreateEquipamientoPayload>(
    getEquipamientosProyecto, crearEquipamiento, eliminarEquipamiento,
    (raw, pid) => ({
      proyecto_id: pid,
      nombre: (raw.nombre_equipo as string) || (raw.nombre as string) || '',
      descripcion: raw.descripcion as string,
      especificaciones: raw.especificaciones as string,
      valor_estimado: raw.costo as number,
    }),
    (e) => e.id_equipamiento,
    getProyectoId(),
  );

  const financiamientosCrud = useRecursoCrud<Financiamiento, CreateFinanciamientoPayload>(
    getFinanciamientosProyecto, crearFinanciamiento, eliminarFinanciamiento,
    (raw, pid) => ({
      proyecto_id: pid,
      entidad_financiadora: (raw.fuente as string) || (raw.entidad_financiadora as string) || '',
      tipo: raw.tipo as string,
      monto: raw.monto as number,
      estado_financiero: raw.estado_financiero as string,
    }),
    (f) => f.id_financiamiento,
    getProyectoId(),
  );

  useEffect(() => {
    const pid = proyectoToEdit?.id_proyecto;
    if (!pid) {
      patentesCrud.resetItems();
      productosCrud.resetItems();
      equipamientosCrud.resetItems();
      financiamientosCrud.resetItems();
      return;
    }
    void patentesCrud.loadItems(pid);
    void productosCrud.loadItems(pid);
    void equipamientosCrud.loadItems(pid);
    void financiamientosCrud.loadItems(pid);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [proyectoToEdit?.id_proyecto]);

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

  const resetForm = (): void => {
    setTitulo('');
    setDocentesSeleccionados([]);
    setDocenteResponsableId(null);
    patentesCrud.resetItems();
    productosCrud.resetItems();
    equipamientosCrud.resetItems();
    financiamientosCrud.resetItems();
  };

  const handleChangeDocentesSeleccionados = (ids: string[]): void => {
    setDocentesSeleccionados(ids);
    setDocenteResponsableId((current) => {
      if (ids.length === 0) { return null; }
      if (current && ids.includes(current)) { return current; }
      return ids[0] ?? null;
    });
  };

  const handleOpenCreate = (): void => {
    resetForm();
    setIsFormOpen(true);
  };

  const handleCloseForm = (): void => {
    if (isLoading) return;
    resetForm();
    setIsFormOpen(false);
  };

  const handleSubmit = async (e: React.SyntheticEvent): Promise<void> => {
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
      const proyecto = await crearProyectoConParticipantes(titulo, docentesSeleccionados, docenteResponsableId);
      const pid = proyecto.id_proyecto;

      const recursosPendientes: Promise<unknown>[] = [];
      for (const item of patentesCrud.items as unknown as Array<Record<string, unknown>>) {
        recursosPendientes.push(crearPatente({
          proyecto_id: pid,
          titulo: (item.titulo_patente as string) || (item.titulo as string) || '',
          numero_patente: item.numero_patente as string,
          estado: item.estado as string,
        }).catch(() => null));
      }
      for (const item of productosCrud.items as unknown as Array<Record<string, unknown>>) {
        recursosPendientes.push(crearProducto({
          proyecto_id: pid,
          nombre: (item.nombre_producto as string) || (item.nombre as string) || '',
          tipo: item.tipo as string,
          etapa: item.etapa as string,
          descripcion: item.descripcion as string,
        }).catch(() => null));
      }
      for (const item of equipamientosCrud.items as unknown as Array<Record<string, unknown>>) {
        recursosPendientes.push(crearEquipamiento({
          proyecto_id: pid,
          nombre: (item.nombre_equipo as string) || (item.nombre as string) || '',
          descripcion: item.descripcion as string,
          especificaciones: item.especificaciones as string,
          valor_estimado: item.costo as number,
        }).catch(() => null));
      }
      for (const item of financiamientosCrud.items as unknown as Array<Record<string, unknown>>) {
        recursosPendientes.push(crearFinanciamiento({
          proyecto_id: pid,
          entidad_financiadora: (item.fuente as string) || (item.entidad_financiadora as string) || '',
          tipo: item.tipo as string,
          monto: item.monto as number,
          estado_financiero: item.estado_financiero as string,
        }).catch(() => null));
      }
      if (recursosPendientes.length > 0) {
        await Promise.all(recursosPendientes);
      }

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

  const handleActualizarProyecto = async (idProyecto: string, payload: ProyectoParticipantesPayload): Promise<void> => {
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

  const handleEliminarProyecto = async (): Promise<void> => {
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

  const handleReactivarProyecto = async (id: string): Promise<void> => {
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
    patentes: patentesCrud.items,
    productos: productosCrud.items,
    equipamientos: equipamientosCrud.items,
    financiamientos: financiamientosCrud.items,
    handlePatentesChange: patentesCrud.handleChange,
    handleProductosChange: productosCrud.handleChange,
    handleEquipamientosChange: equipamientosCrud.handleChange,
    handleFinanciamientosChange: financiamientosCrud.handleChange,
  };
};
