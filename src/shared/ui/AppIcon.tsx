import React from 'react';
import type { LucideIcon } from 'lucide-react';

interface AppIconProps {
  icon: LucideIcon;
  size?: number;
  strokeWidth?: number;
  className?: string;
  ariaHidden?: boolean;
}

export const AppIcon: React.FC<AppIconProps> = ({
  icon: Icon,
  size = 18,
  strokeWidth = 2,
  className,
  ariaHidden = true,
}) => (
  <Icon
    size={size}
    strokeWidth={strokeWidth}
    aria-hidden={ariaHidden}
    className={className}
  />
);