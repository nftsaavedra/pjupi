import React from 'react';

interface DocentesTableToolbarProps {
  busqueda: string;
  estadoFiltro: 'todos' | 'activos' | 'inactivos';
  totalVisibles: number;
  totalTodos: number;
  totalActivos: number;
  totalInactivos: number;
  onBusquedaChange: (value: string) => void;
  onEstadoFiltroChange: (value: 'todos' | 'activos' | 'inactivos') => void;
}

export const DocentesTableToolbar: React.FC<DocentesTableToolbarProps> = ({
  busqueda,
  estadoFiltro,
  totalVisibles,
  totalTodos,
  totalActivos,
  totalInactivos,
  onBusquedaChange,
  onEstadoFiltroChange,
}) => (
  <div className="filter-bar">
    <div className="filter-summary-group">
      <div className="filter-summary">Visibles: {totalVisibles}</div>
      <span className="status-chip status-chip-total">Todos: {totalTodos}</span>
      <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
      <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
    </div>
    <input
      className="form-input filter-search"
      placeholder="Buscar por nombre, DNI o grado"
      value={busqueda}
      onChange={(e) => onBusquedaChange(e.target.value)}
      aria-label="Buscar docentes por nombre, DNI o grado"
    />
    <select
      className="form-input filter-select"
      value={estadoFiltro}
      onChange={(e) => onEstadoFiltroChange(e.target.value as 'todos' | 'activos' | 'inactivos')}
      aria-label="Filtrar docentes por estado"
    >
      <option value="todos">Todos</option>
      <option value="activos">Solo activos</option>
      <option value="inactivos">Solo inactivos</option>
    </select>
  </div>
);