import React, { useEffect, useState } from 'react';
import { FolderOpen, Plus, Beaker, Lightbulb, Package, DollarSign } from 'lucide-react';
import type { DocenteDetalle } from '../../docentes/api';
import { getCatalogos } from '@/features/configuracion/api';
import type { CatalogoItem } from '@/services/tauri/types';
import { FormInput } from '@/shared/forms/FormInput';
import { FormSelect } from '@/shared/forms/FormSelect';
import { FormModal } from '@/shared/forms/FormModal';
import { AppIcon } from '@/shared/ui/AppIcon';
import { DocentesChecklist } from './DocentesChecklist';
import { RelatedEntitiesSection } from './RelatedEntitiesSection';

interface RelatedEntity {
  id: string;
  [key: string]: unknown;
}

interface ProyectoCreateModalProps {
  docentes: DocenteDetalle[];
  docentesSeleccionados: string[];
  docenteResponsableId: string | null;
  isLoading: boolean;
  loadingDocentes: boolean;
  open: boolean;
  refreshingDocentes: boolean;
  titulo: string;
  patentes?: RelatedEntity[];
  productos?: RelatedEntity[];
  equipamientos?: RelatedEntity[];
  financiamientos?: RelatedEntity[];
  onChangeDocentes: (ids: string[]) => void;
  onChangeResponsable: (docenteId: string) => void;
  onClose: () => void;
  onSubmit: (e: React.SyntheticEvent) => void;
  onTituloChange: (value: string) => void;
  onPatentesChange?: (items: RelatedEntity[]) => void;
  onProductosChange?: (items: RelatedEntity[]) => void;
  onEquipamientosChange?: (items: RelatedEntity[]) => void;
  onFinanciamientosChange?: (items: RelatedEntity[]) => void;
}

