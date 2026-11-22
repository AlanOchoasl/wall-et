# implement-finanzas-v1

Especificación e implementación de la primera versión de la app de finanzas personales: servicios recurrentes mensuales en ARS e inversiones en cauciones, según la base v0.2 de `docs/` y `design.md`.

## Artefactos

- `proposal.md`: motivación, alcance, capacidades y observaciones sobre los documentos de origen.
- `specs/`: deltas de especificación por capacidad.
- `design.md`: diseño técnico con el stack aprobado (D-021).
- `tasks.md`: plan de 76 tareas en 11 fases, con dependencias, criterios de aceptación y requisitos.

## Estado

Planificación completa y validada con OpenSpec 1.12.0 (esquema `spec-driven`). Fases 1 y 2 implementadas y verificadas: 13/76 tareas completas (infraestructura, SQLite con migraciones, DTOs, comando de estado y cliente tipado).

## Notas de flujo

- Las especificaciones consolidadas en `openspec/specs/` se generan recién al archivar el cambio (`openspec archive`), como indica el flujo oficial. Hasta entonces, la fuente son los deltas de este cambio; duplicarlos ahora provocaría requisitos repetidos al archivar.
- Las tareas de la sección 11 marcadas como BLOQUEADAS no deben ejecutarse.
- El cálculo provisional de demostración está aprobado (PV-01 a PV-03 en `docs/decisiones.md`, D13 en `design.md`): interés simple, base 365, impuestos como importe absoluto, resultados etiquetados como provisionales. Se implementa en 5.8.
- Las observaciones sobre `docs/` y `design.md` están registradas en `proposal.md` (sección "Contradicciones, vacíos y supuestos detectados").
