# Instrucciones para agentes — App de Finanzas

## Contexto del proyecto

Aplicación personal de finanzas, inicialmente de escritorio y de funcionamiento local. La primera versión busca establecer una base funcional para administrar servicios mensuales y visualizar cómo una inversión en caución se compara con los gastos.

El alcance inicial admite pesos argentinos (ARS) y dólares estadounidenses (USD) sin conversión entre ellos. La conversión de monedas y el historial financiero avanzado quedan reservados para futuras versiones.

## Metodología de trabajo

Antes de ejecutar cualquier tarea de desarrollo, leer y seguir obligatoriamente `WORKFLOW_TAREAS.md`.

## Antes de trabajar

1. Leer `docs/requisitos.md`, `docs/arquitectura.md` y `docs/decisiones.md`.
2. Leer `design.md` antes de crear o modificar la interfaz.
3. Usar `design.md` exclusivamente como guía de identidad visual, composición, jerarquía y apariencia de componentes.
4. Distinguir entre:
   - decisiones confirmadas;
   - comportamientos provisionales o de demostración;
   - extensiones futuras;
   - cuestiones todavía pendientes.
5. Si una tarea depende de una decisión pendiente, preguntar antes de asumirla.
6. Respetar el alcance de la primera versión. No adelantar funcionalidades futuras sin autorización.

## Jerarquía de documentación

- `docs/requisitos.md` define qué debe hacer la aplicación.
- `docs/decisiones.md` registra las decisiones confirmadas y los límites acordados.
- `docs/arquitectura.md` define la estructura técnica y las responsabilidades.
- `design.md` define únicamente cómo debe verse la interfaz.

**Regla fundamental:** el diseño no modifica los requisitos funcionales. La presencia de un botón, campo, gráfico, buscador, tarjeta o cualquier otro elemento en una referencia visual no autoriza a implementar su funcionalidad. Si un elemento visual sugiere una función que no está definida en los requisitos o decisiones, no implementarla ni inferir su comportamiento. Consultar al usuario si resulta necesario.

En caso de contradicción, los requisitos y las decisiones confirmadas prevalecen sobre cualquier referencia visual.

## Stack aprobado

- Tauri 2.
- React.
- TypeScript.
- Vite.
- Rust para el backend local.
- SQLite para persistencia.

## Reglas de producto y negocio

- Las monedas de la primera versión son ARS y USD, con selector explícito y valor predeterminado ARS. No implementar conversión ni cotizaciones en esta etapa; los cálculos requieren monedas compatibles.
- Los servicios iniciales son mensuales; no agregar renovaciones anuales ni prorrateo anual, registrados como funcionalidad futura.
- Los gastos compartidos se dividen siempre en partes iguales.
- La cantidad de personas incluye al usuario.
- `Mi parte` se calcula como costo total dividido por cantidad de personas, en partes iguales y conservando la fracción exacta en dominio; su presentación se rige por D-028.
- Mostrar por separado el costo completo de servicios activos y el importe correspondiente al usuario.
- Congelar un servicio lo oculta de la tabla activa y lo excluye de los totales. La congelación es indefinida hasta que el usuario lo reactive; no eliminar sus datos.
- Los resultados calculados no se editan directamente. Se modifican los datos de origen y se recalculan.
- En inversiones, los campos de entrada comienzan en cero y el usuario ingresa manualmente los datos, incluyendo TNA, plazo e impuestos.
- La fórmula financiera definitiva no está aprobada. Cualquier cálculo provisional o dato de demostración debe identificarse claramente como tal y no presentarse como asesoramiento ni como resultado financiero exacto.
- Los gastos de servicios activos alimentan inicialmente el importe de gastos usado para comparar la inversión.

## Reglas técnicas

- Priorizar soluciones simples, legibles y mantenibles.
- Mantener separadas presentación, aplicación, dominio y persistencia.
- Centralizar los cálculos y reglas de negocio; no duplicarlos en la interfaz ni en integraciones.
- Validar entradas y operaciones antes de persistir cambios.
- Evitar cálculos monetarios con punto flotante binario sin una estrategia explícita de precisión.
- No introducir servicios remotos, sincronización en la nube ni dependencias innecesarias.
- Mantener el diseño extensible para incorporar conversión de monedas e historial en el futuro, sin implementar esas funciones ahora.

## Interfaz y diseño visual

- Antes de implementar componentes visuales, consultar `design.md`.
- Mantener los temas claro y oscuro.
- Respetar los patrones de tablas, formularios, jerarquía tipográfica, colores y espaciado definidos allí.
- Adaptar la guía visual a los requisitos funcionales existentes; nunca adaptar los requisitos para que coincidan con una referencia.
- No implementar elementos decorativos como controles funcionales si su comportamiento no está definido.
- No agregar pantallas, secciones, acciones o datos por inferencia visual.

## OpenCode

- La primera integración prevista es un botón que abre una terminal externa en el directorio del proyecto. El usuario ejecutará `opencode` manualmente en esa terminal.
- No construir por ahora un chat de IA embebido.
- Abrir OpenCode en el directorio del proyecto permite trabajar sobre sus archivos, pero no implica acceso automático y controlado a los datos de la aplicación en ejecución.
- Si más adelante OpenCode debe consultar o modificar datos financieros, esas operaciones deberán pasar por una interfaz controlada de la aplicación, con validaciones y permisos.
- No permitir escritura directa e irrestricta sobre SQLite.
- Las operaciones de escritura futuras deberán informar qué se modificará y aplicar la política de confirmación que se defina.

## Cambios en documentación

Cuando el usuario confirme una decisión:
1. Actualizar `docs/decisiones.md`.
2. Actualizar `docs/requisitos.md` o `docs/arquitectura.md` si corresponde.
3. Mantener consistentes todos los documentos.
4. No presentar propuestas del agente ni cálculos provisionales como decisiones confirmadas.
5. Si el cambio afecta la apariencia, actualizar también `design.md`, sin convertirlo en una fuente de requisitos funcionales.

## Estado del proyecto

La base de requisitos y arquitectura de la primera versión está aprobada y documentada en la versión 0.2. El objetivo es construir una primera versión funcional y acotada. No ampliar el alcance ni implementar extensiones futuras sin autorización del usuario.
