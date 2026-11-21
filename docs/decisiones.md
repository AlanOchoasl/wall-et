# Registro de decisiones — App de Finanzas

Este documento separa las decisiones confirmadas por el usuario de las propuestas y cuestiones pendientes. Actualizarlo cuando se acuerden cambios.

## Decisiones confirmadas

| ID | Tema | Decisión |
|---|---|---|
| D-001 | Plataforma | Aplicación de escritorio inicialmente. |
| D-002 | Ejecución | Local. |
| D-003 | Alcance | Intermedio. |
| D-004 | Monedas | La aplicación deberá contemplar múltiples monedas. La política de conversión aún no está definida. |
| D-005 | Apariencia | Tema claro y oscuro. |
| D-006 | Servicios compartidos | División siempre en partes iguales. |
| D-007 | Cantidad de personas | El número incluye al usuario. |
| D-008 | Mi parte | Cálculo automático: costo total dividido por cantidad de personas. |
| D-009 | Renovación | Se elimina la columna de renovación anual/mensual en la primera versión; se administran servicios mensuales. |
| D-010 | Totales de servicios | Mostrar tanto el costo completo de todos los servicios activos como el total que corresponde pagar al usuario. |
| D-011 | Congelar servicios | El servicio congelado deja de mostrarse en la tabla activa y deja de participar en los totales, pero sus datos se conservan para poder recuperarlo/reactivarlo. |
| D-012 | Celdas calculadas | No se sobrescriben los resultados. Se modifican los datos de origen y la aplicación recalcula. |
| D-013 | IA/OpenCode | Se quiere habilitar interacción desde terminal y se contempla un botón futuro en la aplicación. Las operaciones deben respetar las reglas de la aplicación. |

## Propuestas técnicas aún no confirmadas

| ID | Propuesta | Estado |
|---|---|---|
| P-001 | Tauri 2 para escritorio | Recomendada, pendiente de aprobación. |
| P-002 | React + TypeScript + Vite para interfaz | Recomendada, pendiente de aprobación. |
| P-003 | Rust para backend local | Recomendada, pendiente de aprobación. |
| P-004 | SQLite para persistencia | Recomendada, pendiente de aprobación. |
| P-005 | Arquitectura por capas: presentación, aplicación, dominio, persistencia e integración | Recomendada como dirección arquitectónica. |
| P-006 | Adaptador local de operaciones controladas para OpenCode | Recomendado; mecanismo concreto pendiente. |

## Cuestiones pendientes

1. Cómo presentar y comparar monedas distintas; si habrá moneda base, cotización manual o automática y cuándo se convierte.
2. Porcentaje o regla de impuestos a descontar del rendimiento de cauciones.
3. Fórmula exacta del rendimiento y convención de días (por ejemplo, base anual utilizada).
4. Si los gastos mensuales de inversiones incluirán solo servicios o también otros gastos.
5. Qué información histórica se conservará cuando cambien costos, tasas o datos de inversiones.
6. Si el congelamiento de un servicio será indefinido hasta reactivación o tendrá períodos/meses específicos.
7. Qué operaciones de OpenCode requerirán confirmación y cuáles serán de solo lectura.
8. Política de redondeo y precisión para división de importes y rendimientos.
9. Estrategia de copias de seguridad y recuperación de la base de datos.

## Criterio de mantenimiento

Una decisión solo pasa a “confirmada” cuando el usuario la aprueba explícitamente. Al cambiar una decisión, actualizar este registro y los documentos afectados; no borrar el historial de decisiones relevantes.
