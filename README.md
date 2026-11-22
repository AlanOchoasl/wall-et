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
- [`design.md`](design.md): guía exclusivamente visual.
- [`openspec/`](openspec/): planificación y tareas de la primera versión.

## Desarrollo

### Requisitos del entorno

- Node.js 20 o superior y npm.
- Rust estable con el toolchain MSVC (`rustup`).
- Visual Studio Build Tools con las herramientas de C++.
- WebView2 (incluido en Windows 10/11 actualizado).
- CLI de Tauri instalada como dependencia del proyecto (`@tauri-apps/cli`).

Versiones verificadas en el entorno de desarrollo:

| Herramienta | Versión |
|---|---|
| Node.js | 24.18.0 |
| npm | 11.16.0 |
| rustc / cargo | 1.100.0 |
| Tauri CLI | 2.12.6 |
| WebView2 | 158.0.4370.33 |
| Visual Studio Build Tools 2022 | 17.14.45 |

### Instalación

```powershell
npm install
```

### Comandos

| Comando | Descripción |
|---|---|
| `npm run dev` | Servidor de desarrollo de la interfaz en `http://localhost:1420` |
| `npm run desktop:dev` | Abre la aplicación de escritorio Tauri en modo desarrollo |
| `npm run desktop:build` | Genera el build de escritorio |
| `npm run build` | Verifica tipos y genera el build de la interfaz |
| `npm run typecheck` | Verificación de tipos de TypeScript |
| `npm run lint` | ESLint |
| `npm test` | Pruebas de la interfaz con Vitest |
| `npm run rust:fmt` | Aplica rustfmt |
| `npm run rust:fmt:check` | Verifica el formato de Rust |
| `npm run rust:lint` | Clippy con advertencias como errores |
| `npm run rust:test` | Pruebas de Rust |
| `npm run check` | Ejecuta todas las verificaciones anteriores |

### Estructura

```text
src/                  Interfaz React + TypeScript
├── app/              Shell y navegación
├── features/         Módulos servicios, inversiones y preferencias
├── shared/           Cliente de comandos y utilidades de presentación
└── test/             Configuración de pruebas
src-tauri/            Aplicación Tauri y backend Rust
└── src/
    ├── domain/       Entidades, validaciones y reglas de negocio
    ├── application/  Casos de uso
    ├── infrastructure/ Persistencia y procesos externos
    └── commands/     Comandos Tauri
```

La base de datos SQLite se ubicará en el directorio de datos de la aplicación, no en el repositorio.

## Criterio de desarrollo

Construir primero una base pequeña, funcional y mantenible. No implementar extensiones futuras ni asumir reglas financieras que todavía no fueron definidas.
