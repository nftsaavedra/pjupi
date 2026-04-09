import { useMemo, useState } from 'react';
import { useFetchGrados } from '../../configuracion/grados/hooks/useFetchGrados';
import { toast } from '../../../services/toast';
import { buscarDocentePorDni, consultarDniReniec, consultarRenacytDocente, crearDocente, getTauriErrorMessage, type RenacytLookupResult } from '../api';

type DniValidationStatus = 'idle' | 'checking' | 'duplicate' | 'validated' | 'error';
type RenacytValidationStatus = 'idle' | 'checking' | 'validated' | 'error';

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
  const [renacytQuery, setRenacytQuery] = useState('');
  const [renacytValidationStatus, setRenacytValidationStatus] = useState<RenacytValidationStatus>('idle');
  const [renacytValidationMessage, setRenacytValidationMessage] = useState('Opcional: valide el código RENACYT o ID del investigador para adjuntar su clasificación vigente.');
  const [validatedRenacytQuery, setValidatedRenacytQuery] = useState('');
  const [renacytData, setRenacytData] = useState<RenacytLookupResult | null>(null);

  const { grados } = useFetchGrados(refreshTrigger);

  const formatearTextoReniec = (value: string) => value
    .trim()
    .toLocaleLowerCase('es-PE')
    .split(/\s+/)
    .filter(Boolean)
    .map((segmento) => segmento.charAt(0).toLocaleUpperCase('es-PE') + segmento.slice(1))
    .join(' ');

  const dniLimpio = dni.trim();
  const renacytQueryNormalizado = renacytQuery.trim().toUpperCase();
  const isCheckingDni = dniValidationStatus === 'checking';
  const isCheckingRenacyt = renacytValidationStatus === 'checking';
  const dniFueValidado = dniValidationStatus === 'validated' && validatedDni === dniLimpio;
  const renacytFueValidado = renacytValidationStatus === 'validated' && validatedRenacytQuery === renacytQueryNormalizado;
  const puedeValidarDni = /^\d{8}$/.test(dniLimpio) && !isCheckingDni && !isLoading;
  const puedeValidarRenacyt = Boolean(renacytQueryNormalizado) && dniFueValidado && !isCheckingRenacyt && !isLoading;
  const camposBloqueados = !dniFueValidado || isLoading;
  const nombreCompletoPreview = useMemo(() => [nombres.trim(), apellidoPaterno.trim(), apellidoMaterno.trim()]
    .filter(Boolean)
    .join(' '), [apellidoMaterno, apellidoPaterno, nombres]);

  const resetRenacyt = (keepQuery = false) => {
    if (!keepQuery) {
      setRenacytQuery('');
    }
    setRenacytValidationStatus('idle');
    setRenacytValidationMessage('Opcional: valide el código RENACYT o ID del investigador para adjuntar su clasificación vigente.');
    setValidatedRenacytQuery('');
    setRenacytData(null);
  };

  const resetForm = () => {
    setDni('');
    setIdGrado('');
    setNombres('');
    setApellidoPaterno('');
    setApellidoMaterno('');
    setValidatedDni('');
    setDniValidationStatus('idle');
    setDniValidationMessage('Ingrese el DNI y valide primero para habilitar el resto del registro.');
    resetRenacyt();
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
      resetRenacyt(true);
      setDniValidationStatus('idle');
      setDniValidationMessage('Ingrese el DNI y valide primero para habilitar el resto del registro.');
    }
  };

  const handleRenacytChange = (value: string) => {
    const normalized = value.toUpperCase().replace(/[^A-Z0-9]/g, '').slice(0, 12);
    setRenacytQuery(normalized);

    if (normalized !== validatedRenacytQuery) {
      resetRenacyt(true);
      setRenacytQuery(normalized);
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
        resetRenacyt(true);
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
      resetRenacyt(true);
      setDniValidationStatus('error');
      setDniValidationMessage(getTauriErrorMessage(error));
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleValidarRenacyt = async () => {
    if (!dniFueValidado) {
      toast.warning('Primero valide el DNI antes de consultar RENACYT');
      return;
    }

    if (!renacytQueryNormalizado) {
      toast.warning('Ingrese el código RENACYT o ID del investigador antes de validar');
      return;
    }

    setRenacytValidationStatus('checking');
    setRenacytValidationMessage('Consultando RENACYT y verificando coincidencia con el DNI validado...');

    try {
      const result = await consultarRenacytDocente(renacytQueryNormalizado);

      if (result.numero_documento && result.numero_documento.trim() !== dniLimpio) {
        resetRenacyt(true);
        setRenacytValidationStatus('error');
        setRenacytValidationMessage('El registro RENACYT consultado no corresponde al DNI validado del docente.');
        toast.warning('El registro RENACYT no coincide con el DNI validado.');
        return;
      }

      setRenacytData(result);
      setValidatedRenacytQuery(renacytQueryNormalizado);
      setRenacytValidationStatus('validated');
      setRenacytValidationMessage(`RENACYT validado correctamente${result.nivel ? `. Nivel actual: ${result.nivel}` : ''}.`);
      toast.success('Datos RENACYT validados correctamente.');
    } catch (error) {
      resetRenacyt(true);
      setRenacytValidationStatus('error');
      setRenacytValidationMessage(getTauriErrorMessage(error));
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

    if (renacytQueryNormalizado && !renacytFueValidado) {
      toast.warning('Si ingresa un código RENACYT o ID de investigador, debe validarlo antes de registrar');
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
      await crearDocente(
        dniLimpio,
        idGrado,
        nombresLimpio,
        apellidoPaternoLimpio,
        apellidoMaternoLimpio,
        renacytFueValidado && renacytData ? {
          codigo_registro: renacytData.codigo_registro,
          id_investigador: renacytData.id_investigador,
          nivel: renacytData.nivel ?? null,
          grupo: renacytData.grupo ?? null,
          condicion: renacytData.condicion ?? null,
          fecha_informe_calificacion: renacytData.fecha_informe_calificacion ?? null,
          fecha_registro: renacytData.fecha_registro ?? null,
          fecha_ultima_revision: renacytData.fecha_ultima_revision ?? null,
          orcid: renacytData.orcid ?? null,
          scopus_author_id: renacytData.scopus_author_id ?? null,
          ficha_url: renacytData.ficha_url,
        } : null,
      );
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
  };
};