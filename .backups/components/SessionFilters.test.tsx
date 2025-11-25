import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { act } from '@testing-library/react';
import { SessionFilters } from './SessionFilters';
import { useAppStore } from '../store';

describe('SessionFilters', () => {
  beforeEach(() => {
    // Reset store state before each test
    act(() => {
      useAppStore.setState({
        sessionFilters: { status: [], os: [] },
      });
    });
  });

  it('should render all status filter buttons', () => {
    render(<SessionFilters />);

    expect(screen.getByText('Active')).toBeInTheDocument();
    expect(screen.getByText('Sleep')).toBeInTheDocument();
    expect(screen.getByText('Dead')).toBeInTheDocument();
  });

  it('should render all OS filter buttons', () => {
    render(<SessionFilters />);

    expect(screen.getByText('Windows')).toBeInTheDocument();
    expect(screen.getByText('Linux')).toBeInTheDocument();
    expect(screen.getByText('macOS')).toBeInTheDocument();
  });

  it('should toggle status filter when clicked', () => {
    render(<SessionFilters />);

    const activeButton = screen.getByText('Active');
    fireEvent.click(activeButton);

    expect(useAppStore.getState().sessionFilters.status).toContain('active');

    // Click again to remove
    fireEvent.click(activeButton);
    expect(useAppStore.getState().sessionFilters.status).not.toContain('active');
  });

  it('should toggle OS filter when clicked', () => {
    render(<SessionFilters />);

    const linuxButton = screen.getByText('Linux');
    fireEvent.click(linuxButton);

    expect(useAppStore.getState().sessionFilters.os).toContain('linux');

    // Click again to remove
    fireEvent.click(linuxButton);
    expect(useAppStore.getState().sessionFilters.os).not.toContain('linux');
  });

  it('should show clear button when filters are active', () => {
    act(() => {
      useAppStore.setState({
        sessionFilters: { status: ['active'], os: [] },
      });
    });

    render(<SessionFilters />);

    expect(screen.getByText('Clear')).toBeInTheDocument();
  });

  it('should not show clear button when no filters', () => {
    render(<SessionFilters />);

    expect(screen.queryByText('Clear')).not.toBeInTheDocument();
  });

  it('should clear all filters when clear button clicked', () => {
    act(() => {
      useAppStore.setState({
        sessionFilters: { status: ['active', 'dead'], os: ['linux'] },
      });
    });

    render(<SessionFilters />);

    const clearButton = screen.getByText('Clear');
    fireEvent.click(clearButton);

    expect(useAppStore.getState().sessionFilters.status).toEqual([]);
    expect(useAppStore.getState().sessionFilters.os).toEqual([]);
  });

  it('should allow multiple status filters', () => {
    render(<SessionFilters />);

    fireEvent.click(screen.getByText('Active'));
    fireEvent.click(screen.getByText('Dead'));

    const filters = useAppStore.getState().sessionFilters.status;
    expect(filters).toContain('active');
    expect(filters).toContain('dead');
    expect(filters).toHaveLength(2);
  });
});
