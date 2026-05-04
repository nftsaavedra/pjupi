import { useMemo, useState } from 'react';
import { useFetchUsuarios } from './useFetchUsuarios';
import { useRefreshToast } from '@/shared/hooks/useRefreshToast';
import { toast } from '@/services/toast';
import { actualizarUsuario, crearUsuario, desactivarUsuario, getTauriErrorMessage, reactivarUsuario, type Usuario } from '../../api';

export const useUsuariosTab = (currentUser: Usuario, refreshTrigger = 0, onUsuarioModified: () => void) => {
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

  const handleSubmit = async (e: React.SyntheticEvent) => {
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
        if (editingUsuario.id_usuario === currentUser.id_usuario && editingUsuario.rol !== rol) {
          toast.warning('No puede cambiar su propio rol. Solicite a otro administrador que lo haga.');
          return;
        }

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

    if (usuarioToToggle.id_usuario === currentUser.id_usuario) {
      toast.warning('No puede cambiar el estado de su propio usuario.');
      return;
    }

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

  const totalActivos = useMemo(() => usuarios.filter((usuario) => usuario.activo === 1).length, [usuarios]);
  const totalInactivos = useMemo(() => usuarios.filter((usuario) => usuario.activo === 0).length, [usuarios]);

  const usuariosFiltrados = useMemo(() => usuarios
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
    }), [busqueda, estadoFiltro, usuarios]);

  return {
    busqueda,
    editingUsuario,
    error,
    estadoFiltro,
    handleCloseForm,
    handleEditar,
    handleOpenCreate,
    handleSubmit,
    handleToggleUsuario,
    isFormOpen,
    isLoading,
    loading,
    nombreCompleto,
    password,
    recargar,
    rol,
    setBusqueda,
    setEstadoFiltro,
    setNombreCompleto,
    setPassword,
    setRol,
    setUsername,
    setUsuarioToToggle,
    totalActivos,
    totalInactivos,
    username,
    usuarioToToggle,
    usuarios,
    usuariosFiltrados,
  };
};