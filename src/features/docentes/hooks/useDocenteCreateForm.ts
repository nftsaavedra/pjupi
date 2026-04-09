import { useMemo, useState } from 'react';
import { useFetchGrados } from '../../configuracion/grados/hooks/useFetchGrados';
import { toast } from '../../../services/toast';
import { buscarDocentePorDni, consultarDniReniec, crearDocente, getTauriErrorMessage } from '../api';

type DniValidationStatus = 'idle' | 'checking' | 'duplicate' | 'validated' | 'error';

export const useDocenteCreateForm = (refreshTrigger = 0, onDocenteCreated: () => void, onClose: () => void) => {
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

  const dniLimpio = dni.trim();
  const isCheckingDni = dniValidationStatus === 'checking';
  const dniFueValidado = dniValidationStatus === 'validated' && validatedDni === dniLimpio;
  const puedeValidarDni = /^\d{8}$/.test(dniLimpio) && !isCheckingDni && !isLoading;
  const camposBloqueados = !dniFueValidado || isLoading;
  const nombreCompletoPreview = useMemo(() => [nombres.trim(), apellidoPaterno.trim(), apellidoMaterno.trim()]
    .filter(Boolean)
    .join(' '), [apellidoMaterno, apellidoPaterno, nombres]);

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

  return {
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
  };
};