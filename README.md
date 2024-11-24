# **Rusting2D**

**Rusting2D** is a simple 2D engine written in **Rust**, designed as a learning project. Its primary goal is to explore the development of graphical engines using modern tools like **wgpu** and **winit**. This project is under constant evolution, and breaking changes may occur frequently.

---

## **⚠️ Usage Warning**

**Rusting2D is not recommended for production.**  
If you're looking for more robust and mature solutions, consider these alternatives:
- [**Bevy**](https://bevyengine.org/): A modern ECS-based game engine written in Rust.
- [**Good Rust Game Engines**](https://arewegameyet.rs/): A curated list of game engines available in the Rust ecosystem.

Rusting2D is experimental and serves as a platform for learning and experimentation.

---

## **📚 Features**

- **Window and event handling:** Managed with `winit`.
- **Basic 2D rendering:** Leveraging `wgpu` for low-level graphics.
- **Entity system:** Basic support for creating and managing entities.
- **Focus on learning:** Built to understand and explore underlying technologies.

---

## **🚀 How to Use**

### **Installation**

Add Rusting2D as a dependency in your `Cargo.toml` (published on Crates.io):

```toml
[dependencies]
rusting2d = "0.0.1"
```

Alternatively, clone the repository to explore and modify the code directly.

### **Basic Example**

```rust
use rusting2d::{Engine, Entity};

#[tokio::main]
async fn main() {
    // Create the engine with a window.
    let mut engine = Engine::new("My 2D Engine", 800, 600).await;

    // Add a basic entity.
    engine.add_entity(Entity {
        id: 0,
        name: "Player".to_string(),
    });

    // Run the engine.
    engine.run();
}
```

---

### **Tests**

Run tests with:

```bash
cargo test
```

---

## **📖 Documentation**

Generate documentation locally:

```bash
cargo doc --open
```

---

## **🌱 Contributing**

This project is a learning space. If you encounter bugs or have ideas for improvements, feel free to open a **Pull Request** or file an issue.

---

## **📜 License**

**Rusting2D** is available under the **MIT OR Apache-2.0 license**.

---

### **📝 Note**

As the project is in constant evolution, significant changes may be introduced without prior notice. For production-grade projects, it is recommended to use mature alternatives like those mentioned above.

---

## **🔗 Links**

- **Crates.io**: [Rusting2D](https://crates.io/crates/rusting2d)
- **Repository**: [GitHub](https://github.com/your-username/rusting2d)

---