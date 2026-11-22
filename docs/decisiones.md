# Registro de decisiones — App de Finanzas

Este documento separa decisiones confirmadas, límites de la primera versión y extensiones futuras. Actualizarlo cuando el usuario confirme cambios.

## Decisiones confirmadas

| ID | Tema | Decisión |
|---|---|---|
| D-001 | Plataforma | Aplicación de escritorio inicialmente. |
| D-002 | Ejecución | Funcionamiento local. |
| D-003 | Alcance | Primera versión intermedia y acotada, orientada a establecer una base funcional. |
| D-004 | Moneda inicial | Trabajar exclusivamente en pesos argentinos (ARS) en la primera versión. |
| D-005 | Conversión de monedas | Queda fuera del alcance actual; se mantiene abierta como posible extensión futura. |
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

## Alcance y extensiones futuras

Los siguientes temas están deliberadamente fuera de la primera versión, no son bloqueos para construir la base inicial:

1. Conversión y visualización de otras monedas.
2. Historial de precios, tasas, inversiones y resultados por período.
3. Fórmula definitiva, convención de días y tratamiento impositivo de cauciones.
4. Integración controlada para que OpenCode consulte y modifique datos financieros.
5. Incorporación de otras categorías de gastos.
6. Versión móvil.

## Cuestiones por definir durante futuras iteraciones

- Fórmula de rendimiento de cauciones, base anual y convención de días.
- Regla de cálculo y aplicación de impuestos.
- Precisión y redondeo de resultados.
- Si los gastos mensuales incluirán otras categorías además de servicios.
- Política de confirmación y registro de cambios solicitados por OpenCode cuando exista integración de datos.
- Estrategia de copias de seguridad y recuperación de SQLite.
- Alcance y arquitectura de una eventual versión móvil.

## Criterio de mantenimiento

Una decisión solo pasa a “confirmada” cuando el usuario la aprueba explícitamente. Al cambiar una decisión, actualizar este registro y los documentos afectados. Las extensiones futuras deben permanecer identificadas como tales y no incorporarse silenciosamente al alcance de la primera versión.
