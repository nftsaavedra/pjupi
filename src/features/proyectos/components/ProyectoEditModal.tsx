import React, { useEffect, useMemo, useState } from 'react';
import { FolderOpen, Save } from 'lucide-react';
import type { DocenteDetalle } from '../../docentes/api';
import type { ProyectoDetalle, ProyectoParticipantesPayload } from '../api';
import { toast } from '../../../services/toast';
import { FormInput } from '../../../shared/forms/FormInput';
import { FormSelect } from '../../../shared/forms/FormSelect';
import { FormModal } from '../../../shared/forms/FormModal';
import { ConfirmDialog } from '../../../shared/overlays/ConfirmDialog';
import { AppIcon } from '../../../shared/ui/AppIcon';
import { DocentesChecklist } from './DocentesChecklist';
import { getResponsableProyecto, parseParticipantesProyecto } from '../participantes';

interface PendingProyectoChange {
  title: string;
  message: string;
  confirmText: string;
  onConfirm: () => void;
}

interface ProyectoEditModalProps {
  docentes: DocenteDetalle[];
  isLoading: boolean;
  loadingDocentes: boolean;
  open: boolean;
  proyecto: ProyectoDetalle | null;
  refreshingDocentes: boolean;
  onClose: () => void;
  onSubmit: (idProyecto: string, payload: ProyectoParticipantesPayload) => void;
}

