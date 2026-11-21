# Instrucciones para agentes — App de Finanzas

## Contexto del proyecto
Aplicación personal de finanzas, inicialmente de escritorio y funcionamiento local. El objetivo es administrar servicios recurrentes y analizar inversiones en cauciones para evaluar si sus rendimientos cubren los gastos.

## Antes de trabajar
1. Leer `docs/requisitos.md`, `docs/arquitectura.md` y `docs/decisiones.md`.
2. Distinguir entre:
   - decisiones confirmadas por el usuario;
   - propuestas técnicas todavía no aprobadas;
   - cuestiones pendientes.
3. Si una tarea depende de una decisión pendiente, preguntar antes de asumirla.
4. No iniciar funcionalidades fuera del alcance acordado sin autorización.

## Reglas de trabajo
- No inventar requisitos ni cambiar silenciosamente reglas de negocio.
- Priorizar soluciones simples, legibles y mantenibles.
- Mantener separadas presentación, lógica de aplicación, dominio y persistencia.
- Centralizar los cálculos financieros y las reglas de negocio. No duplicarlos en la interfaz ni en integraciones.
- Las interfaces gráficas y las herramientas externas deben utilizar las mismas operaciones y validaciones.
- No permitir que OpenCode u otra IA modifique directamente la base de datos. Las operaciones externas deben pasar por una interfaz controlada de la aplicación.
- Validar entradas, cantidades, monedas y operaciones antes de persistir cambios.
- No editar directamente resultados calculados: modificar sus datos de origen y recalcular.
- Preservar los datos de servicios congelados; congelar no equivale a eliminar.
- No introducir servicios anuales en el alcance inicial.
- No sumar importes de monedas distintas sin una regla de conversión aprobada.
- No incorporar dependencias, infraestructura remota o complejidad innecesaria.

## Integración con IA
- Aplicar el principio de mínimo privilegio.
- Separar operaciones de lectura y escritura.
- Validar y registrar cambios.
- Solicitar confirmación para modificaciones antes de aplicarlas, salvo que el usuario apruebe explícitamente otro comportamiento.
- No exponer credenciales, secretos ni acceso irrestricto al sistema de archivos.

## Cambios en documentación
Cuando una decisión sea confirmada:
1. Actualizar `docs/decisiones.md`.
2. Actualizar `docs/requisitos.md` o `docs/arquitectura.md` si corresponde.
3. Mantener consistentes los documentos.
4. No presentar propuestas del agente como decisiones del usuario.

## Estado actual
El proyecto está en etapa de definición. No asumir que ya existe código ni comenzar la implementación salvo pedido explícito del usuario.
