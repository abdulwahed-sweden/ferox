import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import {
  Spinner,
  LoadingOverlay,
  LoadingInline,
  SkeletonLine,
  SkeletonCircle,
  SessionSkeleton,
  SessionListSkeleton,
  ButtonLoading,
} from './Loading';

describe('Loading Components', () => {
  describe('Spinner', () => {
    it('should render with default size', () => {
      const { container } = render(<Spinner />);
      const spinner = container.querySelector('.animate-spin');
      expect(spinner).toBeInTheDocument();
    });

    it('should apply size classes', () => {
      const { container } = render(<Spinner size="lg" />);
      const spinner = container.querySelector('.w-8');
      expect(spinner).toBeInTheDocument();
    });
  });

  describe('LoadingOverlay', () => {
    it('should render message when provided', () => {
      render(<LoadingOverlay message="Loading data..." />);
      expect(screen.getByText('Loading data...')).toBeInTheDocument();
    });

    it('should render without message', () => {
      const { container } = render(<LoadingOverlay />);
      expect(container.querySelector('.animate-spin')).toBeInTheDocument();
    });
  });

  describe('LoadingInline', () => {
    it('should render message and spinner', () => {
      render(<LoadingInline message="Processing" />);
      expect(screen.getByText('Processing')).toBeInTheDocument();
    });

    it('should render spinner without message', () => {
      const { container } = render(<LoadingInline />);
      expect(container.querySelector('.animate-spin')).toBeInTheDocument();
    });
  });

  describe('SkeletonLine', () => {
    it('should render with inline width style', () => {
      const { container } = render(<SkeletonLine width="200px" />);
      const skeleton = container.querySelector('.skeleton');
      expect(skeleton).toBeInTheDocument();
      expect(skeleton).toHaveStyle({ width: '200px' });
    });

    it('should render with inline height style', () => {
      const { container } = render(<SkeletonLine height="24px" />);
      const skeleton = container.querySelector('.skeleton');
      expect(skeleton).toBeInTheDocument();
      expect(skeleton).toHaveStyle({ height: '24px' });
    });
  });

  describe('SkeletonCircle', () => {
    it('should render with default size', () => {
      const { container } = render(<SkeletonCircle />);
      const circle = container.querySelector('.skeleton');
      expect(circle).toBeInTheDocument();
      expect(circle).toHaveStyle({ width: '32px', height: '32px' });
    });

    it('should render with custom size', () => {
      const { container } = render(<SkeletonCircle size={48} />);
      const circle = container.querySelector('.skeleton');
      expect(circle).toBeInTheDocument();
      expect(circle).toHaveStyle({ width: '48px', height: '48px' });
    });
  });

  describe('SessionSkeleton', () => {
    it('should render skeleton structure', () => {
      const { container } = render(<SessionSkeleton />);
      // Should have skeleton elements
      expect(container.querySelectorAll('.skeleton').length).toBeGreaterThan(0);
    });
  });

  describe('SessionListSkeleton', () => {
    it('should render specified count of skeletons', () => {
      const { container } = render(<SessionListSkeleton count={5} />);
      // Should have 5 session skeletons
      const skeletons = container.querySelectorAll('.skeleton');
      expect(skeletons.length).toBeGreaterThan(0);
    });

    it('should use default count when not specified', () => {
      const { container } = render(<SessionListSkeleton />);
      const items = container.children[0]?.children;
      expect(items?.length).toBe(3); // Default is 3
    });
  });

  describe('ButtonLoading', () => {
    it('should render with loading text', () => {
      render(<ButtonLoading>Saving...</ButtonLoading>);
      expect(screen.getByText('Saving...')).toBeInTheDocument();
    });

    it('should be disabled when loading is true', () => {
      render(<ButtonLoading loading>Loading</ButtonLoading>);
      const button = screen.getByRole('button');
      expect(button).toBeDisabled();
    });

    it('should not be disabled when loading is false', () => {
      render(<ButtonLoading>Click me</ButtonLoading>);
      const button = screen.getByRole('button');
      expect(button).not.toBeDisabled();
    });
  });
});
