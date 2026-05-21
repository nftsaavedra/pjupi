use futures_util::TryStreamExt;

use crate::recursos::models::{
    CreateEquipamientoRequest, CreateFinanciamientoRequest, CreatePatenteRequest, CreateProductoRequest,
    Equipamiento, Financiamiento, Patente, Producto,
    UpdateEquipamientoRequest, UpdateFinanciamientoRequest, UpdatePatenteRequest, UpdateProductoRequest,
};

// ── Patentes ──────────────────────────────────────────────────────────────────

impl_resource_repository!(
    Patente, CreatePatenteRequest, UpdatePatenteRequest,
    "patentes", id_patente,
    create_patente, get_patentes_by_proyecto, get_patente_by_id,
    update_patente, delete_patente, delete_patentes_by_proyecto,
    "Patente no encontrada.",
    titulo, numero_patente, tipo, estado,
    fecha_solicitud, fecha_concesion, pais, entidad_concedente, descripcion
);

// ── Productos ─────────────────────────────────────────────────────────────────

impl_resource_repository!(
    Producto, CreateProductoRequest, UpdateProductoRequest,
    "productos", id_producto,
    create_producto, get_productos_by_proyecto, get_producto_by_id,
    update_producto, delete_producto, delete_productos_by_proyecto,
    "Producto no encontrado.",
    nombre, tipo, etapa, descripcion, fecha_registro
);

// ── Equipamientos ─────────────────────────────────────────────────────────────

impl_resource_repository!(
    Equipamiento, CreateEquipamientoRequest, UpdateEquipamientoRequest,
    "equipamientos", id_equipamiento,
    create_equipamiento, get_equipamientos_by_proyecto, get_equipamiento_by_id,
    update_equipamiento, delete_equipamiento, delete_equipamientos_by_proyecto,
    "Equipamiento no encontrado.",
    nombre, descripcion, especificaciones, valor_estimado, moneda, proveedor, fecha_adquisicion
);

// ── Financiamientos ───────────────────────────────────────────────────────────

impl_resource_repository!(
    Financiamiento, CreateFinanciamientoRequest, UpdateFinanciamientoRequest,
    "financiamientos", id_financiamiento,
    create_financiamiento, get_financiamientos_by_proyecto, get_financiamiento_by_id,
    update_financiamiento, delete_financiamiento, delete_financiamientos_by_proyecto,
    "Financiamiento no encontrado.",
    entidad_financiadora, tipo, monto, moneda,
    fecha_inicio, fecha_fin, descripcion, estado_financiero
);
