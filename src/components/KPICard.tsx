import React from 'react';
import type { LucideIcon } from 'lucide-react';
import { AppIcon } from './AppIcon';

interface KPICardProps {
  icon: LucideIcon;
  value: number | string;
  label: string;
}

export const KPICard: React.FC<KPICardProps> = ({ icon, value, label }) => (
  <div className="kpi-card">
    <div className="kpi-icon">
      <AppIcon icon={icon} size={26} strokeWidth={2.1} />
    </div>
    <div className="kpi-content">
      <div className="kpi-value">{value}</div>
      <div className="kpi-label">{label}</div>
    </div>
  </div>
);