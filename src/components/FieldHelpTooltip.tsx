import React from 'react';
import { Info } from 'lucide-react';
import { AppIcon } from './AppIcon';
import { FloatingTooltip } from './FloatingTooltip';

interface FieldHelpTooltipProps {
  content: React.ReactNode;
  label: string;
}

export const FieldHelpTooltip: React.FC<FieldHelpTooltipProps> = ({ content, label }) => {
  return (
    <FloatingTooltip
      content={content}
      size="rich"
      placement="top-start"
      offsetValue={10}
      renderTrigger={({ ref, triggerProps }) => (
        <button
          type="button"
          ref={ref as React.Ref<HTMLButtonElement>}
          className="field-help-trigger"
          aria-label={label}
          {...triggerProps}
        >
          <AppIcon icon={Info} size={14} />
        </button>
      )}
    />
  );
};