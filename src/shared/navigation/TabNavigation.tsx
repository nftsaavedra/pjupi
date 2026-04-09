import React from 'react';
import type { LucideIcon } from 'lucide-react';
import { FloatingTooltip } from '../overlays/FloatingTooltip';
import { AppIcon } from '../ui/AppIcon';

export interface Tab {
  id: string;
  label: string;
  icon?: LucideIcon;
  description?: string;
}

interface TabNavigationProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
  variant?: 'topbar' | 'sidebar';
  collapsed?: boolean;
  ariaLabel?: string;
}

export const TabNavigation: React.FC<TabNavigationProps> = ({
  tabs,
  activeTab,
  onTabChange,
  variant = 'topbar',
  collapsed = false,
  ariaLabel,
}) => {
  return (
    <nav
      className={`tab-navigation tab-navigation-${variant} ${collapsed ? 'is-collapsed' : ''}`}
      aria-label={ariaLabel}
    >
      {tabs.map((tab) => {
        const tabAriaLabel = collapsed && tab.description ? `${tab.label}. ${tab.description}` : tab.label;

        if (variant === 'sidebar' && collapsed) {
          return (
            <FloatingTooltip
              key={tab.id}
              content={tab.label}
              size="sm"
              placement="right"
              offsetValue={12}
              renderTrigger={({ ref, triggerProps }) => (
                <button
                  type="button"
                  ref={ref}
                  className={`tab-button ${activeTab === tab.id ? 'active' : ''}`}
                  onClick={() => onTabChange(tab.id)}
                  aria-current={activeTab === tab.id ? 'page' : undefined}
                  aria-label={tabAriaLabel}
                  {...triggerProps}
                >
                  {tab.icon && (
                    <span className="tab-icon">
                      <AppIcon icon={tab.icon} size={18} />
                    </span>
                  )}
                  <span className="tab-button-copy">
                    <span className="tab-button-label">{tab.label}</span>
                    {variant === 'sidebar' && tab.description && (
                      <span className="tab-button-description">{tab.description}</span>
                    )}
                  </span>
                </button>
              )}
            />
          );
        }

        return (
          <button
            key={tab.id}
            type="button"
            className={`tab-button ${activeTab === tab.id ? 'active' : ''}`}
            onClick={() => onTabChange(tab.id)}
            aria-current={activeTab === tab.id ? 'page' : undefined}
            aria-label={tabAriaLabel}
          >
            {tab.icon && (
              <span className="tab-icon">
                <AppIcon icon={tab.icon} size={18} />
              </span>
            )}
            <span className="tab-button-copy">
              <span className="tab-button-label">{tab.label}</span>
              {variant === 'sidebar' && tab.description && (
                <span className="tab-button-description">{tab.description}</span>
              )}
            </span>
          </button>
        );
      })}
    </nav>
  );
};