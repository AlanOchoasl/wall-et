# design.md — App de finanzas personales

> Sistema de diseño visual. Solo estética, layout y componentes: **no define funcionalidades**.
> Referencias: dashboard "glass" oscuro (estructura, sidebar, tarjetas translúcidas, gráfico de barras) + guía "Visual Direction" (paleta, tipografía, componentes).
> Los colores de la paleta base son los hex de la guía; los valores de glass, radios y espaciados son aproximaciones tomadas de las capturas.

---

## 1. Principios

1. **Dark-first.** Fondo casi negro con un resplandor ambiental sutil. El foco está en los datos.
2. **Glass con intención.** Superficies translúcidas con blur sobre un fondo que tiene color. Sin fondo con color no hay glass: el blur necesita algo que desenfocar.
3. **Un solo acento.** El púrpura guía acciones y jerarquía. Los colores semánticos se reservan para estados (positivo, alerta, error, info).
4. **Jerarquía fuerte, contraste suave.** Títulos grandes y blancos, texto secundario en gris azulado, bordes casi invisibles.
5. **Componentes redondeados** para una sensación SaaS moderna y amable.
6. **Los números son protagonistas.** Cifras grandes, semibold, con unidad/etiqueta pequeña al lado.

---

## 2. Color

### 2.1 Paleta base

