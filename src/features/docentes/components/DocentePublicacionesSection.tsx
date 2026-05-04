import { useState } from 'react';
import { BookOpen, ChevronDown, ChevronUp, ExternalLink, RefreshCw } from 'lucide-react';
import { openUrl } from '@tauri-apps/plugin-opener';
import type { Publicacion, SyncPublicacionesResult } from '../api';
import { getPublicacionesDocente, sincronizarPublicacionesPure, getTauriErrorMessage } from '../api';
import { AppIcon } from '@/shared/ui/AppIcon';
import { InlineIconButton } from '@/shared/ui/InlineIconButton';
import { toast } from '@/services/toast';
import { parseAutores } from '@/shared/utils/docenteUtils';

interface DocentePublicacionesSectionProps {
  docenteId: string;
  scopusAuthorId: string | null | undefined;
  canSyncPure: boolean;
}

export const DocentePublicacionesSection: React.FC<DocentePublicacionesSectionProps> = ({
  docenteId,
  scopusAuthorId,
  canSyncPure,
}) => {
  const [expanded, setExpanded] = useState(false);
  const [publicaciones, setPublicaciones] = useState<Publicacion[]>([]);
  const [loaded, setLoaded] = useState(false);
  const [isSyncing, setIsSyncing] = useState(false);
  const tieneScopusId = Boolean(scopusAuthorId);

  const load = async (): Promise<void> => {
    try {
      const data = await getPublicacionesDocente(docenteId);
      setPublicaciones(data);
      setLoaded(true);
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const handleToggle = async (): Promise<void> => {
    const next = !expanded;
    setExpanded(next);
    if (next && !loaded) {
      await load();
    }
  };

  const handleSync = async (): Promise<void> => {
    setIsSyncing(true);
    try {
      const result: SyncPublicacionesResult = await sincronizarPublicacionesPure(docenteId);
      toast.success(
        `Sincronización Pure completada: ${result.nuevas} nuevas, ${result.actualizadas} actualizadas de ${result.total_encontradas} encontradas.`,
      );
      await load();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsSyncing(false);
    }
  };

  const handleOpenExternal = async (url: string, errorMsg: string): Promise<void> => {
    try {
      await openUrl(url);
    } catch {
      toast.error(errorMsg);
    }
  };

  return (
    <div className="renacyt-detail-card">
      <button
        type="button"
        className="renacyt-detail-toggle"
        onClick={() => void handleToggle()}
        aria-expanded={expanded}
      >
        <span className="renacyt-detail-toggle-copy">
          <span className="title-with-icon renacyt-detail-title">
            <AppIcon icon={BookOpen} size={18} />
            <span>Publicaciones (Pure)</span>
          </span>
          {loaded && <span className="badge badge-info">{publicaciones.length}</span>}
        </span>
        <span className="renacyt-detail-toggle-icon" aria-hidden="true">
          <AppIcon icon={expanded ? ChevronUp : ChevronDown} size={18} />
        </span>
      </button>

      {expanded && (
        <>
          {!tieneScopusId && (
            <div className="inline-feedback inline-feedback-warning renacyt-formaciones-feedback">
              <span>
                Este docente no tiene Scopus Author ID. Sincronice primero los datos RENACYT para obtenerlo.
              </span>
            </div>
          )}

          {tieneScopusId && canSyncPure && (
            <div className="renacyt-detail-actions">
              <button
                type="button"
                className="btn-secondary"
                onClick={() => void handleSync()}
                disabled={isSyncing}
              >
                <span className="button-with-icon">
                  <AppIcon icon={RefreshCw} size={16} />
                  <span>{isSyncing ? 'Sincronizando Pure...' : 'Sincronizar desde Pure'}</span>
                </span>
              </button>
            </div>
          )}

          {loaded && publicaciones.length === 0 && (
            <p className="renacyt-detail-empty">
              No hay publicaciones sincronizadas para este docente.
              {canSyncPure && tieneScopusId && ' Use el botón para sincronizar desde Pure.'}
            </p>
          )}

          {publicaciones.length > 0 && (
            <div className="renacyt-formaciones-list">
              {publicaciones.map((pub) => (
                <article key={pub.id_publicacion} className="renacyt-formacion-card">
                  <div className="renacyt-formacion-head">
                    <strong>{pub.titulo}</strong>
                    {pub.anio_publicacion && <span className="badge badge-info">{pub.anio_publicacion}</span>}
                  </div>
                  <div className="renacyt-formacion-grid">
                    {pub.tipo_publicacion && <span><strong>Tipo:</strong> {pub.tipo_publicacion}</span>}
                    {pub.journal_titulo && <span><strong>Journal:</strong> {pub.journal_titulo}</span>}
                    {pub.estado_publicacion && <span><strong>Estado:</strong> {pub.estado_publicacion}</span>}
                    {pub.doi && (
                      <span>
                        <strong>DOI:</strong>{' '}
                        <InlineIconButton
                          icon={ExternalLink}
                          label="Abrir DOI"
                          onClick={() =>
                            void handleOpenExternal(
                              `https://doi.org/${pub.doi}`,
                              'No se pudo abrir el enlace DOI.',
                            )
                          }
                        />
                        {pub.doi}
                      </span>
                    )}
                    {pub.autores_json && parseAutores(pub.autores_json).length > 0 && (
                      <span className="renacyt-formacion-full-col">
                        <strong>Autores:</strong> {parseAutores(pub.autores_json).join('; ')}
                      </span>
                    )}
                  </div>
                </article>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
};
