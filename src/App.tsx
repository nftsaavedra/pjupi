import { useCallback, useMemo, useState } from 'react';
import { BookOpen, ChevronLeft, ChevronRight, FileSpreadsheet, FolderOpen, GraduationCap, LayoutDashboard, LogOut, Settings2, Users } from 'lucide-react';
import { AppIcon } from './shared/ui/AppIcon';
import { type Usuario } from './features/auth/api';
import { SkeletonBlock } from './shared/ui/Skeleton';
import { ToastContainer } from './shared/feedback/ToastContainer';
import { TabNavigation, type Tab } from './shared/navigation/TabNavigation';
import { getRoleLabel, hasPermission } from './shared/auth/permissions';
import { AuthShell } from './app/AuthShell';
import { TabRenderers } from './app/TabRenderers';
import { useAuth } from './app/hooks/useAuth';
import { useAutoRefresh } from './app/hooks/useAutoRefresh';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState<string>('dashboard');
  const [refreshTrigger, setRefreshTrigger] = useState(0);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(() => {
    const savedValue = window.localStorage.getItem('pjupi.sidebarCollapsed');
    if (savedValue === 'true' || savedValue === 'false') {
      return savedValue === 'true';
    }
    return window.innerWidth <= 1360 && window.innerWidth > 1024;
  });
  const [docenteFormOpen, setDocenteFormOpen] = useState(false);

  const handleDataModified = useCallback(() => {
    setRefreshTrigger((prev) => prev + 1);
  }, []);

  const {
    authLoading,
    requiresSetup,
    currentUser,
    handleAuthenticated: baseHandleAuth,
    handleLogout: baseHandleLogout,
  } = useAuth();

  useAutoRefresh(currentUser, handleDataModified);

  const handleAuthenticated = (usuario: Usuario) => {
    baseHandleAuth(usuario);
    handleDataModified();
  };

  const handleLogout = async () => {
    await baseHandleLogout();
    setActiveTab('dashboard');
  };

  const currentRole = currentUser?.rol ?? null;

  const tabs: Tab[] = useMemo(() => [
    ...(hasPermission(currentRole, 'dashboard.view') ? [{ id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard, description: 'Indicadores clave' }] : []),
    ...(hasPermission(currentRole, 'proyectos.view') ? [{ id: 'proyectos', label: 'Proyectos', icon: FolderOpen, description: 'Alta y seguimiento' }] : []),
    ...(hasPermission(currentRole, 'docentes.view') ? [{ id: 'docentes', label: 'Docentes', icon: GraduationCap, description: 'Registro y estado' }] : []),
    ...(hasPermission(currentRole, 'grupos.view') ? [{ id: 'grupos', label: 'Grupos', icon: Users, description: 'Investigación coordinada' }] : []),
    ...(hasPermission(currentRole, 'reportes.view') ? [{ id: 'reportes', label: 'Reportes', icon: FileSpreadsheet, description: 'Vista previa y exportación' }] : []),
    ...(hasPermission(currentRole, 'configuracion.view') ? [{ id: 'configuracion', label: 'Configuración', icon: Settings2, description: 'Accesos y catálogos' }] : []),
  ], [currentRole]);

  const tabHeaderMeta: Record<string, { kicker: string; title: string; subtitle: string }> = {
    dashboard: {
      kicker: 'Indicadores clave',
      title: 'Dashboard',
      subtitle: 'Carga docente y proyectos en una sola vista.',
    },
    proyectos: {
      kicker: 'Gestión operativa',
      title: 'Proyectos',
      subtitle: 'Alta, asignación y seguimiento de proyectos.',
    },
    docentes: {
      kicker: 'Gestión operativa',
      title: 'Docentes',
      subtitle: 'Registro, estado y trazabilidad docente.',
    },
    grupos: {
      kicker: 'Investigación',
      title: 'Grupos de Investigación',
      subtitle: 'Coordinación y líneas de investigación.',
    },
    reportes: {
      kicker: 'Análisis y salida',
      title: 'Reportes',
      subtitle: 'Vista previa, filtros y exportación.',
    },
    configuracion: {
      kicker: 'Administración base',
      title: 'Configuración',
      subtitle: 'Accesos y catálogos del sistema.',
    },
  };

  const validActiveTab = (!currentUser || tabs.length === 0 || tabs.some((tab) => tab.id === activeTab))
    ? activeTab
    : tabs[0].id;

  const activeTabMeta = tabs.find((tab) => tab.id === validActiveTab) ?? tabs[0];
  const activeHeaderMeta = tabHeaderMeta[activeTabMeta.id] ?? tabHeaderMeta.dashboard;

  const handleToggleSidebar = () => {
    setSidebarCollapsed((prev) => {
      const next = !prev;
      window.localStorage.setItem('pjupi.sidebarCollapsed', String(next));
      return next;
    });
  };

  if (authLoading) {
    return (
      <div className="app-container">
        <header className="app-header">
          <div className="header-content">
            <div>
              <h1 className="app-title title-with-icon">
                <AppIcon icon={BookOpen} size={24} />
                <span>UPI Research</span>
              </h1>
              <p className="app-subtitle">Verificando acceso al sistema</p>
            </div>
          </div>
        </header>
        <main className="main-content auth-main">
          <div className="auth-shell">
            <div className="auth-card auth-card-loading" aria-hidden="true">
              <div className="auth-card-header">
                <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
                <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
              </div>
              <div className="form auth-loading-form">
                <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
                <SkeletonBlock className="skeleton skeleton-input" />
                <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
                <SkeletonBlock className="skeleton skeleton-input" />
                <SkeletonBlock className="skeleton skeleton-button" />
              </div>
            </div>
          </div>
        </main>
        <ToastContainer />
      </div>
    );
  }

  return (
    <div className="app-container">
      {currentUser && (
        <a className="skip-link" href="#main-content">
          Saltar al contenido principal
        </a>
      )}
      {!currentUser ? (
        <AuthShell requiresSetup={requiresSetup} onAuthenticated={handleAuthenticated} />
      ) : (
        <div className={`app-shell ${sidebarCollapsed ? 'sidebar-collapsed' : ''}`}>
          <aside id="app-sidebar" className="app-sidebar">
            <div className="sidebar-brand">
              <div className="sidebar-brand-mark">UPI</div>
              <div className="sidebar-brand-copy">
                <div className="sidebar-kicker">Research</div>
              </div>
              <button
                type="button"
                className="sidebar-toggle"
                onClick={handleToggleSidebar}
                aria-label={sidebarCollapsed ? 'Expandir barra lateral' : 'Colapsar barra lateral'}
                aria-controls="app-sidebar"
                aria-expanded={!sidebarCollapsed}
              >
                <AppIcon icon={sidebarCollapsed ? ChevronRight : ChevronLeft} size={18} />
              </button>
            </div>

            <TabNavigation
              tabs={tabs}
              activeTab={validActiveTab}
              onTabChange={setActiveTab}
              variant="sidebar"
              collapsed={sidebarCollapsed}
              ariaLabel="Navegación principal"
            />

            <div className="sidebar-footer">
              <div className="sidebar-user-card">
                <div className="sidebar-user-avatar">{currentUser.nombre_completo.charAt(0).toUpperCase()}</div>
                <div className="sidebar-user-copy">
                  <strong>{currentUser.nombre_completo}</strong>
                  <span>@{currentUser.username}</span>
                  <small>{getRoleLabel(currentUser.rol)}</small>
                </div>
              </div>
              <button className="btn-secondary sidebar-logout" onClick={() => void handleLogout()}>
                <span className="sidebar-logout-icon">
                  <AppIcon icon={LogOut} size={18} />
                </span>
                <span className="sidebar-logout-label">Cerrar sesión</span>
              </button>
            </div>
          </aside>

          <div className="app-workspace">
            <header className="content-header">
              <div className="content-header-meta subtle-module-meta">
                <span className="content-kicker">{activeHeaderMeta.kicker}</span>
                <div className="content-module-inline">
                  {activeTabMeta.icon && <AppIcon icon={activeTabMeta.icon} size={17} />}
                  <strong>{activeHeaderMeta.title}</strong>
                  <span>{activeHeaderMeta.subtitle}</span>
                </div>
              </div>
              <div className="content-header-actions">
                <button
                  type="button"
                  className="content-sidebar-toggle"
                  onClick={handleToggleSidebar}
                  aria-label={sidebarCollapsed ? 'Expandir navegación' : 'Colapsar navegación'}
                  aria-controls="app-sidebar"
                  aria-expanded={!sidebarCollapsed}
                >
                  <span className="button-with-icon">
                    <AppIcon icon={sidebarCollapsed ? ChevronRight : ChevronLeft} size={18} />
                    <span>Menú</span>
                  </span>
                </button>
                <span className="status-chip status-chip-total">Rol: {getRoleLabel(currentUser.rol)}</span>
              </div>
            </header>

            <main id="main-content" className="main-content main-content-shell" tabIndex={-1}>
              <TabRenderers
                validActiveTab={validActiveTab}
                currentUser={currentUser}
                currentRole={currentRole}
                refreshTrigger={refreshTrigger}
                onDataModified={handleDataModified}
                docenteFormOpen={docenteFormOpen}
                setDocenteFormOpen={setDocenteFormOpen}
              />
            </main>
          </div>
        </div>
      )}
      <ToastContainer />
    </div>
  );
}

export default App;
