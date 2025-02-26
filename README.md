# Dioxus Rive Animation Example

A fullstack Dioxus application that demonstrates how to integrate Rive animations into a Dioxus web application with a mobile-first approach.

## Features

- Integration of Rive.js with Dioxus
- Mobile-first responsive design
- Interactive animations
- Fullstack Rust application

## Examples

The project includes several examples:

1. **Main Application**: A Dioxus web application with Rive animations integrated
2. **Standalone HTML Examples**:
   - `rive-example.html`: Basic Rive animation example
   - `rive-android.html`: Android-styled Rive animation
   - `rive-android-mascot.html`: Interactive Android mascot animation

## Project Structure

- `ui/`: Shared UI components
  - `src/rive_animation.rs`: Rive animation component
  - `src/hero.rs`: Hero component that includes the Rive animation
- `web/`: Web platform specific code
  - `public/`: Public assets including standalone HTML examples
  - `public/rive-init.js`: JavaScript for initializing Rive animations

## Getting Started

### Prerequisites

- Rust and Cargo
- Dioxus CLI (`dx`)

### Running the Application

Navigate to the web platform directory:
```bash
cd web
```

And serve the application:
```bash
dx serve
```

## How It Works

The application uses a combination of Rust and JavaScript to integrate Rive animations:

1. The `RiveAnimation` component in Rust creates a canvas element
2. A JavaScript file (`rive-init.js`) is included to load the Rive.js library and initialize the animation
3. The animation is displayed in a responsive container that works well on mobile devices

## License

MIT

## Acknowledgements

- [Dioxus](https://dioxuslabs.com/) - A portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust
- [Rive](https://rive.app/) - Create and ship interactive animations to any platform
