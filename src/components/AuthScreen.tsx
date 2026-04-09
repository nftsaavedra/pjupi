import React, { useEffect, useId, useState } from 'react';
import { LogIn, ShieldCheck } from 'lucide-react';
import { AppIcon } from './AppIcon';
import { getTauriErrorMessage, loginUsuario, registrarPrimerUsuario, type Usuario } from '../services/tauriApi';
import { toast } from '../services/toast';

interface AuthScreenProps {
  mode: 'setup' | 'login';
  onAuthenticated: (usuario: Usuario) => void;
}

export const AuthScreen: React.FC<AuthScreenProps> = ({ mode, onAuthenticated }) => {
  const usernameId = useId();
  const fullNameId = useId();
  const passwordId = useId();
  const confirmPasswordId = useId();
  const [username, setUsername] = useState('');
  const [nombreCompleto, setNombreCompleto] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setUsername('');
    setNombreCompleto('');
    setPassword('');
    setConfirmPassword('');
  }, [mode]);

  const handleSetup = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!username.trim() || !nombreCompleto.trim() || !password.trim()) {
      toast.warning('Complete todos los campos para crear el usuario inicial');
      return;
    }

    if (password.trim().length < 8) {
      toast.warning('La contraseña debe tener al menos 8 caracteres');
      return;
    }

    if (password !== confirmPassword) {
      toast.warning('La confirmación de contraseña no coincide');
      return;
    }

    setIsLoading(true);
    try {
      const usuario = await registrarPrimerUsuario(username, nombreCompleto, password);
      toast.success('Configuración inicial completada');
      onAuthenticated(usuario);
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!username.trim() || !password.trim()) {
      toast.warning('Ingrese usuario y contraseña');
      return;
    }

    setIsLoading(true);
    try {
      const usuario = await loginUsuario(username, password);
      toast.success(`Bienvenido ${usuario.nombre_completo}`);
      onAuthenticated(usuario);
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="auth-shell">
      <div className="auth-card">
        <div className="auth-card-header">
          <h2>{mode === 'setup' ? 'Configuración inicial' : 'Acceso al sistema'}</h2>
          <p>
            {mode === 'setup'
              ? 'La base de datos está vacía. Cree el primer usuario administrador para habilitar el sistema.'
              : 'Ingrese sus credenciales para utilizar el sistema.'}
          </p>
        </div>

        <form className="form" onSubmit={mode === 'setup' ? handleSetup : handleLogin}>
          <div className="form-group">
            <label htmlFor={usernameId}>Usuario</label>
            <input
              id={usernameId}
              className="form-input"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Ej: admin"
              autoComplete="username"
              required
            />
          </div>

          {mode === 'setup' && (
            <div className="form-group">
              <label htmlFor={fullNameId}>Nombre completo</label>
              <input
                id={fullNameId}
                className="form-input"
                value={nombreCompleto}
                onChange={(e) => setNombreCompleto(e.target.value)}
                placeholder="Ej: Administrador General"
                required
              />
            </div>
          )}

          <div className="form-group">
            <label htmlFor={passwordId}>Contraseña</label>
            <input
              id={passwordId}
              type="password"
              className="form-input"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Mínimo 8 caracteres"
              autoComplete={mode === 'setup' ? 'new-password' : 'current-password'}
              required
            />
          </div>

          {mode === 'setup' && (
            <div className="form-group">
              <label htmlFor={confirmPasswordId}>Confirmar contraseña</label>
              <input
                id={confirmPasswordId}
                type="password"
                className="form-input"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="Repita la contraseña"
                autoComplete="new-password"
                required
              />
            </div>
          )}

          {mode === 'setup' && (
            <div className="auth-note">
              El primer usuario se crea automáticamente con rol <strong>Administrador</strong>. La integración RENIEC queda lista al definir <strong>PJUPI_RENIEC_TOKEN</strong> en el archivo de entorno; el endpoint base ya viene configurado por defecto.
            </div>
          )}

          <button type="submit" className="btn-primary auth-submit" disabled={isLoading}>
            {isLoading
              ? 'Procesando...'
              : mode === 'setup'
                ? (
                  <span className="button-with-icon">
                    <AppIcon icon={ShieldCheck} size={18} />
                    <span>Crear usuario administrador</span>
                  </span>
                )
                : (
                  <span className="button-with-icon">
                    <AppIcon icon={LogIn} size={18} />
                    <span>Ingresar</span>
                  </span>
                )}
          </button>
        </form>
      </div>
    </div>
  );
};