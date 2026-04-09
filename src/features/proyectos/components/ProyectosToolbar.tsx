import React from 'react';

interface ProyectosToolbarProps {
  busqueda: string;
  estadoFiltro: 'todos' | 'activos' | 'inactivos';
  totalActivos: number;
  totalInactivos: number;
  totalTodos: number;
  totalVisibles: number;
  onBusquedaChange: (value: string) => void;
  onEstadoFiltroChange: (value: 'todos' | 'activos' | 'inactivos') => void;
}

export const ProyectosToolbar: React.FC<ProyectosToolbarProps> = ({
  busqueda,
  estadoFiltro,
  totalActivos,
  totalInactivos,
  totalTodos,
  totalVisibles,
  onBusquedaChange,
  onEstadoFiltroChange,
}) => (
  <div className="filter-bar">
    <div className="filter-summary-group">
      <div className="filter-summary">Visibles: {totalVisibles}</div>
      <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
      <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
      <span className="status-chip status-chip-total">Todos: {totalTodos}</span>
    </div>
    <input
      className="form-input filter-search"
      placeholder="Buscar por título o perfil docente"
      value={busqueda}
      onChange={(e) => onBusquedaChange(e.target.value)}
      aria-label="Buscar proyectos por título o perfil docente"
    />
    <select
      className="form-input filter-select"
      value={estadoFiltro}
      onChange={(e) => onEstadoFiltroChange(e.target.value as 'todos' | 'activos' | 'inactivos')}
      aria-label="Filtrar proyectos por estado"
    >
      <option value="todos">Todos</option>
      <option value="activos">Solo activos</option>
      <option value="inactivos">Solo inactivos</option>
    </select>
  </div>
);