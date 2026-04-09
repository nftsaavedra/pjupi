import { lazy, Suspense, useEffect, useState } from 'react';
import { BookOpen, ChevronLeft, ChevronRight, FileSpreadsheet, FolderOpen, GraduationCap, LayoutDashboard, LogOut, Settings2 } from 'lucide-react';
import { AppIcon } from './shared/ui/AppIcon';
import { getAuthStatus, type Usuario } from './features/auth/api';
import { AuthScreen } from './features/auth/AuthScreen';
import { SkeletonBlock, SkeletonChart, SkeletonKpiGrid, SkeletonTable } from './shared/ui/Skeleton';
import { ToastContainer } from './shared/feedback/ToastContainer';
import { FloatingTooltip } from './shared/overlays/FloatingTooltip';
import { TabNavigation, type Tab } from './shared/navigation/TabNavigation';
import { toast } from './services/toast';
import './App.css';

const DashboardTab = lazy(async () => {
  const module = await import('./features/dashboard/DashboardTab');
  return { default: module.DashboardTab };
});

const ProyectosTab = lazy(async () => {
  const module = await import('./features/proyectos/ProyectosTab');
  return { default: module.ProyectosTab };
});

const DocenteCreateModal = lazy(async () => {
  const module = await import('./features/docentes/components/DocenteCreateModal');
  return { default: module.DocenteCreateModal };
});

const DocentesTable = lazy(async () => {
  const module = await import('./features/docentes/components/DocentesTable');
  return { default: module.DocentesTable };
});

const ReportesTab = lazy(async () => {
  const module = await import('./features/reportes/ReportesTab');
  return { default: module.ReportesTab };
});

const ConfiguracionTab = lazy(async () => {
  const module = await import('./features/configuracion/ConfiguracionTab');
  return { default: module.ConfiguracionTab };
});

const DashboardFallback = () => (
  <div className="tab-panel">
    <SkeletonKpiGrid />
    <SkeletonChart titleWidth="md" height="lg" />
    <div className="two-col-charts">
      <SkeletonChart titleWidth="md" height="md" />
      <SkeletonChart titleWidth="md" height="md" />
    </div>
  </div>
);

const FormAndTableFallback = ({ columns }: { columns: number }) => (
  <div className="tab-panel">
    <div className="form-card" aria-hidden="true">
      <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
      <div className="form auth-loading-form">
        <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
        <SkeletonBlock className="skeleton skeleton-input" />
        <SkeletonBlock className="skeleton skeleton-line skeleton-line-soft" />
        <SkeletonBlock className="skeleton skeleton-input" />
        <SkeletonBlock className="skeleton skeleton-button" />
      </div>
    </div>
    <div className="table-container">
      <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
      <SkeletonTable columns={columns} rows={6} />
    </div>
  </div>
);

const TableOnlyFallback = ({ columns }: { columns: number }) => (
  <div className="tab-panel">
    <div className="table-container">
      <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
      <SkeletonTable columns={columns} rows={6} />
    </div>
  </div>
);

