# App de Finanzas

Aplicación personal de finanzas, de escritorio y funcionamiento local.

## Estado

La base de requisitos y arquitectura de la primera versión está definida. El objetivo inmediato es construir una versión funcional inicial que permita administrar servicios mensuales en pesos argentinos y visualizar una comparación básica con inversiones en cauciones.

La primera versión prioriza una base clara y extensible antes que cubrir todas las funcionalidades futuras.

## Stack aprobado

- Tauri 2
- React
- TypeScript
- Vite
- Rust
- SQLite

## Alcance inicial

- Gestión de servicios mensuales y gastos compartidos.
- Cálculo automático de la parte correspondiente al usuario.
- Servicios congelados indefinidamente hasta su reactivación.
- Totales de servicios en pesos argentinos.
- Tabla de inversiones con datos ingresados manualmente y valores iniciales en cero.
- Cálculo/visualización provisional para probar la relación entre rendimiento y gastos.
- Tema claro y oscuro.
- Botón para abrir una terminal externa en el directorio del proyecto y ejecutar OpenCode.

## Extensiones futuras, fuera de esta primera versión

- Conversión y visualización en otras monedas.
- Historial de valores y resultados financieros.
- Fórmula definitiva y parametrización completa de los cálculos de cauciones.
- Integración controlada para que OpenCode consulte y modifique datos de la aplicación.
- Posible versión móvil.

## Documentación

- [`AGENTS.md`](AGENTS.md): instrucciones para agentes de IA.
- [`docs/requisitos.md`](docs/requisitos.md): alcance y comportamiento esperado.
- [`docs/arquitectura.md`](docs/arquitectura.md): arquitectura y stack aprobados.
- [`docs/decisiones.md`](docs/decisiones.md): decisiones confirmadas, límites y cuestiones pendientes.

## Criterio de desarrollo

Construir primero una base pequeña, funcional y mantenible. No implementar extensiones futuras ni asumir reglas financieras que todavía no fueron definidas.