export const ProyectoEditModal: React.FC<ProyectoEditModalProps> = ({
  docentes,
  isLoading,
  loadingDocentes,
  open,
  proyecto,
  refreshingDocentes,
  onClose,
  onSubmit,
}) => {
  const participantesIniciales = useMemo(
    () => parseParticipantesProyecto(proyecto?.participantes_json),
    [proyecto?.participantes_json],
  );
  const participantesPorId = useMemo(
    () => new Map(participantesIniciales.map((participante) => [participante.id_docente, participante])),
    [participantesIniciales],
  );

  const [titulo, setTitulo] = useState('');
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [responsableId, setResponsableId] = useState<string | null>(null);
  const [pendingChange, setPendingChange] = useState<PendingProyectoChange | null>(null);

  const initialSelectedIds = useMemo(
    () => participantesIniciales.map((participante) => participante.id_docente),
    [participantesIniciales],
  );
  const initialResponsableId = useMemo(
    () => getResponsableProyecto(participantesIniciales)?.id_docente ?? null,
    [participantesIniciales],
  );

  const addedDocentes = useMemo(
    () => selectedIds
      .filter((id) => !initialSelectedIds.includes(id))
      .map((id) => docentes.find((docente) => docente.id_docente === id)?.nombres_apellidos ?? id),
    [docentes, initialSelectedIds, selectedIds],
  );

  const removedDocentes = useMemo(
    () => initialSelectedIds
      .filter((id) => !selectedIds.includes(id))
      .map((id) => participantesPorId.get(id)?.nombre ?? id),
    [initialSelectedIds, participantesPorId, selectedIds],
  );
  const docentesSeleccionados = useMemo(
    () => docentes.filter((docente) => selectedIds.includes(docente.id_docente)),
    [docentes, selectedIds],
  );
  const responsableOptions = useMemo(
    () => docentesSeleccionados.map((docente) => ({
      value: docente.id_docente,
      label: docente.nombres_apellidos,
    })),
    [docentesSeleccionados],
  );

  const tituloOriginal = proyecto?.titulo_proyecto ?? '';
  const responsableOriginalNombre = initialResponsableId
    ? participantesPorId.get(initialResponsableId)?.nombre ?? null
    : null;
  const responsableActualNombre = responsableId
    ? docentes.find((docente) => docente.id_docente === responsableId)?.nombres_apellidos
      ?? participantesPorId.get(responsableId)?.nombre
      ?? null
    : null;
  const hasDiff = titulo.trim() !== tituloOriginal.trim()
    || addedDocentes.length > 0
    || removedDocentes.length > 0
    || responsableId !== initialResponsableId;

  useEffect(() => {
    if (!open || !proyecto) {
      setTitulo('');
      setSelectedIds([]);
      setResponsableId(null);
      setPendingChange(null);
      return;
    }

    setTitulo(proyecto.titulo_proyecto);
    setSelectedIds(initialSelectedIds);
    setResponsableId(initialResponsableId);
    setPendingChange(null);
  }, [initialResponsableId, initialSelectedIds, open, proyecto]);

  const requestToggleDocente = (docente: DocenteDetalle, nextSelected: boolean) => {
    if (nextSelected) {
      setPendingChange({
        title: 'Agregar docente al proyecto',
        message: `Se agregará a ${docente.nombres_apellidos} al proyecto "${titulo.trim() || proyecto?.titulo_proyecto || ''}".`,
        confirmText: 'Sí, agregar',
        onConfirm: () => {
          setSelectedIds((current) => current.includes(docente.id_docente) ? current : [...current, docente.id_docente]);
        },
      });
      return;
    }

    if (responsableId === docente.id_docente && selectedIds.length > 1) {
      toast.warning('Seleccione otro docente responsable antes de quitar al responsable actual.');
      return;
    }

    setPendingChange({
      title: 'Quitar docente del proyecto',
      message: `Se quitará a ${docente.nombres_apellidos} del proyecto "${titulo.trim() || proyecto?.titulo_proyecto || ''}".`,
      confirmText: 'Sí, quitar',
      onConfirm: () => {
        setSelectedIds((current) => current.filter((id) => id !== docente.id_docente));
        setResponsableId((current) => current === docente.id_docente ? null : current);
      },
    });
  };

  const requestResponsableChange = (docenteId: string) => {
    if (responsableId === docenteId) {
      return;
    }

    const docente = docentes.find((item) => item.id_docente === docenteId);
    if (!docente) {
      return;
    }

    setPendingChange({
      title: 'Cambiar docente responsable',
      message: `Se asignará a ${docente.nombres_apellidos} como docente responsable del proyecto.`,
      confirmText: 'Sí, asignar responsable',
      onConfirm: () => {
        setResponsableId(docenteId);
      },
    });
  };

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();

    if (!proyecto) {
      return;
    }

    if (!titulo.trim()) {
      toast.warning('Ingrese el título del proyecto');
      return;
    }

    if (selectedIds.length > 0 && !responsableId) {
      toast.warning('Seleccione un docente responsable antes de guardar los cambios.');
      return;
    }

    onSubmit(proyecto.id_proyecto, {
      titulo_proyecto: titulo.trim(),
      docentes_ids: selectedIds,
      docente_responsable_id: responsableId,
    });
  };

  return (
    <>
      <FormModal
        open={open}
        title={(
          <span className="title-with-icon form-card-title">
            <AppIcon icon={FolderOpen} size={20} />
            <span>Editar Proyecto</span>
          </span>
        )}
        description="Actualice el título, los docentes vinculados y el responsable del proyecto."
        onClose={onClose}
        onSubmit={handleSubmit}
        submitText={(
          <span className="button-with-icon">
            <AppIcon icon={Save} size={18} />
            <span>Guardar cambios</span>
          </span>
        )}
        isLoading={isLoading}
        submitDisabled={!titulo.trim() || (selectedIds.length > 0 && !responsableId)}
        size="lg"
      >
        <FormInput
          label="Título del Proyecto"
          value={titulo}
          onChange={setTitulo}
          placeholder="Ej: Análisis de Microalgas en Agua Dulce"
          required
        />

        <FormSelect
          label="Docente responsable"
          value={responsableId ?? ''}
          onChange={(value) => requestResponsableChange(value)}
          options={responsableOptions}
          placeholder={selectedIds.length === 0 ? 'Primero agregue docentes al proyecto' : '-- Seleccionar responsable --'}
          disabled={selectedIds.length === 0}
          help="Solo puede elegir como responsable a un docente ya vinculado a este proyecto. Si necesita otro responsable, primero agréguelo a la lista de docentes del proyecto y luego selecciónelo aquí."
        />

        <section className="project-diff-card" aria-label="Resumen visual de cambios pendientes">
          <div className="project-diff-header">
            <strong>Cambios pendientes</strong>
            <span className={`badge ${hasDiff ? 'badge-info' : 'badge-success'}`}>
              {hasDiff ? 'Con cambios' : 'Sin cambios'}
            </span>
          </div>
          {!hasDiff ? (
            <p className="project-diff-empty">Todavía no hay diferencias respecto al proyecto actual.</p>
          ) : (
            <div className="project-diff-list">
              {titulo.trim() !== tituloOriginal.trim() && (
                <article className="project-diff-row">
                  <span className="project-diff-label">Título</span>
                  <div className="project-diff-values">
                    <span className="project-diff-old">{tituloOriginal || 'Sin título'}</span>
                    <span className="project-diff-arrow">→</span>
                    <span className="project-diff-new">{titulo.trim() || 'Sin título'}</span>
                  </div>
                </article>
              )}
              {responsableId !== initialResponsableId && (
                <article className="project-diff-row">
                  <span className="project-diff-label">Responsable</span>
                  <div className="project-diff-values">
                    <span className="project-diff-old">{responsableOriginalNombre ?? 'Sin responsable'}</span>
                    <span className="project-diff-arrow">→</span>
                    <span className="project-diff-new">{responsableActualNombre ?? 'Sin responsable'}</span>
                  </div>
                </article>
              )}
              {addedDocentes.length > 0 && (
                <article className="project-diff-row">
                  <span className="project-diff-label">Agregados</span>
                  <div className="project-diff-chip-row">
                    {addedDocentes.map((nombre) => (
                      <span key={`add-${nombre}`} className="project-diff-chip is-added">{nombre}</span>
                    ))}
                  </div>
                </article>
              )}
              {removedDocentes.length > 0 && (
                <article className="project-diff-row">
                  <span className="project-diff-label">Retirados</span>
                  <div className="project-diff-chip-row">
                    {removedDocentes.map((nombre) => (
                      <span key={`remove-${nombre}`} className="project-diff-chip is-removed">{nombre}</span>
                    ))}
                  </div>
                </article>
              )}
            </div>
          )}
        </section>

        <DocentesChecklist
          docentes={docentes}
          selectedIds={selectedIds}
          onChange={setSelectedIds}
          onToggleDocente={requestToggleDocente}
          responsableId={responsableId}
          loading={loadingDocentes}
          refreshing={refreshingDocentes}
          showSelectedMeta={false}
          showRequiredError={false}
        />
      </FormModal>

      <ConfirmDialog
        open={Boolean(pendingChange)}
        title={pendingChange?.title ?? 'Confirmar cambio'}
        message={pendingChange?.message ?? ''}
        confirmText={pendingChange?.confirmText ?? 'Confirmar'}
        cancelText="Cancelar"
        onConfirm={() => {
          pendingChange?.onConfirm();
          setPendingChange(null);
        }}
        onCancel={() => setPendingChange(null)}
      />
    </>
  );
};