function App() {
  const [activeTab, setActiveTab] = useState<string>('dashboard');
  const [refreshTrigger, setRefreshTrigger] = useState(0);
  const [authLoading, setAuthLoading] = useState(true);
  const [requiresSetup, setRequiresSetup] = useState(false);
  const [currentUser, setCurrentUser] = useState<Usuario | null>(null);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [docenteFormOpen, setDocenteFormOpen] = useState(false);

  const tabs: Tab[] = [
    { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard, description: 'Indicadores clave' },
    { id: 'proyectos', label: 'Proyectos', icon: FolderOpen, description: 'Alta y seguimiento' },
    { id: 'docentes', label: 'Docentes', icon: GraduationCap, description: 'Registro y estado' },
    { id: 'reportes', label: 'Reportes', icon: FileSpreadsheet, description: 'Vista previa y exportación' },
    { id: 'configuracion', label: 'Configuración', icon: Settings2, description: 'Accesos y catálogos' },
  ];
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
  const activeTabMeta = tabs.find((tab) => tab.id === activeTab) ?? tabs[0];
  const activeHeaderMeta = tabHeaderMeta[activeTabMeta.id] ?? tabHeaderMeta.dashboard;

  const handleDataModified = () => {
    setRefreshTrigger(prev => prev + 1);
  };

  const cargarAuthStatus = async () => {
    setAuthLoading(true);
    try {
      const status = await getAuthStatus();
      setRequiresSetup(status.requires_setup);
    } catch (error) {
      toast.error(String(error));
    } finally {
      setAuthLoading(false);
    }
  };

  const handleAuthenticated = (usuario: Usuario) => {
    setCurrentUser(usuario);
    setRequiresSetup(false);
    setRefreshTrigger((prev) => prev + 1);
  };

  const handleLogout = () => {
    setCurrentUser(null);
    setActiveTab('dashboard');
  };

  const handleToggleSidebar = () => {
    setSidebarCollapsed((prev) => {
      const next = !prev;
      window.localStorage.setItem('pjupi.sidebarCollapsed', String(next));
      return next;
    });
  };

  const renderActiveTab = () => {
    switch (activeTab) {
      case 'dashboard':
        return (
          <Suspense fallback={<DashboardFallback />}>
            <DashboardTab refreshTrigger={refreshTrigger} />
          </Suspense>
        );
      case 'proyectos':
        return (
          <Suspense fallback={<FormAndTableFallback columns={5} />}>
            <ProyectosTab onProyectoCreated={handleDataModified} refreshTrigger={refreshTrigger} />
          </Suspense>
        );
      case 'docentes':
        return (
          <Suspense fallback={<FormAndTableFallback columns={6} />}>
            <div className="module-shell docentes-module">
              <DocentesTable
                onCreateClick={() => setDocenteFormOpen(true)}
                refreshTrigger={refreshTrigger}
              />
              {docenteFormOpen && (
                <Suspense fallback={null}>
                  <DocenteCreateModal
                    open={docenteFormOpen}
                    onClose={() => setDocenteFormOpen(false)}
                    onDocenteCreated={handleDataModified}
                    refreshTrigger={refreshTrigger}
                  />
                </Suspense>
              )}
            </div>
          </Suspense>
        );
      case 'configuracion':
        return (
          <Suspense fallback={<FormAndTableFallback columns={5} />}>
            <ConfiguracionTab
              onDataModified={handleDataModified}
              refreshTrigger={refreshTrigger}
              isAdmin={currentUser?.rol === 'admin'}
            />
          </Suspense>
        );
      case 'reportes':
        return (
          <Suspense fallback={<TableOnlyFallback columns={5} />}>
            <ReportesTab refreshTrigger={refreshTrigger} />
          </Suspense>
        );
      default:
        return null;
    }
  };

  useEffect(() => {
    cargarAuthStatus();
  }, []);

  useEffect(() => {
    const savedValue = window.localStorage.getItem('pjupi.sidebarCollapsed');

    if (savedValue === 'true' || savedValue === 'false') {
      setSidebarCollapsed(savedValue === 'true');
      return;
    }

    setSidebarCollapsed(window.innerWidth <= 1360 && window.innerWidth > 1024);
  }, []);

  useEffect(() => {
    if (!currentUser) return;

    const triggerRefresh = () => {
      if (document.visibilityState === 'visible') {
        setRefreshTrigger((prev) => prev + 1);
      }
    };

    const intervalId = window.setInterval(triggerRefresh, 15000);
    window.addEventListener('focus', triggerRefresh);
    document.addEventListener('visibilitychange', triggerRefresh);

    return () => {
      window.clearInterval(intervalId);
      window.removeEventListener('focus', triggerRefresh);
      document.removeEventListener('visibilitychange', triggerRefresh);
    };
  }, [currentUser]);

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
        <>
          <header className="app-header">
            <div className="header-content">
              <div>
                <h1 className="app-title title-with-icon">
                  <AppIcon icon={BookOpen} size={24} />
                  <span>UPI Research</span>
                </h1>
                <p className="app-subtitle">Gestión de Proyectos y Docentes</p>
              </div>
            </div>
          </header>
          <main className="main-content auth-main">
            <AuthScreen
              mode={requiresSetup ? 'setup' : 'login'}
              onAuthenticated={handleAuthenticated}
            />
          </main>
        </>
      ) : (
        <div className={`app-shell ${sidebarCollapsed ? 'sidebar-collapsed' : ''}`}>
          <aside id="app-sidebar" className="app-sidebar">
            <div className="sidebar-brand">
              <div className="sidebar-brand-mark">UPI</div>
              <div className="sidebar-brand-copy">
                <div className="sidebar-kicker">Research</div>
              </div>
              <FloatingTooltip
                content={sidebarCollapsed ? 'Expandir barra lateral' : 'Colapsar barra lateral'}
                size="sm"
                placement="right"
                offsetValue={12}
                renderTrigger={({ ref, triggerProps }) => (
                  <button
                    type="button"
                    ref={ref}
                    className="sidebar-toggle"
                    onClick={handleToggleSidebar}
                    aria-label={sidebarCollapsed ? 'Expandir barra lateral' : 'Colapsar barra lateral'}
                    {...triggerProps}
                  >
                    <AppIcon icon={sidebarCollapsed ? ChevronRight : ChevronLeft} size={18} />
                  </button>
                )}
              />
            </div>

            <TabNavigation
              tabs={tabs}
              activeTab={activeTab}
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
                  <small>{currentUser.rol}</small>
                </div>
              </div>
              <button className="btn-secondary sidebar-logout" onClick={handleLogout}>
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
                <span className="status-chip status-chip-total">Rol: {currentUser.rol}</span>
              </div>
            </header>

            <main id="main-content" className="main-content main-content-shell" tabIndex={-1}>
              {renderActiveTab()}
            </main>
          </div>
        </div>
      )}
      <ToastContainer />
    </div>
  );
}

export default App;
