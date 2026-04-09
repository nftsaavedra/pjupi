import React, { useEffect, useState } from 'react';
import { GraduationCap, Users } from 'lucide-react';
import { AppIcon } from '../shared/ui/AppIcon';
import { GradosTab } from './GradosTab';
import { UsuariosTab } from './UsuariosTab';

type ConfigSection = 'grados' | 'usuarios';

interface ConfiguracionTabProps {
  refreshTrigger?: number;
  isAdmin: boolean;
  onDataModified: () => void;
}

export const ConfiguracionTab: React.FC<ConfiguracionTabProps> = ({
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
            {activeSection === 'grados' && (
              <GradosTab onGradoModified={onDataModified} refreshTrigger={refreshTrigger} />
            )}

            {activeSection === 'usuarios' && isAdmin && (
              <UsuariosTab onUsuarioModified={onDataModified} refreshTrigger={refreshTrigger} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
};