import React from 'react';
import { BadgeCheck, GraduationCap, Plus, Search } from 'lucide-react';
import { useDocenteCreateForm } from '../hooks/useDocenteCreateForm';
import { FieldHelpTooltip } from '../../../shared/forms/FieldHelpTooltip';
import { FormInput } from '../../../shared/forms/FormInput';
import { FormModal } from '../../../shared/forms/FormModal';
import { FormSelect } from '../../../shared/forms/FormSelect';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { formatRenacytNivel } from '../../../shared/utils/renacyt';

interface DocenteCreateModalProps {
  open: boolean;
  onClose: () => void;
  onDocenteCreated: () => void;
  refreshTrigger?: number;
}

const formatDate = (value?: number | null) => {
  if (!value) {
    return 'No disponible';
  }

  return new Intl.DateTimeFormat('es-PE', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  }).format(value);
};

const countFormacionesAcademicas = (value?: string | null) => {
  if (!value) {
    return 0;
  }

  try {
    const parsed = JSON.parse(value);
    return Array.isArray(parsed) ? parsed.length : 0;
  } catch {
    return 0;
  }
};

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
    handleRenacytChange,
    handleSubmit,
    handleValidarDni,
    handleValidarRenacyt,
    idGrado,
    isCheckingDni,
    isCheckingRenacyt,
    isLoading,
    nombreCompletoPreview,
    nombres,
    puedeValidarDni,
    puedeValidarRenacyt,
    renacytData,
    renacytQuery,
    renacytValidationMessage,
    renacytValidationStatus,
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
      description="Valide el DNI, vincule RENACYT si corresponde y luego seleccione el grado antes de registrar al docente."
      onClose={onClose}
      onSubmit={handleSubmit}
      size="lg"
      className="docente-form-modal"
      bodyClassName="docente-form-modal-body"
      submitText={(
        <span className="button-with-icon">
          <AppIcon icon={Plus} size={18} />
          <span>Registrar</span>
        </span>
      )}
      isLoading={isLoading}
      submitDisabled={!dniFueValidado}
    >
      <div className="docente-form-layout">
        <div className="docente-form-grid">
          <div className="form-group docente-form-span-2 docente-dni-group">
            <div className="form-label-row">
              <label htmlFor="docente-dni" className="form-label-text">DNI *</label>
              <FieldHelpTooltip
                label="Ayuda para DNI"
                content="Primero se valida si el DNI ya existe en la base principal. Solo si no existe, se consulta RENIEC para autocompletar los datos y habilitar el registro."
              />
            </div>
            <div className="form-input-action-group docente-dni-input-row">
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

          <div className="form-inline-preview docente-form-preview-card" aria-live="polite">
            <strong>Nombre a registrar</strong>
            <span>{nombreCompletoPreview || 'Complete nombres y apellidos para ver la vista previa.'}</span>
          </div>

          <div className="form-group docente-form-span-2 docente-renacyt-card">
            <div className="form-label-row">
              <label htmlFor="docente-renacyt" className="form-label-text">Validación RENACYT</label>
              <FieldHelpTooltip
                label="Ayuda para RENACYT"
                content="Puede ingresar el código de registro RENACYT o el ID del investigador. La validación confirma que ese registro corresponde al mismo DNI ya validado para el docente."
              />
            </div>
            <div className="form-input-action-group docente-dni-input-row">
              <input
                id="docente-renacyt"
                type="text"
                value={renacytQuery}
                onChange={(event) => handleRenacytChange(event.target.value)}
                placeholder="Ej: P0013866 o 13866"
                className="form-input"
                autoComplete="off"
                disabled={isLoading || isCheckingRenacyt || !dniFueValidado}
              />
              <button
                type="button"
                className="btn-secondary form-input-action-button"
                onClick={handleValidarRenacyt}
                disabled={!puedeValidarRenacyt}
              >
                <span className="button-with-icon">
                  <AppIcon icon={BadgeCheck} size={16} />
                  <span>{isCheckingRenacyt ? 'Validando...' : 'Validar RENACYT'}</span>
                </span>
              </button>
            </div>
            <div className={`form-inline-status form-inline-status-${renacytValidationStatus}`} aria-live="polite">
              {renacytValidationMessage}
            </div>

            {renacytData && renacytValidationStatus === 'validated' && (
              <div className="renacyt-summary-card" aria-live="polite">
                <div className="renacyt-summary-header">
                  <strong>RENACYT vinculado</strong>
                  <span className="badge badge-info">{renacytData.codigo_registro}</span>
                </div>
                <div className="renacyt-summary-grid">
                  <span><strong>Nivel:</strong> {formatRenacytNivel(renacytData.nivel) ?? 'No disponible'}</span>
                  <span><strong>Condición:</strong> {renacytData.condicion ?? 'No disponible'}</span>
                  <span><strong>ORCID:</strong> {renacytData.orcid ?? 'No disponible'}</span>
                  <span><strong>Scopus:</strong> {renacytData.scopus_author_id ?? 'No disponible'}</span>
                  <span><strong>Informe:</strong> {formatDate(renacytData.fecha_informe_calificacion)}</span>
                  <span><strong>Última revisión:</strong> {formatDate(renacytData.fecha_ultima_revision)}</span>
                  <span><strong>Formaciones:</strong> {countFormacionesAcademicas(renacytData.formaciones_academicas_json)}</span>
                </div>
              </div>
            )}
          </div>

          <FormSelect
            label="Grado Académico"
            value={idGrado}
            onChange={setIdGrado}
            options={grados.filter((g) => g.activo !== 0).map((g) => ({ value: g.id_grado, label: g.nombre }))}
            help="Seleccione el grado vigente del docente. Solo se muestran grados activos para preservar consistencia operativa."
            disabled={camposBloqueados}
            required
            containerClassName="docente-form-span-1"
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
            containerClassName="docente-form-span-1"
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
            containerClassName="docente-form-span-1"
          />

          <FormInput
            label="Apellido materno"
            value={apellidoMaterno}
            onChange={setApellidoMaterno}
            placeholder="Ej: García"
            help="Ingrese el apellido materno si corresponde. Puede completarse automáticamente desde RENIEC cuando esté disponible."
            readOnly
            disabled={camposBloqueados}
            containerClassName="docente-form-span-2"
          />
        </div>
      </div>
    </FormModal>
  );
};