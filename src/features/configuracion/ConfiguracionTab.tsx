import React, { lazy, Suspense, useEffect, useState } from 'react';
import { GraduationCap, Users } from 'lucide-react';
import type { Usuario } from '../auth/api';
import { AppIcon } from '../../shared/ui/AppIcon';
import { SkeletonBlock, SkeletonTable } from '../../shared/ui/Skeleton';

const GradosTab = lazy(async () => {
  const module = await import('./grados/GradosTab');
  return { default: module.GradosTab };
});

const UsuariosTab = lazy(async () => {
  const module = await import('./usuarios/UsuariosTab');
  return { default: module.UsuariosTab };
});

type ConfigSection = 'grados' | 'usuarios';

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

  useEffect(() => {
    if (!isAdmin && activeSection === 'usuarios') {
      setActiveSection('grados');
    }
  }, [activeSection, isAdmin]);

  const sections = [
    {
      id: 'grados' as const,
      label: 'Grados',
      icon: GraduationCap,
      description: 'Catálogo académico base para el sistema.',
    },
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
                  aria-selected={activeSection === section.id}
                  aria-controls={`config-panel-${section.id}`}
                  id={`config-tab-${section.id}`}
                  className={`settings-nav-button ${activeSection === section.id ? 'active' : ''}`}
                  onClick={() => setActiveSection(section.id)}
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
            aria-labelledby={`config-tab-${activeSection}`}
            className="settings-content settings-content-panel"
          >
            <Suspense fallback={<ConfigSectionFallback />}>
              {activeSection === 'grados' && (
                currentUser ? <GradosTab onGradoModified={onDataModified} refreshTrigger={refreshTrigger} /> : null
              )}

              {activeSection === 'usuarios' && isAdmin && (
                currentUser ? <UsuariosTab currentUser={currentUser} onUsuarioModified={onDataModified} refreshTrigger={refreshTrigger} /> : null
              )}
            </Suspense>
          </div>
        </div>
      </div>
    </div>
  );
};