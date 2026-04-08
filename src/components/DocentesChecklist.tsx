import React, { useDeferredValue, useId, useState } from 'react';
import { X } from 'lucide-react';
import { Docente } from '../services/tauriApi';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { AppIcon } from './AppIcon';
import { SkeletonChecklist } from './Skeleton';

interface DocentesChecklistProps {
  docentes: Docente[];
  selectedIds: string[];
  onChange: (ids: string[]) => void;
  loading?: boolean;
  refreshing?: boolean;
}

export const DocentesChecklist: React.FC<DocentesChecklistProps> = ({
  docentes,
  selectedIds,
  onChange,
  loading = false,
  refreshing = false,
}) => {
  const searchId = useId();
  const helperId = useId();
  const resultsId = useId();
  const [query, setQuery] = useState('');
  const deferredQuery = useDeferredValue(query.trim().toLowerCase());

  useRefreshToast({
    refreshing,
    message: 'Actualizando lista de docentes',
    toastKey: 'docentes-checklist-refresh',
    cooldownMs: 120000,
  });

  const handleToggle = (id: string) => {
    if (selectedIds.includes(id)) {
      onChange(selectedIds.filter((x) => x !== id));
    } else {
      onChange([...selectedIds, id]);
    }
  };

  const docentesSeleccionados = docentes.filter((docente) => selectedIds.includes(docente.id_docente));
  const requiereBusquedaMinima = docentes.length > 25 && deferredQuery.length < 2;
  const coincidencias = requiereBusquedaMinima
    ? []
    : docentes.filter((docente) => {
        if (!deferredQuery) return docentes.length <= 25 && !selectedIds.includes(docente.id_docente);

        const nombre = docente.nombres_apellidos.toLowerCase();
        const dni = docente.dni.toLowerCase();

        return nombre.includes(deferredQuery) || dni.includes(deferredQuery);
      });
  const docentesVisibles = coincidencias.slice(0, 8);
  const hayMasResultados = coincidencias.length > docentesVisibles.length;

  const limpiarSeleccion = () => {
    onChange([]);
  };

  if (loading && docentes.length === 0) {
    return <SkeletonChecklist />;
  }

  if (docentes.length === 0) {
    return (
      <div className="empty-state">
        No hay docentes registrados. Por favor, registre docentes primero.
      </div>
    );
  }

  return (
    <div className="form-group">
      <div className="field-header">
        <label htmlFor={searchId}>Seleccionar Docentes *</label>
      </div>
      <div className="docentes-selector">
        <div className="docentes-selector-toolbar">
          <input
            id={searchId}
            className="form-input docentes-selector-search"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Buscar docente por nombre o DNI"
            aria-describedby={helperId}
            aria-controls={resultsId}
          />
          <div className="docentes-selector-meta">
            <span className="status-chip status-chip-total">Disponibles: {docentes.length}</span>
            <span className="status-chip status-chip-success">Seleccionados: {selectedIds.length}</span>
            {selectedIds.length > 0 && (
              <button type="button" className="btn-secondary docentes-selector-clear" onClick={limpiarSeleccion}>
                Limpiar selección
              </button>
            )}
          </div>
        </div>

        <div id={helperId} className="visually-hidden">
          Busque docentes por nombre o DNI, use los botones para agregarlos o quitarlos de la selección.
        </div>

        <div className="docentes-selected-list" aria-live="polite">
          {docentesSeleccionados.length > 0 ? (
            docentesSeleccionados.map((docente) => (
              <button
                key={docente.id_docente}
                type="button"
                className="docente-chip"
                onClick={() => handleToggle(docente.id_docente)}
                title="Quitar de la selección"
              >
                <span>{docente.nombres_apellidos}</span>
                <span className="docente-chip-remove">
                  <AppIcon icon={X} size={14} />
                </span>
              </button>
            ))
          ) : (
            <div className="docentes-selector-empty">Aún no ha seleccionado docentes para este proyecto.</div>
          )}
        </div>

        <div id={resultsId} className="docentes-checklist docentes-selector-results" aria-label="Resultados de docentes">
          {requiereBusquedaMinima ? (
            <div className="docentes-selector-empty">
              Escriba al menos 2 caracteres para buscar dentro de una lista grande de docentes.
            </div>
          ) : !deferredQuery && docentes.length > 25 ? (
            <div className="docentes-selector-empty">
              Use el buscador para encontrar docentes y agregarlos al proyecto sin recorrer una lista completa.
            </div>
          ) : docentesVisibles.length === 0 ? (
            <div className="docentes-selector-empty">
              No se encontraron docentes con ese criterio.
            </div>
          ) : (
            <>
              {docentesVisibles.map((docente) => {
                const seleccionado = selectedIds.includes(docente.id_docente);

                return (
                  <button
                    key={docente.id_docente}
                    type="button"
                    className={`checkbox-item docente-option ${seleccionado ? 'selected' : ''}`}
                    onClick={() => handleToggle(docente.id_docente)}
                    aria-pressed={seleccionado}
                  >
                    <div className="docente-option-main">
                      <span className="docente-option-name">{docente.nombres_apellidos}</span>
                      <span className="docente-option-dni">DNI: {docente.dni}</span>
                    </div>
                    <span className={`badge ${seleccionado ? 'badge-success' : 'badge-info'}`}>
                      {seleccionado ? 'Seleccionado' : 'Agregar'}
                    </span>
                  </button>
                );
              })}
              {hayMasResultados && (
                <div className="docentes-selector-footnote">
                  Mostrando {docentesVisibles.length} de {coincidencias.length} coincidencias. Refine la búsqueda para acotar resultados.
                </div>
              )}
            </>
          )}
        </div>
      </div>
      {selectedIds.length === 0 && (
        <small className="field-error">
          Seleccione al menos un docente
        </small>
      )}
    </div>
  );
};