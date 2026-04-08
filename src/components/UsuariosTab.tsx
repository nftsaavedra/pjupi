import React, { useState } from 'react';
import { Pencil, Plus, RotateCcw, Save, ShieldPlus, Trash2 } from 'lucide-react';
import {
  actualizarUsuario,
  crearUsuario,
  desactivarUsuario,
  getTauriErrorMessage,
  reactivarUsuario,
  type Usuario,
} from '../services/tauriApi';
import { useFetchUsuarios } from '../hooks/useFetch';
import { useRefreshToast } from '../hooks/useRefreshToast';
import { AppIcon } from './AppIcon';
import { ConfirmDialog } from './ConfirmDialog';
import { FormModal } from './FormModal';
import { FormInput } from './FormInput';
import { FormSelect } from './FormSelect';
import { SkeletonTable } from './Skeleton';
import { toast } from '../services/toast';

interface UsuariosTabProps {
  onUsuarioModified: () => void;
  refreshTrigger?: number;
}

const roles = [
  { value: 'admin', label: 'Administrador' },
  { value: 'operador', label: 'Operador' },
  { value: 'consulta', label: 'Consulta' },
];

export const UsuariosTab: React.FC<UsuariosTabProps> = ({ onUsuarioModified, refreshTrigger = 0 }) => {
  const [username, setUsername] = useState('');
  const [nombreCompleto, setNombreCompleto] = useState('');
  const [rol, setRol] = useState('operador');
  const [password, setPassword] = useState('');
  const [editingUsuario, setEditingUsuario] = useState<Usuario | null>(null);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [usuarioToToggle, setUsuarioToToggle] = useState<Usuario | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [estadoFiltro, setEstadoFiltro] = useState<'todos' | 'activos' | 'inactivos'>('activos');
  const [busqueda, setBusqueda] = useState('');

  const { usuarios, loading, refreshing, error, recargar } = useFetchUsuarios(refreshTrigger);

  useRefreshToast({
    refreshing,
    message: 'Actualizando usuarios',
    toastKey: 'usuarios-refresh',
  });

  const resetForm = () => {
    setUsername('');
    setNombreCompleto('');
    setRol('operador');
    setPassword('');
    setEditingUsuario(null);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!username.trim() || !nombreCompleto.trim() || !rol) {
      toast.warning('Complete todos los campos del usuario');
      return;
    }

    if (!editingUsuario && password.trim().length < 8) {
      toast.warning('La contraseña debe tener al menos 8 caracteres');
      return;
    }

    setIsLoading(true);
    try {
      if (editingUsuario) {
        await actualizarUsuario(
          editingUsuario.id_usuario,
          username,
          nombreCompleto,
          rol,
          password || undefined,
        );
        toast.success('Usuario actualizado correctamente');
      } else {
        await crearUsuario(username, nombreCompleto, rol, password);
        toast.success('Usuario creado correctamente');
      }

      resetForm();
      setIsFormOpen(false);
      await recargar();
      onUsuarioModified();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleEditar = (usuario: Usuario) => {
    setEditingUsuario(usuario);
    setUsername(usuario.username);
    setNombreCompleto(usuario.nombre_completo);
    setRol(usuario.rol);
    setPassword('');
    setIsFormOpen(true);
  };

  const handleOpenCreate = () => {
    resetForm();
    setIsFormOpen(true);
  };

  const handleCloseForm = () => {
    if (isLoading) return;
    resetForm();
    setIsFormOpen(false);
  };

  const handleToggleUsuario = async () => {
    if (!usuarioToToggle) return;

    try {
      if (usuarioToToggle.activo === 1) {
        await desactivarUsuario(usuarioToToggle.id_usuario);
        toast.info('Usuario desactivado correctamente');
      } else {
        await reactivarUsuario(usuarioToToggle.id_usuario);
        toast.success('Usuario reactivado correctamente');
      }

      setUsuarioToToggle(null);
      await recargar();
      onUsuarioModified();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  };

  const totalActivos = usuarios.filter((usuario) => usuario.activo === 1).length;
  const totalInactivos = usuarios.filter((usuario) => usuario.activo === 0).length;

  const usuariosFiltrados = usuarios
    .filter((usuario) => {
      if (estadoFiltro === 'activos') return usuario.activo === 1;
      if (estadoFiltro === 'inactivos') return usuario.activo === 0;
      return true;
    })
    .filter((usuario) => {
      const texto = busqueda.trim().toLowerCase();
      if (!texto) return true;
      return (
        usuario.username.toLowerCase().includes(texto) ||
        usuario.nombre_completo.toLowerCase().includes(texto) ||
        usuario.rol.toLowerCase().includes(texto)
      );
    });

  return (
    <div className="tab-panel">
      <div className="table-container">
        <div className="section-header">
          <h2>Usuarios registrados</h2>
          <div className="section-header-actions">
            <button type="button" className="btn-primary" onClick={handleOpenCreate}>
              <span className="button-with-icon">
                <AppIcon icon={Plus} size={18} />
                <span>Nuevo usuario</span>
              </span>
            </button>
          </div>
        </div>
        {error && (
          <div className="inline-feedback inline-feedback-warning">
            <span>No se pudo refrescar la lista. Se muestran los datos disponibles.</span>
            <button type="button" className="btn-secondary" onClick={() => void recargar()}>
              Reintentar
            </button>
          </div>
        )}
        <div className="filter-bar">
          <div className="filter-summary-group">
            <div className="filter-summary">Visibles: {usuariosFiltrados.length}</div>
            <span className="status-chip status-chip-total">Todos: {usuarios.length}</span>
            <span className="status-chip status-chip-success">Activos: {totalActivos}</span>
            <span className="status-chip status-chip-warning">Inactivos: {totalInactivos}</span>
          </div>
          <input
            className="form-input filter-search"
            placeholder="Buscar por usuario, nombre o rol"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            aria-label="Buscar usuarios por nombre, usuario o rol"
          />
          <select
            className="form-input filter-select"
            value={estadoFiltro}
            onChange={(e) => setEstadoFiltro(e.target.value as 'todos' | 'activos' | 'inactivos')}
            aria-label="Filtrar usuarios por estado"
          >
            <option value="todos">Todos</option>
            <option value="activos">Solo activos</option>
            <option value="inactivos">Solo inactivos</option>
          </select>
        </div>

        {loading ? (
          <SkeletonTable columns={5} rows={5} />
        ) : usuariosFiltrados.length === 0 ? (
          <div className="empty-state">No hay usuarios para el filtro seleccionado</div>
        ) : (
          <table className="table" aria-label="Tabla de usuarios registrados">
            <thead>
              <tr>
                <th>Usuario</th>
                <th>Nombre</th>
                <th>Rol</th>
                <th>Estado</th>
                <th>Acciones</th>
              </tr>
            </thead>
            <tbody>
              {usuariosFiltrados.map((usuario) => (
                <tr key={usuario.id_usuario}>
                  <td>{usuario.username}</td>
                  <td>{usuario.nombre_completo}</td>
                  <td>
                    <span className="badge badge-info">{usuario.rol}</span>
                  </td>
                  <td>
                    {usuario.activo === 1 ? (
                      <span className="badge badge-success">Activo</span>
                    ) : (
                      <span className="badge badge-warning">Inactivo</span>
                    )}
                  </td>
                  <td className="table-actions">
                    <button className="btn-edit" onClick={() => handleEditar(usuario)}>
                      <span className="button-with-icon">
                        <AppIcon icon={Pencil} size={16} />
                        <span>Editar</span>
                      </span>
                    </button>
                    <button className={usuario.activo === 1 ? 'btn-delete' : 'btn-primary'} onClick={() => setUsuarioToToggle(usuario)}>
                      <span className="button-with-icon">
                        <AppIcon icon={usuario.activo === 1 ? Trash2 : RotateCcw} size={16} />
                        <span>{usuario.activo === 1 ? 'Desactivar' : 'Reactivar'}</span>
                      </span>
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <FormModal
        open={isFormOpen}
        title={(
          <span className="title-with-icon form-card-title">
            <AppIcon icon={editingUsuario ? Pencil : ShieldPlus} size={20} />
            <span>{editingUsuario ? 'Editar Usuario' : 'Crear Usuario'}</span>
          </span>
        )}
        description="Defina las credenciales, el nombre visible y el rol operativo antes de guardar los cambios."
        onClose={handleCloseForm}
        onSubmit={handleSubmit}
        submitText={(
          <span className="button-with-icon">
            <AppIcon icon={Save} size={18} />
            <span>{editingUsuario ? 'Guardar cambios' : 'Crear usuario'}</span>
          </span>
        )}
        isLoading={isLoading}
      >
        <FormInput
          label="Usuario"
          value={username}
          onChange={setUsername}
          placeholder="Ej: jlopez"
          required
        />
        <FormInput
          label="Nombre completo"
          value={nombreCompleto}
          onChange={setNombreCompleto}
          placeholder="Ej: Juan López"
          required
        />
        <FormSelect
          label="Rol"
          value={rol}
          onChange={setRol}
          options={roles}
          required
        />
        <FormInput
          label={editingUsuario ? 'Nueva contraseña (opcional)' : 'Contraseña'}
          value={password}
          onChange={setPassword}
          placeholder={editingUsuario ? 'Dejar vacío para conservarla' : 'Mínimo 8 caracteres'}
          required={!editingUsuario}
        />
      </FormModal>

      <ConfirmDialog
        open={Boolean(usuarioToToggle)}
        title={usuarioToToggle?.activo === 1 ? 'Desactivar usuario' : 'Reactivar usuario'}
        message={
          usuarioToToggle?.activo === 1
            ? `¿Desea desactivar al usuario "${usuarioToToggle?.username ?? ''}"?` 
            : `¿Desea reactivar al usuario "${usuarioToToggle?.username ?? ''}"?`
        }
        confirmText={usuarioToToggle?.activo === 1 ? 'Sí, desactivar' : 'Sí, reactivar'}
        cancelText="Cancelar"
        onConfirm={handleToggleUsuario}
        onCancel={() => setUsuarioToToggle(null)}
      />
    </div>
  );
};