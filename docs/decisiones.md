# Registro de decisiones — App de Finanzas

Este documento separa decisiones confirmadas, límites de la primera versión y extensiones futuras. Actualizarlo cuando el usuario confirme cambios.

## Decisiones confirmadas

| ID | Tema | Decisión |
|---|---|---|
| D-001 | Plataforma | Aplicación de escritorio inicialmente. |
| D-002 | Ejecución | Funcionamiento local. |
| D-003 | Alcance | Primera versión intermedia y acotada, orientada a establecer una base funcional. |
| D-004 | Moneda inicial (supersedida por D-024) | Trabajar exclusivamente en pesos argentinos (ARS) en la primera versión. |
| D-005 | Conversión de monedas | Queda fuera del alcance actual; se mantiene abierta como posible extensión futura. La coexistencia de ARS y USD en la primera versión no habilita conversión entre ellas. |
| D-006 | Apariencia | Tema claro y oscuro. |
| D-007 | Servicios compartidos | División siempre en partes iguales. |
| D-008 | Cantidad de personas | El número de personas incluye al usuario. |
| D-009 | Mi parte | Cálculo automático: costo total dividido por cantidad de personas. |
| D-010 | Renovación | No se incluye una columna de renovación anual; la primera versión administra servicios mensuales. |
| D-011 | Totales de servicios | Mostrar el costo completo de los servicios activos y el total correspondiente al usuario. |
| D-012 | Congelar servicios | El servicio congelado se oculta de la tabla activa y deja de participar en los totales, pero sus datos se conservan. |
| D-013 | Duración del congelamiento | Indefinido, hasta que el usuario decida reactivarlo. |
| D-014 | Celdas calculadas | No se sobrescriben resultados; se modifican los datos de origen y se recalcula. |
| D-015 | Actualización de precios | El usuario actualizará los precios manualmente o, cuando esté disponible la integración prevista, mediante OpenCode. No se requiere actualización automática. |
| D-016 | Datos iniciales de inversiones | Los campos de entrada comienzan en cero. El usuario ingresa manualmente los datos, incluyendo TNA, plazo e impuestos. |
| D-017 | Fórmula de cauciones | La fórmula definitiva no se define ahora. Se permite un cálculo o dato provisional de demostración para visualizar el flujo, claramente identificado como no definitivo. |
| D-018 | Gastos de inversiones | Inicialmente se alimentan con el gasto personal de los servicios activos. |
| D-019 | OpenCode — primera etapa | La aplicación tendrá un botón para abrir una terminal externa en el directorio del proyecto; el usuario ejecutará `opencode` manualmente. |
| D-020 | OpenCode — datos financieros | La integración para consultar o modificar datos de la aplicación queda para una etapa posterior y deberá usar operaciones controladas. |
| D-021 | Stack | Tauri 2, React, TypeScript, Vite, Rust y SQLite quedan aprobados. |
| D-022 | Historial | No implementar historial financiero en la primera versión. Mantener abierta la posibilidad de incorporarlo luego sin alterar innecesariamente la arquitectura. |
| D-023 | Representación de la TNA | La TNA se almacena como entero en basis points: 1 bp equivale a 0,01 puntos porcentuales (por ejemplo, 25,50 % = 2550 bp). Es una decisión de representación técnica, no una regla financiera; la conversión desde texto decimal se realiza en el backend sin punto flotante. |
| D-024 | Multidivisa en la primera versión | Se admiten ARS y USD desde v1, sin conversión. Los importes y las entidades correspondientes incorporan la moneda; las operaciones y comparaciones exigen la misma moneda; los totales se informan separadamente por moneda; la arquitectura debe permitir incorporar nuevas monedas. Esta decisión reemplaza la restricción de ARS exclusivo de D-004 y no modifica D-005. |
| D-025 | Renovación anual | La primera versión mantiene únicamente la renovación mensual y no implementa prorrateo anual. La renovación anual queda registrada como funcionalidad futura sin eliminar el requisito original del roadmap; D-010 permanece vigente. |
| D-026 | Mi parte y precisión | Se confirman D-007 y D-009: la parte del usuario se calcula automáticamente en partes iguales como costo total dividido por la cantidad de personas, incluido el usuario. El dominio conserva la fracción exacta sin redondeo anticipado; su presentación se rige por D-028. |
| D-027 | Selección de moneda | Los formularios de servicios e inversiones incluyen un selector explícito ARS/USD, con ARS como valor predeterminado y USD disponible desde v1. La moneda se persiste en cada registro. No existe conversión automática y los cálculos de cobertura requieren monedas compatibles. |
| D-028 | Presentación de fracciones exactas | Los importes monetarios se representan internamente mediante centavos enteros y las partes individuales se conservan como fracciones exactas. Cuando una parte no representa una cantidad entera de centavos, su presentación debe conservar explícitamente la operación de división; por ejemplo, ARS 100,00 compartido entre tres personas se presenta como `ARS 100,00 ÷ 3`, no como ARS 33,33. Cuando el denominador sea 1, puede mostrarse el importe monetario convencional. No se permite redondear ni truncar resultados financieros para presentarlos. Los totales y operaciones financieras utilizan valores exactos, nunca valores visuales redondeados. La presentación visual no modifica ni persiste resultados derivados. |
| D-029 | Formulario de servicios | La alta y la edición usan un único modal reutilizable. El botón **Nuevo servicio** abre el modal de alta y cada fila activa ofrece una acción **Editar** que abre el modal con los datos del registro correspondiente. Después de guardar correctamente, la pantalla recarga los servicios activos. |
| D-030 | Moneda durante la edición | En el modal de edición, el selector ARS/USD permanece visible pero deshabilitado y conserva la moneda original del servicio. La edición envía los datos de origen sin `currencyCode`; la moneda no se modifica desde esta pantalla. |
| D-031 | Texto y estados de cobertura | La cobertura de la primera versión tiene cuatro resultados: `Cubierto`, `Déficit de: [importe exacto]`, `Gastos en cero` y `Monedas incompatibles`. El déficit muestra el código de moneda y conserva la fracción exacta según D-028. `Gastos en cero` se usa cuando no hay gastos activos en ninguna moneda. `Monedas incompatibles` se usa cuando hay gastos no nulos en otra moneda o en ambas monedas; no emite veredicto, no calcula déficit y no convierte ni compara parcialmente los importes. El sufijo `(disponible)` se reserva para `Cubierto` y `Déficit` cuando el estado general es disponible; los estados sin veredicto conservan la etiqueta provisional o no disponible de la ruta correspondiente. |
| D-032 | Resumen mensual | El resumen mensual es una consulta puntual y de solo lectura. Usa los servicios activos actuales para derivar los gastos del mes de las inversiones del mes solicitado; no representa una fotografía histórica de los servicios activos en ese mes. No persiste resultados derivados, no impone unicidad de inversión por mes, no agrupa inversiones en un total, no convierte monedas y conserva las fracciones exactas según D-028, que prevalece sobre la tarea pendiente 11.5. Los resultados y sus estados se reutilizan de los cálculos existentes; la interfaz no decide validez financiera. La actualización automática (6.3), la verificación de consistencia (6.4), el historial (11.7) y la navegación (7.4) quedan fuera de esta decisión. |

