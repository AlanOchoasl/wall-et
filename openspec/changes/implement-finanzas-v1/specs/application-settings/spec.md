## Purpose

Define las preferencias visuales y la estructura de navegación de la aplicación, limitada a Servicios e Inversiones, con tema claro y oscuro persistente y presentación legible de tablas y formularios.

## ADDED Requirements

### Requirement: Ofrecer tema claro y oscuro

El sistema SHALL permitir elegir entre tema claro y tema oscuro. El tema elegido MUST aplicarse a toda la interfaz y MUST persistir entre ejecuciones. Si no existe una preferencia guardada, el sistema MUST iniciar con el tema claro.

#### Scenario: Cambio de tema

- **WHEN** el usuario selecciona el tema oscuro
- **THEN** toda la interfaz cambia al tema oscuro de inmediato

#### Scenario: Persistencia del tema

- **WHEN** el usuario elige un tema, cierra y vuelve a abrir la aplicación
- **THEN** la aplicación inicia con el tema elegido

#### Scenario: Sin preferencia guardada

- **WHEN** la aplicación se ejecuta por primera vez sin preferencia de tema
- **THEN** inicia con el tema claro

### Requirement: Limitar la navegación a Servicios e Inversiones

La navegación principal SHALL ofrecer únicamente las secciones Servicios e Inversiones. El sistema MUST NOT incluir buscadores, notificaciones, gráficos históricos, reportes ni módulos adicionales en la primera versión.

#### Scenario: Secciones disponibles

- **WHEN** el usuario observa la navegación principal
- **THEN** ve únicamente las secciones Servicios e Inversiones

#### Scenario: Acceso a servicios

- **WHEN** el usuario selecciona Servicios
- **THEN** el sistema muestra la tabla de servicios activos, la vista de congelados y los totales

#### Scenario: Acceso a inversiones

- **WHEN** el usuario selecciona Inversiones
- **THEN** el sistema muestra la tabla de inversiones con sus datos de origen y resultados derivados

### Requirement: Presentar tablas y formularios legibles

El sistema SHALL mostrar tablas con encabezados que identifiquen cada columna y formularios con etiqueta visible por campo. Los mensajes de validación MUST mostrarse asociados al campo que los origina. La información MUST ser legible tanto en tema claro como en tema oscuro.

#### Scenario: Tabla con encabezados

- **WHEN** el usuario abre la tabla de servicios o de inversiones
- **THEN** cada columna muestra un encabezado que identifica su contenido

#### Scenario: Error asociado al campo

- **WHEN** el usuario ingresa un valor inválido en un campo
- **THEN** el mensaje de validación aparece junto a ese campo

#### Scenario: Legibilidad en ambos temas

- **WHEN** el usuario cambia entre tema claro y oscuro
- **THEN** el texto, los encabezados y los mensajes de validación siguen siendo legibles en ambos temas

### Requirement: Persistir las preferencias localmente

El sistema SHALL guardar las preferencias de la aplicación de forma local y MUST recuperarlas al iniciar. La pérdida o ausencia de una preferencia MUST NOT impedir el funcionamiento de la aplicación ni afectar los datos financieros.

#### Scenario: Recuperación de preferencias

- **WHEN** la aplicación inicia con preferencias guardadas
- **THEN** aplica las preferencias recuperadas antes de mostrar las secciones

#### Scenario: Preferencia ausente o dañada

- **WHEN** una preferencia no existe o no puede interpretarse
- **THEN** el sistema usa el valor inicial correspondiente y continúa funcionando
