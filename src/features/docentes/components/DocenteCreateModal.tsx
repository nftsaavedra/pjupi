import React from 'react';
import { GraduationCap, Plus, Search } from 'lucide-react';
import { useDocenteCreateForm } from '../hooks/useDocenteCreateForm';
import { FieldHelpTooltip } from '../../../shared/forms/FieldHelpTooltip';
import { FormInput } from '../../../shared/forms/FormInput';
import { FormModal } from '../../../shared/forms/FormModal';
import { FormSelect } from '../../../shared/forms/FormSelect';
import { AppIcon } from '../../../shared/ui/AppIcon';

interface DocenteCreateModalProps {
  open: boolean;
  onClose: () => void;
  onDocenteCreated: () => void;
  refreshTrigger?: number;
}

export const DocenteCreateModal: React.FC<DocenteCreateModalProps> = ({
  open,
  onClose,
  onDocenteCreated,
  refreshTrigger = 0,
}) => {
  const {
    apellidoMaterno,
    apellidoPaterno,
    camposBloqueados,
    dni,
    dniFueValidado,
    dniValidationMessage,
    dniValidationStatus,
    grados,
    handleDniChange,
    handleSubmit,
    handleValidarDni,
    idGrado,
    isCheckingDni,
    isLoading,
    nombreCompletoPreview,
    nombres,
    puedeValidarDni,
    setApellidoMaterno,
    setApellidoPaterno,
    setIdGrado,
    setNombres,
  } = useDocenteCreateForm(refreshTrigger, onDocenteCreated, onClose);

  return (
    <FormModal
      open={open}
      title={(
        <span className="title-with-icon form-card-title">
          <AppIcon icon={GraduationCap} size={20} />
          <span>Registrar Nuevo Docente</span>
        </span>
      )}
      description="Ingrese el DNI, el grado académico y los datos personales del docente. Si RENIEC está configurado, puede autocompletar nombres y apellidos desde la consulta oficial."
      onClose={onClose}
      onSubmit={handleSubmit}
      submitText={(
        <span className="button-with-icon">
          <AppIcon icon={Plus} size={18} />
          <span>Registrar</span>
        </span>
      )}
      isLoading={isLoading}
      submitDisabled={!dniFueValidado}
    >
      <div className="form-group">
        <div className="form-label-row">
          <label htmlFor="docente-dni" className="form-label-text">DNI *</label>
          <FieldHelpTooltip
            label="Ayuda para DNI"
            content="Primero se valida si el DNI ya existe en la base principal. Solo si no existe, se consulta RENIEC para autocompletar los datos y habilitar el registro."
          />
        </div>
        <div className="form-input-action-group">
          <input
            id="docente-dni"
            type="text"
            value={dni}
            onChange={(event) => handleDniChange(event.target.value)}
            placeholder="Ej: 45678912"
            maxLength={8}
            required
            className="form-input"
            inputMode="numeric"
            autoComplete="off"
            disabled={isLoading || isCheckingDni}
          />
          <button
            type="button"
            className="btn-secondary form-input-action-button"
            onClick={handleValidarDni}
            disabled={!puedeValidarDni}
          >
            <span className="button-with-icon">
              <AppIcon icon={Search} size={16} />
              <span>{isCheckingDni ? 'Validando...' : 'Validar DNI'}</span>
            </span>
          </button>
        </div>
        <div className={`form-inline-status form-inline-status-${dniValidationStatus}`} aria-live="polite">
          {dniValidationMessage}
        </div>
      </div>

      <FormSelect
        label="Grado Académico"
        value={idGrado}
        onChange={setIdGrado}
        options={grados.filter((g) => g.activo !== 0).map((g) => ({ value: g.id_grado, label: g.nombre }))}
        help="Seleccione el grado vigente del docente. Solo se muestran grados activos para preservar consistencia operativa."
        disabled={camposBloqueados}
        required
      />

      <FormInput
        label="Nombres"
        value={nombres}
        onChange={setNombres}
        placeholder="Ej: Juan Carlos"
        help="Ingrese los nombres del docente. Este campo se usa junto con los apellidos para construir el nombre mostrado en la aplicación."
        readOnly
        disabled={camposBloqueados}
        required
      />

      <FormInput
        label="Apellido paterno"
        value={apellidoPaterno}
        onChange={setApellidoPaterno}
        placeholder="Ej: Pérez"
        help="Ingrese el apellido paterno. Es obligatorio para mejorar la identificación y futuros filtros avanzados."
        readOnly
        disabled={camposBloqueados}
        required
      />

      <FormInput
        label="Apellido materno"
        value={apellidoMaterno}
        onChange={setApellidoMaterno}
        placeholder="Ej: García"
        help="Ingrese el apellido materno si corresponde. Puede completarse automáticamente desde RENIEC cuando esté disponible."
        readOnly
        disabled={camposBloqueados}
      />

      <div className="form-inline-preview" aria-live="polite">
        <strong>Nombre a registrar:</strong>
        <span>{nombreCompletoPreview || 'Complete nombres y apellidos para ver la vista previa.'}</span>
      </div>
    </FormModal>
  );
};