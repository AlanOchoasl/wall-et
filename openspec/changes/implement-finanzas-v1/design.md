## Context

Ver `proposal.md` para la motivación y las contradicciones detectadas. El repositorio contiene únicamente documentación (`AGENTS.md`, `README.md`, `docs/`) y no tiene código de aplicación. Esta propuesta define la primera versión desde cero.

Restricciones que condicionan el diseño:

- Stack aprobado (D-021): Tauri 2, React, TypeScript, Vite, Rust y SQLite.
- Primera versión en ARS y USD, sin conversión, con servicios mensuales únicamente (D-024, D-025, D-027).
- Varias reglas financieras están pendientes de aprobación: fórmula definitiva de rendimiento, convención de días, tratamiento impositivo definitivo y redondeo (D-028).
- El cálculo provisional de demostración (D-017, PV-01 a PV-03) está aprobado, siempre identificado como no definitivo, aislado y reemplazable; la ruta definitiva permanece no disponible.
- La identidad visual está definida en `design.md`: acento púrpura, glassmorphism moderado, tipografía Inter, tokens de color para tema claro y oscuro.
- La integración con OpenCode se limita a abrir una terminal externa (D-019, D-020).

## Goals / Non-Goals

**Goals:**

- Dejar preparada una base técnica donde las reglas de negocio tengan una única implementación en Rust.
- Garantizar que la interfaz no contenga cálculos financieros.
- Permitir que los módulos de servicios e inversiones compartan totales y gastos del mes sin duplicar lógica.
- Aislar las decisiones financieras pendientes detrás de un punto único, para habilitarlas sin reescribir la aplicación.
- Mantener la persistencia local con migraciones versionadas y una separación clara de capas.

**Non-Goals:**

- Conversión entre monedas, servicios anuales, historial de cambios, backups automáticos, reportes, gráficos, buscadores y notificaciones.
- Acceso de OpenCode a los datos o escritura directa sobre SQLite.
- Chat de IA integrado.
- Definir la identidad visual o los tokens de diseño definitivos.

## Decisions

### D1. Tauri 2 como contenedor y Rust como backend autoritativo

Se usa Tauri 2 para empaquetar la aplicación y exponer comandos locales, según el stack aprobado en D-021. Rust concentra casos de uso, validaciones, reglas de negocio y acceso a SQLite.

Alternativa considerada: lógica de negocio en TypeScript. Se descarta porque duplicaría reglas entre interfaz e integraciones futuras y debilitaría el principio de fuente única de cálculo.

Consecuencia: la interfaz consume comandos tipados y nunca importa fórmulas financieras.

### D2. Organización en capas dentro del backend Rust

```text
src-tauri/src/
├── domain/           # entidades, value objects, reglas y cálculos
├── application/      # casos de uso y DTOs de entrada/salida
├── infrastructure/   # repositorios SQLite, migraciones, procesos externos
├── commands/         # comandos Tauri, validación de borde y mapeo de errores
└── main.rs / lib.rs
```

- `domain` no depende de Tauri ni de SQLite.
- `application` orquesta casos de uso y devuelve DTOs.
- `infrastructure` implementa repositorios y la apertura de terminal.
- `commands` traduce comandos Tauri a casos de uso y errores de dominio a errores de interfaz.

### D3. Frontend en React + TypeScript organizado por capacidad

```text
src/
├── app/              # shell, navegación, proveedores de tema
├── features/
│   ├── services/
│   ├── investments/
│   └── settings/
├── shared/           # cliente de comandos, formateo de importes, componentes base
└── main.tsx
```

La carpeta `shared` incluye el cliente tipado de comandos y utilidades de formato (por ejemplo, separadores de miles y decimales), pero no fórmulas de negocio. El formateo de importes es presentación; el valor sigue viniendo del backend.

### D4. Comunicación frontend/backend mediante comandos Tauri tipados

