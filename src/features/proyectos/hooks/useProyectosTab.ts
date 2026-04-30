import { useEffect, useMemo, useState } from 'react';
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
import type { Patente, Producto, Equipamiento, Financiamiento } from '../../../services/tauri/types';
import {
  crearPatente, getPatentesProyecto, eliminarPatente,
  crearProducto, getProductosProyecto, eliminarProducto,
  crearEquipamiento, getEquipamientosProyecto, eliminarEquipamiento,
  crearFinanciamiento, getFinanciamientosProyecto, eliminarFinanciamiento,
  type CreatePatentePayload,
  type CreateProductoPayload,
  type CreateEquipamientoPayload,
  type CreateFinanciamientoPayload,
} from '../../../services/tauri/recursos';

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
  
  // Recursos (patentes, productos, equipamientos, financiamientos)
  const [patentes, setPatentes] = useState<Patente[]>([]);
  const [productos, setProductos] = useState<Producto[]>([]);
  const [equipamientos, setEquipamientos] = useState<Equipamiento[]>([]);
  const [financiamientos, setFinanciamientos] = useState<Financiamiento[]>([]);

  // Cargar recursos existentes al abrir edición de un proyecto
  useEffect(() => {
    if (!proyectoToEdit?.id_proyecto) {
      setPatentes([]);
      setProductos([]);
      setEquipamientos([]);
      setFinanciamientos([]);
      return;
    }
    const id = proyectoToEdit.id_proyecto;
    getPatentesProyecto(id).then(setPatentes).catch(() => setPatentes([]));
    getProductosProyecto(id).then(setProductos).catch(() => setProductos([]));
    getEquipamientosProyecto(id).then(setEquipamientos).catch(() => setEquipamientos([]));
    getFinanciamientosProyecto(id).then(setFinanciamientos).catch(() => setFinanciamientos([]));
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

  const resetForm = () => {
    setTitulo('');
    setDocentesSeleccionados([]);
    setDocenteResponsableId(null);
    setPatentes([]);
    setProductos([]);
    setEquipamientos([]);
    setFinanciamientos([]);
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
      const proyecto = await crearProyectoConParticipantes(titulo, docentesSeleccionados, docenteResponsableId);
      const pid = proyecto.id_proyecto;

      // Persistir recursos pendientes (colectados durante el modal de creación)
      const recursosPendientes: Promise<unknown>[] = [];
      for (const item of patentes) {
        recursosPendientes.push(crearPatente({
          proyecto_id: pid,
          titulo: (item as unknown as Record<string, string>).titulo_patente || (item as unknown as Record<string, string>).titulo || '',
          numero_patente: (item as unknown as Record<string, string>).numero_patente,
          estado: (item as unknown as Record<string, string>).estado,
        }).catch(() => null));
      }
      for (const item of productos) {
        recursosPendientes.push(crearProducto({
          proyecto_id: pid,
          nombre: (item as unknown as Record<string, string>).nombre_producto || (item as unknown as Record<string, string>).nombre || '',
          tipo: (item as unknown as Record<string, string>).tipo,
          etapa: (item as unknown as Record<string, string>).etapa,
          descripcion: (item as unknown as Record<string, string>).descripcion,
        }).catch(() => null));
      }
      for (const item of equipamientos) {
        recursosPendientes.push(crearEquipamiento({
          proyecto_id: pid,
          nombre: (item as unknown as Record<string, string>).nombre_equipo || (item as unknown as Record<string, string>).nombre || '',
          descripcion: (item as unknown as Record<string, string>).descripcion,
          especificaciones: (item as unknown as Record<string, string>).especificaciones,
          valor_estimado: (item as unknown as Record<string, unknown>).costo as number,
        }).catch(() => null));
      }
      for (const item of financiamientos) {
        recursosPendientes.push(crearFinanciamiento({
          proyecto_id: pid,
          entidad_financiadora: (item as unknown as Record<string, string>).fuente || (item as unknown as Record<string, string>).entidad_financiadora || '',
          tipo: (item as unknown as Record<string, string>).tipo,
          monto: (item as unknown as Record<string, unknown>).monto as number,
          estado_financiero: (item as unknown as Record<string, string>).estado_financiero,
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

  // ── Handlers para patentes ──────────────────────────────────────────────────
  const handlePatentesChange = async (items: Array<{ id: string; [key: string]: unknown }>) => {
    // En flujo de creación: solo actualizar estado local (sin proyecto_id aún)
    if (!proyectoToEdit?.id_proyecto) {
      setPatentes(items as unknown as Patente[]);
      return;
    }

    const nuevas: CreatePatentePayload[] = [];
    const eliminadas: string[] = [];

    for (const item of items) {
      if (item.id.startsWith('temp-')) {
        // Nuevo item
        nuevas.push({
          proyecto_id: proyectoToEdit?.id_proyecto,
          titulo: (item.titulo_patente as string) || '',
          numero_patente: item.numero_patente as string,
          estado: item.estado as string,
        });
      }
    }

    // Detectar eliminadas (items que estaban pero ya no están)
    const itemIds = items.map((i) => i.id);
    for (const patente of patentes) {
      if (!itemIds.includes(patente.id_patente)) {
        eliminadas.push(patente.id_patente);
      }
    }

    // Ejecutar cambios
    for (const nueva of nuevas) {
      try {
        await crearPatente(nueva);
      } catch (error) {
        toast.error('Error creando patente: ' + getTauriErrorMessage(error));
      }
    }

    for (const id of eliminadas) {
      try {
        await eliminarPatente(id);
      } catch (error) {
        toast.error('Error eliminando patente: ' + getTauriErrorMessage(error));
      }
    }

    // Recargar patentes
    if (proyectoToEdit?.id_proyecto) {
      try {
        const p = await getPatentesProyecto(proyectoToEdit.id_proyecto);
        setPatentes(p);
      } catch (error) {
        toast.error('Error cargando patentes: ' + getTauriErrorMessage(error));
      }
    }
  };

  // ── Handlers para productos ────────────────────────────────────────────────
  const handleProductosChange = async (items: Array<{ id: string; [key: string]: unknown }>) => {
    if (!proyectoToEdit?.id_proyecto) {
      setProductos(items as unknown as Producto[]);
      return;
    }
    const nuevas: CreateProductoPayload[] = [];
    const eliminadas: string[] = [];

    for (const item of items) {
      if (item.id.startsWith('temp-')) {
        nuevas.push({
          proyecto_id: proyectoToEdit?.id_proyecto,
          nombre: (item.nombre_producto as string) || '',
          tipo: item.tipo as string,
          etapa: item.etapa as string,
          descripcion: item.descripcion as string,
        });
      }
    }

    const itemIds = items.map((i) => i.id);
    for (const producto of productos) {
      if (!itemIds.includes(producto.id_producto)) {
        eliminadas.push(producto.id_producto);
      }
    }

    for (const nueva of nuevas) {
      try {
        await crearProducto(nueva);
      } catch (error) {
        toast.error('Error creando producto: ' + getTauriErrorMessage(error));
      }
    }

    for (const id of eliminadas) {
      try {
        await eliminarProducto(id);
      } catch (error) {
        toast.error('Error eliminando producto: ' + getTauriErrorMessage(error));
      }
    }

    if (proyectoToEdit?.id_proyecto) {
      try {
        const p = await getProductosProyecto(proyectoToEdit.id_proyecto);
        setProductos(p);
      } catch (error) {
        toast.error('Error cargando productos: ' + getTauriErrorMessage(error));
      }
    }
  };

  // ── Handlers para equipamientos ────────────────────────────────────────────
  const handleEquipamientosChange = async (items: Array<{ id: string; [key: string]: unknown }>) => {
    if (!proyectoToEdit?.id_proyecto) {
      setEquipamientos(items as unknown as Equipamiento[]);
      return;
    }
    const nuevas: CreateEquipamientoPayload[] = [];
    const eliminadas: string[] = [];

    for (const item of items) {
      if (item.id.startsWith('temp-')) {
        nuevas.push({
          proyecto_id: proyectoToEdit?.id_proyecto,
          nombre: (item.nombre_equipo as string) || '',
          descripcion: item.descripcion as string,
          especificaciones: item.especificaciones as string,
          valor_estimado: item.costo as number,
        });
      }
    }

    const itemIds = items.map((i) => i.id);
    for (const equipo of equipamientos) {
      if (!itemIds.includes(equipo.id_equipamiento)) {
        eliminadas.push(equipo.id_equipamiento);
      }
    }

    for (const nueva of nuevas) {
      try {
        await crearEquipamiento(nueva);
      } catch (error) {
        toast.error('Error creando equipamiento: ' + getTauriErrorMessage(error));
      }
    }

    for (const id of eliminadas) {
      try {
        await eliminarEquipamiento(id);
      } catch (error) {
        toast.error('Error eliminando equipamiento: ' + getTauriErrorMessage(error));
      }
    }

    if (proyectoToEdit?.id_proyecto) {
      try {
        const e = await getEquipamientosProyecto(proyectoToEdit.id_proyecto);
        setEquipamientos(e);
      } catch (error) {
        toast.error('Error cargando equipamientos: ' + getTauriErrorMessage(error));
      }
    }
  };

  // ── Handlers para financiamientos ──────────────────────────────────────────
  const handleFinanciamientosChange = async (items: Array<{ id: string; [key: string]: unknown }>) => {
    if (!proyectoToEdit?.id_proyecto) {
      setFinanciamientos(items as unknown as Financiamiento[]);
      return;
    }
    const nuevas: CreateFinanciamientoPayload[] = [];
    const eliminadas: string[] = [];

    for (const item of items) {
      if (item.id.startsWith('temp-')) {
        nuevas.push({
          proyecto_id: proyectoToEdit?.id_proyecto,
          entidad_financiadora: (item.fuente as string) || '',
          tipo: item.tipo as string,
          monto: item.monto as number,
          estado_financiero: item.estado_financiero as string,
        });
      }
    }

    const itemIds = items.map((i) => i.id);
    for (const fin of financiamientos) {
      if (!itemIds.includes(fin.id_financiamiento)) {
        eliminadas.push(fin.id_financiamiento);
      }
    }

    for (const nueva of nuevas) {
      try {
        await crearFinanciamiento(nueva);
      } catch (error) {
        toast.error('Error creando financiamiento: ' + getTauriErrorMessage(error));
      }
    }

    for (const id of eliminadas) {
      try {
        await eliminarFinanciamiento(id);
      } catch (error) {
        toast.error('Error eliminando financiamiento: ' + getTauriErrorMessage(error));
      }
    }

    if (proyectoToEdit?.id_proyecto) {
      try {
        const f = await getFinanciamientosProyecto(proyectoToEdit.id_proyecto);
        setFinanciamientos(f);
      } catch (error) {
        toast.error('Error cargando financiamientos: ' + getTauriErrorMessage(error));
      }
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
    // Recursos
    patentes,
    productos,
    equipamientos,
    financiamientos,
    handlePatentesChange,
    handleProductosChange,
    handleEquipamientosChange,
    handleFinanciamientosChange,
  };
};