> Nota histórica: D-004 conserva su texto original y quedó supersedida por D-024. D-005 permanece vigente.

## Alcance y extensiones futuras

Los siguientes temas están deliberadamente fuera de la primera versión, no son bloqueos para construir la base inicial:

1. Conversión entre monedas y cotizaciones; incorporación de monedas adicionales a ARS y USD.
2. Renovación anual y prorrateo de servicios anuales.
3. Historial de precios, tasas, inversiones y resultados por período.
4. Fórmula definitiva, convención de días y tratamiento impositivo de cauciones.
5. Integración controlada para que OpenCode consulte y modifique datos financieros.
6. Incorporación de otras categorías de gastos.
7. Versión móvil.

## Comportamientos provisionales de demostración

Los siguientes comportamientos fueron aprobados para una implementación provisional que permite probar la interfaz y el flujo de datos de la primera versión. **No son la fórmula financiera definitiva ni asesoramiento financiero**; deben estar aislados, identificados como provisionales y poder reemplazarse por la ruta definitiva, que sigue pendiente de las decisiones correspondientes.

| ID | Tema | Comportamiento | Estado |
|---|---|---|---|
| PV-01 | Rendimiento bruto provisional | Interés simple: `rendimiento bruto = capital × (TNA / 100) × (plazo en días / 365)`. Base anual de 365 días, sin capitalización. Precisión interna: se conserva la fracción exacta. Persistencia: no se almacenan resultados derivados redondeados. Presentación: se rige por D-028. | Provisional / demostración |
| PV-02 | Impuestos provisionales | El importe de impuestos ingresado manualmente se resta del rendimiento bruto: `rendimiento neto = rendimiento bruto − impuestos`. | Provisional / demostración |
| PV-03 | Cobertura provisional | La cobertura compara el rendimiento neto provisional con los gastos del mes y se identifica visualmente como provisional. | Provisional / demostración |

Estos comportamientos no modifican las cuestiones pendientes sobre fórmula definitiva, convención de días y tratamiento impositivo.

La representación de resultados fraccionarios y la relación entre importes individuales mostrados y totales quedan definidas en D-028.

## Cuestiones por definir durante futuras iteraciones

- Fórmula de rendimiento de cauciones, base anual y convención de días.
- Regla de cálculo y aplicación de impuestos.
- Precisión y redondeo de resultados: definido en D-028.
- Si los gastos mensuales incluirán otras categorías además de servicios.
- Política de confirmación y registro de cambios solicitados por OpenCode cuando exista integración de datos.
- Estrategia de copias de seguridad y recuperación de SQLite.
- Alcance y arquitectura de una eventual versión móvil.

## Criterio de mantenimiento

Una decisión solo pasa a “confirmada” cuando el usuario la aprueba explícitamente. Al cambiar una decisión, actualizar este registro y los documentos afectados. Las extensiones futuras deben permanecer identificadas como tales y no incorporarse silenciosamente al alcance de la primera versión.
