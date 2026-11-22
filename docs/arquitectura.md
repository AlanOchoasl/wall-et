# Arquitectura inicial — App de Finanzas

**Versión:** 0.2  
**Estado:** Stack y dirección arquitectónica aprobados para la primera versión.

## 1. Objetivos arquitectónicos

- Aplicación de escritorio local.
- Base simple para un proyecto personal de alcance intermedio.
- Separación clara entre interfaz, casos de uso, reglas de dominio y persistencia.
- Cálculos centralizados y reutilizables.
- Posibilidad de ampliar la aplicación gradualmente.
- Mantener abierta la evolución futura hacia historial, conversión de monedas e integración avanzada con OpenCode, sin implementar esas funciones ahora.

## 2. Stack tecnológico aprobado

| Capa | Tecnología | Función |
|---|---|---|
| Contenedor de escritorio | Tauri 2 | Empaquetar y ejecutar la aplicación |
| Interfaz | React | Componentes y pantallas |
| Lenguaje de interfaz | TypeScript | Tipado estático |
| Desarrollo frontend | Vite | Servidor de desarrollo y build |
| Backend local | Rust | Casos de uso, validaciones y dominio |
| Base de datos | SQLite | Persistencia local |

Este stack está confirmado por el usuario y debe utilizarse como base de implementación.

## 3. Arquitectura lógica

### Presentación

Responsable de:
- Navegación.
- Tabla y formularios de servicios.
- Tabla y formularios de inversiones.
- Totales e indicadores.
- Preferencias visuales y tema.
- Acción para abrir una terminal externa en el directorio del proyecto.

La presentación no debe contener fórmulas financieras ni reglas de negocio críticas.

### Aplicación

Coordina los casos de uso:
- Crear, editar, eliminar, congelar y reactivar servicios.
- Consultar totales de servicios.
- Crear y modificar inversiones.
- Consultar resultados provisionales de rendimiento y cobertura.
- Abrir una terminal externa desde la aplicación.

### Dominio

Contiene entidades, validaciones y reglas financieras:
- División igualitaria de servicios.
- Cálculo provisional de rendimiento, aislado para poder reemplazarlo.
- Comparación de cobertura.
- Validación de importes y cantidades.

La fórmula definitiva de cauciones todavía no está definida. Los cálculos provisionales deben estar claramente separados de las reglas definitivas y no deben filtrarse a la interfaz como si fueran valores financieros oficiales.

### Persistencia

Responsable de:
- Guardar y recuperar servicios e inversiones.
- Gestionar migraciones del esquema SQLite.
- Mantener integridad y consistencia de los datos.

### Integraciones y procesos externos

La primera versión solo necesita poder abrir una terminal externa en el directorio del proyecto para que el usuario ejecute OpenCode manualmente.

No se requiere todavía un adaptador que permita a OpenCode consultar o modificar los datos financieros de la aplicación.

## 4. Flujo de operaciones

**Interfaz:** React → comandos/casos de uso locales → dominio → repositorios → SQLite.

**Apertura de OpenCode:** interfaz Tauri → apertura de terminal externa en el directorio del proyecto → el usuario ejecuta `opencode`.

Abrir OpenCode desde la carpeta del proyecto le permite trabajar sobre los archivos del repositorio según los permisos y herramientas que tenga configurados. Esto no equivale a una API de la aplicación ni le otorga automáticamente acceso estructurado a la base de datos en uso.

Si en una etapa futura se habilitan operaciones sobre datos financieros, deberán exponerse mediante una interfaz local controlada que reutilice los casos de uso y validaciones de la aplicación.

## 5. Modelo de datos conceptual

### Service

- `id`
- `name`
- `total_cost_ars`
- `shared_people_count`
- `notes`
- `status` (active/frozen)
- `created_at`
- `updated_at`

`my_share` se calcula a partir de `total_cost_ars / shared_people_count`; no es un valor editable independiente.

La congelación es indefinida hasta que el usuario reactive el servicio. Los datos del servicio congelado se conservan.

### Investment

- `id`
- `month`
- `capital_ars`
- `tna`
- `term_days`
- `taxes`
- `created_at`
- `updated_at`

Los campos de entrada comienzan en cero. Los resultados de rendimiento y cobertura se calculan y no se editan directamente.

La fórmula financiera definitiva no forma parte de esta versión. El cálculo provisional debe poder reemplazarse sin modificar la estructura de presentación ni duplicar lógica.

### AppSettings

Preferencias de la aplicación, por ejemplo:
- Tema visual (claro/oscuro).
- Configuración general de interfaz.

### Historial y auditoría

No se implementa un historial financiero en la primera versión. Se debe evitar una estructura que impida incorporarlo posteriormente, por ejemplo mediante registros por período o instantáneas cuando esa necesidad se defina.

No es necesario construir ahora un sistema completo de auditoría o ChangeLog salvo que una necesidad concreta de la primera versión lo justifique.

## 6. Consideraciones de datos y dinero

- Todos los importes de la primera versión se interpretan como pesos argentinos (ARS).
- No almacenar ni convertir otras monedas en esta etapa.
- Mantener el diseño suficientemente desacoplado para incorporar códigos de moneda y conversiones en el futuro.
- Utilizar tipos numéricos adecuados para importes monetarios; evitar cálculos financieros con punto flotante binario sin una estrategia explícita de precisión.
- Definir y aplicar validaciones de datos en el backend.
- Usar migraciones versionadas para cambios en el esquema.

## 7. Apertura de terminal y OpenCode

La primera versión debe ofrecer una acción visible para abrir una terminal externa en la carpeta del proyecto.

Requisitos de comportamiento:
- Abrir la terminal del sistema operativo.
- Establecer como directorio de trabajo la carpeta raíz del repositorio/proyecto.
- No ejecutar automáticamente instrucciones de OpenCode; el usuario escribirá `opencode`.
- Manejar de forma clara los casos en que no pueda abrirse una terminal.

La implementación concreta deberá respetar las capacidades y permisos de Tauri 2 y funcionar en el sistema operativo objetivo del usuario.

La futura integración de OpenCode con los datos de la aplicación queda fuera del alcance actual.

## 8. Estructura de carpetas sugerida

```text
app-finanzas/
├── AGENTS.md
├── README.md
├── docs/
│   ├── requisitos.md
│   ├── arquitectura.md
│   └── decisiones.md
├── src/                 # Interfaz React + TypeScript
├── src-tauri/           # Aplicación Tauri y backend Rust
└── ...
```

La estructura podrá ajustarse a las convenciones de Tauri y Vite al crear el proyecto, manteniendo la separación lógica definida.

## 9. Decisiones técnicas para futuras iteraciones

- Estrategia de backups y recuperación del archivo SQLite.
- Modelo de historial/instantáneas por período.
- Soporte de monedas y mecanismo de cotización.
- Fórmula definitiva de cauciones, impuestos y redondeos.
- Adaptador local seguro para operaciones de OpenCode sobre datos financieros.
- Evaluación de una futura aplicación móvil.
