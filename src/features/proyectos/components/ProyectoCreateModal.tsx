import React from 'react';
import { FolderOpen, Plus } from 'lucide-react';
import type { DocenteDetalle } from '../../docentes/api';
import { FormInput } from '../../../shared/forms/FormInput';
import { FormSelect } from '../../../shared/forms/FormSelect';
import { FormModal } from '../../../shared/forms/FormModal';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { DocentesChecklist } from './DocentesChecklist';

interface ProyectoCreateModalProps {
  docentes: DocenteDetalle[];
  docentesSeleccionados: string[];
  docenteResponsableId: string | null;
  isLoading: boolean;
  loadingDocentes: boolean;
  open: boolean;
  refreshingDocentes: boolean;
  titulo: string;
  onChangeDocentes: (ids: string[]) => void;
  onChangeResponsable: (docenteId: string) => void;
  onClose: () => void;
  onSubmit: (e: React.FormEvent) => void;
  onTituloChange: (value: string) => void;
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
  onChangeDocentes,
  onChangeResponsable,
  onClose,
  onSubmit,
  onTituloChange,
}) => {
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
      description="Defina el título, los docentes participantes y el responsable del proyecto."
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
    </FormModal>
  );
};