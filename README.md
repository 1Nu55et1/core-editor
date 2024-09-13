# Core Editor

Core es un editor de texto moderno, eficiente y extensible inspirado en editores basados en consola y centrados en el teclado. Sus atajos de teclado están ligeramente basados en Vim, pero con mejoras y características únicas.

## Características principales

- Interfaz de usuario basada en terminal (TUI)
- Modos de edición inspirados en Vim (Normal, Inserción, Comando)
- Navegación y edición eficiente mediante atajos de teclado
- Soporte para múltiples cursores
- Búsqueda y reemplazo de texto
- Deshacer y rehacer cambios
- Personalizable y extensible (soporte futuro para plugins y temas)
- Desarrollado completamente con la biblioteca estándar de Rust, sin dependencias externas

## Instalación

Para instalar Core Editor, asegúrate de tener Rust y Cargo instalados en tu sistema. Luego, sigue estos pasos:

1. Clona el repositorio:
   ```
   git clone https://github.com/tu-usuario/core-editor.git
   cd core-editor
   ```

2. Compila el proyecto:
   ```
   cargo build --release
   ```

3. El ejecutable se encontrará en `target/release/core-editor`

## Uso básico

Para iniciar Core Editor, ejecuta:

```
./target/release/core-editor [nombre_archivo]
```

### Modos principales

1. Modo Normal: El modo predeterminado para navegación y comandos rápidos.
2. Modo Inserción: Para insertar y editar texto.
3. Modo Comando: Para ejecutar comandos más complejos.

### Comandos globales

- `ESC`: Salir del editor
- Flechas de dirección: Mover el cursor
- `Backspace`: Borrar el carácter anterior
- `Delete`: Borrar el carácter actual
- `Home`: Ir al inicio de la línea
- `End`: Ir al final de la línea
- `PageUp`: Mover el cursor 10 líneas hacia arriba
- `PageDown`: Mover el cursor 10 líneas hacia abajo

### Comandos en Modo Normal

Algunos comandos útiles en modo normal:

- `h`, `j`, `k`, `l`: Mover el cursor (izquierda, abajo, arriba, derecha)
- `w`: Mover al inicio de la siguiente palabra
- `b`: Mover al inicio de la palabra anterior
- `0`: Ir al inicio de la línea
- `$`: Ir al final de la línea
- `gg`: Ir al inicio del archivo
- `G`: Ir al final del archivo
- `i`: Entrar en modo inserción
- `:`: Entrar en modo comando

Para ver la lista completa de comandos, consulta la sección "Modos" en el archivo de ayuda.

## Personalización

Core está diseñado para ser altamente personalizable. En futuras actualizaciones, se incluirá:

- Soporte para plugins
- Temas personalizables
- Configuración mediante archivo de configuración

## Desarrollo

El proyecto está estructurado de la siguiente manera:

- `src/main.rs`: Punto de entrada de la aplicación
- `src/lib.rs`: Módulo principal de la biblioteca
- `src/ui/`: Módulos relacionados con la interfaz de usuario
- `src/editor/`: Lógica del editor y manejo de buffers
- `src/utils/`: Utilidades y funciones auxiliares

Core Editor está desarrollado completamente utilizando la biblioteca estándar de Rust, lo que garantiza un rendimiento óptimo, seguridad de memoria y portabilidad sin depender de bibliotecas externas.

Para contribuir al proyecto:

1. Haz un fork del repositorio
2. Crea una nueva rama para tu característica (`git checkout -b feature/nueva-caracteristica`)
3. Haz tus cambios y commitea (`git commit -am 'Añade nueva característica'`)
4. Haz push a la rama (`git push origin feature/nueva-caracteristica`)
5. Crea un nuevo Pull Request

## Licencia

Este proyecto está licenciado bajo [insertar tipo de licencia aquí].

## Contacto

Para preguntas, sugerencias o reportar problemas, por favor abre un issue en el repositorio de GitHub.