- Cada operación expuesta es un comando Tauri con nombre explícito y DTOs serializables (`serde`).
- Los DTOs de resultado incluyen valores derivados y su estado (`disponible`, `no_disponible`, `provisional`) para que la interfaz no decida si un cálculo es válido.
- Los errores se modelan como un tipo serializable con código y mensaje, y opcionalmente el campo afectado, para que el formulario asocie el mensaje al campo correcto.
- No se exponen comandos genéricos de SQL ni de sistema de archivos.

Operaciones previstas (nombres ilustrativos, contrato definitivo en implementación): listar/crear/editar/eliminar/congelar/reactivar servicios; listar congelados; obtener totales de servicios; listar/crear/editar inversiones; obtener resumen mensual; leer y guardar preferencias; abrir terminal.

### D5. Persistencia con SQLite y migraciones versionadas

- Archivo SQLite en el directorio de datos de la aplicación (resuelto por Tauri), no en el repositorio.
- Migraciones numeradas y aplicadas al iniciar, registradas en una tabla de control de esquema.
- Acceso exclusivamente desde los repositorios Rust; la interfaz no abre la base.
- Claves primarias UUID generadas por la aplicación.
- Fechas en UTC con formato ISO 8601.

Modelo de datos propuesto:

```text
service
  id TEXT PK
  name TEXT NOT NULL
  total_cost_minor INTEGER NOT NULL        -- unidades mínimas de la moneda (ARS/USD: centavos)
  currency_code TEXT NOT NULL              -- "ARS" | "USD"
  shared_people_count INTEGER NOT NULL
  notes TEXT
  status TEXT NOT NULL                     -- active | frozen
  created_at TEXT NOT NULL
  updated_at TEXT NOT NULL

investment
  id TEXT PK
  month TEXT NOT NULL                      -- YYYY-MM
  capital_minor INTEGER NOT NULL
  currency_code TEXT NOT NULL              -- "ARS" | "USD"
  tna_basis_points INTEGER NOT NULL        -- 1 bp = 0,01 pp (D-023)
  term_days INTEGER NOT NULL
  tax_minor INTEGER NOT NULL               -- impuesto ingresado manualmente
  created_at TEXT NOT NULL
  updated_at TEXT NOT NULL

app_setting
  key TEXT PK
  value TEXT NOT NULL
  updated_at TEXT NOT NULL

schema_migrations
  version INTEGER PK
  applied_at TEXT NOT NULL
```

Notas:

- `my_share`, totales, gastos del mes, rendimientos y cobertura no se persisten: se derivan.
- La columna `tna_basis_points` representa la TNA como entero escalado; la escala exacta quedó definida en D-023: 1 bp = 0,01 pp (25,50 % = 2550 bp). La conversión desde texto decimal se hace en dominio, sin punto flotante.
- `tax_minor` almacena el importe ingresado; su interpretación definitiva (importe absoluto o base para una alícuota) queda bloqueada hasta la decisión impositiva. Si la decisión exige una alícuota, se agregará una migración.
- No se incorpora tabla de historial ni de auditoría en la primera versión.
- La unicidad de una inversión por mes no se impone como restricción de esquema mientras no se decida; el caso de uso esperado es una inversión por mes.
- `currency_code` admite `"ARS"` o `"USD"` y se persiste en cada servicio e inversión (D-024, D-027). El esquema ya contiene la columna, por lo que no se requiere una nueva migración. No se realizan conversiones ni se suman o comparan importes de monedas distintas.

### D6. Importes monetarios

- Importes en enteros de unidades mínimas; ARS y USD usan centavos y la abstracción permite incorporar monedas con otro exponente.
- Cada importe lleva su moneda; las operaciones exigen monedas compatibles y los totales se agrupan por moneda.
- Precisión interna: la parte del usuario y otros cocientes se calculan con un tipo racional de precisión explícita, nunca con `f32`/`f64`, y se conservan como fracción exacta en dominio durante la fase 3.
- Persistencia: no se almacenan importes derivados redondeados.
- Presentación: la interfaz puede mostrar dos decimales solo como formato; esto no fija una política de redondeo (D-028, pendiente). Falta decidir cómo representar resultados fraccionarios y cómo resolver diferencias entre importes individuales mostrados y totales. Mientras tanto, los resultados se etiquetan según su estado y no se presentan como definitivos (ver `financial-calculations`).

