import React, { type ButtonHTMLAttributes } from 'react';
import type { LucideIcon } from 'lucide-react';
import { FloatingTooltip } from '../overlays/FloatingTooltip';
import { AppIcon } from './AppIcon';

interface TableActionButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'children'> {
  icon: LucideIcon;
  label: string;
  iconSize?: number;
}

export const TableActionButton: React.FC<TableActionButtonProps> = ({
  icon,
  label,
  className,
  type = 'button',
  iconSize = 16,
  ...buttonProps
}) => (
  <FloatingTooltip
    content={label}
    size="sm"
    placement="top"
    offsetValue={8}
    renderTrigger={({ ref, triggerProps }) => (
      <span className="table-action-button-wrapper">
        <button
          type={type}
          ref={ref}
          className={className ? `table-action-button ${className}` : 'table-action-button'}
          aria-label={label}
          {...triggerProps}
          {...buttonProps}
        >
          <AppIcon icon={icon} size={iconSize} />
          <span className="visually-hidden">{label}</span>
        </button>
      </span>
    )}
  />
);