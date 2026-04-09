import React, { useState } from 'react';
import { GraduationCap, Search, Plus } from 'lucide-react';
import { buscarDocentePorDni, consultarDniReniec, crearDocente, getTauriErrorMessage } from '../services/tauriApi';
import { toast } from '../services/toast';
import { useFetchGrados } from '../hooks/useFetch';
import { AppIcon } from './AppIcon';
import { FormModal } from './FormModal';
import { FormInput } from './FormInput';
import { FormSelect } from './FormSelect';
import { FieldHelpTooltip } from './FieldHelpTooltip';

interface DocentesTabProps {
  open: boolean;
  onClose: () => void;
  onDocenteCreated: () => void;
  refreshTrigger?: number;
}

export const DocentesTab: React.FC<DocentesTabProps> = ({
  open,
  onClose,
  onDocenteCreated,
  refreshTrigger = 0,
}) => {
  type DniValidationStatus = 'idle' | 'checking' | 'duplicate' | 'validated' | 'error';

  const [dni, setDni] = useState('');
  const [idGrado, setIdGrado] = useState('');
  const [nombres, setNombres] = useState('');
  const [apellidoPaterno, setApellidoPaterno] = useState('');
  const [apellidoMaterno, setApellidoMaterno] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [dniValidationStatus, setDniValidationStatus] = useState<DniValidationStatus>('idle');
  const [dniValidationMessage, setDniValidationMessage] = useState('Ingrese el DNI y valide primero para habilitar el resto del registro.');
  const [validatedDni, setValidatedDni] = useState('');

  const { grados } = useFetchGrados(refreshTrigger);

  const formatearTextoReniec = (value: string) => value
    .trim()
    .toLocaleLowerCase('es-PE')
    .split(/\s+/)
    .filter(Boolean)
    .map((segmento) => segmento.charAt(0).toLocaleUpperCase('es-PE') + segmento.slice(1))
    .join(' ');

  const nombreCompletoPreview = [nombres.trim(), apellidoPaterno.trim(), apellidoMaterno.trim()]
    .filter(Boolean)
    .join(' ');

  const dniLimpio = dni.trim();
  const isCheckingDni = dniValidationStatus === 'checking';
  const dniFueValidado = dniValidationStatus === 'validated' && validatedDni === dniLimpio;
  const puedeValidarDni = /^\d{8}$/.test(dniLimpio) && !isCheckingDni && !isLoading;
  const camposBloqueados = !dniFueValidado || isLoading;

  const resetForm = () => {
    setDni('');
    setIdGrado('');
    setNombres('');
    setApellidoPaterno('');
    setApellidoMaterno('');
    setValidatedDni('');
    setDniValidationStatus('idle');
    setDniValidationMessage('Ingrese el DNI y valide primero para habilitar el resto del registro.');
  };

  const clearValidatedIdentity = () => {
    setNombres('');
    setApellidoPaterno('');
    setApellidoMaterno('');
    setValidatedDni('');
  };

  const handleDniChange = (value: string) => {
    const nextDni = value.replace(/\D/g, '').slice(0, 8);
    setDni(nextDni);

    if (nextDni !== validatedDni) {
      clearValidatedIdentity();
      setDniValidationStatus('idle');
      setDniValidationMessage('Ingrese el DNI y valide primero para habilitar el resto del registro.');
    }
  };

  const handleValidarDni = async () => {
    if (!/^\d{8}$/.test(dniLimpio)) {
      toast.warning('Ingrese un DNI válido de 8 dígitos antes de validar');
      return;
    }

    setDniValidationStatus('checking');
    setDniValidationMessage('Validando DNI contra la base principal y consultando RENIEC...');
    try {
      const docenteExistente = await buscarDocentePorDni(dniLimpio);
      if (docenteExistente) {
        clearValidatedIdentity();
        setDniValidationStatus('duplicate');
        setDniValidationMessage(
          docenteExistente.activo === 1
            ? 'Este docente ya está registrado en la base principal. No puede volver a crearse.'
            : 'Este docente ya existe en la base principal y actualmente está inactivo. No puede registrarse nuevamente.'
        );
        toast.warning('El DNI ingresado ya pertenece a un docente registrado.');
        return;
      }

      const data = await consultarDniReniec(dniLimpio);
      setNombres(formatearTextoReniec(data.first_name));
      setApellidoPaterno(formatearTextoReniec(data.first_last_name));
      setApellidoMaterno(formatearTextoReniec(data.second_last_name));
      setValidatedDni(dniLimpio);
      setDniValidationStatus('validated');
      setDniValidationMessage('DNI validado correctamente. Los datos fueron cargados desde RENIEC y el registro está listo para completarse.');
      toast.success('DNI validado y datos RENIEC cargados correctamente.');
    } catch (error) {
      clearValidatedIdentity();
      setDniValidationStatus('error');
      setDniValidationMessage(getTauriErrorMessage(error));
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const nombresLimpio = nombres.trim();
    const apellidoPaternoLimpio = apellidoPaterno.trim();
    const apellidoMaternoLimpio = apellidoMaterno.trim();

    if (!dniLimpio || !idGrado || !nombresLimpio || !apellidoPaternoLimpio) {
      toast.warning('Complete todos los campos');
      return;
    }

    if (!dniFueValidado) {
      toast.warning('Valide el DNI antes de registrar al docente');
      return;
    }

    if (!/^\d{8}$/.test(dniLimpio)) {
      toast.warning('El DNI debe tener exactamente 8 dígitos numéricos');
      return;
    }

    if (grados.length === 0) {
      toast.warning('No hay grados académicos registrados. Cree un grado antes de registrar docentes.');
      return;
    }

    setIsLoading(true);
    try {
      await crearDocente(dniLimpio, idGrado, nombresLimpio, apellidoPaternoLimpio, apellidoMaternoLimpio);
      toast.success('Docente registrado exitosamente');
      resetForm();
      onDocenteCreated();
      onClose();
    } catch (error) {
      toast.error('Error al registrar docente: ' + getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

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