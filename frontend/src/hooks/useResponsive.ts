import { useState, useEffect } from 'react';

// Breakpoints definition
export const Breakpoints = {
  xs: 0, // <576px
  sm: 576, // >=576px
  md: 768, // >=768px
  lg: 992, // >=992px
  xl: 1200, // >=1200px
  xxl: 1600, // >=1600px
} as const;

export type BreakpointKey = keyof typeof Breakpoints;

// Responsive hook
export const useResponsive = () => {
  const [size, setSize] = useState<BreakpointKey>('xs');
  const [windowWidth, setWindowWidth] = useState<number>(0);

  useEffect(() => {
    // Initial width
    const initialWidth = window.innerWidth;
    setWindowWidth(initialWidth);

    // Helper function to determine current breakpoint
    const getBreakpoint = (width: number): BreakpointKey => {
      if (width >= Breakpoints.xxl) return 'xxl';
      if (width >= Breakpoints.xl) return 'xl';
      if (width >= Breakpoints.lg) return 'lg';
      if (width >= Breakpoints.md) return 'md';
      if (width >= Breakpoints.sm) return 'sm';
      return 'xs';
    };

    // Set initial breakpoint
    setSize(getBreakpoint(initialWidth));

    // Resize handler
    const handleResize = () => {
      const newWidth = window.innerWidth;
      setWindowWidth(newWidth);
      setSize(getBreakpoint(newWidth));
    };

    // Add event listener
    window.addEventListener('resize', handleResize);

    // Cleanup
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  // Responsive predicates
  const isMobile = size === 'xs' || size === 'sm';
  const isTablet = size === 'sm' || size === 'md';
  const isDesktop = size === 'md' || size === 'lg' || size === 'xl' || size === 'xxl';
  const isLargeDesktop = size === 'xl' || size === 'xxl';

  // Helper function to check if current size matches breakpoint
  const matches = (breakpoint: BreakpointKey): boolean => {
    return windowWidth >= Breakpoints[breakpoint];
  };

  // Helper function to check if current size is less than breakpoint
  const lessThan = (breakpoint: BreakpointKey): boolean => {
    return windowWidth < Breakpoints[breakpoint];
  };

  // Helper function to check if current size is greater than or equal to breakpoint
  const greaterThanOrEqual = (breakpoint: BreakpointKey): boolean => {
    return windowWidth >= Breakpoints[breakpoint];
  };

  // Helper function to get columns for grid
  const getGridProps = (xs = 24, sm = 12, md = 8, lg = 6, xl = 6, xxl = 6) => ({
    xs,
    sm,
    md,
    lg,
    xl,
    xxl,
  });

  return {
    size,
    width: windowWidth,
    isMobile,
    isTablet,
    isDesktop,
    isLargeDesktop,
    matches,
    lessThan,
    greaterThanOrEqual,
    getGridProps,
  };
};

export default useResponsive;
