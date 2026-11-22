## Why

El proyecto cuenta con requisitos, arquitectura y decisiones en etapa de definición, pero no tiene una especificación ejecutable ni una implementación. Se necesita una propuesta OpenSpec que traduzca lo aprobado a comportamientos verificables y a un plan de tareas, para que la primera versión pueda implementarse sin reinterpretar los documentos de origen.

## What Changes

- Se especifica la primera versión funcional de la aplicación de escritorio: servicios recurrentes mensuales en ARS e inversiones en cauciones.
- Se define el comportamiento observable de cinco capacidades: servicios, inversiones, cálculos financieros, preferencias/navegación y lanzador de terminal para OpenCode.
- Se documenta el diseño técnico con el stack indicado como aprobado para esta propuesta: Tauri 2, React, TypeScript, Vite, Rust y SQLite.
- Se produce un plan de tareas por fases, con dependencias, criterios de aceptación y trazabilidad a requisitos.
- Se especifica un cálculo provisional de demostración, explícitamente etiquetado y aislado, condicionado a la aprobación de su fórmula; sin inventar reglas financieras.
- Se separan explícitamente las tareas bloqueadas por decisiones financieras pendientes (fórmula provisional y definitiva, convención de días, tratamiento impositivo, precisión y redondeo).
- No se implementa código en este cambio: solo se generan artefactos de planificación.

## Capabilities

### New Capabilities

- `services`: alta, edición, eliminación, congelación y reactivación de servicios mensuales; cálculo de la parte del usuario; totales de servicios activos.
- `investments`: registro mensual de inversiones en cauciones, con datos de origen manuales, resultados derivados y estado de cobertura.
- `financial-calculations`: reglas transversales de cálculo, centralización, no edición de resultados, recálculo y precisión monetaria.
- `application-settings`: preferencias visuales (tema claro/oscuro), navegación limitada a Servicios e Inversiones y legibilidad de tablas y formularios.
- `opencode-launcher`: apertura de una terminal externa en el directorio del proyecto, sin chat integrado ni acceso a los datos financieros.

Justificación de la separación: cada capacidad tiene un dueño de comportamiento distinto. Los totales y la parte del usuario pertenecen a `services`; los gastos del mes y la cobertura pertenecen a `investments`; las reglas que afectan a ambas (no editar resultados, recálculo, moneda y precisión) se concentran en `financial-calculations` para evitar duplicación. La navegación y el tema se agrupan en `application-settings` porque son presentación y no generan requisitos funcionales nuevos.

## Impact

- Documentación: se agregan artefactos OpenSpec; `docs/requisitos.md`, `docs/decisiones.md` y `docs/arquitectura.md` no se modifican en este cambio.
- Alcance funcional nuevo: ninguno respecto de lo aprobado; se excluyen conversión multidivisa, servicios anuales, historial, backups, reportes, gráficos, buscadores, notificaciones y acceso de OpenCode a los datos.
- Técnico (cuando se implemente): proyecto Tauri 2 + React + TypeScript + Vite, backend Rust, base SQLite local con migraciones.
- Riesgo principal: varias tareas de cálculo quedan bloqueadas hasta que se aprueben las decisiones financieras pendientes.

## Contradicciones, vacíos y supuestos detectados

Los documentos v0.2 del repositorio (`requisitos`, `decisiones`, `arquitectura`, `design.md`) son la base de esta propuesta y no contradicen el pedido. Se registran estas observaciones:

1. **Cálculo provisional sin fórmula definida.** `docs/requisitos.md` §3 y D-017 permiten un cálculo o dato provisional de demostración, pero no definen su fórmula ni sus datos. Esta propuesta no inventa una regla: especifica que el cálculo provisional solo puede mostrarse si el usuario aprueba explícitamente su fórmula, siempre etiquetado como provisional y aislado de la implementación definitiva. Esa aprobación queda como tarea bloqueada (11.1).
2. **Modelo de datos y moneda.** `docs/arquitectura.md` §5 modela `total_cost_ars` y no incluye un campo de moneda. El diseño de esta propuesta conserva un campo de moneda fijo en ARS para evitar una migración cuando se habilite la extensión futura, lo que §6 permite explícitamente ("mantener el diseño desacoplado para incorporar códigos de moneda"). No es una contradicción funcional.
3. **D-015 y OpenCode.** `docs/decisiones.md` D-015 menciona que el usuario podrá actualizar precios mediante OpenCode cuando exista la integración. Esa integración está fuera del alcance (D-020); esta propuesta no la especifica.
4. **Diseño visual.** `design.md` define identidad púrpura, glassmorphism moderado, tokens de color y tipografía Inter. Se toma exclusivamente como guía visual; no genera requisitos funcionales. La navegación se limita a Servicios e Inversiones y el botón de OpenCode se representa como apertura de terminal externa.
5. **Supuestos menores** (a confirmar, sin impacto estructural): importes no negativos con hasta dos decimales; confirmación explícita antes de eliminar un servicio; tema claro como valor inicial si no hay preferencia guardada; una inversión por mes como caso de uso esperado, sin imponer unicidad mientras no se decida.
