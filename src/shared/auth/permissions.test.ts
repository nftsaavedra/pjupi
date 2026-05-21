import { describe, it, expect } from 'vitest';
import { hasPermission, normalizeAppRole, getRoleLabel } from './permissions';

describe('normalizeAppRole', () => {
  it('returns admin for admin string', () => {
    expect(normalizeAppRole('admin')).toBe('admin');
  });

  it('returns operador for operador string', () => {
    expect(normalizeAppRole('operador')).toBe('operador');
  });

  it('returns consulta for consulta string', () => {
    expect(normalizeAppRole('consulta')).toBe('consulta');
  });

  it('returns consulta for unknown roles', () => {
    expect(normalizeAppRole('superadmin')).toBe('consulta');
  });

  it('handles null and undefined', () => {
    expect(normalizeAppRole(null)).toBe('consulta');
    expect(normalizeAppRole(undefined)).toBe('consulta');
  });

  it('trims whitespace and lowercases', () => {
    expect(normalizeAppRole(' Admin ')).toBe('admin');
  });
});

describe('hasPermission', () => {
  it('admin has all permissions', () => {
    expect(hasPermission('admin', 'dashboard.view')).toBe(true);
    expect(hasPermission('admin', 'usuarios.manage')).toBe(true);
    expect(hasPermission('admin', 'configuracion.view')).toBe(true);
  });

  it('operador has operational permissions but not config', () => {
    expect(hasPermission('operador', 'docentes.manage')).toBe(true);
    expect(hasPermission('operador', 'reportes.export')).toBe(true);
    expect(hasPermission('operador', 'usuarios.manage')).toBe(false);
    expect(hasPermission('operador', 'configuracion.view')).toBe(false);
  });

  it('consulta has only view permissions', () => {
    expect(hasPermission('consulta', 'dashboard.view')).toBe(true);
    expect(hasPermission('consulta', 'docentes.view')).toBe(true);
    expect(hasPermission('consulta', 'docentes.manage')).toBe(false);
    expect(hasPermission('consulta', 'reportes.export')).toBe(false);
  });
});

describe('getRoleLabel', () => {
  it('returns readable labels', () => {
    expect(getRoleLabel('admin')).toBe('Administrador');
    expect(getRoleLabel('operador')).toBe('Operador');
    expect(getRoleLabel('consulta')).toBe('Consulta');
  });
});
