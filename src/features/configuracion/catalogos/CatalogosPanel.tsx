import React, { Suspense, lazy, useEffect, useMemo, useState } from 'react';
import { Banknote, CheckCircle, DollarSign, FileText, Layers, LibraryBig, Package, TrendingUp, type LucideIcon } from 'lucide-react';
import { AppIcon } from '@/shared/ui/AppIcon';
import { SkeletonBlock, SkeletonTable } from '@/shared/ui/Skeleton';
import { getAllCatalogosAdmin } from '../api';
import type { CatalogoItem } from '@/services/tauri/types';

const CatalogosTab = lazy(async () => {
  const module = await import('./CatalogosTab');
  return { default: module.CatalogosTab };
});

interface CatalogoTipoEntry {
  tipo: string;
  titulo: string;
  icon: LucideIcon;
  descripcion: string;
}

interface CatalogoGrupo {
  categoria: string;
  icon: LucideIcon;
  tipos: CatalogoTipoEntry[];
}

const CATALOGO_GRUPOS: CatalogoGrupo[] = [
  {
    categoria: 'Propiedad Intelectual',
    icon: FileText,
    tipos: [
      { tipo: 'tipo_patente', titulo: 'Tipos de Patente', icon: FileText, descripcion: 'Clasificación de patentes registradas en proyectos' },
      { tipo: 'estado_patente', titulo: 'Estados de Patente', icon: CheckCircle, descripcion: 'Ciclo de vida de cada registro de propiedad intelectual' },
    ],
  },
  {
    categoria: 'Productos I+D+i',
    icon: Package,
    tipos: [
      { tipo: 'tipo_producto', titulo: 'Tipos de Producto', icon: Package, descripcion: 'Clases de resultados de investigación y desarrollo' },
      { tipo: 'etapa_producto', titulo: 'Etapas de Producto', icon: Layers, descripcion: 'Fases del ciclo de desarrollo de un producto' },
    ],
  },
  {
    categoria: 'Financiamiento',
    icon: Banknote,
    tipos: [
      { tipo: 'tipo_financiamiento', titulo: 'Tipos de Financiamiento', icon: Banknote, descripcion: 'Origen y clasificación de los fondos de proyectos' },
      { tipo: 'estado_financiero', titulo: 'Estados de Financiamiento', icon: TrendingUp, descripcion: 'Situación actual de cada fuente de financiamiento' },
    ],
  },
  {
    categoria: 'General',
    icon: DollarSign,
    tipos: [
      { tipo: 'moneda', titulo: 'Monedas', icon: DollarSign, descripcion: 'Divisas disponibles para costos y financiamientos' },
    ],
  },
];

const ALL_TIPOS = CATALOGO_GRUPOS.flatMap((g) => g.tipos);

const CatalogosSkeleton = () => (
  <div className="catalogos-grid catalogos-grid-loading">
    {Array.from({ length: 4 }).map((_, i) => (
      <div key={i} className="catalogo-summary-card" aria-hidden="true">
        <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
        <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
        <SkeletonTable columns={2} rows={2} />
      </div>
    ))}
  </div>
);

interface CatalogosPanelProps {
  currentUser: unknown;
  onDataModified: () => void;
  refreshTrigger?: number;
}

