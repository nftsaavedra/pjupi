import React, { useCallback, useRef, useState } from 'react';
import {
  FloatingArrow,
  FloatingPortal,
  Placement,
  autoUpdate,
  flip,
  offset,
  safePolygon,
  shift,
  useDismiss,
  useFocus,
  useFloating,
  useHover,
  useInteractions,
  useRole,
} from '@floating-ui/react';

interface FloatingTooltipProps {
  content: React.ReactNode;
  renderTrigger: (props: {
    ref: React.RefCallback<HTMLButtonElement>;
    triggerProps: React.ButtonHTMLAttributes<HTMLButtonElement>;
  }) => React.ReactNode;
  size?: 'sm' | 'md' | 'rich';
  placement?: Placement;
  offsetValue?: number;
  tooltipClassName?: string;
  contentClassName?: string;
  arrowClassName?: string;
}

export const FloatingTooltip: React.FC<FloatingTooltipProps> = ({
  content,
  renderTrigger,
  size = 'md',
  placement = 'top-start',
  offsetValue = 10,
  tooltipClassName,
  contentClassName,
  arrowClassName,
}) => {
  const [open, setOpen] = useState(false);
  const arrowRef = useRef<SVGSVGElement | null>(null);

  const { refs, floatingStyles, context } = useFloating({
    open,
    onOpenChange: setOpen,
    placement,
    middleware: [
      offset(offsetValue),
      flip({ padding: 8 }),
      shift({ padding: 8 }),
    ],
    whileElementsMounted: autoUpdate,
  });

  const hover = useHover(context, {
    move: false,
    delay: { open: 90, close: 60 },
    handleClose: safePolygon({ buffer: 4 }),
  });
  const focus = useFocus(context);
  const dismiss = useDismiss(context);
  const role = useRole(context, { role: 'tooltip' });

  const { getReferenceProps, getFloatingProps } = useInteractions([
    hover,
    focus,
    dismiss,
    role,
  ]);

  const referenceRef = useCallback(
    (node: HTMLElement | null) => { refs.setReference(node); },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [refs.setReference],
  );
  const floatingRef = useCallback(
    (node: HTMLElement | null) => { refs.setFloating(node); },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [refs.setFloating],
  );

  return (
    <>
      {renderTrigger({
        ref: referenceRef,
        triggerProps: getReferenceProps(),
      })}
      {open && (
        <FloatingPortal>
          <div
            ref={floatingRef}
            style={floatingStyles}
            className={tooltipClassName ? `floating-tooltip floating-tooltip-${size} ${tooltipClassName}` : `floating-tooltip floating-tooltip-${size}`}
            {...getFloatingProps()}
          >
            <FloatingArrow
              ref={arrowRef}
              context={context}
              className={arrowClassName ? `floating-tooltip-arrow ${arrowClassName}` : 'floating-tooltip-arrow'}
              fill="rgba(15, 23, 42, 0.96)"
            />
            <div className={contentClassName ? `floating-tooltip-content ${contentClassName}` : 'floating-tooltip-content'}>{content}</div>
          </div>
        </FloatingPortal>
      )}
    </>
  );
};