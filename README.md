# Weekly Planner

A Rust program that generates an HTML weekly planner based on JSON input data.

## 📋 Table of Contents

- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
- [Usage](#-usage)
- [Project Structure](#-project-structure)
- [Configuration](#-configuration)
- [Dependencies](#-dependencies)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgments](#-acknowledgments)

## ✨ Features

- 📅 Reads plan data from a JSON file
- 🖥️ Generates an HTML representation of the weekly plan
- ⏰ Customizable time slots and activities
- 🎨 Styled HTML output for easy viewing

## 🛠️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

## 📥 Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/weekly-planner.git
   cd weekly-planner
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

## 🚀 Usage

1. Prepare your plan data in JSON format and save it as `data/plan.json`. The format should match the structure expected by the `WeekPlan` struct.

2. Run the program:
   ```sh
   cargo run --release
   ```

3. The generated HTML file will be saved as `output/week_plan.html`.

## 📁 Project Structure

```
weekly-planner/
├── src/
│   └── main.rs
├── data/
│   └── plan.json
├── output/
│   └── week_plan.html
├── Cargo.toml
└── README.md
```

## ⚙️ Configuration

The `plan.json` file should follow this structure:

```json
{
  "start": "08:30",
  "slot_duration": 90,
  "slots": 7,
  "plan": [
    {
      "weekday": "Monday",
      "time": "10:00",
      "activity": "Team Meeting"
    },
    // ... more activities
  ]
}
```

## 📚 Dependencies

This project uses the following crates:

- `serde`: For JSON serialization and deserialization
- `serde_json`: For working with JSON data

## 🤝 Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Please make sure to update tests as appropriate.

---

<div align="center">
Made with ❤️ by LoZack19
</div>