export const CatalogosPanel: React.FC<CatalogosPanelProps> = ({
  currentUser: _currentUser,
  onDataModified,
  refreshTrigger = 0,
}) => {
  const [activeTipo, setActiveTipo] = useState<string | null>(null);
  const [categoriaActiva, setCategoriaActiva] = useState<string | null>(null);
  const [stats, setStats] = useState<Record<string, { activos: number; inactivos: number }>>({});
  const [statsLoaded, setStatsLoaded] = useState(false);

  useEffect(() => {
    const fetchAll = async () => {
      const allStats: Record<string, { activos: number; inactivos: number }> = {};
      for (const entry of ALL_TIPOS) {
        try {
          const items: CatalogoItem[] = await getAllCatalogosAdmin(entry.tipo);
          allStats[entry.tipo] = {
            activos: items.filter((i) => i.activo !== 0).length,
            inactivos: items.filter((i) => i.activo === 0).length,
          };
        } catch {
          allStats[entry.tipo] = { activos: 0, inactivos: 0 };
        }
      }
      setStats(allStats);
      setStatsLoaded(true);
    };
    void fetchAll();
  }, [refreshTrigger]);

  const tiposFiltrados = useMemo(() => {
    if (!categoriaActiva) return ALL_TIPOS;
    return CATALOGO_GRUPOS.find((g) => g.categoria === categoriaActiva)?.tipos ?? ALL_TIPOS;
  }, [categoriaActiva]);

  if (activeTipo) {
    const entry = ALL_TIPOS.find((e) => e.tipo === activeTipo);
    if (!entry) return null;
    return (
      <div className="catalogos-detail-view">
        <button type="button" className="catalogos-back-btn" onClick={() => { setActiveTipo(null); }} aria-label="Volver a catálogos">
          <AppIcon icon={LibraryBig} size={16} />
          <span>Ver todos los catálogos</span>
        </button>
        <Suspense fallback={<SkeletonTable columns={4} rows={5} />}>
          <CatalogosTab
            tipo={entry.tipo}
            titulo={entry.titulo}
            canManage
            onModified={onDataModified}
            refreshTrigger={refreshTrigger}
          />
        </Suspense>
      </div>
    );
  }

  return (
    <div className="catalogos-panel">
      <div className="catalogos-header">
        <div className="catalogos-header-title">
          <AppIcon icon={LibraryBig} size={22} />
          <div>
            <h2>Catálogos del Sistema</h2>
            <p className="catalogos-header-desc">Administre los valores de referencia para patentes, productos, financiamiento y otros parámetros configurables del sistema.</p>
          </div>
        </div>
      </div>

      <div className="catalogos-category-strip">
        <button
          type="button"
          className={`catalogos-category-pill ${!categoriaActiva ? 'active' : ''}`}
          onClick={() => { setCategoriaActiva(null); }}
        >
          Todos
        </button>
        {CATALOGO_GRUPOS.map((grupo) => (
          <button
            key={grupo.categoria}
            type="button"
            className={`catalogos-category-pill ${categoriaActiva === grupo.categoria ? 'active' : ''}`}
            onClick={() => { setCategoriaActiva(categoriaActiva === grupo.categoria ? null : grupo.categoria); }}
          >
            <AppIcon icon={grupo.icon} size={14} />
            <span>{grupo.categoria}</span>
          </button>
        ))}
      </div>

      {!statsLoaded ? (
        <CatalogosSkeleton />
      ) : (
        <div className="catalogos-grid">
          {tiposFiltrados.map((entry) => {
            const s = stats[entry.tipo] ?? { activos: 0, inactivos: 0 };
            return (
              <button
                key={entry.tipo}
                type="button"
                className="catalogo-summary-card"
                onClick={() => { setActiveTipo(entry.tipo); }}
                aria-label={`Administrar ${entry.titulo}`}
              >
                <div className="catalogo-summary-header">
                  <span className="catalogo-summary-icon">
                    <AppIcon icon={entry.icon} size={20} />
                  </span>
                  <div className="catalogo-summary-title-group">
                    <span className="catalogo-summary-title">{entry.titulo}</span>
                    <span className="catalogo-summary-desc">{entry.descripcion}</span>
                  </div>
                </div>
                <div className="catalogo-summary-stats">
                  <span className="catalogo-stat">
                    <span className="catalogo-stat-value">{s.activos}</span>
                    <span className="catalogo-stat-label">activos</span>
                  </span>
                  <span className="catalogo-stat-divider" aria-hidden="true" />
                  <span className="catalogo-stat">
                    <span className="catalogo-stat-value muted">{s.inactivos}</span>
                    <span className="catalogo-stat-label">inactivos</span>
                  </span>
                </div>
                <span className="catalogo-summary-hint">Administrar &rarr;</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
};
