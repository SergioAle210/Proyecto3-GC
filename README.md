# Rust Graphics Renderer

Este proyecto es un renderizador 3D simple basado en Rust. Utiliza la librería `minifb` para crear una ventana y `nalgebra_glm` para manejar las matemáticas de las transformaciones 3D. El renderizador puede cargar modelos 3D en formato `.obj` y aplicar transformaciones como traslación, rotación y escalado. También implementa un sistema de iluminación básica, incluyendo una luz direccional y luz ambiente.

## Características

- Renderizado de modelos 3D a partir de archivos `.obj`.
- Iluminación estática direccional y ambiental.
- Transformaciones como rotación, escalado y traslación.
- Nave espacial interactiva con controles personalizados.
- Simulación del Sistema Solar con:
  - 6 planetas con shaders únicos.
  - Una luna orbitando la Tierra.
  - Un cometa con trayectoria dinámica.
  - Representación de órbitas planetarias.
  - Skybox para el fondo con estrellas.
- Interacción con la cámara para orbitar alrededor de los modelos.
- Control de zoom mediante teclado y mouse.

## Requisitos

Para ejecutar este proyecto, necesitarás tener instalados los siguientes elementos:

- Rust: [Instalación](https://www.rust-lang.org/tools/install)
- Cargo (viene con Rust)

Además, el proyecto utiliza las siguientes dependencias, que se especifican en el archivo `Cargo.toml`:

```toml
minifb = "0.27.0"
nalgebra-glm = "0.19.0"
tobj = "4.0.2"
fastnoise-lite = "0.8.0"
```

#### Estructura del Proyecto

- `camera.rs`: Implementa la cámara y sus movimientos, incluyendo zoom y rotaciones.
- `color.rs`: Define la estructura de color y operaciones aritméticas con colores.
- `fragment.rs`: Maneja los fragmentos del pipeline gráfico.
- `framebuffer.rs`: Implementa el framebuffer, donde se almacenan los píxeles renderizados.
- `line.rs`: Algoritmo para dibujar líneas.
- `obj.rs`: Carga archivos `.obj` y los convierte en un array de vértices.
- `shaders.rs`: Define los shaders para personalizar la apariencia de los objetos.
- `texture.rs`: Carga y gestiona texturas aplicadas a los modelos.
- `triangle.rs`: Implementa la rasterización de triángulos y la aplicación de iluminación.
- `vertex.rs`: Define la estructura de un vértice.

## Instrucciones de Uso

### 1. Clonar el repositorio

Primero, clona este repositorio en tu máquina local:

```bash
git clone https://github.com/SergioAle210/Proyecto3-GC.git
cd Proyecto3-GC
```

### 2. Compilar el proyecto

Asegúrate de que tienes Rust instalado en tu máquina. Puedes compilar el proyecto usando Cargo:

```bash
cargo build
```

### 3. Ejecutar el proyecto

#### En Windows

Para ejecutar el renderizador en Windows, simplemente utiliza el siguiente comando en la terminal (cmd):

```bash
./run.bat
```

#### En Linux

Para ejecutar el renderizador en Linux, usa el siguiente comando en tu terminal:

```bash
./run.sh
```

Ambos archivos (tanto `run.bat` como `run.sh`) cambian automáticamente al directorio donde están ubicados y ejecutan el proyecto usando `cargo run --release`.

Esto abrirá una ventana donde se renderizarán los modelos 3D y se interactuará con el entorno.

### 4. Controles de cámara

#### Controles

- `ESC`: Salir del programa.

#### Controles de la nave

- `W`: Rotar hacia arriba.
- `S`: Rotar hacia abajo.
- `A`: Rotar a la izquierda.
- `D`: Rotar a la derecha.
- **Flecha `↑`:** Mover la nave hacia adelante.
- **Flecha `↓`:** Mover la nave hacia atrás.
- **Clic derecho**: Permite controlar la orientación de la nave moviendo el mouse.
- **Scroll del mouse**: Ajusta la posición relativa de la cámara respecto a la nave (zoom in/out).

### 5. Modelos 3D

Los modelos 3D deben estar en la carpeta `assets/models/`. Puedes cambiar los modelos cargando diferentes archivos `.obj` en el código fuente.

```rust
let obj = Obj::load("assets/models/nuevo_modelo.obj").expect("Failed to load obj");
```

# Ejemplo de Uso

El siguiente es un ejemplo visual de lo que verás al ejecutar el programa:

- Un modelo `tiefighter` y un `charizard` rotan constantemente.
- La luz direccional estática ilumina el modelo.

![](https://github.com/SergioAle210/Proyecto3-GC/blob/main/assets/videos/Proyecto3.gif)

# Laboratorio 4 - Shaders

El siguiente gif es un ejemplo visual de como se ve el programa luego de ejecutar los cambios del nuevo programa:

- 7 Planetas donde cada uno tiene un shader diferente
- 1 luna que orbita alrededor del planeta tierra
- 1 cometa que esta orbitando por todo el sistema solar (con forma de árbol de navidad, porque ya se acerca la fecha)

![](https://github.com/SergioAle210/Proyecto3-GC/blob/main/assets/videos/Laboratorio4.gif)

# Proyecto final - Space Travel

El siguiente gif es un ejemplo visual de cómo se ve el programa luego de ejecutar los cambios del nuevo programa:

- 6 Planetas donde cada uno tiene un shader diferente.
- 1 Luna que orbita alrededor del planeta Tierra.
- 1 Sol.
- 1 Cometa que está orbitando por todo el sistema solar (con forma de árbol de Navidad, porque ya se acerca la fecha).
- Cuenta con las órbitas de cada planeta.
- Representación de estrellas en el fondo mediante un Skybox.
- Nave interactiva que se mueve según las teclas presionadas.

![](https://github.com/SergioAle210/Proyecto3-GC/blob/main/assets/videos/ProyectoFinal.gif)

## Notas importantes:

- La funcionalidad de la nave permite explorar el sistema solar, evitando colisiones con planetas y ajustando la posición automáticamente.
- Los shaders aplicados son configurados dinámicamente para cada objeto en el sistema.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Para más detalles, consulta el archivo `LICENSE`.
