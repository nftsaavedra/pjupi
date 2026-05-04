import React, { useCallback, useEffect, useState } from 'react';
import { Plus, Trash2, Edit2, Search } from 'lucide-react';
import { AppIcon } from '@/shared/ui/AppIcon';
import { toast } from '@/services/toast';
import { FormInput } from '@/shared/forms/FormInput';
import { FormModal } from '@/shared/forms/FormModal';
import { ConfirmDialog } from '@/shared/overlays/ConfirmDialog';
import {
  createGrupo,
  deleteGrupo,
  getAllGrupos,
  getTauriErrorMessage,
  updateGrupo,
  type GrupoInvestigacion,
} from './api';

type Grupo = GrupoInvestigacion & {
  coordinador_nombre?: string;
};

interface GruposTabProps {
  canManage: boolean;
}

export const GruposTab: React.FC<GruposTabProps> = ({ canManage }) => {
  const [grupos, setGrupos] = useState<Grupo[]>([]);
  const [busqueda, setBusqueda] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingGrupo, setEditingGrupo] = useState<Grupo | null>(null);
  const [grupoToDelete, setGrupoToDelete] = useState<Grupo | null>(null);
  const [formData, setFormData] = useState({
    nombre: '',
    descripcion: '',
    linea: '',
  });
  const [lineas, setLineas] = useState<string[]>([]);

  const cargarGrupos = useCallback(async () => {
    try {
      const data = await getAllGrupos();
      setGrupos(data);
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    }
  }, []);

  useEffect(() => {
    const load = async () => { await cargarGrupos(); };
    load().catch(console.error);
  }, [cargarGrupos]);

  const handleOpenCreate = () => {
    setEditingGrupo(null);
    setFormData({ nombre: '', descripcion: '', linea: '' });
    setLineas([]);
    setIsFormOpen(true);
  };

  const handleEditGrupo = (grupo: Grupo) => {
    setEditingGrupo(grupo);
    setFormData({
      nombre: grupo.nombre,
      descripcion: grupo.descripcion || '',
      linea: '',
    });
    setLineas([...grupo.lineas_investigacion]);
    setIsFormOpen(true);
  };

  const handleAddLinea = () => {
    if (formData.linea.trim() && !lineas.includes(formData.linea.trim())) {
      setLineas([...lineas, formData.linea.trim()]);
      setFormData({ ...formData, linea: '' });
    }
  };

  const handleRemoveLinea = (linea: string) => {
    setLineas(lineas.filter((l) => l !== linea));
  };

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault();

    if (!formData.nombre.trim()) {
      toast.warning('Ingrese el nombre del grupo');
      return;
    }

    if (lineas.length === 0) {
      toast.warning('Agregue al menos una línea de investigación');
      return;
    }

    const request = {
      nombre: formData.nombre.trim(),
      descripcion: formData.descripcion.trim() || null,
      coordinador_id: null,
      lineas_investigacion: lineas,
    };

    setIsLoading(true);
    try {
      if (editingGrupo) {
        await updateGrupo(editingGrupo.id_grupo, request);
        toast.success('Grupo actualizado correctamente');
      } else {
        await createGrupo(request);
        toast.success('Grupo creado correctamente');
      }

      await cargarGrupos();
      setIsFormOpen(false);
      setFormData({ nombre: '', descripcion: '', linea: '' });
      setLineas([]);
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteGrupo = async () => {
    if (!grupoToDelete) return;

    setIsLoading(true);
    try {
      await deleteGrupo(grupoToDelete.id_grupo);
      toast.success('Grupo eliminado correctamente');
      setGrupoToDelete(null);
      await cargarGrupos();
    } catch (error) {
      toast.error(getTauriErrorMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  const gruposFiltrados = grupos.filter((grupo) =>
    grupo.nombre.toLowerCase().includes(busqueda.toLowerCase()) ||
    (grupo.descripcion ?? '').toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <div className="tab-panel module-shell grupos-module">
      <div className="table-container">
        <div className="section-header">
          <h2>Grupos de Investigación</h2>
          {canManage && (
            <div className="section-header-actions">
              <button type="button" className="btn-primary" onClick={handleOpenCreate}>
                <span className="button-with-icon">
                  <AppIcon icon={Plus} size={18} />
                  <span>Nuevo grupo</span>
                </span>
              </button>
            </div>
          )}
        </div>

        {!canManage && (
          <div className="inline-feedback inline-feedback-info">
            <span>Modo consulta: puede revisar grupos de investigación y líneas, pero no crear, editar ni eliminar.</span>
          </div>
        )}

        <div className="toolbar-section">
          <div className="search-box">
            <AppIcon icon={Search} size={18} />
            <input
              type="text"
              placeholder="Buscar por nombre o coordinador..."
              value={busqueda}
              onChange={(e) => { setBusqueda(e.target.value); }}
              className="search-input"
            />
          </div>
          <span className="badge badge-info">{gruposFiltrados.length} grupos</span>
        </div>

        <div className="grupos-grid">
          {gruposFiltrados.length === 0 ? (
            <div className="empty-state">
              <p>No hay grupos de investigación registrados</p>
              {canManage && (
                <button type="button" className="btn-secondary" onClick={handleOpenCreate}>
                  Crear primer grupo
                </button>
              )}
            </div>
          ) : (
            gruposFiltrados.map((grupo) => (
              <div key={grupo.id_grupo} className="grupo-card">
                <div className="grupo-card-header">
                  <div>
                    <h3>{grupo.nombre}</h3>
                    <p className="grupo-coordinador">
                      {grupo.coordinador_nombre ? `Coordinador: ${grupo.coordinador_nombre}` : 'Sin coordinador asignado'}
                    </p>
                  </div>
                  {canManage && (
                    <div className="grupo-card-actions">
                      <button
                        type="button"
                        className="btn-icon"
                        onClick={() => { handleEditGrupo(grupo); }}
                        title="Editar grupo"
                      >
                        <AppIcon icon={Edit2} size={16} />
                      </button>
                      <button
                        type="button"
                        className="btn-icon btn-danger"
                        onClick={() => { setGrupoToDelete(grupo); }}
                        title="Eliminar grupo"
                      >
                        <AppIcon icon={Trash2} size={16} />
                      </button>
                    </div>
                  )}
                </div>

                {grupo.descripcion && (
                  <p className="grupo-descripcion">{grupo.descripcion}</p>
                )}

                <div className="grupo-lineas">
                  <strong>Líneas de investigación:</strong>
                  <div className="lineas-tags">
                    {grupo.lineas_investigacion.length > 0 ? (
                      grupo.lineas_investigacion.map((linea) => (
                        <span key={linea} className="linea-tag">
                          {linea}
                        </span>
                      ))
                    ) : (
                      <span className="linea-tag linea-tag-empty">Sin líneas registradas</span>
                    )}
                  </div>
                </div>

                <div className="grupo-footer">
                  <span className={`badge badge-${grupo.activo !== 0 ? 'success' : 'warning'}`}>
                    {grupo.activo !== 0 ? 'Activo' : 'Inactivo'}
                  </span>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      {canManage && (
        <FormModal
          open={isFormOpen}
          title={
            <span className="title-with-icon form-card-title">
              <span>{editingGrupo ? 'Editar grupo' : 'Crear nuevo grupo'}</span>
            </span>
          }
          description="Configure los detalles del grupo de investigación"
          onClose={() => { setIsFormOpen(false); }}
          onSubmit={(e) => { void handleSubmit(e); }}
          submitText={editingGrupo ? 'Actualizar grupo' : 'Crear grupo'}
          isLoading={isLoading}
          size="lg"
        >
          <FormInput
            label="Nombre del Grupo"
            value={formData.nombre}
            onChange={(value) => { setFormData({ ...formData, nombre: value }); }}
            placeholder="Ej: Grupo de Sostenibilidad Ambiental"
            required
          />

          <div className="form-group">
            <label htmlFor="descripcion">Descripción</label>
            <textarea
              id="descripcion"
              value={formData.descripcion}
              onChange={(e) => { setFormData({ ...formData, descripcion: e.target.value }); }}
              placeholder="Breve descripción del grupo y sus objetivos"
              rows={3}
            />
          </div>

          <div className="form-group">
            <label htmlFor="linea">Líneas de Investigación</label>
            <div className="linea-input-group">
              <input
                id="linea"
                type="text"
                value={formData.linea}
                onChange={(e) => { setFormData({ ...formData, linea: e.target.value }); }}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    e.preventDefault();
                    handleAddLinea();
                  }
                }}
                placeholder="Ingrese una línea y presione Enter"
              />
              <button type="button" className="btn-secondary" onClick={handleAddLinea}>
                Agregar
              </button>
            </div>

            {lineas.length > 0 && (
              <div className="lineas-list">
                {lineas.map((linea) => (
                  <div key={linea} className="linea-item">
                    <span>{linea}</span>
                    <button
                      type="button"
                      className="btn-icon-small"
                      onClick={() => { handleRemoveLinea(linea); }}
                    >
                      ✕
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </FormModal>
      )}

      {canManage && (
        <ConfirmDialog
          open={Boolean(grupoToDelete)}
          title="Eliminar grupo"
          message={`¿Está seguro de que desea eliminar el grupo "${grupoToDelete?.nombre ?? ''}"?`}
          confirmText="Sí, eliminar"
          cancelText="Cancelar"
          onConfirm={() => { void handleDeleteGrupo(); }}
          onCancel={() => { setGrupoToDelete(null); }}
        />
      )}
    </div>
  );
};
