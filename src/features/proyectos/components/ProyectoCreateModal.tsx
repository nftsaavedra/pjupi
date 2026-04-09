import React from 'react';
import { FolderOpen, Plus } from 'lucide-react';
import type { DocenteDetalle } from '../../docentes/api';
import { FormInput } from '../../../shared/forms/FormInput';
import { FormModal } from '../../../shared/forms/FormModal';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { DocentesChecklist } from './DocentesChecklist';

interface ProyectoCreateModalProps {
  docentes: DocenteDetalle[];
  docentesSeleccionados: string[];
  isLoading: boolean;
  loadingDocentes: boolean;
  open: boolean;
  refreshingDocentes: boolean;
  titulo: string;
  onChangeDocentes: (ids: string[]) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => void;
  onTituloChange: (value: string) => void;
}

export const ProyectoCreateModal: React.FC<ProyectoCreateModalProps> = ({
  docentes,
  docentesSeleccionados,
  isLoading,
  loadingDocentes,
  open,
  refreshingDocentes,
  titulo,
  onChangeDocentes,
  onClose,
  onSubmit,
  onTituloChange,
}) => (
  <FormModal
    open={open}
    title={(
      <span className="title-with-icon form-card-title">
        <AppIcon icon={FolderOpen} size={20} />
        <span>Registrar Nuevo Proyecto</span>
      </span>
    )}
    description="Defina el título del proyecto y seleccione los docentes participantes antes de registrar la relación completa."
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

    <DocentesChecklist
      docentes={docentes}
      selectedIds={docentesSeleccionados}
      onChange={onChangeDocentes}
      loading={loadingDocentes}
      refreshing={refreshingDocentes}
    />
  </FormModal>
);