### D7. Punto único para las decisiones financieras pendientes

El dominio define un módulo de cálculos con funciones explícitas para rendimiento bruto, rendimiento neto y cobertura. Cada una devuelve un resultado con estado (`disponible`, `no_disponible`, `provisional`) y motivo.

La ruta definitiva devuelve `no_disponible` mientras falte la decisión definitiva. La ruta provisional implementa PV-01 a PV-03 en `domain/calculations/provisional`, separada de `domain/calculations/definitive`, y etiqueta todo resultado como `provisional`. No se introducen reglas financieras nuevas ni se presenta un valor provisional como definitivo.

Esto permite habilitar el cálculo definitivo cambiando un único módulo y sus pruebas, sin tocar la interfaz ni la persistencia.

### D8. Validaciones en dos niveles

- Borde (comandos Tauri): tipos, campos obligatorios, formato y rangos básicos.
- Dominio: reglas de negocio (cantidad de personas entera y mayor o igual a 1, importes no negativos, moneda admitida ARS/USD, compatibilidad entre monedas, estado válido).

Ambos niveles usan las mismas definiciones del dominio para evitar divergencias. Los mensajes de error se devuelven con el campo afectado.

### D9. Apertura de procesos externos

- La apertura de terminal se implementa en `infrastructure` con un comando del sistema operativo y argumentos fijos.
- La ruta de trabajo se valida como directorio existente y se pasa como directorio de trabajo del proceso, no como texto concatenado en una línea de comandos.
- No se ejecuta `opencode` ni se escribe en la terminal.
- En Windows se usa una terminal disponible del sistema; la elección concreta se aísla en un único módulo para poder ajustarla.

### D10. Estrategia de pruebas

- **Dominio (Rust):** pruebas unitarias de división, totales, exclusión de congelados, derivación de gastos del mes, cobertura y estados de resultados.
- **Persistencia (Rust):** pruebas de integración con SQLite temporal, incluyendo migraciones y repositorios.
- **Contrato de comandos (Rust):** pruebas de los DTOs y del mapeo de errores.
- **Interfaz (TypeScript):** pruebas de formateo, validaciones de formulario y estados de resultados; no se prueban fórmulas porque no existen en la interfaz.
- **Verificación de frontera:** prueba o revisión que confirme que `src/` no contiene cálculos financieros.
- **Humo manual:** recorrido de alta de servicio, congelación, alta de inversión, cambio de tema y apertura de terminal.

### D11. Tema y preferencias

- El tema se implementa con variables CSS y un atributo en el elemento raíz (`data-theme`), según los tokens de `design.md`.
- La preferencia se guarda en `app_setting` y se lee al iniciar antes de renderizar las secciones.
- Valor inicial: tema claro cuando no hay preferencia.

### D12. Identidad visual y composición

- Se adoptan los tokens de `design.md`: acento púrpura, superficies translúcidas con fallback sólido, tipografía Inter y cifras tabulares.
- La composición usa barra superior con marca, selector de tema y acción "Abrir OpenCode"; navegación lateral limitada a Servicios e Inversiones.
- Los colores semánticos se reservan para estados definidos (cobertura, error, congelado) y nunca comunican estado solo por color.
- Los resultados provisionales se identifican con etiqueta textual, además del tratamiento visual.
- `design.md` es guía de apariencia; no agrega controles ni funcionalidades.

### D13. Cálculo provisional de demostración (PV-01 a PV-03)

Aprobado por el usuario para una implementación provisional que desbloquea la visualización de la primera versión. Se registra en `docs/decisiones.md` como comportamiento provisional, no como regla financiera confirmada; la ruta definitiva sigue pendiente de las decisiones correspondientes.