| Token | Hex | Uso |
|---|---|---|
| \`--bg-0\` | \`#0D0E17\` | Fondo de la app |
| \`--bg-1\` | \`#12131C\` | Superficies sólidas, inputs, filas |
| \`--primary\` | \`#8A62FF\` | Acción primaria, foco, selección, línea de gráficos |
| \`--primary-soft\` | \`#A78BFA\` | Hover del primario, textos de acento, IDs, links |
| \`--success\` | \`#7CFFB2\` | Positivo, ingresos, "pagado", variación al alza |
| \`--warning\` | \`#F59E0B\` | Pendiente, alertas suaves |
| \`--danger\` | \`#EF4444\` | Vencido, error, variación a la baja |
| \`--info\` | \`#60A5FA\` | Información, series secundarias de gráficos |

### 2.2 Texto

| Token | Valor | Uso |
|---|---|---|
| \`--text-1\` | \`#FFFFFF\` | Títulos, cifras |
| \`--text-2\` | \`rgba(255,255,255,0.72)\` | Texto de cuerpo |
| \`--text-3\` | \`rgba(255,255,255,0.48)\` | Etiquetas, captions, placeholders |
| \`--text-4\` | \`rgba(255,255,255,0.28)\` | Deshabilitado, ejes de gráficos |

### 2.3 Tintes para tarjetas glass
Cada tarjeta KPI puede llevar un tinte distinto (como las 4 tarjetas de colores de la referencia). Se obtienen mezclando el color semántico con transparencia sobre el glass base:

| Tinte | Fondo | Borde |
|---|---|---|
| Púrpura | \`rgba(138,98,255,0.16)\` | \`rgba(167,139,250,0.28)\` |
| Azul | \`rgba(96,165,250,0.14)\` | \`rgba(96,165,250,0.26)\` |
| Verde | \`rgba(124,255,178,0.12)\` | \`rgba(124,255,178,0.24)\` |
| Ámbar | \`rgba(245,158,11,0.13)\` | \`rgba(245,158,11,0.26)\` |
| Neutro | \`rgba(255,255,255,0.05)\` | \`rgba(255,255,255,0.08)\` |

Regla: **máximo una tarjeta tintada por fila de 4**, salvo en la fila de KPIs donde se permite una por tarjeta. El resto de la pantalla es neutro.

### 2.4 Fondo ambiental
Base \`--bg-0\` + resplandores difusos detrás del contenido:

\`\`\`css
body {
  background:
    radial-gradient(60% 50% at 12% 8%,  rgba(138,98,255,0.28) 0%, transparent 60%),
    radial-gradient(50% 45% at 88% 20%, rgba(96,165,250,0.14) 0%, transparent 60%),
    radial-gradient(55% 50% at 70% 100%, rgba(124,255,178,0.08) 0%, transparent 65%),
    #0D0E17;
  background-attachment: fixed;
}
\`\`\`
Opcional: viñeta oscura en los bordes (\`radial-gradient(transparent 55%, rgba(0,0,0,0.5))\`) para el efecto de profundidad de la referencia.

---

## 3. Glass (receta)

\`\`\`css
.glass {
  background: rgba(255,255,255,0.045);
  backdrop-filter: blur(24px) saturate(140%);
  -webkit-backdrop-filter: blur(24px) saturate(140%);
  border: 1px solid rgba(255,255,255,0.08);
  border-radius: 24px;
  box-shadow:
    0 1px 0 rgba(255,255,255,0.06) inset,   /* brillo superior */
    0 12px 40px rgba(0,0,0,0.35);
}
.glass--strong { background: rgba(255,255,255,0.07); backdrop-filter: blur(32px) saturate(150%); }
.glass--solid  { background: #12131C; backdrop-filter: none; } /* fallback */
\`\`\`

Niveles de elevación:

| Nivel | Uso | Fondo | Blur |
|---|---|---|---|
| 0 | Fondo de página | \`--bg-0\` + glows | — |
| 1 | Tarjetas, paneles | \`rgba(255,255,255,0.045)\` | 24px |
| 2 | Sidebar, barra superior, popovers | \`rgba(255,255,255,0.07)\` | 32px |
| 3 | Modales, menús, tooltips | \`rgba(18,19,28,0.85)\` | 40px |

**Accesibilidad / rendimiento:** respetar \`prefers-reduced-transparency\` y navegadores sin \`backdrop-filter\` usando \`.glass--solid\`. Verificar contraste mínimo 4.5:1 del texto sobre el glass más claro.

---

## 4. Tipografía

Familia: **Inter** (400, 500, 600). Fallback: \`system-ui, -apple-system, "Segoe UI", sans-serif\`. Cifras con \`font-variant-numeric: tabular-nums\`.

La guía de referencia muestra la escala a tamaño de presentación (56 / 32 / 22 / 14). Para la app se recomienda esta escala de interfaz:

| Rol | Peso | Tamaño / interlineado | Notas |
|---|---|---|---|
| Display (saludo de página) | 600 | 40–56 / 1.05 | \`letter-spacing: -0.03em\` |
| H1 | 600 | 32 / 1.15 | \`-0.02em\` |
| H2 (título de tarjeta) | 600 | 20 / 1.25 | |
| Cifra KPI | 600 | 32–40 / 1.0 | tabular-nums |
| Cuerpo | 500 | 15 / 1.5 | \`--text-2\` |
| Cuerpo pequeño | 400 | 14 / 1.45 | |
| Caption / etiqueta | 400 | 12–13 / 1.4 | \`--text-3\` |
| Overline | 600 | 11 / 1.2 | MAYÚSCULAS, \`letter-spacing: 0.08em\` |

Detalle de la referencia: el saludo del encabezado lleva un **subrayado grueso en el color de acento** (\`text-decoration: underline; text-decoration-color: var(--primary); text-decoration-thickness: 4px; text-underline-offset: 6px\`).

---

## 5. Espaciado, radios y grilla

- Unidad base: **4px**. Escala: 4, 8, 12, 16, 20, 24, 32, 40, 48.
- Padding de tarjetas: 20–24px. Gap entre tarjetas: 16px.
- Radios: \`--r-sm 10px\` (badges internos, inputs pequeños) · \`--r-md 14px\` (inputs, botones cuadrados, filas) · \`--r-lg 24px\` (tarjetas) · \`--r-pill 999px\` (botones primarios, search, chips, avatar).
- Contenedor de contenido: ancho máximo 1440px, márgenes laterales 24–32px.
- Grilla desktop de 12 columnas:
  - Fila 1: 4 tarjetas KPI (3 col c/u).
  - Fila 2: gráfico principal (8 col) + panel lateral de lista con barras de progreso (4 col).
  - Fila 3: 3 tarjetas inferiores (4 col c/u).
- Tablet: 2 columnas. Móvil: 1 columna.

---

## 6. Layout general

\`\`\`
┌─────────┬──────────────────────────────────────────────┐
│         │  Top bar: breadcrumb · search ⌘K · acciones │
│ Sidebar ├──────────────────────────────────────────────┤
│ (glass) │  Encabezado de página (saludo + meta + CTA)  │
│         │  Grilla de contenido                         │
└─────────┴──────────────────────────────────────────────┘
\`\`\`

- La sidebar y la barra superior **flotan** sobre el fondo ambiental con un margen de 12–16px respecto al borde de la ventana y radio grande (24px), como en la referencia.
- El contenido hace scroll; la sidebar permanece fija.

---

## 7. Sidebar (componente clave)

Dos estados, ambos **totalmente funcionales** (navegación completa en cualquiera de ellos).

### 7.1 Expandida (por defecto en desktop)
- Ancho **264px**. Glass nivel 2, radio 24px, padding 16px.
- Estructura de arriba hacia abajo:
  1. **Marca**: logo cuadrado redondeado (32px, relleno \`--primary\`) + nombre de la app + botón de colapsar (icono de panel, 36px, glass).
  2. **Grupo "Espacios"** (overline \`--text-3\`): selector de espacios/cuentas como filas con icono en cuadrado tintado (cada uno con un tinte distinto) + título + subtítulo pequeño. El activo tiene fondo \`rgba(255,255,255,0.08)\`.
  3. **Grupo "Menú"** (overline): ítems de navegación.
  4. Separador y **ítems utilitarios** (ajustes, ayuda, toggle de modo oscuro con switch).
  5. **Perfil** al fondo: avatar 36px + nombre + rol + chevron.
- **Ítem de navegación**: altura 40px, radio 14px, icono 20px (trazo 1.5px) + etiqueta + badge opcional (contador a la derecha).
  - Reposo: texto \`--text-3\`, sin fondo.
  - Hover: fondo \`rgba(255,255,255,0.05)\`, texto \`--text-1\`.
  - **Activo**: fondo \`rgba(255,255,255,0.09)\` + borde \`rgba(255,255,255,0.10)\`, texto \`--text-1\`, y un **punto de 6px** en \`--primary-soft\` junto a la etiqueta (equivale al punto de "Overview" de la referencia). Alternativa: barra vertical de 3px en \`--primary\` a la izquierda.
  - Badge contador: pill 20px de alto, fondo \`rgba(255,255,255,0.08)\`, texto \`--text-2\` 12px.

### 7.2 Colapsada (modo compacto)
- Ancho **76px**. Solo iconos centrados en botones de 44×44px, radio 14px.
- El selector de espacios se reduce a un **grupo de mini-iconos apilados** en la parte superior.
- Los contadores pasan a **punto de notificación** de 8px en la esquina del icono (color \`--primary\` o \`--danger\` según urgencia), como en la referencia.
- El ítem activo conserva fondo glass + borde + glow púrpura sutil: \`box-shadow: 0 0 0 1px rgba(138,98,255,0.35), 0 0 24px rgba(138,98,255,0.25)\`.
- **Tooltip** al hacer hover o foco: glass nivel 3, radio 10px, aparece a la derecha con 8px de separación, 120ms de retardo.
- El perfil se reduce al avatar; el toggle de tema a un icono.

### 7.3 Comportamiento
- **Click en el botón de panel** alterna expandida ↔ colapsada. También con atajo (\`[\`).
- Opcional: **click en cualquier zona vacía de la sidebar colapsada la expande** (a pedido del usuario: "que se expanda al hacerle click").
- Transición: \`width 260ms cubic-bezier(0.32, 0.72, 0, 1)\`. Las etiquetas hacen fade + \`translateX(-4px)\` en 160ms y se ocultan antes de que termine el estrechamiento para evitar saltos de texto.
- El estado se **persiste** entre sesiones.
- Breakpoints: ≥1280px expandida por defecto · 768–1279px colapsada por defecto · <768px la sidebar pasa a **drawer** deslizable con scrim \`rgba(0,0,0,0.5)\` (o barra inferior de 5 iconos).
- Accesibilidad: \`<nav aria-label>\`, \`aria-current="page"\` en el activo, \`aria-expanded\` en el botón de colapsar, anillo de foco visible, navegación por teclado con flechas.

---

## 8. Barra superior

- Altura 56px, glass nivel 2, radio pill/24px, flotante.
- Izquierda: **breadcrumb** (\`Espacio / Sección\`), el último segmento en \`--text-1\`, el resto en \`--text-3\`.
- Centro-derecha: **campo de búsqueda pill** de 280–320px con icono de lupa, placeholder en \`--text-3\` y atajo \`⌘K\` en una keycap pequeña a la derecha.
- Derecha: botón primario pill "+ Nuevo", **campana** (botón circular 40px) con badge numérico en \`--danger\`/\`--primary\`, y avatar circular de 40px.

---

## 9. Componentes

### 9.1 Botones
| Variante | Estilo |
|---|---|
| **Primario** | Pill (o radio 14px), fondo \`--primary\`, texto blanco 500, altura 40px, \`padding: 0 18px\`. Sombra \`0 6px 20px rgba(138,98,255,0.35)\`. Hover: \`--primary-soft\`. Active: \`scale(0.98)\`. |
| **Secundario** | Fondo \`rgba(138,98,255,0.14)\`, texto \`--primary-soft\`, borde \`rgba(138,98,255,0.22)\`. |
| **Ghost / glass** | Fondo \`rgba(255,255,255,0.05)\`, borde \`rgba(255,255,255,0.08)\`, texto \`--text-2\`. Para "Reportes", iconos de la barra superior. |
| **Destructivo** | Fondo \`rgba(239,68,68,0.14)\`, texto \`#F87171\`. |
| **Icono** | 40×40, circular o radio 14px. |

Foco visible en todos: \`outline: 2px solid var(--primary-soft); outline-offset: 2px\`.

### 9.2 Input
- Fondo \`--bg-1\` (o \`rgba(0,0,0,0.25)\` sobre glass), borde \`rgba(255,255,255,0.08)\`, radio 14px, altura 44px, icono a la izquierda en \`--text-3\`.
- Focus: borde \`--primary\` + \`box-shadow: 0 0 0 4px rgba(138,98,255,0.18)\`.
- Error: borde \`--danger\` + mensaje 12px en \`#F87171\`.

### 9.3 Badges de estado
Pill 24px de alto, **punto de 6px + etiqueta**, fondo del color al 14–16%, texto del color:
- Positivo: fondo \`rgba(124,255,178,0.14)\`, texto \`--success\`.
- Pendiente: fondo \`rgba(245,158,11,0.14)\`, texto \`--warning\`.
- Vencido/Error: fondo \`rgba(239,68,68,0.14)\`, texto \`#F87171\`.
- Info: fondo \`rgba(96,165,250,0.14)\`, texto \`--info\`.

### 9.4 Toggle / Switch
Pista 40×22px, radio pill. **On**: \`--primary\` con thumb blanco. **Off**: \`rgba(255,255,255,0.12)\` con thumb \`rgba(255,255,255,0.6)\`. Transición 180ms.

### 9.5 Tarjeta KPI
- Glass (neutro o tintado), padding 20px, radio 24px, altura ~120px.
- Contenido: etiqueta (caption \`--text-3\`) → **cifra** (32–40px semibold) → línea secundaria (caption) o barra de progreso fina de 6px (pista \`rgba(255,255,255,0.12)\`, relleno blanco o \`--primary\`).
- **Indicador de variación**: flecha + porcentaje en \`--success\` (sube) o \`#F87171\` (baja), 13px, semibold.
- Hover: \`translateY(-2px)\` + borde un poco más claro, 200ms.

### 9.6 Gráfico de barras (ancho completo)
- Contenedor glass con título H2 + subtítulo caption + leyenda a la derecha (cuadradito de color + etiqueta; la meta como línea punteada).
- **Barras**: ancho uniforme, radio superior 10px, color \`--info\` al ~85% de opacidad. Cada barra se apoya sobre una **columna de fondo** \`rgba(255,255,255,0.04)\` de altura completa (patrón de la referencia).
- **Barra actual/destacada**: relleno degradado \`--primary-soft → --primary\` (en lugar del lima de la referencia) con glow \`0 0 24px rgba(138,98,255,0.45)\`.
- **Línea de meta**: punteada, \`rgba(255,255,255,0.35)\`, 1px, con etiqueta en la leyenda.
- Ejes: solo etiquetas inferiores en \`--text-4\` 12px; sin líneas de grilla verticales, horizontales a \`rgba(255,255,255,0.04)\` como máximo.
- Tooltip: glass nivel 3 con cifra grande y etiqueta.

### 9.7 Gráfico de línea / área
- Línea de 2px en \`--primary-soft\`, relleno degradado vertical \`rgba(138,98,255,0.35) → transparent\`. Curva suave (monotone). Punto activo con halo de 8px.

### 9.8 Lista con barras de progreso (panel lateral)
- Cada fila: título + subtítulo pequeño a la izquierda · **barra de 8px** pill en el centro · valor alineado a la derecha en \`--text-1\` semibold.
- Colores de barra por categoría (\`--info\`, \`--warning\`, \`--primary-soft\`, \`--success\`, neutro \`rgba(255,255,255,0.25)\`).
- Separación vertical 16px. Encabezado con enlace de acción ("Tablero ›") a la derecha en \`--text-3\`.

### 9.9 Filas de lista/tabla (estilo tarjetas)
- Altura 52px, fondo \`--bg-1\`, radio 14px, borde \`rgba(255,255,255,0.05)\`, separación 8px entre filas.
- Orden: ID en \`--primary-soft\` 12px (monoespaciado opcional) · nombre en \`--text-1\` · importe alineado a la derecha tabular · badge de estado.
- Hover: fondo \`rgba(255,255,255,0.05)\`.

### 9.10 Notificación / aviso en línea
- Fila glass con icono circular 36px (\`!\` en \`--warning\`/\`--danger\` sobre fondo del color al 15%), título 14px, subtítulo caption, marca de tiempo a la derecha en \`--text-3\`.

### 9.11 Chips y avatares
- Chip: pill 28px, fondo \`rgba(255,255,255,0.06)\`, texto 13px.
- Avatar: circular, borde \`rgba(255,255,255,0.12)\`; en grupos se solapan -8px.

### 9.12 Estados vacíos y de carga
- Skeleton: bloques \`rgba(255,255,255,0.06)\` con shimmer lento (1.6s).
- Vacío: icono 40px en \`--text-4\`, título H2, texto \`--text-3\`, un botón secundario.

---

## 10. Encabezado de página
- Saludo/título en Display con subrayado de acento (ver §4).
- Debajo, línea meta en caption \`--text-3\` separada por puntos medios.
- A la derecha: botón ghost + botón primario pill.

---

## 11. Iconografía
- Estilo línea, trazo 1.5px, esquinas redondeadas, 20px en navegación y 16px en botones. Compatible con Lucide / Phosphor (regular).
- Color por defecto \`--text-3\`; activo \`--text-1\`; acento \`--primary-soft\`.
- Iconos de espacios/categorías sobre un cuadrado redondeado de 32–36px con el tinte correspondiente (§2.3).

---

## 12. Movimiento
| Elemento | Duración | Curva |
|---|---|---|
| Hover/pressed | 120–180ms | \`ease-out\` |
| Sidebar | 260ms | \`cubic-bezier(0.32, 0.72, 0, 1)\` |
| Entrada de tarjetas | 400ms, escalonado 40ms | \`ease-out\`, fade + \`translateY(8px)\` |
| Barras de gráfico | 600ms, escalonado 30ms | crecen desde la base |
| Modales | 220ms | fade + \`scale(0.97 → 1)\` |

Respetar \`prefers-reduced-motion\` (desactivar escalonados y transformaciones).

---

## 13. Tokens (CSS)

\`\`\`css
:root {
  /* color */
  --bg-0:#0D0E17; --bg-1:#12131C;
  --primary:#8A62FF; --primary-soft:#A78BFA;
  --success:#7CFFB2; --warning:#F59E0B; --danger:#EF4444; --info:#60A5FA;
  --text-1:#fff; --text-2:rgba(255,255,255,.72); --text-3:rgba(255,255,255,.48); --text-4:rgba(255,255,255,.28);
  --line:rgba(255,255,255,.08);

  /* glass */
  --glass-1:rgba(255,255,255,.045);
  --glass-2:rgba(255,255,255,.07);
  --glass-3:rgba(18,19,28,.85);
  --blur-1:24px; --blur-2:32px; --blur-3:40px;

  /* forma */
  --r-sm:10px; --r-md:14px; --r-lg:24px; --r-pill:999px;

  /* layout */
  --sidebar-w:264px; --sidebar-w-collapsed:76px; --topbar-h:56px;

  /* tipografía */
  --font:"Inter",system-ui,-apple-system,"Segoe UI",sans-serif;
}
\`\`\`

Extensión sugerida para Tailwind:

\`\`\`js
theme: { extend: {
  colors: { bg0:'#0D0E17', bg1:'#12131C', primary:{DEFAULT:'#8A62FF', soft:'#A78BFA'},
            success:'#7CFFB2', warning:'#F59E0B', danger:'#EF4444', info:'#60A5FA' },
  borderRadius: { lg:'24px', md:'14px', sm:'10px' },
  fontFamily: { sans:['Inter','system-ui','sans-serif'] },
}}
\`\`\`

---

## 14. Reglas rápidas (Do / Don't)

**Hacer**
- Usar el púrpura solo para acciones, selección y énfasis.
- Mantener bordes de 1px casi invisibles; la separación la da el glass y el espacio.
- Dar mucho aire: tarjetas con 16px de gap y 20–24px de padding.
- Reservar verde/ámbar/rojo para estados, nunca como decoración.

**Evitar**
- Más de 2–3 niveles de blur apilados (costo de rendimiento y pérdida de claridad).
- Texto gris sobre glass tintado sin comprobar contraste.
- Sombras duras o bordes opacos blancos.
- Mezclar el púrpura con otros acentos fuertes (el lima de la referencia original **se reemplaza** por púrpura + verde menta semántico).

---

## 15. Checklist de implementación
- [ ] Fondo ambiental con glows aplicado a nivel \`body\`.
- [ ] Clase \`.glass\` con fallback sólido.
- [ ] Sidebar con ambos estados, persistencia, tooltips y drawer móvil.
- [ ] Barra superior con búsqueda \`⌘K\`.
- [ ] Tokens de color/forma/tipografía centralizados.
- [ ] Revisión de contraste y de \`prefers-reduced-*\`.