export const ProyectoCreateModal: React.FC<ProyectoCreateModalProps> = ({
  docentes,
  docentesSeleccionados,
  docenteResponsableId,
  isLoading,
  loadingDocentes,
  open,
  refreshingDocentes,
  titulo,
  patentes = [],
  productos = [],
  equipamientos = [],
  financiamientos = [],
  onChangeDocentes,
  onChangeResponsable,
  onClose,
  onSubmit,
  onTituloChange,
  onPatentesChange,
  onProductosChange,
  onEquipamientosChange,
  onFinanciamientosChange,
}) => {
  const [catOptsEstadoPatente, setCatOptsEstadoPatente] = useState<{ value: string; label: string }[]>([]);
  const [catOptsEtapa, setCatOptsEtapa] = useState<{ value: string; label: string }[]>([]);
  const [catOptsTipoFin, setCatOptsTipoFin] = useState<{ value: string; label: string }[]>([]);
  const [catOptsEstadoFin, setCatOptsEstadoFin] = useState<{ value: string; label: string }[]>([]);
  useEffect(() => {
    const map = (items: CatalogoItem[]) => items.map((i) => ({ value: i.codigo, label: i.nombre }));
    void getCatalogos('estado_patente').then((i) => { setCatOptsEstadoPatente(map(i)); }).catch(() => {});
    void getCatalogos('etapa_producto').then((i) => { setCatOptsEtapa(map(i)); }).catch(() => {});
    void getCatalogos('tipo_financiamiento').then((i) => { setCatOptsTipoFin(map(i)); }).catch(() => {});
    void getCatalogos('estado_financiero').then((i) => { setCatOptsEstadoFin(map(i)); }).catch(() => {});
  }, []);

  const docentesSeleccionadosOptions = docentes
    .filter((docente) => docentesSeleccionados.includes(docente.id_docente))
    .map((docente) => ({
      value: docente.id_docente,
      label: docente.nombres_apellidos,
    }));

  return (
    <FormModal
      open={open}
      title={(
        <span className="title-with-icon form-card-title">
          <AppIcon icon={FolderOpen} size={20} />
          <span>Registrar Nuevo Proyecto</span>
        </span>
      )}
      description="Defina el título, los docentes participantes, el responsable y opcionalmente entidades relacionadas (patentes, productos, equipamiento, financiamiento)."
      onClose={onClose}
      onSubmit={onSubmit}
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
        onChange={onTituloChange}
        placeholder="Ej: Análisis de Microalgas en Agua Dulce"
        help="Registre el nombre con el que el proyecto será identificado en listados, reportes y relaciones con docentes."
        required
      />

      <FormSelect
        label="Docente responsable"
        value={docenteResponsableId ?? ''}
        onChange={onChangeResponsable}
        options={docentesSeleccionadosOptions}
        placeholder={docentesSeleccionados.length === 0 ? 'Primero agregue docentes al proyecto' : '-- Seleccionar responsable --'}
        disabled={docentesSeleccionados.length === 0}
        help="Solo puede elegir como responsable a un docente ya agregado al proyecto. Si falta alguien, agréguelo primero en la lista de docentes."
      />

      <DocentesChecklist
        docentes={docentes}
        selectedIds={docentesSeleccionados}
        onChange={onChangeDocentes}
        responsableId={docenteResponsableId}
        loading={loadingDocentes}
        refreshing={refreshingDocentes}
        showSelectedMeta={false}
      />

      {/* ── Entidades relacionadas opcionales ── */}
      <div className="related-entities-container">
        {onPatentesChange && (
          <RelatedEntitiesSection
            title="Patentes"
            icon={<AppIcon icon={Beaker} size={18} />}
            description="Agregue patentes asociadas con este proyecto (opcional)."
            items={patentes}
            fields={[
              { name: 'numero_patente', label: 'Número de Patente', placeholder: 'Ej: PE-2024-00123', required: true },
              { name: 'titulo_patente', label: 'Título', placeholder: 'Ej: Proceso de purificación de agua', required: true },
              { name: 'estado', label: 'Estado', type: 'select', options: catOptsEstadoPatente, required: false },
            ]}
            onItemsChange={onPatentesChange}
          />
        )}

        {onProductosChange && (
          <RelatedEntitiesSection
            title="Productos I+D+i"
            icon={<AppIcon icon={Lightbulb} size={18} />}
            description="Agregue productos innovadores del proyecto (opcional)."
            items={productos}
            fields={[
              { name: 'nombre_producto', label: 'Nombre del Producto', placeholder: 'Ej: Sistema de tratamiento', required: true },
              { name: 'descripcion', label: 'Descripción', placeholder: 'Breve descripción del producto', type: 'textarea', required: false },
              { name: 'etapa', label: 'Etapa de Desarrollo', type: 'select', options: catOptsEtapa, required: false },
            ]}
            onItemsChange={onProductosChange}
          />
        )}

        {onEquipamientosChange && (
          <RelatedEntitiesSection
            title="Equipamiento"
            icon={<AppIcon icon={Package} size={18} />}
            description="Agregue equipamiento adquirido o desarrollado (opcional)."
            items={equipamientos}
            fields={[
              { name: 'nombre_equipo', label: 'Nombre del Equipo', placeholder: 'Ej: Cromatógrafo de gases', required: true },
              { name: 'especificaciones', label: 'Especificaciones', placeholder: 'Detalles técnicos', type: 'textarea', required: false },
              { name: 'costo', label: 'Costo Estimado (S/)', type: 'number', placeholder: '0.00', required: false },
            ]}
            onItemsChange={onEquipamientosChange}
          />
        )}

        {onFinanciamientosChange && (
          <RelatedEntitiesSection
            title="Financiamiento"
            icon={<AppIcon icon={DollarSign} size={18} />}
            description="Agregue fuentes de financiamiento del proyecto (opcional)."
            items={financiamientos}
            fields={[
              { name: 'fuente', label: 'Tipo de Financiamiento', type: 'select', options: catOptsTipoFin, required: true },
              { name: 'monto', label: 'Monto (S/)', type: 'number', placeholder: '0.00', required: false },
              { name: 'estado_financiero', label: 'Estado', type: 'select', options: catOptsEstadoFin, required: false },
            ]}
            onItemsChange={onFinanciamientosChange}
          />
        )}
      </div>
    </FormModal>
  );
};