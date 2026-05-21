import { SkeletonBlock, SkeletonChart, SkeletonKpiGrid, SkeletonTable } from './Skeleton';

export const DashboardFallback = () => (
  <div className="tab-panel">
    <SkeletonKpiGrid />
    <SkeletonChart titleWidth="md" height="lg" />
    <div className="two-col-charts">
      <SkeletonChart titleWidth="md" height="md" />
      <SkeletonChart titleWidth="md" height="md" />
    </div>
  </div>
);

export const FormAndTableFallback = ({ columns }: { columns: number }) => (
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

export const TableOnlyFallback = ({ columns }: { columns: number }) => (
  <div className="tab-panel">
    <div className="table-container">
      <SkeletonBlock className="skeleton skeleton-line skeleton-title-md" />
      <SkeletonTable columns={columns} rows={6} />
    </div>
  </div>
);
