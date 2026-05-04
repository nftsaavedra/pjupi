import React, { useState } from 'react';
import { ChevronDown, ChevronUp, Plus, Trash2 } from 'lucide-react';
import { AppIcon } from '@/shared/ui/AppIcon';

interface RelatedEntity {
  id: string;
  [key: string]: unknown;
}

interface RelatedEntityFieldConfig {
  name: string;
  label: string;
  placeholder?: string;
  type?: 'text' | 'number' | 'textarea';
  required?: boolean;
}

interface RelatedEntitiesSectionProps {
  title: string;
  icon: React.ReactNode;
  description: string;
  items: RelatedEntity[];
  fields: RelatedEntityFieldConfig[];
  onItemsChange: (items: RelatedEntity[]) => void;
  renderItem?: (item: RelatedEntity, index: number) => React.ReactNode;
  emptyMessage?: string;
}

export const RelatedEntitiesSection: React.FC<RelatedEntitiesSectionProps> = ({
  title,
  icon,
  description,
  items,
  fields,
  onItemsChange,
  renderItem,
  emptyMessage = 'No hay elementos registrados',
}) => {
  const [expanded, setExpanded] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editingItem, setEditingItem] = useState<RelatedEntity | null>(null);

  const getField = (item: RelatedEntity | null, name: string): string => {
    if (!item) return '';
    const val = item[name];
    return typeof val === 'string' ? val : JSON.stringify(val ?? '');
  };

  const handleAddItem = () => {
    const newId = `temp-${Date.now()}`;
    const newItem: RelatedEntity = { id: newId };
    fields.forEach((field) => {
      newItem[field.name] = field.type === 'number' ? 0 : '';
    });
    setEditingId(newId);
    setEditingItem(newItem);
  };

  const handleEditItem = (item: RelatedEntity) => {
    setEditingId(item.id);
    setEditingItem({ ...item });
  };

  const handleSaveItem = () => {
    if (!editingItem) return;

    // Validation
    for (const field of fields) {
      if (field.required && !editingItem[field.name]) {
        alert(`${field.label} es requerido`);
        return;
      }
    }

    const isNew = editingId?.startsWith('temp-');
    if (isNew) {
      const finalId = `${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
      onItemsChange([...items, { ...editingItem, id: finalId }]);
    } else {
      const updated = items.map((item) => (item.id === editingId ? editingItem : item));
      onItemsChange(updated);
    }

    setEditingId(null);
    setEditingItem(null);
  };

  const handleDeleteItem = (id: string) => {
    if (confirm('¿Está seguro de que desea eliminar este elemento?')) {
      onItemsChange(items.filter((item) => item.id !== id));
    }
  };

  const handleCancelEdit = () => {
    setEditingId(null);
    setEditingItem(null);
  };

  return (
    <div className="related-entities-section">
      <button
        type="button"
        className="related-entities-toggle"
        onClick={() => { setExpanded((prev) => !prev); }}
        aria-expanded={expanded}
      >
        <span className="related-entities-toggle-copy">
          <span className="title-with-icon">
            {icon}
            <span>{title}</span>
          </span>
          {items.length > 0 && <span className="badge badge-info">{items.length}</span>}
        </span>
        <span className="related-entities-toggle-icon" aria-hidden="true">
          <AppIcon icon={expanded ? ChevronUp : ChevronDown} size={18} />
        </span>
      </button>

      {expanded && (
        <>
          <p className="related-entities-description">{description}</p>

          {items.length === 0 && !editingId && (
            <p className="related-entities-empty">{emptyMessage}</p>
          )}

          {items.length > 0 && !editingId && (
            <div className="related-entities-list">
              {items.map((item, index) =>
                renderItem ? (
                  <div key={item.id} className="related-entity-item">
                    {renderItem(item, index)}
                    <div className="related-entity-actions">
                      <button
                        type="button"
                        className="btn-small btn-secondary"
                        onClick={() => { handleEditItem(item); }}
                      >
                        Editar
                      </button>
                      <button
                        type="button"
                        className="btn-small btn-danger"
                        onClick={() => { handleDeleteItem(item.id); }}
                      >
                        <AppIcon icon={Trash2} size={14} />
                      </button>
                    </div>
                  </div>
                ) : (
                  <div key={item.id} className="related-entity-item">
                    <div className="related-entity-content">
                      {fields.map((field) => (
                        <span key={field.name}>
                          <strong>{field.label}:</strong> {getField(item, field.name)}
                        </span>
                      ))}
                    </div>
                    <div className="related-entity-actions">
                      <button
                        type="button"
                        className="btn-small btn-secondary"
                        onClick={() => { handleEditItem(item); }}
                      >
                        Editar
                      </button>
                      <button
                        type="button"
                        className="btn-small btn-danger"
                        onClick={() => { handleDeleteItem(item.id); }}
                      >
                        <AppIcon icon={Trash2} size={14} />
                      </button>
                    </div>
                  </div>
                ),
              )}
            </div>
          )}

          {editingId && editingItem && (
            <div className="related-entity-form">
              <h4 className="related-entity-form-title">
                {editingId.startsWith('temp-') ? `Agregar ${title.toLowerCase()}` : `Editar ${title.toLowerCase()}`}
              </h4>
              {fields.map((field) => (
                <div key={field.name} className="form-group">
                  <label htmlFor={`${field.name}-${editingId}`}>
                    {field.label}
                    {field.required && <span className="required">*</span>}
                  </label>
                  {field.type === 'textarea' ? (
                    <textarea
                        id={`${field.name}-${editingId}`}
                        value={getField(editingItem, field.name)}
                        onChange={(e) =>
                          { setEditingItem({ ...editingItem, [field.name]: e.target.value }); }
                        }
                        placeholder={field.placeholder}
                        rows={3}
                      />
                    ) : (
                      <input
                        id={`${field.name}-${editingId}`}
                        type={field.type || 'text'}
                        value={getField(editingItem, field.name)}
                        onChange={(e) =>
                          { setEditingItem({ ...editingItem, [field.name]: e.target.value }); }
                        }
                        placeholder={field.placeholder}
                      />
                  )}
                </div>
              ))}
              <div className="related-entity-form-actions">
                <button type="button" className="btn-secondary" onClick={handleCancelEdit}>
                  Cancelar
                </button>
                <button type="button" className="btn-primary" onClick={handleSaveItem}>
                  Guardar
                </button>
              </div>
            </div>
          )}

          {!editingId && (
            <div className="related-entities-add">
              <button
                type="button"
                className="btn-secondary"
                onClick={handleAddItem}
              >
                <span className="button-with-icon">
                  <AppIcon icon={Plus} size={16} />
                  <span>Agregar {title.toLowerCase()}</span>
                </span>
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
};
