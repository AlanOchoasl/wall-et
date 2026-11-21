# Arquitectura inicial — App de Finanzas

**Versión:** 0.1  
**Estado:** Propuesta técnica, pendiente de aprobación.

## 1. Objetivos arquitectónicos

- Aplicación de escritorio local.
- Arquitectura simple para un proyecto personal de alcance intermedio.
- Separación clara entre interfaz, reglas financieras y persistencia.
- Reglas de negocio compartidas por la interfaz y las integraciones.
- Posibilidad de ampliar módulos y evaluar una futura versión móvil.

## 2. Stack tecnológico propuesto

| Capa | Tecnología propuesta | Función |
|---|---|---|
| Contenedor de escritorio | Tauri 2 | Empaquetar y ejecutar la aplicación |
| Interfaz | React | Componentes y pantallas |
| Lenguaje de interfaz | TypeScript | Tipado estático |
| Desarrollo frontend | Vite | Servidor y build de frontend |
| Backend local | Rust | Casos de uso, validaciones y dominio |
| Base de datos | SQLite | Persistencia local |

**Importante:** este stack es una recomendación técnica, no una decisión confirmada todavía. Debe revisarse antes de iniciar el desarrollo.

## 3. Arquitectura lógica

### Presentación
Responsable de:
- Navegación.
- Tablas de servicios e inversiones.
- Formularios de alta y edición.
- Totales e indicadores.
- Preferencias visuales y tema.

La presentación no debe contener fórmulas financieras ni reglas de negocio críticas.

### Aplicación
Coordina los casos de uso:
- Crear, editar, eliminar, congelar y reactivar servicios.
- Consultar totales por moneda.
- Crear y modificar inversiones.
- Consultar resultados de rendimiento y cobertura.
- Ejecutar operaciones solicitadas por integraciones.

### Dominio
Contiene entidades, validaciones y reglas financieras:
- División igualitaria de servicios.
- Cálculo de rendimiento bruto.
- Cálculo de rendimiento neto.
- Comparación de cobertura.
- Reglas de moneda y redondeo una vez definidas.

Los cálculos deben tener una única implementación autoritativa.

### Persistencia
Responsable de:
- Guardar y recuperar entidades.
- Gestionar migraciones del esquema.
- Mantener integridad y consistencia de los datos.

### Integraciones
Expone operaciones controladas para herramientas como OpenCode. No debe permitir acceso directo e irrestricto a SQLite.

## 4. Flujo de operaciones

**Interfaz:** React → comandos/casos de uso locales → dominio → repositorios → SQLite.

**OpenCode:** CLI/adaptador local → operaciones permitidas de la aplicación → dominio → repositorios → SQLite.

Ambos caminos deben utilizar las mismas validaciones y reglas de negocio.

## 5. Modelo de datos conceptual

### Service
- `id`
- `name`
- `total_cost`
- `currency_code`
- `shared_people_count`
- `notes`
- `status` (active/frozen)
- `created_at`
- `updated_at`

`my_share` se calcula a partir de `total_cost / shared_people_count`; no debe persistirse como un valor editable independiente salvo que una necesidad histórica justifique almacenar una instantánea.

### Investment
- `id`
- `month`
- `capital`
- `currency_code`
- `tna`
- `term_days`
- `created_at`
- `updated_at`

Los rendimientos, impuestos y cobertura se calculan a partir de los datos de origen y las reglas vigentes. La estrategia de instantáneas históricas queda pendiente.

### AppSettings
Preferencias de la aplicación, por ejemplo tema visual y configuración general.

### ChangeLog
Registro de operaciones relevantes:
- Identificador.
- Tipo e identificador de entidad afectada.
- Operación.
- Valores anteriores y nuevos (con cuidado de no registrar secretos).
- Origen de la operación (interfaz o integración).
- Fecha y hora.

El nivel de detalle y retención del historial deberá definirse.

## 6. Consideraciones de datos y dinero

- Utilizar tipos numéricos adecuados para importes monetarios; evitar cálculos financieros con punto flotante binario sin una estrategia explícita de precisión.
- Guardar el código de moneda junto con cada importe.
- No sumar importes de monedas diferentes sin una cotización y política de conversión definida.
- Definir redondeo y precisión antes de implementar los cálculos.
- Usar migraciones versionadas para cambios en el esquema.

## 7. Integración con OpenCode

La integración debería exponer un conjunto acotado de operaciones de lectura y escritura, por ejemplo:
- `list_services`
- `create_service`
- `update_service`
- `freeze_service`
- `reactivate_service`
- `list_investments`
- `create_investment`
- `update_investment`
- `get_monthly_summary`

Los nombres son ilustrativos, no una API aprobada.

Las operaciones de escritura deben:
- Validar datos.
- Respetar reglas del dominio.
- Informar claramente qué se modificará.
- Solicitar confirmación según la política que se defina.
- Registrar el origen del cambio.

El mecanismo concreto (CLI, protocolo local u otro adaptador) se elegirá al implementar la integración.

## 8. Estructura de carpetas sugerida

```text
app-finanzas/
├── AGENTS.md
├── README.md
├── docs/
│   ├── requisitos.md
│   ├── arquitectura.md
│   └── decisiones.md
├── src/                 # Interfaz, si se confirma React
├── src-tauri/           # Backend Tauri/Rust, si se confirma
└── ...
```

La estructura de código es orientativa y deberá ajustarse al stack finalmente aprobado.

## 9. Riesgos y decisiones técnicas pendientes

- Definir si el backend Rust de Tauri será suficiente para todas las operaciones iniciales.
- Elegir mecanismo seguro y práctico de comunicación con OpenCode.
- Definir estrategia de backups del archivo SQLite.
- Definir historial y tratamiento de cambios retroactivos.
- Definir soporte multidivisa y cotizaciones, si se incorporan conversiones.
- Evaluar el alcance real de una futura versión móvil antes de imponer abstracciones prematuras.