- `rendimiento_bruto = capital × (TNA / 100) × (plazo_días / 365)` — interés simple, base 365, sin capitalización.
- `rendimiento_neto = rendimiento_bruto − impuestos` — impuestos como importe absoluto ingresado manualmente.
- `cobertura` compara `rendimiento_neto` con `gastos_del_mes`.
- Todo resultado se etiqueta "provisional" en la interfaz y en los DTOs (`estado = provisional`).
- Se implementa en un módulo `domain/calculations/provisional` separado de `domain/calculations/definitive`, con una interfaz común, para reemplazarlo sin tocar presentación ni persistencia.
- Precisión interna: aritmética decimal/racional con fracción exacta. Persistencia: no se guardan resultados derivados redondeados. Presentación: puede mostrar dos decimales solo como formato, sin fijar una política de redondeo (D-028, pendiente).
- Al aprobarse la fórmula definitiva, el módulo provisional se retira y las pruebas de referencia se mueven al definitivo.

### D14. Monedas, precisión y alcance aprobado (D-023 a D-027)

- ARS y USD están disponibles desde v1 con selector explícito y valor predeterminado ARS; no hay conversión ni cotizaciones.
- Cada servicio e inversión persiste su `currency_code`; los totales se calculan por moneda y las comparaciones exigen monedas compatibles.
- La TNA se representa en basis points (1 bp = 0,01 pp); la conversión desde texto decimal se realiza en dominio sin punto flotante.
- La primera versión administra solo renovación mensual; la renovación anual y el prorrateo quedan como funcionalidad futura (D-025).
- `Mi parte` conserva la fracción exacta en dominio y no se persiste redondeada; la presentación puede mostrar dos decimales solo como formato. D-028 sigue pendiente: falta decidir cómo representar resultados fraccionarios y cómo resolver diferencias entre importes individuales mostrados y totales.

## Risks / Trade-offs

- **Decisiones financieras pendientes** → el rendimiento y la cobertura definitivos quedan sin veredicto real. Mitigación: módulo único con estado `no_disponible`, cálculo provisional aislado y etiquetado, tareas bloqueadas separadas y pruebas que impidan presentar resultados inventados.
- **Fórmula provisional no aprobada** → ya resuelto: PV-01 a PV-03 están aprobados como demostración. Riesgo restante: que un resultado provisional se confunda con uno definitivo. Mitigación: etiqueta textual obligatoria y estado `provisional` en los DTOs.
- **Precisión de TNA** → resuelta por D-023: escala fija de 1 bp = 0,01 pp en `tna_basis_points`, con conversión desde texto decimal en dominio. Riesgo restante: una futura necesidad de mayor resolución exigiría una migración y un cambio de tipo aislado.
- **Impuestos con semántica abierta** → el campo manual puede necesitar cambiar de importe a alícuota. Mitigación: aislar la conversión en el dominio y prever una migración.
- **Glassmorphism y legibilidad** → el blur puede reducir contraste o rendimiento. Mitigación: fallback sólido, contraste mínimo 4.5:1 y verificación en ambos temas.
- **Congelamiento indefinido** → no cubre congelamientos por período si esa decisión cambia. Mitigación: modelo de estado simple y migración futura si se aprueban períodos.
- **Apertura de terminal multiplataforma** → diferencias entre sistemas operativos. Mitigación: encapsular en un módulo y validar manualmente en Windows, que es el entorno actual.
- **Sin backups en la primera versión** → pérdida de datos ante fallo. Mitigación: documentar la ubicación de la base y dejar la estrategia como tarea bloqueada.

## Open Questions

- Fórmula definitiva de rendimiento, convención de días y tratamiento impositivo. No bloquea el cálculo provisional; sí bloquea las tareas 11.2 y siguientes.
- Directorio exacto que debe usarse como "directorio del proyecto" en una instalación empaquetada. Se asume el directorio desde el que se ejecuta la aplicación, configurable en un único punto.
- Terminal concreta a abrir en Windows (por ejemplo, `cmd` o Windows Terminal). Se aísla en un módulo; no altera especificaciones ni tareas.
