/* eslint-disable @typescript-eslint/no-explicit-any */
import { useState } from 'react';
import { toast } from '@/services/toast';
import { getTauriErrorMessage } from '@/services/tauri/error';

export function useRecursoCrud<TItem, TCreatePayload>(
  getItems: (proyectoId: string) => Promise<TItem[]>,
  createItem: (payload: TCreatePayload) => Promise<any>,
  deleteItem: (id: string) => Promise<any>,
  mapToCreatePayload: (item: Record<string, any>, proyectoId: string) => TCreatePayload,
  getItemId: (item: TItem) => string,
  proyectoId: string | undefined,
) {
  const [items, setItems] = useState<TItem[]>([]);

  const loadItems = async (id: string): Promise<void> => {
    try {
      const data = await getItems(id);
      setItems(data);
    } catch (error) {
      toast.error('Error al cargar: ' + getTauriErrorMessage(error));
    }
  };

  const handleChange = async (incomingItems: Array<{ id: string; [key: string]: any }>): Promise<void> => {
    if (!proyectoId) {
      setItems(incomingItems as unknown as TItem[]);
      return;
    }

    const nuevas: TCreatePayload[] = [];
    const eliminadas: string[] = [];

    for (const raw of incomingItems) {
      if (raw.id.startsWith('temp-')) {
        nuevas.push(mapToCreatePayload(raw, proyectoId));
      }
    }

    const incomingIds = new Set(incomingItems.map((i) => i.id));
    for (const existing of items) {
      const existingId = getItemId(existing);
      if (!incomingIds.has(existingId)) {
        eliminadas.push(existingId);
      }
    }

    const promises: Promise<void>[] = [];
    for (const payload of nuevas) {
      promises.push(
        createItem(payload)
          .then(() => { /* ok */ })
          .catch((error: unknown) => {
            toast.error('Error al crear: ' + getTauriErrorMessage(error));
          }),
      );
    }
    for (const id of eliminadas) {
      promises.push(
        deleteItem(id)
          .then(() => { /* ok */ })
          .catch((error: unknown) => {
            toast.error('Error al eliminar: ' + getTauriErrorMessage(error));
          }),
      );
    }
    await Promise.all(promises);

    if (proyectoId) {
      await loadItems(proyectoId);
    }
  };

  const resetItems = (): void => {
    setItems([]);
  };

  return { items, setItems, handleChange, loadItems, resetItems } as const;
}
