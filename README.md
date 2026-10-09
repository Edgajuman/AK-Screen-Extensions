# AK Screen · Extensiones

Widgets, herramientas y efectos creados por la comunidad para AK Screen 2.3. El catálogo publica **versiones revisadas y firmadas**. El código del editor permanece privado.

**[Explorar catálogo](https://edgajuman.github.io/AK-Screen-Extensions/)** · [Descargas](https://github.com/Edgajuman/AKScreen-Downloads/releases) · [Temas](https://github.com/Edgajuman/AK-Screen-Themes)

## Instalar

En el editor: **Herramientas → Extensiones → Actualizar catálogo**. Selecciona una versión, instala y revisa los permisos antes de activarla. También puedes importar un archivo `.akext`. Los paquetes locales no pueden reemplazar un ID verificado.

Cada usuario guarda los paquetes en su carpeta personal de AK Screen, dentro de `Extensiones/packages/ID/versión/`. Sus notas, chats y ajustes se conservan aparte, en `Extensiones/data/ID/`, incluso al actualizar o desinstalar. Las claves de API se guardan en el almacén de credenciales del sistema operativo. No se incluyen claves en los paquetes.

## Colección inicial de Edgajuman

| Extensión | Tipo | Función |
|---|---|---|
| Notas de producción | Widget | Notas persistentes por usuario |
| Checklist de entrega | Widget | Revisar tareas antes de exportar |
| Títulos de movimiento | Herramienta | Insertar textos con animación |
| Cámara de composición | Herramienta | Ajustes de cámara 2D/3D |
| Acabado | Herramienta | Presets de efectos |
| Chat AI | Asistente | OpenAI, OpenRouter, Gemini, Claude y Grok; claves propias y contexto opcional |

Chat AI necesita conexión y una cuenta del proveedor elegido. Es un asistente de consulta: no ejecuta acciones en el editor. El resto de las extensiones iniciales no necesita red.

## Contribuir

1. Haz un **fork** de este repositorio.
2. Añade `extensions/autor/nombre/1.0.0/manifest.json` y sus componentes. Consulta [el SDK](sdk/README.md) y los ejemplos.
3. Cada archivo debe declararse con tamaño y SHA-256. Para una actualización, conserva el ID y añade una carpeta SemVer nueva; no alteres versiones publicadas.
4. Abre un **pull request** con descripción, capturas propias, permisos justificados, licencia y cambios. El mantenedor revisa el código y aprueba la publicación.
5. Tras aceptar el PR, el publicador genera los paquetes y firma el catálogo. El editor mostrará la nueva versión.

Los componentes WebAssembly no disponen de WASI, archivos, procesos, sockets ni bibliotecas nativas. La red pasa por un broker con dominios HTTPS exactos y permiso del usuario; las operaciones del proyecto pasan por comandos con deshacer. El aislamiento reduce riesgos y requiere mantenimiento continuo. No se aceptan DLL, ejecutables, scripts externos ni permisos innecesarios.

## Versiones y seguridad

El catálogo conserva versiones anteriores para poder elegirlas. Las actualizaciones no borran datos; se realiza una copia antes de activar otra versión. Una reversión no transforma automáticamente un esquema de datos nuevo en uno antiguo: tu extensión debe contemplar compatibilidad o migración.

Una versión insegura puede retirarse mediante `revoked.json`, usando `ID@versión`. Los usuarios recibirán su desactivación al consultar el catálogo. Para reportar un problema sin publicar detalles de explotación, utiliza un aviso privado de seguridad en GitHub.

MIT · Edgajuman
