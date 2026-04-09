import React, { type ButtonHTMLAttributes } from 'react';
import type { LucideIcon } from 'lucide-react';
import { FloatingTooltip } from '../overlays/FloatingTooltip';
import { AppIcon } from './AppIcon';

interface InlineIconButtonProps extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'children'> {
  icon: LucideIcon;
  label: string;
  iconSize?: number;
}

export const InlineIconButton: React.FC<InlineIconButtonProps> = ({
  icon,
  label,
  className,
  type = 'button',
  iconSize = 14,
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
          ref={ref as React.Ref<HTMLButtonElement>}
          className={className ? `renacyt-inline-icon-button ${className}` : 'renacyt-inline-icon-button'}
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