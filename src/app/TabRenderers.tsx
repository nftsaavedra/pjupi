import { Suspense } from 'react';
import { hasPermission } from '@/shared/auth/permissions';
import { DashboardFallback, FormAndTableFallback, TableOnlyFallback } from '@/shared/ui/SkeletonFallbacks';
import { type Usuario } from '@/features/auth/api';
import {
  DashboardTab,
  ProyectosTab,
  GruposTab,
  DocenteCreateModal,
  DocentesTable,
  ReportesTab,
  ConfiguracionTab,
} from '@/app/lazyImports';

interface TabRenderersProps {
  validActiveTab: string;
  currentUser: Usuario | null;
  currentRole: string | null;
  refreshTrigger: number;
  onDataModified: () => void;
  docenteFormOpen: boolean;
  setDocenteFormOpen: (open: boolean) => void;
}

export function TabRenderers({
  validActiveTab,
  currentUser,
  currentRole,
  refreshTrigger,
  onDataModified,
  docenteFormOpen,
  setDocenteFormOpen,
}: TabRenderersProps) {
  if (!currentUser) {
    return null;
  }

  switch (validActiveTab) {
    case 'dashboard':
      return (
        <Suspense fallback={<DashboardFallback />}>
          <DashboardTab refreshTrigger={refreshTrigger} />
        </Suspense>
      );
    case 'proyectos':
      return (
        <Suspense fallback={<FormAndTableFallback columns={5} />}>
          <ProyectosTab canManage={hasPermission(currentRole, 'proyectos.manage')} onProyectoCreated={onDataModified} refreshTrigger={refreshTrigger} />
        </Suspense>
      );
    case 'docentes':
      return (
        <Suspense fallback={<FormAndTableFallback columns={6} />}>
          <div className="module-shell docentes-module">
            <DocentesTable
              canManage={hasPermission(currentRole, 'docentes.manage')}
              onCreateClick={() => { setDocenteFormOpen(true); }}
              refreshTrigger={refreshTrigger}
            />
            {docenteFormOpen && hasPermission(currentRole, 'docentes.manage') && (
              <Suspense fallback={null}>
                <DocenteCreateModal
                  open={docenteFormOpen}
                  onClose={() => { setDocenteFormOpen(false); }}
                  onDocenteCreated={onDataModified}
                  refreshTrigger={refreshTrigger}
                />
              </Suspense>
            )}
          </div>
        </Suspense>
      );
    case 'grupos':
      return (
        <Suspense fallback={<FormAndTableFallback columns={4} />}>
          <GruposTab canManage={hasPermission(currentRole, 'grupos.manage')} />
        </Suspense>
      );
    case 'configuracion':
      if (!hasPermission(currentRole, 'configuracion.view')) {
        return null;
      }

      return (
        <Suspense fallback={<FormAndTableFallback columns={5} />}>
          <ConfiguracionTab
            currentUser={currentUser}
            onDataModified={onDataModified}
            refreshTrigger={refreshTrigger}
            isAdmin={hasPermission(currentRole, 'usuarios.manage')}
          />
        </Suspense>
      );
    case 'reportes':
      return (
        <Suspense fallback={<TableOnlyFallback columns={5} />}>
          <ReportesTab canExport={hasPermission(currentRole, 'reportes.export')} refreshTrigger={refreshTrigger} />
        </Suspense>
      );
    default:
      return null;
  }
}
