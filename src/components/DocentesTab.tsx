import React, { useState } from 'react';
import { GraduationCap, Plus } from 'lucide-react';
import { crearDocente, getTauriErrorMessage } from '../services/tauriApi';
import { toast } from '../services/toast';
import { useFetchGrados } from '../hooks/useFetch';
import { AppIcon } from './AppIcon';
import { FormModal } from './FormModal';
import { FormInput } from './FormInput';
import { FormSelect } from './FormSelect';

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
  const [dni, setDni] = useState('');
  const [idGrado, setIdGrado] = useState('');
  const [nombres, setNombres] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const { grados } = useFetchGrados(refreshTrigger);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const dniLimpio = dni.trim();
    const nombresLimpio = nombres.trim();

    if (!dniLimpio || !idGrado || !nombresLimpio) {
      toast.warning('Complete todos los campos');
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
      await crearDocente(dniLimpio, idGrado, nombresLimpio);
      toast.success('Docente registrado exitosamente');
      setDni('');
      setIdGrado('');
      setNombres('');
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
      description="Ingrese el DNI, el grado académico y el nombre completo para registrar un nuevo docente en el padrón activo."
      onClose={onClose}
      onSubmit={handleSubmit}
      submitText={(
        <span className="button-with-icon">
          <AppIcon icon={Plus} size={18} />
          <span>Registrar</span>
        </span>
      )}
      isLoading={isLoading}
    >
        <FormInput
          label="DNI"
          value={dni}
          onChange={setDni}
          placeholder="Ej: 45678912"
          maxLength={8}
          required
        />

        <FormSelect
          label="Grado Académico"
          value={idGrado}
          onChange={setIdGrado}
          options={grados.filter((g) => g.activo !== 0).map((g) => ({ value: g.id_grado, label: g.nombre }))}
          required
        />

        <FormInput
          label="Nombres y Apellidos"
          value={nombres}
          onChange={setNombres}
          placeholder="Ej: Juan Pérez García"
          required
        />
    </FormModal>
  );
};