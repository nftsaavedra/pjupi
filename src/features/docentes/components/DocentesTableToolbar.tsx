import React from 'react';

interface DocentesTableToolbarProps {
  busqueda: string;
  estadoFiltro: 'todos' | 'activos' | 'inactivos';
  gradoFiltro: string;
  gradosDisponibles: string[];
  nivelesRenacytDisponibles: string[];
  renacytNivelFiltro: string;
  totalVisibles: number;
  totalTodos: number;
  totalActivos: number;
  totalInactivos: number;
  onBusquedaChange: (value: string) => void;
  onEstadoFiltroChange: (value: 'todos' | 'activos' | 'inactivos') => void;
  onGradoFiltroChange: (value: string) => void;
  onRenacytNivelFiltroChange: (value: string) => void;
}

export const DocentesTableToolbar: React.FC<DocentesTableToolbarProps> = ({
  busqueda,
  estadoFiltro,
  gradoFiltro,
  gradosDisponibles,
  nivelesRenacytDisponibles,
  renacytNivelFiltro,
  totalVisibles,
  totalTodos,
  totalActivos,
  totalInactivos,
  onBusquedaChange,
  onEstadoFiltroChange,
  onGradoFiltroChange,
  onRenacytNivelFiltroChange,
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
      placeholder="Buscar por nombre, DNI, grado o nivel RENACYT"
      value={busqueda}
      onChange={(e) => { onBusquedaChange(e.target.value); }}
      aria-label="Buscar docentes por nombre, DNI, grado o nivel RENACYT"
    />
    <select
      className="form-input filter-select"
      value={gradoFiltro}
      onChange={(e) => { onGradoFiltroChange(e.target.value); }}
      aria-label="Filtrar docentes por grado"
    >
      <option value="todos">Todos los grados</option>
      {gradosDisponibles.map((grado) => (
        <option key={grado} value={grado}>{grado}</option>
      ))}
    </select>
    <select
      className="form-input filter-select"
      value={renacytNivelFiltro}
      onChange={(e) => { onRenacytNivelFiltroChange(e.target.value); }}
      aria-label="Filtrar docentes por nivel RENACYT"
    >
      <option value="todos">Todos los niveles RENACYT</option>
      {nivelesRenacytDisponibles.map((nivel) => (
        <option key={nivel} value={nivel}>{nivel}</option>
      ))}
    </select>
    <select
      className="form-input filter-select"
      value={estadoFiltro}
      onChange={(e) => { onEstadoFiltroChange(e.target.value as 'todos' | 'activos' | 'inactivos'); }}
      aria-label="Filtrar docentes por estado"
    >
      <option value="todos">Todos</option>
      <option value="activos">Solo activos</option>
      <option value="inactivos">Solo inactivos</option>
    </select>
  </div>
);