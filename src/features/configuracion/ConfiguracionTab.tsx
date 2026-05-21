import React, { lazy, Suspense, useMemo, useState } from 'react';
import { GraduationCap, LibraryBig, Users } from 'lucide-react';
import type { Usuario } from '../auth/api';
import { hasPermission } from '@/shared/auth/permissions';
import { AppIcon } from '@/shared/ui/AppIcon';
import { SkeletonBlock, SkeletonTable } from '@/shared/ui/Skeleton';

const GradosTab = lazy(async () => {
  const module = await import('./grados/GradosTab');
  return { default: module.GradosTab };
});

const CatalogosPanel = lazy(async () => {
  const module = await import('./catalogos/CatalogosPanel');
  return { default: module.CatalogosPanel };
});

const UsuariosTab = lazy(async () => {
  const module = await import('./usuarios/UsuariosTab');
  return { default: module.UsuariosTab };
});

type ConfigSection = 'grados' | 'catalogos' | 'usuarios';

interface ConfiguracionTabProps {
  currentUser: Usuario | null;
  refreshTrigger?: number;
  isAdmin: boolean;
  onDataModified: () => void;
}

const ConfigSectionFallback = () => (
  <div className="tab-panel">
    <div className="table-container">
      <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
      <SkeletonTable columns={5} rows={5} />
    </div>
  </div>
);

export const ConfiguracionTab: React.FC<ConfiguracionTabProps> = ({
  currentUser,
  refreshTrigger = 0,
  isAdmin,
  onDataModified,
}) => {
  const [activeSection, setActiveSection] = useState<ConfigSection>(isAdmin ? 'usuarios' : 'grados');
  const panelId = `config-panel-${activeSection}`;
  const canManageCatalogos = hasPermission(currentUser?.rol, 'grados.manage');
  const effectiveSection = useMemo(() => {
    if (!isAdmin && activeSection === 'usuarios') return 'grados';
    if (!canManageCatalogos && activeSection === 'catalogos') return 'grados';
    return activeSection;
  }, [activeSection, canManageCatalogos, isAdmin]);

  const sections = [
    {
      id: 'grados' as const,
      label: 'Grados',
      icon: GraduationCap,
      description: 'Catálogo académico base para el sistema.',
    },
    ...(canManageCatalogos
      ? [
          {
            id: 'catalogos' as const,
            label: 'Catálogos',
            icon: LibraryBig,
            description: 'Tipos de patentes, productos, financiamiento y monedas.',
          },
        ]
      : []),
    ...(isAdmin
      ? [
          {
            id: 'usuarios' as const,
            label: 'Usuarios',
            icon: Users,
            description: 'Altas, bajas y permisos de acceso al sistema.',
          },
        ]
      : []),
  ];

  return (
    <div className="tab-panel">
      <div className="settings-shell">
        <div className="settings-layout">
          <div className="settings-nav-panel">
            <div className="settings-nav" role="tablist" aria-label="Secciones de configuración">
              {sections.map((section) => (
                <button
                  key={section.id}
                  type="button"
                  role="tab"
                  aria-selected={effectiveSection === section.id}
                  aria-controls={`config-panel-${section.id}`}
                  id={`config-tab-${section.id}`}
                  className={`settings-nav-button ${effectiveSection === section.id ? 'active' : ''}`}
                  onClick={() => { setActiveSection(section.id); }}
                >
                  <span className="settings-nav-icon">
                    <AppIcon icon={section.icon} size={18} />
                  </span>
                  <span className="settings-nav-copy">
                    <strong>{section.label}</strong>
                    <small>{section.description}</small>
                  </span>
                </button>
              ))}
            </div>
          </div>

          <div
            id={panelId}
            role="tabpanel"
            aria-labelledby={`config-tab-${effectiveSection}`}
            className="settings-content settings-content-panel"
          >
            <Suspense fallback={<ConfigSectionFallback />}>
              {effectiveSection === 'grados' && (
                currentUser ? <GradosTab onGradoModified={onDataModified} refreshTrigger={refreshTrigger} /> : null
              )}

              {effectiveSection === 'catalogos' && canManageCatalogos && (
                <CatalogosPanel currentUser={currentUser} onDataModified={onDataModified} refreshTrigger={refreshTrigger} />
              )}

              {effectiveSection === 'usuarios' && isAdmin && (
                currentUser ? <UsuariosTab currentUser={currentUser} onUsuarioModified={onDataModified} refreshTrigger={refreshTrigger} /> : null
              )}
            </Suspense>
          </div>
        </div>
      </div>
    </div>
  );
};