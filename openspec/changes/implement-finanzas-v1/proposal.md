## Why

El proyecto cuenta con requisitos, arquitectura y decisiones en etapa de definición, pero no tiene una especificación ejecutable ni una implementación. Se necesita una propuesta OpenSpec que traduzca lo aprobado a comportamientos verificables y a un plan de tareas, para que la primera versión pueda implementarse sin reinterpretar los documentos de origen.

## What Changes

- Se especifica la primera versión funcional de la aplicación de escritorio: servicios recurrentes mensuales en ARS e inversiones en cauciones.
- Se define el comportamiento observable de cinco capacidades: servicios, inversiones, cálculos financieros, preferencias/navegación y lanzador de terminal para OpenCode.
- Se documenta el diseño técnico con el stack indicado como aprobado para esta propuesta: Tauri 2, React, TypeScript, Vite, Rust y SQLite.
- Se produce un plan de tareas por fases, con dependencias, criterios de aceptación y trazabilidad a requisitos.
- Se separan explícitamente las tareas bloqueadas por decisiones financieras pendientes (fórmula de rendimiento, convención de días, tratamiento impositivo, precisión y redondeo).
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

## Contradicciones y supuestos detectados

Estas diferencias se documentan en lugar de resolverse por interpretación arbitraria:

1. **Versión de requisitos.** El pedido indica que existe una versión 0.2 de `docs/requisitos.md` limitada a ARS y servicios mensuales, pero el archivo del repositorio es la versión 0.1 e incluye "Soporte de múltiples monedas". Esta propuesta sigue la delimitación indicada en el pedido (ARS, servicios mensuales) y no incorpora conversión multidivisa. Queda pendiente actualizar `docs/requisitos.md` a 0.2 para que el repositorio refleje esa delimitación.
2. **Decisiones de moneda.** `docs/decisiones.md` D-004 confirma "contemplar múltiples monedas" con política pendiente. La primera versión especificada opera solo en ARS; la decisión D-004 se considera diferida a una versión futura y no se reincorpora como requisito de esta entrega.
3. **Stack.** `docs/decisiones.md` lista Tauri 2, React/TypeScript/Vite, Rust y SQLite como propuestas P-001 a P-004 "pendientes de aprobación". El pedido los declara stack aprobado. Se documentan como aprobados para esta propuesta, sin modificar el registro de decisiones.
4. **Identidad visual.** El pedido exige leer `design.md`, pero ese archivo no existe en el repositorio. No se inventa identidad visual: los requisitos de apariencia se limitan a tema claro/oscuro, legibilidad y navegación restringida. La identidad visual queda pendiente de un `design.md` aprobado.
5. **OpenCode.** `docs/requisitos.md` RF-04 y D-013 contemplan integración futura con acceso a datos. El pedido reduce la primera versión a abrir una terminal externa donde el usuario ejecuta `opencode` manualmente. Esta propuesta especifica solo esa reducción y deja el acceso a datos fuera de alcance.
6. **Congelamiento.** La cuestión pendiente 6 de `docs/decisiones.md` plantea congelamiento por mes o indefinido. El pedido lo define como indefinido hasta reactivación; se especifica así y se registra como decisión a formalizar en `docs/decisiones.md`.
7. **Impuestos.** El pedido indica que los impuestos se ingresan manualmente, mientras que la cuestión pendiente 2 sigue abierta respecto de la regla o porcentaje aplicable. La especificación captura el dato manual y deja bloqueado su tratamiento de cálculo.
8. **Supuestos menores** (a confirmar, sin impacto estructural): importes no negativos con hasta dos decimales; confirmación explícita antes de eliminar un servicio; tema claro como valor inicial si no hay preferencia guardada; una inversión por mes como caso de uso esperado, sin imponer unicidad mientras no se decida.
